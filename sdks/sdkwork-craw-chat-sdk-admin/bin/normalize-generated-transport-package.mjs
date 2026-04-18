#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function normalizeNewlines(value) {
  return value.replace(/\r?\n/g, '\n');
}

function ensureParentDirectory(filePath) {
  mkdirSync(path.dirname(filePath), { recursive: true });
}

function writeIfChanged(filePath, source) {
  const nextSource = normalizeNewlines(source);
  const currentSource = existsSync(filePath)
    ? normalizeNewlines(readFileSync(filePath, 'utf8'))
    : null;

  if (currentSource === nextSource) {
    return;
  }

  ensureParentDirectory(filePath);
  writeFileSync(filePath, nextSource, 'utf8');
}

function normalizePackageJson(filePath) {
  const currentPackage = existsSync(filePath)
    ? JSON.parse(readFileSync(filePath, 'utf8'))
    : {};

  const nextPackage = {
    ...currentPackage,
    description: 'Generated TypeScript transport package for the Craw Chat control-plane API',
    keywords: [
      'sdk',
      'api',
      'backend',
      'sdkwork',
      'craw-chat',
      'admin',
      'control-plane',
    ],
  };

  writeIfChanged(filePath, `${JSON.stringify(nextPackage, null, 2)}\n`);
}

function renderGeneratedReadme() {
  return `# @sdkwork/craw-chat-admin-backend-sdk

Generated TypeScript transport package for the Craw Chat control-plane API.

## Package Role

This package is the generator-owned transport layer for the checked-in control-plane OpenAPI
contract. Use it when you need direct access to generated HTTP operations and root-exported
transport types.

For business-facing integrations, prefer the composed package \`@sdkwork/craw-chat-sdk-admin\`,
which keeps the transport package behind a stable admin-oriented facade.

## Installation

\`\`\`bash
npm install @sdkwork/craw-chat-admin-backend-sdk
# or
yarn add @sdkwork/craw-chat-admin-backend-sdk
# or
pnpm add @sdkwork/craw-chat-admin-backend-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { SdkworkBackendClient } from '@sdkwork/craw-chat-admin-backend-sdk';

const client = new SdkworkBackendClient({
  baseUrl: 'http://127.0.0.1:18081',
  timeout: 30000,
});

client.setApiKey('your-control-plane-api-key');

const governance = await client.protocol.getApiV1ControlProtocolGovernance();
\`\`\`

## Authentication Modes

Choose exactly one authentication mode per client instance.

### Mode A: API Key

Recommended for service-to-service control-plane automation.

\`\`\`typescript
const client = new SdkworkBackendClient({ baseUrl: 'http://127.0.0.1:18081' });
client.setApiKey('your-control-plane-api-key');
// Sends: Authorization: Bearer <apiKey>
\`\`\`

### Mode B: Dual Token

Use this when the target deployment expects both a bearer token and a delegated access token.

\`\`\`typescript
const client = new SdkworkBackendClient({ baseUrl: 'http://127.0.0.1:18081' });
client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
\`\`\`

Do not combine \`setApiKey(...)\` with \`setAuthToken(...)\` and \`setAccessToken(...)\` on the
same client instance.

## Endpoint Targeting

- For standalone governance development, point \`baseUrl\` to the direct \`control-plane-api\`
  origin, typically \`http://127.0.0.1:18081\`.
- For packaged installs, point \`baseUrl\` to the unified \`craw-chat-server\` or \`web-gateway\`
  public origin.
- Keep one deployment model per client configuration. Do not mix direct control-plane and unified
  gateway assumptions in the same client instance.

## Surface Groups

- \`client.cluster\` - cluster governance and node lifecycle operations
- \`client.protocol\` - protocol governance and contract inspection operations
- \`client.providers\` - provider-binding and provider runtime operations
- \`client.social\` - social runtime control-plane operations
- \`client.system\` - health and system-level control-plane operations

## Package Boundary

- Use only the package root entrypoint: \`@sdkwork/craw-chat-admin-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private generator paths from downstream code.
- Keep business orchestration in the composed package \`@sdkwork/craw-chat-sdk-admin\` instead of
  re-exporting generated internals.

## Regeneration Contract

- Generator-owned files are tracked in \`.sdkwork/sdkwork-generator-manifest.json\`.
- Each run also writes \`.sdkwork/sdkwork-generator-changes.json\` so automation can inspect
  created, updated, deleted, unchanged, scaffolded, and backed-up files for the latest generation.
- Apply mode also writes \`.sdkwork/sdkwork-generator-report.json\` with the full execution report,
  including \`schemaVersion\`, \`generator\`, stable artifact paths, and the execution handoff
  commands that match CLI \`--json\` output.
- Put hand-written wrappers, adapters, and orchestration in \`custom/\`.
- Files scaffolded under \`custom/\` are created once and preserved across regenerations.
`;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-admin-typescript',
  'generated',
  'server-openapi',
);

normalizePackageJson(path.join(generatedRoot, 'package.json'));
writeIfChanged(path.join(generatedRoot, 'README.md'), renderGeneratedReadme());

console.log('[sdkwork-craw-chat-sdk-admin] Normalized generated transport package metadata.');
