import { createReadStream } from 'node:fs';
import { access } from 'node:fs/promises';
import http from 'node:http';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { applyWebSecurityHeaders } from '../../../../scripts/dev/web-security-headers.mjs';

const scriptLibRoot = path.dirname(fileURLToPath(import.meta.url));
const scriptsRoot = path.resolve(scriptLibRoot, '..');
const appRoot = path.resolve(scriptsRoot, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

const contentTypes = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.map': 'application/json; charset=utf-8',
};
const vendorMappings = [
  {
    prefix: '/__vendor__/sdkwork-im-sdk/',
    root: path.join(
      repoRoot,
      'sdks',
      'sdkwork-im-sdk',
      'sdkwork-im-sdk-typescript',
      'dist',
    ),
  },
  {
    prefix: '/__vendor__/sdkwork-sdk-common/',
    root: path.join(
      repoRoot,
      'sdks',
      'sdkwork-im-sdk',
      'sdkwork-im-sdk-typescript',
      'generated',
      'server-openapi',
      'node_modules',
      '@sdkwork',
      'sdk-common',
      'dist',
    ),
  },
];

function resolveContentType(filePath) {
  return contentTypes[path.extname(filePath)] ?? 'text/plain; charset=utf-8';
}

export function isPathInsideRoot(rootPath, candidatePath) {
  const resolvedRoot = path.resolve(rootPath);
  const resolvedCandidate = path.resolve(candidatePath);
  const relativePath = path.relative(resolvedRoot, resolvedCandidate);

  return relativePath === ''
    || (
      !relativePath.startsWith('..')
      && !path.isAbsolute(relativePath)
    );
}

function toRelativeRequestPath(urlPath) {
  return urlPath.replace(/^\/+/, '');
}

async function exists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch {
    return false;
  }
}

function resolveVendorCandidate(urlPath) {
  for (const vendorMapping of vendorMappings) {
    if (!urlPath.startsWith(vendorMapping.prefix)) {
      continue;
    }

    const relativePath = urlPath.slice(vendorMapping.prefix.length) || 'index.js';
    const safeRoot = path.resolve(vendorMapping.root);
    const candidate = path.resolve(safeRoot, relativePath);
    return isPathInsideRoot(safeRoot, candidate)
      ? candidate
      : path.join(safeRoot, 'index.js');
  }

  return null;
}

function sendNotFound(response, target) {
  response.statusCode = 404;
  applyWebSecurityHeaders(response);
  response.setHeader('Content-Type', 'text/plain; charset=utf-8');
  response.end(`Not found: ${target}`);
}

export function resolvePreviewRoot(root = '.') {
  return path.resolve(appRoot, root);
}

export function createPreviewServer({ root = '.', port = 4176 } = {}) {
  const resolvedRoot = resolvePreviewRoot(root);

  return http.createServer(async (request, response) => {
    const url = new URL(request.url ?? '/', `http://127.0.0.1:${port}`);
    const target = url.pathname === '/' ? '/index.html' : url.pathname;
    const vendorCandidate = resolveVendorCandidate(target);

    if (vendorCandidate) {
      if (!await exists(vendorCandidate)) {
        sendNotFound(response, target);
        return;
      }

      applyWebSecurityHeaders(response);
      response.setHeader('Content-Type', resolveContentType(vendorCandidate));
      createReadStream(vendorCandidate).pipe(response);
      return;
    }

    const candidate = path.resolve(resolvedRoot, toRelativeRequestPath(target));
    const safeCandidate =
      isPathInsideRoot(resolvedRoot, candidate)
        ? candidate
        : path.join(resolvedRoot, 'index.html');
    const filePath =
      await exists(safeCandidate) ? safeCandidate : path.join(resolvedRoot, 'index.html');

    applyWebSecurityHeaders(response);
    response.setHeader('Content-Type', resolveContentType(filePath));
    createReadStream(filePath).pipe(response);
  });
}

export function startPreviewServer({ root = '.', port = 4176, host = '0.0.0.0' } = {}) {
  const server = createPreviewServer({ root, port });

  return new Promise((resolve, reject) => {
    server.once('error', reject);
    server.listen(port, host, () => {
      server.removeListener('error', reject);
      const address = server.address();
      const resolvedPort = typeof address === 'object' && address ? address.port : port;
      resolve({ server, port: resolvedPort });
    });
  });
}
