import { access, copyFile, mkdir, readFile, readdir, rm, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptLibRoot = path.dirname(fileURLToPath(import.meta.url));
const scriptsRoot = path.resolve(scriptLibRoot, '..');
const projectRoot = path.resolve(scriptsRoot, '..');
const distRoot = path.join(projectRoot, 'dist');
const buildLockRoot = path.join(projectRoot, '.build-lock');
const repoRoot = path.resolve(projectRoot, '..', '..');
const vendorSources = [
  {
    sourceRoot: path.join(
      repoRoot,
      'sdks',
      'sdkwork-craw-chat-sdk',
      'sdkwork-craw-chat-sdk-typescript',
      'composed',
      'dist',
    ),
    targetRoot: path.join(distRoot, '__vendor__', 'sdkwork-craw-chat-sdk'),
  },
  {
    sourceRoot: path.join(
      repoRoot,
      'sdks',
      'sdkwork-craw-chat-sdk',
      'sdkwork-craw-chat-sdk-typescript',
      'generated',
      'server-openapi',
      'browser',
    ),
    targetRoot: path.join(distRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk'),
  },
  {
    sourceRoot: path.join(
      repoRoot,
      'sdks',
      'sdkwork-craw-chat-sdk',
      'sdkwork-craw-chat-sdk-typescript',
      'generated',
      'server-openapi',
      'node_modules',
      '@sdkwork',
      'sdk-common',
      'dist',
    ),
    targetRoot: path.join(distRoot, '__vendor__', 'sdkwork-sdk-common'),
  },
];
const forbiddenReleaseRelativePaths = new Set([
  'packages/craw-chat-portal-portal-api/src/mockData.js',
  'packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js',
]);
const forbiddenReleaseContentMarkers = Object.freeze([
  {
    description: 'portal mock data export',
    pattern: /\bportalMockData\b/g,
  },
  {
    description: 'portal mock data source export',
    pattern: /\bmockPortalDataSource\b/g,
  },
  {
    description: 'demo portal session token seed',
    pattern: /tenant-demo-session/g,
  },
  {
    description: 'default demo tenant id',
    pattern: /value="t_demo"/g,
  },
  {
    description: 'default demo operator login',
    pattern: /value="ops_demo"/g,
  },
  {
    description: 'default demo operator password',
    pattern: /Portal#2026/g,
  },
]);
const releaseScanTextExtensions = new Set([
  '.css',
  '.html',
  '.js',
  '.json',
  '.mjs',
  '.svg',
  '.txt',
]);
const lockRetryDelayMs = 50;
const lockWaitTimeoutMs = 15000;
const staleLockThresholdMs = 60000;

function wait(durationMs) {
  return new Promise((resolve) => {
    setTimeout(resolve, durationMs);
  });
}

async function tryClearStaleLock() {
  try {
    const lockStats = await stat(buildLockRoot);
    const lockAgeMs = Date.now() - lockStats.mtimeMs;

    if (lockAgeMs < staleLockThresholdMs) {
      return false;
    }

    await rm(buildLockRoot, { force: true, recursive: true });
    return true;
  } catch (error) {
    if (error?.code === 'ENOENT') {
      return true;
    }

    throw error;
  }
}

async function acquireBuildLock() {
  const waitDeadline = Date.now() + lockWaitTimeoutMs;

  while (true) {
    try {
      await mkdir(buildLockRoot);
      return;
    } catch (error) {
      if (error?.code !== 'EEXIST') {
        throw error;
      }
    }

    if (await tryClearStaleLock()) {
      continue;
    }

    if (Date.now() >= waitDeadline) {
      throw new Error(`Timed out waiting for the portal build lock at ${buildLockRoot}.`);
    }

    await wait(lockRetryDelayMs);
  }
}

async function withBuildLock(task) {
  await acquireBuildLock();

  try {
    return await task();
  } finally {
    await rm(buildLockRoot, { force: true, recursive: true });
  }
}

function normalizeRelativePath(relativePath) {
  return relativePath.split(path.sep).join('/');
}

function isForbiddenReleasePath(relativePath) {
  return forbiddenReleaseRelativePaths.has(normalizeRelativePath(relativePath));
}

function shouldScanReleaseTextFile(filePath) {
  return releaseScanTextExtensions.has(path.extname(filePath).toLowerCase());
}

export function collectPortalReleaseContentViolations(files = {}) {
  const violations = [];

  for (const [fileName, source] of Object.entries(files)) {
    if (typeof source !== 'string' || source.length === 0) {
      continue;
    }

    for (const marker of forbiddenReleaseContentMarkers) {
      const matches = source.match(marker.pattern);

      if (!matches) {
        continue;
      }

      violations.push({
        description: marker.description,
        fileName,
        match: matches[0],
      });
    }
  }

  return violations;
}

async function collectReleaseTextFiles(root, currentRoot = root, files = {}) {
  const entries = await readdir(currentRoot, { withFileTypes: true });

  for (const entry of entries) {
    const entryPath = path.join(currentRoot, entry.name);

    if (entry.isDirectory()) {
      await collectReleaseTextFiles(root, entryPath, files);
      continue;
    }

    if (!shouldScanReleaseTextFile(entryPath)) {
      continue;
    }

    files[normalizeRelativePath(path.relative(root, entryPath))] = await readFile(entryPath, 'utf8');
  }

  return files;
}

export async function assertPortalDistReleaseSafe(root = distRoot) {
  const files = await collectReleaseTextFiles(root);
  const violations = collectPortalReleaseContentViolations(files);

  if (violations.length === 0) {
    return;
  }

  const formattedViolations = violations
    .map(({ fileName, description, match }) => `${fileName}: ${description} (${match})`)
    .join('; ');

  throw new Error(
    `Portal dist contains forbidden mock/demo release content. Remove the leak before shipping: ${formattedViolations}`,
  );
}

async function copyDirectory(sourceRoot, targetRoot, relativeRoot = '') {
  await mkdir(targetRoot, { recursive: true });

  const entries = await readdir(sourceRoot, { withFileTypes: true });

  for (const entry of entries) {
    const sourcePath = path.join(sourceRoot, entry.name);
    const targetPath = path.join(targetRoot, entry.name);
    const entryRelativePath = path.join(relativeRoot, entry.name);

    if (entry.isDirectory()) {
      await copyDirectory(sourcePath, targetPath, entryRelativePath);
      continue;
    }

    if (isForbiddenReleasePath(entryRelativePath)) {
      continue;
    }

    await copyFile(sourcePath, targetPath);
  }
}

export async function rebuildDist() {
  await withBuildLock(async () => {
    await rm(distRoot, { force: true, recursive: true });
    await mkdir(distRoot, { recursive: true });

    await copyFile(
      path.join(projectRoot, 'index.html'),
      path.join(distRoot, 'index.html'),
    );

    for (const directoryName of ['src', 'packages']) {
      await copyDirectory(
        path.join(projectRoot, directoryName),
        path.join(distRoot, directoryName),
        directoryName,
      );
    }

    for (const vendorSource of vendorSources) {
      await access(vendorSource.sourceRoot);
      await copyDirectory(vendorSource.sourceRoot, vendorSource.targetRoot);
    }

    await access(path.join(distRoot, 'index.html'));
    await assertPortalDistReleaseSafe(distRoot);
  });
}
