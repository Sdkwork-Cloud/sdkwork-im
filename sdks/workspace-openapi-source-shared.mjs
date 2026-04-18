import { spawn } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

export function failOpenApiSource({ prefix, message }) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

export function parseOpenApiSourceArgs(
  argv,
  {
    prefix,
    allowPreferDerived = false,
    allowTargetLanguage = false,
  },
) {
  const parsed = {
    base: '',
    derived: '',
    preferDerived: false,
    targetLanguage: '',
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--base') {
      parsed.base = (argv[index + 1] || '').trim();
      index += 1;
      continue;
    }
    if (current === '--derived') {
      parsed.derived = (argv[index + 1] || '').trim();
      index += 1;
      continue;
    }
    if (current === '--prefer-derived') {
      if (!allowPreferDerived) {
        failOpenApiSource({ prefix, message: `Unknown argument: ${current}` });
      }
      parsed.preferDerived = true;
      continue;
    }
    if (current === '--target-language') {
      if (!allowTargetLanguage) {
        failOpenApiSource({ prefix, message: `Unknown argument: ${current}` });
      }
      parsed.targetLanguage = (argv[index + 1] || '').trim().toLowerCase();
      index += 1;
      continue;
    }
    failOpenApiSource({ prefix, message: `Unknown argument: ${current}` });
  }

  if (!parsed.base) {
    failOpenApiSource({ prefix, message: 'Missing required argument: --base' });
  }
  if (!parsed.derived) {
    failOpenApiSource({ prefix, message: 'Missing required argument: --derived' });
  }

  return parsed;
}

export function parseFetchOpenApiSourceArgs(
  argv,
  {
    prefix,
    defaultUrl,
    defaultTimeoutMs = 60_000,
    defaultLaunchRuntime = true,
  },
) {
  const parsed = {
    url: defaultUrl,
    authority: '',
    timeoutMs: defaultTimeoutMs,
    launchRuntime: defaultLaunchRuntime,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--url') {
      parsed.url = (argv[index + 1] || '').trim();
      index += 1;
      continue;
    }
    if (current === '--authority') {
      parsed.authority = (argv[index + 1] || '').trim();
      index += 1;
      continue;
    }
    if (current === '--timeout-ms') {
      parsed.timeoutMs = Number.parseInt(argv[index + 1] || '', 10);
      index += 1;
      continue;
    }
    if (current === '--skip-launch') {
      parsed.launchRuntime = false;
      continue;
    }
    failOpenApiSource({ prefix, message: `Unknown argument: ${current}` });
  }

  if (!parsed.url) {
    failOpenApiSource({ prefix, message: 'Missing required runtime OpenAPI URL.' });
  }
  if (!Number.isFinite(parsed.timeoutMs) || parsed.timeoutMs < 1_000) {
    failOpenApiSource({ prefix, message: `Invalid --timeout-ms value: ${parsed.timeoutMs}` });
  }

  return parsed;
}

export function cloneOpenApiJson(value) {
  return JSON.parse(JSON.stringify(value));
}

export async function fetchOpenApiDocument(url, timeoutMs) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetch(url, {
      method: 'GET',
      headers: {
        accept: 'application/json',
      },
      signal: controller.signal,
    });
    if (!response.ok) {
      throw new Error(`OpenAPI endpoint returned HTTP ${response.status}`);
    }
    return await response.json();
  } finally {
    clearTimeout(timer);
  }
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function waitForRuntimeOpenApi(
  url,
  timeoutMs,
  {
    requestTimeoutMs = 3_000,
    retryDelayMs = 500,
  } = {},
) {
  const startedAt = Date.now();
  let lastError = null;

  while (Date.now() - startedAt < timeoutMs) {
    try {
      return await fetchOpenApiDocument(url, Math.min(requestTimeoutMs, timeoutMs));
    } catch (error) {
      lastError = error;
      await sleep(retryDelayMs);
    }
  }

  throw lastError ?? new Error(`Timed out waiting for runtime OpenAPI at ${url}`);
}

export function startRuntimeProcess({
  command,
  args,
  cwd,
  stdio = ['ignore', 'inherit', 'inherit'],
  windowsHide = true,
  describeFailure,
}) {
  const child = spawn(command, args, {
    cwd,
    stdio,
    windowsHide,
  });

  return {
    child,
    describeFailure: typeof describeFailure === 'function' ? describeFailure : () => '',
  };
}

export async function stopRuntimeProcess(child, { gracePeriodMs = 3_000 } = {}) {
  if (!child || child.exitCode !== null) {
    return;
  }

  child.kill();
  await Promise.race([
    new Promise((resolve) => child.once('exit', resolve)),
    sleep(gracePeriodMs),
  ]);
}

export function assertOpenApi3Document({
  prefix,
  document,
  sourceLabel,
}) {
  if (!document || typeof document !== 'object' || Array.isArray(document)) {
    failOpenApiSource({
      prefix,
      message: `${sourceLabel} did not decode to an object.`,
    });
  }
  if (typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    failOpenApiSource({
      prefix,
      message: `${sourceLabel} is not an OpenAPI 3.x document.`,
    });
  }
}

export function loadOpenApiDocument({
  prefix,
  filePath,
  yaml,
}) {
  if (!existsSync(filePath)) {
    failOpenApiSource({ prefix, message: `OpenAPI file not found: ${filePath}` });
  }

  const raw = readFileSync(filePath, 'utf8');
  const trimmed = raw.trim();
  if (!trimmed) {
    failOpenApiSource({ prefix, message: `OpenAPI file is empty: ${filePath}` });
  }

  const document = trimmed.startsWith('{') || trimmed.startsWith('[')
    ? JSON.parse(trimmed)
    : yaml.load(raw);

  assertOpenApi3Document({
    prefix,
    document,
    sourceLabel: `OpenAPI file ${filePath}`,
  });

  return document;
}

export function writeOpenApiYamlDocument({
  filePath,
  document,
  yaml,
}) {
  const nextContents = yaml.dump(document, {
    noRefs: true,
    sortKeys: false,
    lineWidth: 120,
  });

  if (existsSync(filePath) && readFileSync(filePath, 'utf8') === nextContents) {
    return;
  }

  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, nextContents, 'utf8');
}

export function sortKeysDeep(value) {
  if (Array.isArray(value)) {
    return value.map((item) => sortKeysDeep(item));
  }
  if (!value || typeof value !== 'object') {
    return value;
  }

  const next = {};
  for (const key of Object.keys(value).sort()) {
    next[key] = sortKeysDeep(value[key]);
  }
  return next;
}
