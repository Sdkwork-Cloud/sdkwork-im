#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { existsSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

function fail(message) {
  throw new Error(message);
}

function parseArgs(argv) {
  const parsed = {
    withDart: false,
  };

  for (const current of argv) {
    if (current === '--with-dart') {
      parsed.withDart = true;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

function normalizePathEntry(value) {
  return String(value || '')
    .trim()
    .replace(/^"(.*)"$/, '$1')
    .replace(/[\\/]+$/, '');
}

function resolveDartCommand() {
  if (process.platform !== 'win32') {
    return 'dart';
  }

  const candidates = new Set();
  const pushCandidate = (candidate) => {
    const normalizedCandidate = normalizePathEntry(candidate);
    if (!normalizedCandidate) {
      return;
    }
    candidates.add(normalizedCandidate);
  };

  pushCandidate(process.env.SDKWORK_DART);
  pushCandidate(process.env.DART_EXECUTABLE);
  if (process.env.FLUTTER_ROOT) {
    pushCandidate(path.join(process.env.FLUTTER_ROOT, 'bin', 'cache', 'dart-sdk', 'bin', 'dart.exe'));
  }

  for (const entry of (process.env.PATH || '').split(path.delimiter)) {
    const normalizedEntry = normalizePathEntry(entry);
    if (!normalizedEntry) {
      continue;
    }
    pushCandidate(path.join(normalizedEntry, 'dart.exe'));
    pushCandidate(path.join(normalizedEntry, 'cache', 'dart-sdk', 'bin', 'dart.exe'));
  }

  for (const candidate of candidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  return 'dart';
}

function run(command, args, options = {}) {
  const result = spawnSync(command === 'dart' ? resolveDartCommand() : command, args, {
    cwd: options.cwd,
    env: options.env,
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
verifyLanguageWorkspace({
  language: 'flutter',
  workspace: 'sdkwork-craw-chat-sdk-flutter',
  primaryClient: 'CrawChatClient',
  maturityTier: 'tier-a',
  requiredPackageLayers: ['generated', 'composed'],
  readmeRequiredTerms: [
    'sdk.createXxxMessage()',
    'sdk.send()',
    'sdk.decodeMessage()',
    'TypeScript',
    'craw_chat_sdk',
    'backend_sdk',
    'CrawChatClient',
    'AuthApi',
    'PortalApi',
    'sdk.auth',
    'sdk.portal',
    'client.auth',
    'client.portal',
    'WebSocket adapter',
  ],
  generatedReadmeRequiredTerms: [
    'package:backend_sdk/backend_sdk.dart',
    'AuthApi',
    'PortalApi',
    'client.auth',
    'client.portal',
  ],
  composedReadmeRequiredTerms: [
    'manual-owned consumer layer',
    'official Flutter app-consumer package',
    'package:craw_chat_sdk/craw_chat_sdk.dart',
    'AuthApi',
    'PortalApi',
    'sdk.auth',
    'sdk.portal',
    'client.auth',
    'client.portal',
    'WebSocket adapter',
  ],
  consumerPackage: {
    name: 'craw_chat_sdk',
    packagePath: 'sdkwork-craw-chat-sdk-flutter/composed',
    manifestPath: 'sdkwork-craw-chat-sdk-flutter/composed/pubspec.yaml',
  },
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedDir = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'generated',
  'server-openapi',
);
const composedDir = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'composed',
);
const composedOverridePath = path.join(composedDir, 'pubspec_overrides.yaml');
const flutterDartAnalysisScript = path.join(scriptDir, 'verify-flutter-dart-analysis.dart');
const dartEnv = {
  ...process.env,
  PUB_CACHE: path.join(workspaceRoot, '.sdkwork', 'dart', 'pub-cache'),
  DART_SUPPRESS_ANALYTICS: 'true',
  FLUTTER_SUPPRESS_ANALYTICS: 'true',
  CI: 'true',
};
const hadOriginalComposedOverride = existsSync(composedOverridePath);
const originalComposedOverrideSource = hadOriginalComposedOverride
  ? readFileSync(composedOverridePath, 'utf8')
  : '';

try {
  run('node', [path.join(scriptDir, 'verify-flutter-generated-models.mjs')], {
    cwd: workspaceRoot,
    step: 'flutter:generated-regression',
  });
  run('node', [path.join(scriptDir, 'verify-auth-surface-alignment.mjs'), '--language', 'flutter'], {
    cwd: workspaceRoot,
    step: 'flutter:auth-surface',
  });
  run('node', [path.join(scriptDir, 'verify-flutter-composed-parity.mjs')], {
    cwd: workspaceRoot,
    step: 'flutter:composed-parity',
  });
  run('node', [path.join(scriptDir, 'verify-flutter-public-api-boundary.mjs')], {
    cwd: workspaceRoot,
    step: 'flutter:public-api-boundary',
  });
  run('node', [path.join(scriptDir, 'sync-flutter-pubspec-overrides.mjs')], {
    cwd: workspaceRoot,
    step: 'flutter:sync-pubspec-overrides',
  });
  run('node', [path.join(scriptDir, 'verify-flutter-package-metadata.mjs')], {
    cwd: workspaceRoot,
    step: 'flutter:package-metadata',
  });

  if (args.withDart) {
    run('dart', ['--version'], {
      cwd: workspaceRoot,
      env: dartEnv,
      step: 'flutter:dart-version',
      timeoutMs: 10000,
    });
    run('dart', ['pub', 'get'], {
      cwd: generatedDir,
      env: dartEnv,
      step: 'flutter:generated-pub-get',
      timeoutMs: 600000,
    });
    run('dart', ['pub', 'get'], {
      cwd: composedDir,
      env: dartEnv,
      step: 'flutter:composed-pub-get',
      timeoutMs: 600000,
    });
    if (process.platform === 'win32') {
      const generatedPackageConfig = path.join(generatedDir, '.dart_tool', 'package_config.json');
      run('dart', [`--packages=${generatedPackageConfig}`, flutterDartAnalysisScript, generatedDir], {
        cwd: workspaceRoot,
        env: dartEnv,
        step: 'flutter:generated-analyze',
        timeoutMs: 300000,
      });
      run('dart', [`--packages=${generatedPackageConfig}`, flutterDartAnalysisScript, composedDir], {
        cwd: workspaceRoot,
        env: dartEnv,
        step: 'flutter:composed-analyze',
        timeoutMs: 300000,
      });
    } else {
      run('dart', ['analyze'], {
        cwd: generatedDir,
        env: dartEnv,
        step: 'flutter:generated-analyze',
        timeoutMs: 300000,
      });
      run('dart', ['analyze'], {
        cwd: composedDir,
        env: dartEnv,
        step: 'flutter:composed-analyze',
        timeoutMs: 300000,
      });
    }
  }
} catch (error) {
  console.error(`[sdkwork-craw-chat-sdk] ${error.message}`);
  process.exitCode = 1;
} finally {
  if (hadOriginalComposedOverride) {
    writeFileSync(composedOverridePath, originalComposedOverrideSource, 'utf8');
  } else if (existsSync(composedOverridePath)) {
    rmSync(composedOverridePath);
  }
}

if (process.exitCode && process.exitCode !== 0) {
  process.exit(process.exitCode);
}
