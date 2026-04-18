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
const officialLanguages = [
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
];
const defaultLanguages = officialLanguages;
const languageSet = new Set(args.languages.length > 0 ? args.languages : defaultLanguages);
for (const language of languageSet) {
  if (!officialLanguages.includes(language)) {
    fail(`Unsupported language: ${language}`);
  }
}

run('node', [path.join(scriptDir, 'verify-sdk-automation.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:automation',
});
run('node', [path.join(scriptDir, 'verify-internal-docs.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:internal-docs',
});
run('node', [path.join(scriptDir, 'verify-docs-contract-tests.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:docs-contract-tests',
});
run('node', [path.join(scriptDir, 'verify-powershell-wrapper-args.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:powershell-wrapper-args',
});
run('node', [path.join(scriptDir, 'verify-derived-discovery-metadata.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:derived-discovery-metadata',
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
  run('node', [path.join(scriptDir, 'verify-typescript-workspace-concurrency.mjs')], {
    cwd: workspaceRoot,
    step: 'typescript:workspace-concurrency',
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

if (languageSet.has('rust')) {
  run('node', [path.join(scriptDir, 'verify-rust-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'rust:workspace',
  });
}

if (languageSet.has('java')) {
  run('node', [path.join(scriptDir, 'verify-java-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'java:workspace',
  });
}

if (languageSet.has('csharp')) {
  run('node', [path.join(scriptDir, 'verify-csharp-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'csharp:workspace',
  });
}

if (languageSet.has('swift')) {
  run('node', [path.join(scriptDir, 'verify-swift-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'swift:workspace',
  });
}

if (languageSet.has('kotlin')) {
  run('node', [path.join(scriptDir, 'verify-kotlin-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'kotlin:workspace',
  });
}

if (languageSet.has('go')) {
  run('node', [path.join(scriptDir, 'verify-go-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'go:workspace',
  });
}

if (languageSet.has('python')) {
  run('node', [path.join(scriptDir, 'verify-python-workspace.mjs')], {
    cwd: workspaceRoot,
    step: 'python:workspace',
  });
}

run('node', [path.join(scriptDir, 'assemble-sdk.mjs')], {
  cwd: workspaceRoot,
  step: 'workspace:assemble',
});
