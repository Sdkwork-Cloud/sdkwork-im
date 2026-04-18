#!/usr/bin/env node
import { existsSync, mkdirSync, readdirSync, renameSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    language: '',
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        fail('Missing value for --language');
      }
      parsed.language = value;
      index += 1;
      continue;
    }

    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.language) {
    fail('Missing required --language argument');
  }

  return parsed;
}

function toPosixPath(value) {
  return value.replaceAll('\\', '/');
}

function isPathInside(parentPath, candidatePath) {
  const normalizedParent = toPosixPath(path.resolve(parentPath)).replace(/\/+$/, '');
  const normalizedCandidate = toPosixPath(path.resolve(candidatePath)).replace(/\/+$/, '');
  return normalizedCandidate === normalizedParent || normalizedCandidate.startsWith(`${normalizedParent}/`);
}

function ensurePathInside(parentPath, candidatePath, description) {
  if (!isPathInside(parentPath, candidatePath)) {
    fail(`${description} escapes the allowed root: ${candidatePath}`);
  }
}

function collectStaleBackupRoots(rootDirectory) {
  if (!existsSync(rootDirectory)) {
    return [];
  }

  return readdirSync(rootDirectory, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(rootDirectory, entry.name))
    .sort()
    .reverse();
}

function tryPruneStaleBackups(rootDirectory, keepCount) {
  const staleBackupRoots = collectStaleBackupRoots(rootDirectory).slice(keepCount);
  for (const staleBackupRoot of staleBackupRoots) {
    try {
      ensurePathInside(rootDirectory, staleBackupRoot, 'Stale backup root');
      rmSync(staleBackupRoot, { recursive: true, force: true });
    } catch {
      // Ignore pruning failures. The current run only needs the active output path cleared.
    }
  }
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

const languageConfigs = {
  typescript: {
    outputRoot: path.join(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-typescript',
      'generated',
      'server-openapi',
    ),
    stagePaths: [
      'src',
      'bin',
      'tsconfig.json',
      'vite.config.ts',
      'sdkwork-sdk.json',
      'package-lock.json',
      '.sdkwork/manual-backups',
      '.sdkwork/sdkwork-generator-manifest.json',
      '.sdkwork/sdkwork-generator-changes.json',
      '.sdkwork/sdkwork-generator-report.json',
    ],
  },
};

const config = languageConfigs[args.language];
if (!config) {
  process.stdout.write(`[sdkwork-craw-chat-sdk] No preclean required for ${args.language}.\n`);
  process.exit(0);
}

const outputRoot = path.resolve(config.outputRoot);
ensurePathInside(workspaceRoot, outputRoot, 'Generated output root');

if (!existsSync(outputRoot)) {
  process.stdout.write(
    `[sdkwork-craw-chat-sdk] No generated output root exists for ${args.language}; skipping preclean.\n`,
  );
  process.exit(0);
}

const runId = `run-${Date.now()}-${process.pid}-${Math.random().toString(16).slice(2, 10)}`;
const backupBaseRoot = path.join(
  workspaceRoot,
  '.sdkwork',
  'tmp',
  'prepare-generated-output',
  args.language,
);
const backupRunRoot = path.join(backupBaseRoot, runId);
const stagedEntries = [];

try {
  for (const relativePath of config.stagePaths) {
    const sourcePath = path.join(outputRoot, ...relativePath.split('/'));
    if (!existsSync(sourcePath)) {
      continue;
    }

    const backupPath = path.join(backupRunRoot, ...relativePath.split('/'));
    ensurePathInside(outputRoot, sourcePath, 'Generated source path');
    ensurePathInside(backupRunRoot, backupPath, 'Generated backup path');

    mkdirSync(path.dirname(backupPath), { recursive: true });
    renameSync(sourcePath, backupPath);
    stagedEntries.push({
      relativePath,
      sourcePath,
      backupPath,
    });
  }
} catch (error) {
  for (const stagedEntry of stagedEntries.reverse()) {
    try {
      if (!existsSync(stagedEntry.backupPath)) {
        continue;
      }
      mkdirSync(path.dirname(stagedEntry.sourcePath), { recursive: true });
      renameSync(stagedEntry.backupPath, stagedEntry.sourcePath);
    } catch {
      // Preserve the first failure and surface the path that needs manual attention.
    }
  }

  fail(
    `Failed to stage ${args.language} generated output before regeneration: ${error.message}`,
  );
}

tryPruneStaleBackups(backupBaseRoot, 3);

if (stagedEntries.length === 0) {
  process.stdout.write(
    `[sdkwork-craw-chat-sdk] No conflicting ${args.language} generated entries required staging.\n`,
  );
  process.exit(0);
}

process.stdout.write(
  `[sdkwork-craw-chat-sdk] Staged ${stagedEntries.length} ${args.language} generated entries before regeneration.\n`,
);
