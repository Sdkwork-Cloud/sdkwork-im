#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const workspaceRoot = path.resolve(repositoryRoot, '..');
const workflowPath = path.join(repositoryRoot, 'sdkwork.workflow.json');
const dependencyRootRelative = '.sdkwork/dependencies';
const dependencyRoot = path.join(repositoryRoot, ...dependencyRootRelative.split('/'));
const args = new Set(process.argv.slice(2));
const jsonOutput = args.has('--json');
const dryRun = args.has('--dry-run');
const mode = args.has('--apply') ? 'apply' : 'check';

if (args.has('--help')) {
  printHelp();
  process.exit(0);
}

function printHelp() {
  console.log(`Usage: node scripts/prepare-local-dependencies.mjs [--check|--apply] [--dry-run] [--json]

Materializes SDKWork source dependencies under ${dependencyRootRelative}/<dependency-id>.

Options:
  --check    Verify materialized dependencies without writing. This is the default.
  --apply    Create missing links from sibling repositories when possible.
  --dry-run  Report planned writes without creating links.
  --json     Print machine-readable JSON output.
`);
}

function readWorkflowDependencyIds() {
  const workflow = JSON.parse(fs.readFileSync(workflowPath, 'utf-8'));
  if (!Array.isArray(workflow.dependencies)) {
    throw new Error('sdkwork.workflow.json must declare a dependencies array');
  }

  return workflow.dependencies
    .map((dependency) => dependency?.id)
    .map((dependencyId) => {
      if (typeof dependencyId !== 'string' || !/^sdkwork-[a-z0-9][a-z0-9-]*$/.test(dependencyId)) {
        throw new Error(`Invalid SDKWork dependency id in sdkwork.workflow.json: ${String(dependencyId)}`);
      }
      return dependencyId;
    });
}

function readLinkTarget(entryPath) {
  try {
    return path.resolve(path.dirname(entryPath), fs.readlinkSync(entryPath));
  } catch (error) {
    if (error?.code === 'EINVAL') {
      return null;
    }
    throw error;
  }
}

function isEmptyDirectory(entryPath) {
  return fs.statSync(entryPath).isDirectory() && fs.readdirSync(entryPath).length === 0;
}

function inspectDependency(dependencyId) {
  const source = path.join(workspaceRoot, dependencyId);
  const target = path.join(dependencyRoot, dependencyId);
  const result = {
    id: dependencyId,
    source,
    target,
    action: 'none',
    ok: true,
    message: '',
  };

  if (!fs.existsSync(source)) {
    result.ok = false;
    result.action = 'missing-source';
    result.message = `Missing local dependency source: ${source}`;
    return result;
  }

  if (!fs.existsSync(target)) {
    result.ok = mode === 'apply' || dryRun;
    result.action = mode === 'apply' ? (dryRun ? 'would-link' : 'link') : 'missing-link';
    result.message = `Missing dependency link: ${target} -> ${source}`;
    return result;
  }

  const stats = fs.lstatSync(target);
  if (stats.isSymbolicLink()) {
    const currentTarget = readLinkTarget(target);
    if (currentTarget !== source) {
      result.ok = false;
      result.action = 'mismatched-link';
      result.message = `Dependency link target mismatch: ${target} -> ${currentTarget}; expected ${source}`;
      return result;
    }
    result.action = 'verified-link';
    result.message = `Verified dependency link: ${target} -> ${source}`;
    return result;
  }

  if (stats.isDirectory() && !isEmptyDirectory(target)) {
    result.action = 'verified-directory';
    result.message = `Verified existing dependency checkout: ${target}`;
    return result;
  }

  if (stats.isDirectory() && isEmptyDirectory(target)) {
    result.ok = false;
    result.action = 'empty-directory';
    result.message = `Dependency target already exists as an empty real directory; remove it or replace it with a link: ${target}`;
    return result;
  }

  result.ok = false;
  result.action = 'blocked';
  result.message = `Dependency target already exists and is not a directory link or checkout: ${target}`;
  return result;
}

function applyDependency(result) {
  if (result.action !== 'link') {
    return result;
  }

  fs.mkdirSync(dependencyRoot, { recursive: true });
  fs.symlinkSync(result.source, result.target, process.platform === 'win32' ? 'junction' : 'dir');
  result.action = 'created-link';
  result.message = `Created dependency link: ${result.target} -> ${result.source}`;
  return result;
}

const dependencyIds = readWorkflowDependencyIds();
const results = dependencyIds.map(inspectDependency).map((result) => {
  if (mode === 'apply' && !dryRun && result.ok) {
    return applyDependency(result);
  }
  return result;
});
const failures = results.filter((result) => !result.ok);

if (jsonOutput) {
  console.log(JSON.stringify({
    mode,
    dryRun,
    dependencyRoot: path.relative(repositoryRoot, dependencyRoot).replaceAll('\\', '/'),
    dependencies: results.map((result) => ({
      id: result.id,
      action: result.action,
      ok: result.ok,
      source: path.relative(repositoryRoot, result.source).replaceAll('\\', '/'),
      target: path.relative(repositoryRoot, result.target).replaceAll('\\', '/'),
      message: result.message,
    })),
  }, null, 2));
} else {
  for (const result of results) {
    const prefix = result.ok ? '[ok]' : '[error]';
    console.log(`${prefix} ${result.message}`);
  }
  const action = mode === 'apply' ? (dryRun ? 'planned' : 'prepared') : 'verified';
  console.log(`Local SDKWork dependencies ${action} at ${path.relative(repositoryRoot, dependencyRoot)}`);
}

if (failures.length > 0) {
  process.exitCode = 1;
}
