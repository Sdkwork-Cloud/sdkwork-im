#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { verifySdkSiteDocs } from '../../../docs/sites/sdk/verify-sdk-site-docs.mjs';

function read(workspaceRoot, relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

export function verifySdkAutomation(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = options.workspaceRoot || path.resolve(scriptDir, '..');
  const repoRoot = path.resolve(workspaceRoot, '..', '..');
  const failures = [];
  failures.push(...verifySdkSiteDocs({ rootDir: repoRoot }));
  const typeScriptReadmeSource = existsSync(
    path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-admin-typescript', 'README.md'),
  )
    ? read(workspaceRoot, 'sdkwork-craw-chat-sdk-admin-typescript/README.md')
    : '';

  const requiredPaths = [
    '.gitignore',
    '.sdkwork-assembly.json',
    'bin/assemble-sdk.mjs',
    'bin/build-typescript-generated-package',
    'bin/build-typescript-generated-package.cmd',
    'bin/build-typescript-generated-package.ps1',
    'bin/build-typescript-generated-package.mjs',
    'bin/generate-sdk.ps1',
    'bin/generate-sdk.sh',
    'bin/materialize-admin-flutter-workspace.mjs',
    'bin/normalize-generated-transport-package.mjs',
    'bin/verify-flutter-workspace.mjs',
    'bin/verify-typescript-workspace.mjs',
    'bin/verify-sdk.mjs',
    'openapi/README.md',
    'openapi/craw-chat-control-plane.openapi.json',
    'openapi/craw-chat-control-plane.sdkgen.json',
  ];

  for (const relativePath of requiredPaths) {
    if (!existsSync(path.join(workspaceRoot, relativePath))) {
      failures.push(`Missing required admin SDK workspace file: ${relativePath}`);
    }
  }

  if (existsSync(path.join(workspaceRoot, 'README.md'))) {
    const readmeSource = read(workspaceRoot, 'README.md');
    if (!/## Verification/.test(readmeSource)) {
      failures.push('Workspace README must document a verification entrypoint.');
    }
    if (!/verify-sdk/.test(readmeSource)) {
      failures.push('Workspace README must reference the verify-sdk command.');
    }
    if (!/craw-chat-control-plane\.openapi\.json/.test(readmeSource)) {
      failures.push('Workspace README must document the checked-in authority contract path.');
    }
    if (!/@sdkwork\/craw-chat-admin-backend-sdk/.test(readmeSource)) {
      failures.push('Workspace README must document the generated admin backend package root entrypoint.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(readmeSource)) {
      failures.push('Workspace README must explicitly forbid importing generated private source paths.');
    }
    if (/template_only_pending_generation|placeholder-only|Flutter remains placeholder-only/.test(readmeSource)) {
      failures.push('Workspace README must not describe the admin Flutter workspace as placeholder-only after materialization.');
    }
    if (!/Flutter workspace verification/.test(readmeSource)) {
      failures.push('Workspace README must document Flutter workspace verification.');
    }
  }

  if (existsSync(path.join(workspaceRoot, 'bin', 'verify-sdk.mjs'))) {
    const verifySdkSource = read(workspaceRoot, 'bin/verify-sdk.mjs');
    if (!/verifySdkAutomation/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifySdkAutomation.');
    }
    if (!/assembleSdk/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run assembleSdk.');
    }
    if (!/verifyTypeScriptWorkspace/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifyTypeScriptWorkspace.');
    }
    if (!/verifyFlutterWorkspace/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifyFlutterWorkspace.');
    }
  }

  for (const [wrapperPath, wrapperLabel] of [
    ['bin/generate-sdk.ps1', 'PowerShell generator wrapper'],
    ['bin/generate-sdk.sh', 'Shell generator wrapper'],
  ]) {
    if (!existsSync(path.join(workspaceRoot, wrapperPath))) {
      continue;
    }
    const wrapperSource = read(workspaceRoot, wrapperPath);
    if (!/normalize-generated-transport-package\.mjs/.test(wrapperSource)) {
      failures.push(`${wrapperLabel} must invoke normalize-generated-transport-package.mjs.`);
    }
  }

  const workspaceForwarders = [
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-gen.ps1',
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-gen.sh',
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-assemble.ps1',
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-assemble.sh',
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-verify.ps1',
    'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-verify.sh',
    'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi/package.json',
    'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-craw-chat-sdk-admin-typescript/composed/package.json',
    'sdkwork-craw-chat-sdk-admin-typescript/composed/src/index.ts',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-gen.ps1',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-gen.sh',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-assemble.ps1',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-assemble.sh',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-verify.ps1',
    'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-verify.sh',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/pubspec.yaml',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/pubspec_overrides.yaml',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/README.md',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/lib/craw_chat_admin_backend_sdk.dart',
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/lib/backend_client.dart',
    'sdkwork-craw-chat-sdk-admin-flutter/composed/pubspec.yaml',
    'sdkwork-craw-chat-sdk-admin-flutter/composed/pubspec_overrides.yaml',
    'sdkwork-craw-chat-sdk-admin-flutter/composed/README.md',
    'sdkwork-craw-chat-sdk-admin-flutter/composed/lib/craw_chat_sdk_admin.dart',
  ];

  for (const relativePath of workspaceForwarders) {
    if (!existsSync(path.join(workspaceRoot, relativePath))) {
      failures.push(`Missing TypeScript admin workspace file: ${relativePath}`);
    }
  }

  const generatedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-admin-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  if (existsSync(generatedPackagePath)) {
    const generatedPackage = JSON.parse(
      read(workspaceRoot, 'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi/package.json'),
    );
    if (!/Craw Chat control-plane API/i.test(String(generatedPackage.description || ''))) {
      failures.push('Admin generated package description must describe the Craw Chat control-plane API.');
    }
    const generatedKeywords = Array.isArray(generatedPackage.keywords) ? generatedPackage.keywords : [];
    for (const expectedKeyword of ['craw-chat', 'admin', 'control-plane']) {
      if (!generatedKeywords.includes(expectedKeyword)) {
        failures.push(`Admin generated package keywords must include "${expectedKeyword}".`);
      }
    }
  }

  if (!typeScriptReadmeSource) {
    failures.push('Missing TypeScript admin workspace README.');
  } else {
    if (!/sdk-verify/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript admin workspace README must reference sdk-verify.');
    }
    if (!/@sdkwork\/craw-chat-admin-backend-sdk/.test(typeScriptReadmeSource)) {
      failures.push(
        'TypeScript admin workspace README must document the generated admin backend package root entrypoint.',
      );
    }
    if (!/generated\/server-openapi\/src\/\*/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript admin workspace README must explicitly forbid importing generated private source paths.');
    }
  }

  const flutterReadmePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-admin-flutter',
    'README.md',
  );
  if (!existsSync(flutterReadmePath)) {
    failures.push('Missing Flutter admin workspace README.');
  } else {
    const flutterReadmeSource = read(workspaceRoot, 'sdkwork-craw-chat-sdk-admin-flutter/README.md');
    if (/template_only_pending_generation|future Flutter generated and composed/.test(flutterReadmeSource)) {
      failures.push('Flutter admin workspace README must not claim generation is pending after materialization.');
    }
    if (!/sdk-verify/.test(flutterReadmeSource)) {
      failures.push('Flutter admin workspace README must reference sdk-verify.');
    }
    if (!/CrawChatAdminClient/.test(flutterReadmeSource)) {
      failures.push('Flutter admin workspace README must document CrawChatAdminClient.');
    }
    if (!/craw_chat_admin_backend_sdk/.test(flutterReadmeSource)) {
      failures.push('Flutter admin workspace README must document the generated backend package root.');
    }
    if (!/generated\/server-openapi\/lib\/src/.test(flutterReadmeSource)) {
      failures.push('Flutter admin workspace README must forbid importing generated private lib/src paths.');
    }
  }

  const generatedReadmePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-admin-typescript',
    'generated',
    'server-openapi',
    'README.md',
  );
  if (existsSync(generatedReadmePath)) {
    const generatedReadmeSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi/README.md',
    );
    if (/Professional TypeScript SDK for SDKWork API\./.test(generatedReadmeSource)) {
      failures.push('Admin generated package README must not keep the generic SDKWork placeholder summary.');
    }
    for (const requiredSection of [
      '## Package Role',
      '## Quick Start',
      '## Authentication Modes',
      '## Endpoint Targeting',
      '## Surface Groups',
      '## Package Boundary',
    ]) {
      if (!generatedReadmeSource.includes(requiredSection)) {
        failures.push(`Admin generated package README must include a "${requiredSection}" section.`);
      }
    }
    if (!/@sdkwork\/craw-chat-admin-backend-sdk/.test(generatedReadmeSource)) {
      failures.push('Admin generated package README must document the generated package root entrypoint.');
    }
    if (!/@sdkwork\/craw-chat-sdk-admin/.test(generatedReadmeSource)) {
      failures.push('Admin generated package README must direct business consumers to the composed admin package.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(generatedReadmeSource)) {
      failures.push('Admin generated package README must forbid importing generated private source paths.');
    }
  }

  return failures;
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const failures = verifySdkAutomation();
  if (failures.length > 0) {
    console.error('[sdkwork-craw-chat-sdk-admin] SDK automation verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[sdkwork-craw-chat-sdk-admin] SDK automation verification passed.');
}
