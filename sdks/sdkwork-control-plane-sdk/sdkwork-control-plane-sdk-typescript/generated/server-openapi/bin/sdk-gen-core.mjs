#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { createNpmCommandArgs } from '../../../../bin/npm-runtime.mjs';

const marker = '[sdk-gen]';

function log(message) {
  console.log(`${marker} ${message}`);
}

function fail(message) {
  console.error(`${marker} ERROR: ${message}`);
  process.exit(1);
}

function quoteArg(arg) {
  return /\s/.test(arg) ? `"${arg.replace(/"/g, '\\"')}"` : arg;
}

function printHelp() {
  console.log('SDKWork Control-Plane TypeScript generated workspace helper');
  console.log('');
  console.log('Usage:');
  console.log('  node bin/sdk-gen-core.mjs build [--project-dir <dir>]');
  console.log('');
  console.log('Notes:');
  console.log('  - resolves npm from the active Node.js runtime when bare npm is not on PATH');
  console.log('  - intended for the internal generated/server-openapi workspace only');
}

function parseArgs(argv, defaultProjectDir) {
  const parsed = {
    task: '',
    projectDir: defaultProjectDir,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--project-dir') {
      const projectDirValue = argv[index + 1];
      if (!projectDirValue) {
        fail('Missing value for --project-dir');
      }
      parsed.projectDir = path.resolve(projectDirValue);
      index += 1;
      continue;
    }
    if (current === '--help' || current === '-h') {
      printHelp();
      process.exit(0);
    }
    if (!parsed.task) {
      parsed.task = current.trim().toLowerCase();
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.task) {
    printHelp();
    process.exit(1);
  }
  if (parsed.task !== 'build') {
    fail(`Unsupported task "${parsed.task}". Expected: build.`);
  }
  if (!existsSync(parsed.projectDir)) {
    fail(`Project directory does not exist: ${parsed.projectDir}`);
  }
  if (!existsSync(path.join(parsed.projectDir, 'package.json'))) {
    fail(`Project directory must contain package.json: ${parsed.projectDir}`);
  }

  return parsed;
}

function runNpm(step, npmArgs, options) {
  const invocation = createNpmCommandArgs(npmArgs);
  const commandLine = [invocation.command, ...invocation.args].map(quoteArg).join(' ');
  log(`> ${commandLine} (cwd=${options.cwd})`);
  const result = spawnSync(invocation.command, invocation.args, {
    cwd: options.cwd,
    env: options.env,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    fail(`${step} failed to start: ${result.error.message}`);
  }
  if ((result.status ?? 1) !== 0) {
    fail(`${step} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${step} terminated with signal ${result.signal}`);
  }
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const defaultProjectDir = path.resolve(scriptDir, '..');
const args = parseArgs(process.argv.slice(2), defaultProjectDir);

runNpm('npm-install', ['install'], {
  cwd: args.projectDir,
  env: process.env,
});
runNpm('npm-build', ['run', 'build'], {
  cwd: args.projectDir,
  env: process.env,
});

log('Done.');
