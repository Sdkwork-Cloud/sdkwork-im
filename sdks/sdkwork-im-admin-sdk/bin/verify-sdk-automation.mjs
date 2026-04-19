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
    'bin/materialize-im-admin-authority.mjs',
    'bin/materialize-im-admin-flutter-workspace.mjs',
    'bin/materialize-im-admin-typescript-workspace.mjs',
    'bin/verify-flutter-workspace.mjs',
    'bin/verify-sdk.mjs',
    'bin/verify-sdk-automation.mjs',
    'bin/verify-typescript-generated-package.mjs',
    'bin/verify-typescript-workspace.mjs',
    'openapi/README.md',
    'openapi/im-admin.openapi.json',
    'openapi/im-admin.sdkgen.json',
    'sdkwork-im-admin-sdk-flutter/README.md',
    'sdkwork-im-admin-sdk-typescript/README.md',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/pubspec.yaml',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/pubspec_overrides.yaml',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/README.md',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/lib/im_admin_backend_sdk.dart',
    'sdkwork-im-admin-sdk-flutter/generated/server-openapi/lib/backend_client.dart',
    'sdkwork-im-admin-sdk-flutter/composed/pubspec.yaml',
    'sdkwork-im-admin-sdk-flutter/composed/pubspec_overrides.yaml',
    'sdkwork-im-admin-sdk-flutter/composed/README.md',
    'sdkwork-im-admin-sdk-flutter/composed/lib/im_admin_sdk.dart',
    'sdkwork-im-admin-sdk-typescript/generated/server-openapi/package.json',
    'sdkwork-im-admin-sdk-typescript/generated/server-openapi/sdkwork-sdk.json',
    'sdkwork-im-admin-sdk-typescript/composed/package.json',
    'sdkwork-im-admin-sdk-typescript/composed/src/index.ts',
  ];

  for (const relativePath of requiredPaths) {
    if (!existsSync(path.join(workspaceRoot, relativePath))) {
      failures.push(`Missing required IM admin SDK workspace file: ${relativePath}`);
    }
  }

  if (existsSync(path.join(workspaceRoot, 'README.md'))) {
    const readmeSource = read(workspaceRoot, 'README.md');
    if (!/verify-sdk/.test(readmeSource)) {
      failures.push('Workspace README must reference the verify-sdk command.');
    }
    if (!/@sdkwork\/im-admin-backend-sdk/.test(readmeSource)) {
      failures.push('Workspace README must document the generated IM admin backend package root entrypoint.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(readmeSource)) {
      failures.push('Workspace README must explicitly forbid importing generated private source paths.');
    }
    if (!/materializes a standard two-layer TypeScript SDK workspace/.test(readmeSource)) {
      failures.push('Workspace README must describe the materialized IM admin TypeScript workspace.');
    }
    if (/template_only_pending_generation|placeholder-only|Flutter remains placeholder-only/.test(readmeSource)) {
      failures.push('Workspace README must not describe the IM admin Flutter workspace as placeholder-only after materialization.');
    }
    if (!/Flutter workspace verification/.test(readmeSource)) {
      failures.push('Workspace README must document Flutter workspace verification.');
    }
  }

  if (existsSync(path.join(workspaceRoot, 'sdkwork-im-admin-sdk-typescript', 'README.md'))) {
    const typeScriptReadmeSource = read(workspaceRoot, 'sdkwork-im-admin-sdk-typescript/README.md');
    if (/TypeScript generation: pending/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript workspace README must not claim generation is pending after workspace materialization.');
    }
    if (!/sdk-verify/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript IM admin workspace README must reference sdk-verify.');
    }
    if (!/ImAdminSdkClient/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript IM admin workspace README must document ImAdminSdkClient.');
    }
    if (!/@sdkwork\/im-admin-backend-sdk/.test(typeScriptReadmeSource)) {
      failures.push(
        'TypeScript IM admin workspace README must document the generated IM admin backend package root entrypoint.',
      );
    }
    if (!/generated\/server-openapi\/src\/\*/.test(typeScriptReadmeSource)) {
      failures.push('TypeScript IM admin workspace README must explicitly forbid importing generated private source paths.');
    }
  }

  const generatedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  if (existsSync(generatedPackagePath)) {
    const generatedPackage = JSON.parse(
      read(workspaceRoot, 'sdkwork-im-admin-sdk-typescript/generated/server-openapi/package.json'),
    );
    if (!/IM admin backend/i.test(String(generatedPackage.description || ''))) {
      failures.push(
        'IM admin generated package description must describe the IM admin backend.',
      );
    }
    const generatedKeywords = Array.isArray(generatedPackage.keywords) ? generatedPackage.keywords : [];
    for (const expectedKeyword of ['sdkwork', 'im', 'admin']) {
      if (!generatedKeywords.includes(expectedKeyword)) {
        failures.push(`IM admin generated package keywords must include "${expectedKeyword}".`);
      }
    }
  }

  const generatedReadmePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'generated',
    'server-openapi',
    'README.md',
  );
  if (existsSync(generatedReadmePath)) {
    const generatedReadmeSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/generated/server-openapi/README.md',
    );
    if (/Generated TypeScript transport package for the IM admin backend\./.test(generatedReadmeSource)
      && !/## Package Role/.test(generatedReadmeSource)) {
      failures.push('IM admin generated package README must be expanded beyond the minimal placeholder summary.');
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
        failures.push(`IM admin generated package README must include a "${requiredSection}" section.`);
      }
    }
    if (!/@sdkwork\/im-admin-backend-sdk/.test(generatedReadmeSource)) {
      failures.push('IM admin generated package README must document the generated package root entrypoint.');
    }
    if (!/@sdkwork\/im-admin-sdk/.test(generatedReadmeSource)) {
      failures.push('IM admin generated package README must direct business consumers to the composed IM admin package.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(generatedReadmeSource)) {
      failures.push('IM admin generated package README must forbid importing generated private source paths.');
    }
  }

  const composedReadmePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'README.md',
  );
  if (existsSync(composedReadmePath)) {
    const composedReadmeSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/README.md',
    );
    if (!/ImAdminSdkClient/.test(composedReadmeSource)) {
      failures.push('IM admin composed package README must document ImAdminSdkClient.');
    }
    if (!/generated\/server-openapi\/src\/\*/.test(composedReadmeSource)) {
      failures.push('IM admin composed package README must explicitly forbid importing generated private source paths.');
    }
  }

  const flutterReadmePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-flutter',
    'README.md',
  );
  if (existsSync(flutterReadmePath)) {
    const flutterReadmeSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-flutter/README.md',
    );
    if (/template_only_pending_generation|future Flutter generated and composed|Flutter generation: pending/.test(flutterReadmeSource)) {
      failures.push('Flutter IM admin workspace README must not claim generation is pending after materialization.');
    }
    if (!/sdk-verify/.test(flutterReadmeSource)) {
      failures.push('Flutter IM admin workspace README must reference sdk-verify.');
    }
    if (!/ImAdminSdkClient/.test(flutterReadmeSource)) {
      failures.push('Flutter IM admin workspace README must document ImAdminSdkClient.');
    }
    if (!/im_admin_backend_sdk/.test(flutterReadmeSource)) {
      failures.push('Flutter IM admin workspace README must document the generated backend package root.');
    }
    if (!/generated\/server-openapi\/lib\/src/.test(flutterReadmeSource)) {
      failures.push('Flutter IM admin workspace README must forbid importing generated private lib/src paths.');
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
    console.error('[sdkwork-im-admin-sdk] SDK automation verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[sdkwork-im-admin-sdk] SDK automation verification passed.');
}
