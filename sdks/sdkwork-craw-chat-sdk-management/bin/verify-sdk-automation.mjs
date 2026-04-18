#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { verifySdkSiteDocs } from '../../../docs/sites/sdk/verify-sdk-site-docs.mjs';

function read(workspaceRoot, relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

export function verifySdkAutomation(options = {}) {
  const workspaceRoot = options.workspaceRoot || path.resolve(import.meta.dirname, '..');
  const repoRoot = path.resolve(workspaceRoot, '..', '..');
  const failures = [];
  failures.push(...verifySdkSiteDocs({ rootDir: repoRoot }));

  const requiredPaths = [
    '.sdkwork-assembly.json',
    'README.md',
    'bin/assemble-sdk.mjs',
    'bin/build-typescript-generated-package',
    'bin/build-typescript-generated-package.cmd',
    'bin/build-typescript-generated-package.ps1',
    'bin/build-typescript-generated-package.mjs',
    'bin/generate-sdk.cmd',
    'bin/generate-sdk.ps1',
    'bin/generate-sdk.sh',
    'bin/materialize-management-authority.mjs',
    'bin/materialize-management-flutter-workspace.mjs',
    'bin/materialize-management-typescript-workspace.mjs',
    'bin/verify-flutter-workspace.mjs',
    'bin/verify-sdk.mjs',
    'bin/verify-sdk-automation.mjs',
    'bin/verify-typescript-generated-package.mjs',
    'bin/verify-typescript-workspace.mjs',
    'openapi/README.md',
    'openapi/craw-chat-management.openapi.json',
    'openapi/craw-chat-management.sdkgen.json',
    'sdkwork-craw-chat-sdk-management-flutter/README.md',
    'sdkwork-craw-chat-sdk-management-typescript/README.md',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/pubspec.yaml',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/pubspec_overrides.yaml',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/README.md',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/lib/craw_chat_management_backend_sdk.dart',
    'sdkwork-craw-chat-sdk-management-flutter/generated/server-openapi/lib/backend_client.dart',
    'sdkwork-craw-chat-sdk-management-flutter/composed/pubspec.yaml',
    'sdkwork-craw-chat-sdk-management-flutter/composed/pubspec_overrides.yaml',
    'sdkwork-craw-chat-sdk-management-flutter/composed/README.md',
    'sdkwork-craw-chat-sdk-management-flutter/composed/lib/craw_chat_sdk_management.dart',
    'sdkwork-craw-chat-sdk-management-typescript/generated/server-openapi/package.json',
    'sdkwork-craw-chat-sdk-management-typescript/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-craw-chat-sdk-management-typescript/composed/package.json',
    'sdkwork-craw-chat-sdk-management-typescript/composed/src/index.ts',
  ];

  for (const relativePath of requiredPaths) {
    if (!existsSync(path.join(workspaceRoot, relativePath))) {
      failures.push(`Missing required management SDK workspace file: ${relativePath}`);
    }
  }

  if (existsSync(path.join(workspaceRoot, 'README.md'))) {
    const readmeSource = read(workspaceRoot, 'README.md');
    if (!/verify-sdk/.test(readmeSource)) {
      failures.push('Workspace README must reference the verify-sdk command.');
    }
    if (!/@sdkwork\/craw-chat-management-backend-sdk/.test(readmeSource)) {
      failures.push('Workspace README must document the generated management backend package root entrypoint.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(readmeSource)) {
      failures.push('Workspace README must explicitly forbid importing generated private source paths.');
    }
    if (!/materializes a standard two-layer TypeScript SDK workspace/.test(readmeSource)) {
      failures.push('Workspace README must describe the materialized TypeScript workspace.');
    }
    if (/template_only_pending_generation|placeholder-only|Flutter remains placeholder-only/.test(readmeSource)) {
      failures.push('Workspace README must not describe the management Flutter workspace as placeholder-only after materialization.');
    }
    if (!/Flutter workspace verification/.test(readmeSource)) {
      failures.push('Workspace README must document Flutter workspace verification.');
    }
  }

  if (existsSync(path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-management-typescript', 'README.md'))) {
    const typeScriptReadmeSource = read(workspaceRoot, 'sdkwork-craw-chat-sdk-management-typescript/README.md');
    if (/TypeScript generation: pending/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript workspace README must not claim generation is pending after workspace materialization.');
    }
    if (!/sdk-verify/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript management workspace README must reference sdk-verify.');
    }
    if (!/CrawChatSdkManagementClient/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript management workspace README must document CrawChatSdkManagementClient.');
    }
    if (!/@sdkwork\/craw-chat-management-backend-sdk/.test(typeScriptReadmeSource)) {
      failures.push(
        'TypeScript management workspace README must document the generated management backend package root entrypoint.',
      );
    }
    if (!/generated\/server-openapi\/src\/\*/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript management workspace README must explicitly forbid importing generated private source paths.');
    }
  }

  const generatedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  if (existsSync(generatedPackagePath)) {
    const generatedPackage = JSON.parse(
      read(workspaceRoot, 'sdkwork-craw-chat-sdk-management-typescript/generated/server-openapi/package.json'),
    );
    if (!/operator-console management backend/i.test(String(generatedPackage.description || ''))) {
      failures.push(
        'Management generated package description must describe the Craw Chat operator-console management backend.',
      );
    }
    const generatedKeywords = Array.isArray(generatedPackage.keywords) ? generatedPackage.keywords : [];
    for (const expectedKeyword of ['craw-chat', 'management', 'operator-console']) {
      if (!generatedKeywords.includes(expectedKeyword)) {
        failures.push(`Management generated package keywords must include "${expectedKeyword}".`);
      }
    }
  }

  const generatedReadmePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'generated',
    'server-openapi',
    'README.md',
  );
  if (existsSync(generatedReadmePath)) {
    const generatedReadmeSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/generated/server-openapi/README.md',
    );
    if (/Generated transport package for the Craw Chat operator-console management backend\./.test(generatedReadmeSource)
      && !/## Package Role/.test(generatedReadmeSource)) {
      failures.push('Management generated package README must be expanded beyond the minimal placeholder summary.');
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
        failures.push(`Management generated package README must include a "${requiredSection}" section.`);
      }
    }
    if (!/@sdkwork\/craw-chat-management-backend-sdk/.test(generatedReadmeSource)) {
      failures.push('Management generated package README must document the generated package root entrypoint.');
    }
    if (!/@sdkwork\/craw-chat-sdk-management/.test(generatedReadmeSource)) {
      failures.push('Management generated package README must direct business consumers to the composed management package.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(generatedReadmeSource)) {
      failures.push('Management generated package README must forbid importing generated private source paths.');
    }
  }

  const composedReadmePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'README.md',
  );
  if (existsSync(composedReadmePath)) {
    const composedReadmeSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/composed/README.md',
    );
    if (!/CrawChatSdkManagementClient/.test(composedReadmeSource)) {
      failures.push('Management composed package README must document CrawChatSdkManagementClient.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(composedReadmeSource)) {
      failures.push('Management composed package README must explicitly forbid importing generated private source paths.');
    }
  }

  const flutterReadmePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-flutter',
    'README.md',
  );
  if (existsSync(flutterReadmePath)) {
    const flutterReadmeSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-flutter/README.md',
    );
    if (/template_only_pending_generation|future Flutter generated and composed|Flutter generation: pending/.test(flutterReadmeSource)) {
      failures.push('Flutter management workspace README must not claim generation is pending after materialization.');
    }
    if (!/sdk-verify/.test(flutterReadmeSource)) {
      failures.push('Flutter management workspace README must reference sdk-verify.');
    }
    if (!/CrawChatManagementClient/.test(flutterReadmeSource)) {
      failures.push('Flutter management workspace README must document CrawChatManagementClient.');
    }
    if (!/craw_chat_management_backend_sdk/.test(flutterReadmeSource)) {
      failures.push('Flutter management workspace README must document the generated backend package root.');
    }
    if (!/generated\/server-openapi\/lib\/src/.test(flutterReadmeSource)) {
      failures.push('Flutter management workspace README must forbid importing generated private lib/src paths.');
    }
  }

  if (existsSync(path.join(workspaceRoot, 'bin', 'verify-sdk.mjs'))) {
    const verifySdkSource = read(workspaceRoot, 'bin/verify-sdk.mjs');
    if (!/verifySdkAutomation/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifySdkAutomation.');
    }
    if (!/verifyTypeScriptWorkspace/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifyTypeScriptWorkspace.');
    }
    if (!/verifyFlutterWorkspace/.test(verifySdkSource)) {
      failures.push('verify-sdk.mjs must run verifyFlutterWorkspace.');
    }
  }

  return failures;
}

if (process.argv[1] && path.resolve(process.argv[1]) === import.meta.filename) {
  const failures = verifySdkAutomation();
  if (failures.length > 0) {
    console.error('[sdkwork-craw-chat-sdk-management] SDK automation verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[sdkwork-craw-chat-sdk-management] SDK automation verification passed.');
}
