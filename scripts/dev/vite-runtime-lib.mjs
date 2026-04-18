import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');

function shouldSkipWorkspaceEntry(entryName) {
  return entryName.startsWith('.')
    || entryName === 'node_modules'
    || entryName === 'packages'
    || entryName === 'dist'
    || entryName === 'coverage'
    || entryName === 'target';
}

function resolveWorkspaceRoot(candidateRoot = repoRoot) {
  const normalizedRoot = path.resolve(candidateRoot);
  return path.basename(path.dirname(normalizedRoot)) === '.worktrees'
    ? path.resolve(normalizedRoot, '..', '..')
    : normalizedRoot;
}

function normalizeRelativeEntry(relativeEntry) {
  if (Array.isArray(relativeEntry)) {
    return relativeEntry;
  }

  return [relativeEntry];
}

function defaultFileExists(filePath) {
  return fs.existsSync(filePath);
}

function defaultSymlinkDirectory(targetPath, linkPath, platform = process.platform) {
  fs.symlinkSync(targetPath, linkPath, platform === 'win32' ? 'junction' : 'dir');
}

function defaultRenamePath(sourcePath, destinationPath) {
  fs.renameSync(sourcePath, destinationPath);
}

function listWorkspaceAppRoots(searchRoot, maxDepth = 2) {
  if (!defaultFileExists(searchRoot)) {
    return [];
  }

  const discoveredRoots = [];

  function walk(currentRoot, depth) {
    let entries;
    try {
      entries = fs.readdirSync(currentRoot, { withFileTypes: true });
    } catch {
      return;
    }

    for (const entry of entries.sort((left, right) => left.name.localeCompare(right.name))) {
      if (!entry.isDirectory() || shouldSkipWorkspaceEntry(entry.name)) {
        continue;
      }

      const candidateRoot = path.join(currentRoot, entry.name);
      const candidatePackageJson = path.join(candidateRoot, 'package.json');
      const candidateNodeModules = path.join(candidateRoot, 'node_modules');

      if (
        defaultFileExists(candidatePackageJson)
        && defaultFileExists(candidateNodeModules)
      ) {
        discoveredRoots.push(candidateRoot);
        continue;
      }

      if (depth >= maxDepth) {
        continue;
      }

      walk(candidateRoot, depth + 1);
    }
  }

  walk(searchRoot, 0);
  return discoveredRoots;
}

function listWorkspaceWorktreeAppRoots(worktreesRoot) {
  if (!defaultFileExists(worktreesRoot)) {
    return [];
  }

  let entries;
  try {
    entries = fs.readdirSync(worktreesRoot, { withFileTypes: true });
  } catch {
    return [];
  }

  return entries
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith('.'))
    .flatMap((entry) => listWorkspaceAppRoots(path.join(worktreesRoot, entry.name, 'apps')));
}

function defaultOpenFile(filePath) {
  return fs.openSync(filePath, 'r');
}

function defaultCloseFile(fileDescriptor) {
  fs.closeSync(fileDescriptor);
}

function defaultResolveFromRoot(root, specifier) {
  return createRequire(path.join(root, 'package.json')).resolve(specifier);
}

function hasRequiredPackagesAtNodeModules(
  nodeModulesRoot,
  requiredPackages = [],
  fileExists = defaultFileExists,
) {
  return requiredPackages.every((packageName) => fileExists(path.join(
    nodeModulesRoot,
    ...packageName.split('/'),
    'package.json',
  )));
}

export function probeReadableFile(
  filePath,
  {
    fileExists = defaultFileExists,
    openFile = defaultOpenFile,
    closeFile = defaultCloseFile,
  } = {},
) {
  if (!fileExists(filePath)) {
    return false;
  }

  try {
    const fileDescriptor = openFile(filePath);
    closeFile(fileDescriptor);
    return true;
  } catch {
    return false;
  }
}

function defaultIsReadable(filePath) {
  return probeReadableFile(filePath);
}

export function resolveWorkspaceDonorRoots(appRoot) {
  const normalizedAppRoot = path.resolve(appRoot);
  const workspaceRoot = resolveWorkspaceRoot(repoRoot);
  const activeAppsRoot = path.join(repoRoot, 'apps');
  const workspaceAppsRoot = path.join(workspaceRoot, 'apps');
  const knownWorkspaceApps = [
    ...listWorkspaceAppRoots(activeAppsRoot),
    ...(workspaceAppsRoot === activeAppsRoot ? [] : listWorkspaceAppRoots(workspaceAppsRoot)),
    ...listWorkspaceAppRoots(path.resolve(workspaceRoot, '..')),
    ...listWorkspaceWorktreeAppRoots(path.join(workspaceRoot, '.worktrees')),
  ];

  return knownWorkspaceApps
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => (
      candidateRoot !== normalizedAppRoot
      && roots.indexOf(candidateRoot) === index
    ));
}

export function ensureLocalNodeModules({
  appRoot,
  donorRoots = resolveWorkspaceDonorRoots(appRoot),
  requiredPackages = [],
  fileExists = defaultFileExists,
  renamePath = defaultRenamePath,
  symlinkDirectory = defaultSymlinkDirectory,
  platform = process.platform,
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  const normalizedAppRoot = path.resolve(appRoot);
  const localNodeModulesPath = path.join(normalizedAppRoot, 'node_modules');
  const hasRequiredPackages = hasRequiredPackagesAtNodeModules(
    localNodeModulesPath,
    requiredPackages,
    fileExists,
  );

  if (fileExists(localNodeModulesPath) && hasRequiredPackages) {
    return localNodeModulesPath;
  }

  const donorNodeModulesPath = donorRoots
    .map((candidateRoot) => path.join(path.resolve(candidateRoot), 'node_modules'))
    .find((candidatePath) => (
      fileExists(candidatePath)
      && hasRequiredPackagesAtNodeModules(candidatePath, requiredPackages, fileExists)
    ));

  if (!donorNodeModulesPath) {
    const requiredPackagesSuffix = requiredPackages.length > 0
      ? ` satisfying [${requiredPackages.join(', ')}]`
      : '';
    throw new Error(
      `unable to materialize local node_modules for ${normalizedAppRoot}; no readable donor node_modules${requiredPackagesSuffix} were found`,
    );
  }

  if (fileExists(localNodeModulesPath)) {
    let backupIndex = 0;
    let backupPath = `${localNodeModulesPath}.__stale__donor`;
    while (fileExists(backupPath)) {
      backupIndex += 1;
      backupPath = `${localNodeModulesPath}.__stale__donor_${backupIndex}`;
    }

    renamePath(localNodeModulesPath, backupPath);
  }

  try {
    symlinkDirectory(donorNodeModulesPath, localNodeModulesPath, platform);
  } catch (error) {
    if (!(error && error.code === 'EEXIST' && fileExists(localNodeModulesPath))) {
      throw error;
    }
  }

  return localNodeModulesPath;
}

export function resolveReadablePackageEntry({
  appRoot,
  donorRoots = [],
  packageName,
  relativeEntry,
  fileExists = defaultFileExists,
  isReadable = defaultIsReadable,
}) {
  const entrySegments = normalizeRelativeEntry(relativeEntry);
  const candidateRoots = [appRoot, ...donorRoots]
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => roots.indexOf(candidateRoot) === index);

  for (const candidateRoot of candidateRoots) {
    const candidateEntry = path.join(
      candidateRoot,
      'node_modules',
      packageName,
      ...entrySegments,
    );
    if (fileExists(candidateEntry) && isReadable(candidateEntry)) {
      return candidateEntry;
    }
  }

  throw new Error(
    `unable to resolve a readable ${packageName} entry (${entrySegments.join('/')}) from ${candidateRoots.join(', ')}`,
  );
}

export function resolveReadablePackageImportUrl(options) {
  return pathToFileURL(resolveReadablePackageEntry(options)).href;
}

export function findReadableModuleResolution({
  appRoot,
  donorRoots = [],
  specifier,
  resolveFromRoot = defaultResolveFromRoot,
  isReadable = defaultIsReadable,
}) {
  const candidateRoots = [appRoot, ...donorRoots]
    .map((candidateRoot) => path.resolve(candidateRoot))
    .filter((candidateRoot, index, roots) => roots.indexOf(candidateRoot) === index);

  for (const candidateRoot of candidateRoots) {
    let resolvedPath;
    try {
      resolvedPath = resolveFromRoot(candidateRoot, specifier);
    } catch {
      continue;
    }

    if (isReadable(resolvedPath)) {
      return {
        candidateRoot,
        resolvedPath,
      };
    }
  }

  throw new Error(
    `unable to resolve readable module specifier "${specifier}" from ${candidateRoots.join(', ')}`,
  );
}

export function resolveReadableModuleSpecifier(options) {
  return findReadableModuleResolution(options).resolvedPath;
}

export function resolveReadablePackageRoot({
  relativeEntry,
  ...options
}) {
  return path.dirname(resolveReadablePackageEntry({
    ...options,
    relativeEntry: 'package.json',
  }));
}

export async function importReadablePackageDefault(options) {
  const moduleUrl = resolveReadablePackageImportUrl(options);
  const loadedModule = await import(moduleUrl);
  return loadedModule.default ?? loadedModule;
}

export async function applyWindowsVitePreload({
  platform = process.platform,
} = {}) {
  if (platform !== 'win32') {
    return;
  }

  await import(pathToFileURL(path.join(scriptDir, 'vite-windows-realpath-preload.mjs')).href);
}
