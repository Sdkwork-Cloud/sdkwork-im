import net from 'node:net';
import path from 'node:path';

const DEFAULT_SERVER_HOST = '127.0.0.1';
const DEFAULT_SERVER_PORT = 18079;
const DEFAULT_MAX_SERVER_PORT_ATTEMPTS = 50;
const DEFAULT_RESERVED_SERVER_PORTS = new Set([18080]);

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function normalizePort(value, label = 'port') {
  const normalized = normalizeText(value);
  if (!normalized || !/^\d+$/u.test(normalized)) {
    throw new Error(`${label} must be a TCP port number`);
  }
  const port = Number.parseInt(normalized, 10);
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`${label} must be between 1 and 65535`);
  }
  return port;
}

function parseBindAddr(value) {
  const normalized = normalizeText(value);
  if (!normalized) {
    return undefined;
  }

  const lastColonIndex = normalized.lastIndexOf(':');
  if (lastColonIndex <= 0 || lastColonIndex === normalized.length - 1) {
    throw new Error(`SDKWORK_CHAT_SERVER_BIND must use host:port, got ${normalized}`);
  }

  const host = normalized.slice(0, lastColonIndex).replace(/^\[|\]$/gu, '');
  return {
    host: normalizeText(host) ?? DEFAULT_SERVER_HOST,
    port: normalizePort(normalized.slice(lastColonIndex + 1), 'SDKWORK_CHAT_SERVER_BIND'),
  };
}

function isReservedPort(reservedPorts, port) {
  if (!reservedPorts) {
    return false;
  }
  if (typeof reservedPorts.has === 'function') {
    return reservedPorts.has(port);
  }
  if (Array.isArray(reservedPorts)) {
    return reservedPorts.includes(port);
  }
  return false;
}

export function isTcpPortAvailable(port, host = DEFAULT_SERVER_HOST) {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.unref();
    server.once('error', () => resolve(false));
    server.listen({ host, port }, () => {
      server.close(() => resolve(true));
    });
  });
}

export function createCrawChatServerCargoEnv({
  env = process.env,
  repoRoot,
} = {}) {
  if (!repoRoot) {
    throw new Error('repoRoot is required for craw-chat server cargo env resolution');
  }

  const explicitTargetDir = normalizeText(env.CARGO_TARGET_DIR);
  const cargoTargetDir = explicitTargetDir
    ? path.resolve(repoRoot, explicitTargetDir)
    : path.join(repoRoot, '.runtime', 'cargo-target', 'craw-chat-server-dev');

  return {
    env: {
      ...env,
      CARGO_TARGET_DIR: cargoTargetDir,
    },
    usingDefaultTargetDir: !explicitTargetDir,
  };
}

export async function resolveCrawChatServerBindEnv({
  env = process.env,
  isPortAvailable = isTcpPortAvailable,
  maxAttempts = DEFAULT_MAX_SERVER_PORT_ATTEMPTS,
  reservedPorts = DEFAULT_RESERVED_SERVER_PORTS,
} = {}) {
  const explicitBind = parseBindAddr(env.SDKWORK_CHAT_SERVER_BIND);
  if (explicitBind) {
    const bindAddr = `${explicitBind.host}:${explicitBind.port}`;
    return {
      bindAddr,
      env: {
        ...env,
        SDKWORK_CHAT_SERVER_BIND: bindAddr,
        SDKWORK_CHAT_SERVER_API_BASE_URL: `http://${bindAddr}`,
      },
      portChanged: false,
    };
  }

  for (let offset = 0; offset < maxAttempts; offset += 1) {
    const candidatePort = DEFAULT_SERVER_PORT + offset;
    if (candidatePort > 65535) {
      break;
    }
    if (isReservedPort(reservedPorts, candidatePort)) {
      continue;
    }
    if (await isPortAvailable(candidatePort, DEFAULT_SERVER_HOST)) {
      const bindAddr = `${DEFAULT_SERVER_HOST}:${candidatePort}`;
      return {
        bindAddr,
        env: {
          ...env,
          SDKWORK_CHAT_SERVER_BIND: bindAddr,
          SDKWORK_CHAT_SERVER_API_BASE_URL: `http://${bindAddr}`,
        },
        portChanged: candidatePort !== DEFAULT_SERVER_PORT,
      };
    }
  }

  throw new Error(
    `No available craw-chat server port found from ${DEFAULT_SERVER_PORT} after ${maxAttempts} attempts`,
  );
}
