import { access, copyFile, mkdir, readdir, rm, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const appRoot = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(appRoot, '..');
const distRoot = path.join(projectRoot, 'dist');
const buildLockRoot = path.join(projectRoot, '.build-lock');
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

async function copyDirectory(sourceRoot, targetRoot) {
  await mkdir(targetRoot, { recursive: true });

  const entries = await readdir(sourceRoot, { withFileTypes: true });

  for (const entry of entries) {
    const sourcePath = path.join(sourceRoot, entry.name);
    const targetPath = path.join(targetRoot, entry.name);

    if (entry.isDirectory()) {
      await copyDirectory(sourcePath, targetPath);
      continue;
    }

    await copyFile(sourcePath, targetPath);
  }
}

async function rebuildDist() {
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
      );
    }

    await access(path.join(distRoot, 'index.html'));
  });
}

await rebuildDist();
