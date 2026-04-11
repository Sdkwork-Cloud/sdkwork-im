#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    languages: [],
    withDart: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        fail('Missing value for --language');
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }
    if (current === '--with-dart') {
      parsed.withDart = true;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    stdio: 'inherit',
    shell: false,
    timeout: options.timeoutMs,
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${options.step || command} terminated with signal ${result.signal}`);
  }
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const languageSet = new Set(args.languages.length > 0 ? args.languages : ['typescript', 'flutter']);
for (const language of languageSet) {
  if (!['typescript', 'flutter'].includes(language)) {
    fail(`Unsupported language: ${language}`);
  }
}

run('node', [path.join(scriptDir, 'verify-sdk-automation.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:automation',
});
run('node', [path.join(scriptDir, 'verify-powershell-wrapper-args.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:powershell-wrapper-args',
});

if (languageSet.has('typescript')) {
  run('node', [path.join(scriptDir, 'verify-typescript-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'typescript:workspace',
  });
  run('node', [path.join(scriptDir, 'verify-typescript-generated-build-determinism.mjs')], {
    cwd: workspaceRoot,
    step: 'typescript:generated-build-determinism',
  });
  run('node', [path.join(scriptDir, 'verify-typescript-generated-build-concurrency.mjs')], {
    cwd: workspaceRoot,
    step: 'typescript:generated-build-concurrency',
  });
}

if (languageSet.has('flutter')) {
  const flutterWorkspaceArgs = [path.join(scriptDir, 'verify-flutter-workspace.mjs')];
  if (args.withDart) {
    flutterWorkspaceArgs.push('--with-dart');
  }
  run('node', flutterWorkspaceArgs, {
    cwd: workspaceRoot,
    step: 'flutter:workspace',
  });
}

run('node', [path.join(scriptDir, 'assemble-sdk.mjs'), ...[...languageSet].flatMap((language) => ['--language', language])], {
  cwd: workspaceRoot,
  step: 'workspace:assemble',
});
