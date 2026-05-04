import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import { routeGroups, routes } from './materialize-im-admin-authority.mjs';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const typescriptRoot = path.join(
  workspaceRoot,
  'sdkwork-im-admin-sdk-typescript',
);
const generatedRoot = path.join(typescriptRoot, 'generated', 'server-openapi');
const composedRoot = path.join(typescriptRoot, 'composed');

function writeFile(targetPath, source) {
  mkdirSync(path.dirname(targetPath), { recursive: true });
  writeFileSync(targetPath, source, 'utf8');
}

function pascalCase(value) {
  return value
    .replace(/(^|[-_/])([a-z])/g, (_, __, letter) => letter.toUpperCase())
    .replace(/[^A-Za-z0-9]/g, '');
}

function extractRouteParams(routePath) {
  return [...routePath.matchAll(/\{([^}]+)\}/g)].map((match) => match[1]);
}

function renderInterpolatedPath(routePath) {
  const escaped = routePath.replace(/`/g, '\\`');
  return escaped.replace(/\{([^}]+)\}/g, (_, paramName) => `\${encodeURIComponent(String(${paramName}))}`);
}

function renderMethod(route) {
  const [method, routePath, operationId] = route;
  const params = extractRouteParams(routePath);
  const signatureParts = params.map((paramName) => `${paramName}: string | number`);
  const callArgs = [...params];

  if (method !== 'get' && method !== 'delete') {
    signatureParts.push('body: LooseJsonObject');
    callArgs.push('body');
  }

  const signature = signatureParts.join(', ');
  const pathExpression = `backendApiPath(\`${renderInterpolatedPath(routePath)}\`)`;
  const bodyArgument = callArgs.length > params.length ? ', body' : '';
  const invocation = `this.client.${method}<LooseJsonValue>(${pathExpression}${bodyArgument})`;

  return [
    `  async ${operationId}(${signature}): Promise<LooseJsonValue> {`,
    `    return ${invocation};`,
    '  }',
  ].join('\n');
}

function renderApiFile(groupName) {
  const className = `${pascalCase(groupName)}Api`;
  const groupRoutes = routes.filter((route) => route[3] === groupName);
  const methods = groupRoutes.map(renderMethod).join('\n\n');

  return `import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class ${className} {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

${methods}
}

export function create${className}(client: HttpClient): ${className} {
  return new ${className}(client);
}
`;
}

function renderApiIndex() {
  return [
    "export { BACKEND_API_PREFIX, backendApiPath } from './paths';",
    ...Object.keys(routeGroups).map((groupName) => {
      const className = `${pascalCase(groupName)}Api`;
      return `export { ${className}, create${className} } from './${groupName}';`;
    }),
    '',
  ].join('\n');
}

function renderGeneratedSdk() {
  const imports = Object.keys(routeGroups).map((groupName) => {
    const className = `${pascalCase(groupName)}Api`;
    return `import { ${className}, create${className} } from './api/${groupName}';`;
  }).join('\n');

  const fields = Object.keys(routeGroups).map((groupName) => {
    const className = `${pascalCase(groupName)}Api`;
    return `  public readonly ${groupName}: ${className};`;
  }).join('\n');

  const assignments = Object.keys(routeGroups).map((groupName) => {
    const className = `${pascalCase(groupName)}Api`;
    return `    this.${groupName} = create${className}(this.httpClient);`;
  }).join('\n');

  return `import { HttpClient, createHttpClient } from './http/client';
import type { ImAdminBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

${imports}

export class ImAdminBackendClient {
  private httpClient: HttpClient;

${fields}

  constructor(config: ImAdminBackendConfig) {
    this.httpClient = createHttpClient(config);
${assignments}
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createImAdminBackendClient(config: ImAdminBackendConfig): ImAdminBackendClient {
  return new ImAdminBackendClient(config);
}

export default ImAdminBackendClient;
`;
}

function renderGeneratedPackageJson() {
  const stableBuildCommand = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';

  return `${JSON.stringify({
    name: '@sdkwork/im-admin-backend-sdk',
    version: '0.1.0',
    description: 'Generated TypeScript transport package for the IM admin backend',
    author: 'SDKWork Team',
    license: 'MIT',
    type: 'module',
    main: './dist/index.cjs',
    module: './dist/index.js',
    types: './dist/index.d.ts',
    files: ['dist'],
    exports: {
      '.': {
        types: './dist/index.d.ts',
        import: './dist/index.js',
        require: './dist/index.cjs',
      },
    },
    scripts: {
      build: stableBuildCommand,
      dev: 'vite build --watch',
      prepublishOnly: 'npm run build',
    },
    dependencies: {
      '@sdkwork/sdk-common': '^1.0.2',
    },
    devDependencies: {
      '@types/node': '^20.0.0',
      typescript: '^5.3.0',
      vite: '^7.0.0',
      'vite-plugin-dts': '^4.0.0',
    },
    keywords: ['sdk', 'api', 'backend', 'sdkwork', 'im', 'admin'],
  }, null, 2)}\n`;
}

function renderGeneratedReadme() {
  return `# @sdkwork/im-admin-backend-sdk

Generated TypeScript transport package for the IM admin backend.

## Package Role

This package is the generator-owned transport layer for the checked-in IM admin OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing admin integrations, prefer the composed package
\`@sdkwork/im-admin-sdk\`, which keeps the transport layer behind the stable
\`ImAdminSdkClient\` facade.

## Installation

\`\`\`bash
npm install @sdkwork/im-admin-backend-sdk
# or
yarn add @sdkwork/im-admin-backend-sdk
# or
pnpm add @sdkwork/im-admin-backend-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { ImAdminBackendClient } from '@sdkwork/im-admin-backend-sdk';

const client = new ImAdminBackendClient({
  baseUrl: 'https://your-admin-origin.example.com',
  timeout: 30000,
});

client.setAuthToken('operator-session-token');

const tenants = await client.tenants.listTenants();
console.log(tenants);
\`\`\`

## Authentication Modes

This admin backend surface is bearer-token based.

\`\`\`typescript
const client = new ImAdminBackendClient({
  baseUrl: 'https://your-admin-origin.example.com',
});

client.setAuthToken('operator-session-token');
\`\`\`

## Endpoint Targeting

- Configure \`baseUrl\` to the origin that serves the checked-in \`/api/admin/*\` contract for the
  current environment.
- In packaged installs, that target is the unified public origin that fronts the admin gateway.
- In direct backend development, point \`baseUrl\` to the IM admin backend origin that already
  owns the \`/api/admin/*\` surface for that environment.

## Surface Groups

${Object.keys(routeGroups).map((groupName) => `- \`client.${groupName}\``).join('\n')}

## Package Boundary

- Use only the package root entrypoint: \`@sdkwork/im-admin-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private generator paths from downstream code.
- Keep business orchestration in the composed package \`@sdkwork/im-admin-sdk\`
  instead of re-exporting generated internals.

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

function renderGeneratedSdkworkConfig() {
  return `${JSON.stringify({
    language: 'typescript',
    name: 'sdkwork-im-admin-sdk',
    version: '0.1.0',
    sdkType: 'backend',
    packageName: '@sdkwork/im-admin-backend-sdk',
    generator: '@sdkwork/sdk-generator',
    capabilities: {
      supportsGeneratedTests: true,
      supportsReadme: true,
      supportsCustomScaffold: true,
      supportsPublishWorkflow: true,
      hasDistinctBuildStep: true,
    },
    generation: {
      readme: true,
      tests: false,
    },
    ownership: {
      generatedOwnership: 'generated',
      scaffoldOwnership: 'scaffold',
      scaffoldRoots: ['custom/'],
      stateRoots: ['.sdkwork/'],
    },
  }, null, 2)}\n`;
}

function renderComposedPackageJson() {
  const renderTaskCommand = (task) =>
    `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;

  return `${JSON.stringify({
    name: '@sdkwork/im-admin-sdk',
    version: '0.1.0',
    description: 'Composed IM admin TypeScript SDK built on the generated admin backend SDK',
    type: 'module',
    main: './dist/index.js',
    module: './dist/index.js',
    types: './dist/index.d.ts',
    exports: {
      '.': {
        types: './dist/index.d.ts',
        import: './dist/index.js',
        default: './dist/index.js',
      },
    },
    sideEffects: false,
    files: ['dist', 'src'],
    dependencies: {
      '@sdkwork/im-admin-backend-sdk': 'file:../generated/server-openapi',
    },
    scripts: {
      typecheck: renderTaskCommand('typecheck'),
      build: renderTaskCommand('build'),
      test: renderTaskCommand('test'),
    },
  }, null, 2)}\n`;
}

function renderComposedRunTsc() {
  return `#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { resolveGeneratorModulePath } from '../../../bin/generator-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const workspaceRoot = path.resolve(packageRoot, '..', '..');
const tscPath = resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc');

const result = spawnSync(process.execPath, [tscPath, ...process.argv.slice(2)], {
  cwd: packageRoot,
  stdio: 'inherit',
  shell: false,
});

if (result.error) {
  throw result.error;
}

process.exit(result.status ?? 1);
`;
}

function renderComposedPackageTask() {
  return `#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(\`[sdkwork-im-admin-sdk] \${message}\`);
  process.exit(1);
}

function run(step, args, cwd = packageRoot) {
  const result = spawnSync(process.execPath, args, {
    cwd,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    fail(\`\${step} failed to start: \${result.error.message}\`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(\`\${step} failed with exit code \${result.status}\`);
  }
  if (result.signal) {
    fail(\`\${step} terminated with signal \${result.signal}\`);
  }
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const task = (process.argv[2] || '').trim();

switch (task) {
  case 'typecheck':
    run('typescript:composed-typecheck', [path.join(scriptDir, 'run-tsc.mjs'), '-p', 'tsconfig.build.json', '--noEmit']);
    break;
  case 'build':
    run('typescript:composed-build', [path.join(scriptDir, 'run-tsc.mjs'), '-p', 'tsconfig.build.json']);
    run('typescript:composed-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    break;
  case 'test':
    run('typescript:composed-test', [path.join(packageRoot, 'test', 'im-admin-sdk-client.test.mjs')]);
    break;
  default:
    fail(\`Unsupported package task "\${task}". Expected one of: typecheck, build, test.\`);
}
`;
}

function renderComposedTypes() {
  const groups = Object.keys(routeGroups);
  const clientGroupKeys = groups.map((groupName) => `  | '${groupName}'`).join('\n');

  return `import type {
  ImAdminBackendClient,
  ImAdminBackendConfig,
} from '@sdkwork/im-admin-backend-sdk';

export type { ImAdminBackendConfig } from '@sdkwork/im-admin-backend-sdk';

export interface ImAdminSdkClientCreateOptions {
  backendClient?: ImAdminBackendClientLike;
  backendConfig?: ImAdminBackendConfig;
}

export interface ImAdminSdkClientOptions {
  backendClient: ImAdminBackendClientLike;
}

export type ImAdminBackendClientLike = Pick<
  ImAdminBackendClient,
${clientGroupKeys}
  | 'setAuthToken'
  | 'setTokenManager'
  | 'http'
>;
`;
}

function renderComposedSdkContext() {
  return `import type {
  ImAdminBackendClientLike,
  ImAdminSdkClientCreateOptions,
  ImAdminBackendConfig,
} from './types.js';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

async function dynamicImportModule(moduleName: string): Promise<unknown> {
  const dynamicImport = new Function('name', 'return import(name);') as (name: string) => Promise<unknown>;
  return dynamicImport(moduleName);
}

export async function createGeneratedBackendClient(
  backendConfig: ImAdminBackendConfig,
): Promise<ImAdminBackendClientLike> {
  const moduleExport = await dynamicImportModule('@sdkwork/im-admin-backend-sdk');
  const createClient = isRecord(moduleExport) ? moduleExport.createImAdminBackendClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error('Unable to resolve @sdkwork/im-admin-backend-sdk createImAdminBackendClient factory');
  }
  return createClient(backendConfig) as Promise<ImAdminBackendClientLike>;
}

export async function resolveBackendClient(
  options: ImAdminSdkClientCreateOptions,
): Promise<ImAdminBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }
  if (options.backendConfig) {
    return createGeneratedBackendClient(options.backendConfig);
  }
  throw new Error('backendClient or backendConfig is required');
}

export class ImAdminSdkContext {
  constructor(readonly backendClient: ImAdminBackendClientLike) {}

  setAuthToken(token: string): void {
    this.backendClient.setAuthToken?.(token);
  }
}
`;
}

function renderComposedSdk() {
  const groups = Object.keys(routeGroups);
  const fields = groups.map((groupName) => `  readonly ${groupName}: ImAdminBackendClientLike['${groupName}'];`).join('\n');
  const assignments = groups.map((groupName) => `    this.${groupName} = options.backendClient.${groupName};`).join('\n');

  return `import { ImAdminSdkContext, resolveBackendClient } from './sdk-context.js';
import type {
  ImAdminBackendClientLike,
  ImAdminSdkClientCreateOptions,
  ImAdminSdkClientOptions,
} from './types.js';

export class ImAdminSdkClient {
  private readonly context: ImAdminSdkContext;

  readonly backendClient: ImAdminBackendClientLike;
${fields}

  constructor(options: ImAdminSdkClientOptions) {
    this.context = new ImAdminSdkContext(options.backendClient);
    this.backendClient = options.backendClient;
${assignments}
  }

  static async create(
    options: ImAdminSdkClientCreateOptions,
  ): Promise<ImAdminSdkClient> {
    return new ImAdminSdkClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }
}

export async function createImAdminSdkClient(
  options: ImAdminSdkClientCreateOptions,
): Promise<ImAdminSdkClient> {
  return ImAdminSdkClient.create(options);
}
`;
}

function renderComposedTest() {
  const groups = Object.keys(routeGroups);
  const groupStubs = groups.map((groupName) => `  const ${groupName} = { marker: '${groupName}' };`).join('\n');
  const backendFields = groups.map((groupName) => `    ${groupName},`).join('\n');
  const assertions = groups.map((groupName) => `  assert.equal(sdk.${groupName}, backendClient.${groupName});`).join('\n');

  return `import assert from 'node:assert/strict';

import { ImAdminSdkClient } from '../dist/index.js';

function createBackendClientStub() {
  const calls = [];
${groupStubs}

  const backendClient = {
${backendFields}
    http: {
      async request() {
        calls.push({ method: 'http.request' });
        return {};
      },
    },
    setAuthToken(token) {
      calls.push({ method: 'setAuthToken', token });
      return backendClient;
    },
    setTokenManager(manager) {
      calls.push({ method: 'setTokenManager', manager });
      return backendClient;
    },
  };

  return { backendClient, calls };
}

async function testCreateFactoryAndTokenHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = await ImAdminSdkClient.create({ backendClient });

  sdk.setAuthToken('auth-token');

  assert.deepEqual(calls.slice(-1), [
    { method: 'setAuthToken', token: 'auth-token' },
  ]);
}

function testConstructorSurface() {
  const { backendClient } = createBackendClientStub();
  const sdk = new ImAdminSdkClient({ backendClient });

  assert.equal(sdk.backendClient, backendClient);
${assertions}
}

await testCreateFactoryAndTokenHelpers();
testConstructorSurface();

console.log('im-admin composed sdk smoke tests passed');
`;
}

function renderTypeScriptWorkspaceReadme() {
  return `# sdkwork-im-admin-sdk-typescript

TypeScript language workspace for \`sdkwork-im-admin-sdk\`.

## Package Layers

- generated package: \`@sdkwork/im-admin-backend-sdk\`
- composed package: \`@sdkwork/im-admin-sdk\`

The preferred consumer entrypoint is \`ImAdminSdkClient\`.

## Verification

- \`./bin/sdk-verify.sh\`
- \`./bin/sdk-verify.ps1\`
- root workspace command \`node ./sdks/sdkwork-im-admin-sdk/bin/verify-sdk.mjs\`

## Package Boundary

- Consume generated transport symbols only through \`@sdkwork/im-admin-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private source paths from manual or downstream code.

## Contract Inputs

- \`../openapi/im-admin.openapi.json\`
- \`../openapi/im-admin.sdkgen.json\`

## Smoke Coverage

- composed smoke test at \`composed/test/im-admin-sdk-client.test.mjs\`
`;
}

function renderGeneratedHttpClient() {
  return `import type { ImAdminBackendConfig } from '../types/common';
import type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { BaseHttpClient, SdkError, SUCCESS_CODES, withRetry } from '@sdkwork/sdk-common';

type HttpRequestOptions = RequestOptions & {
  body?: unknown;
  headers?: Record<string, string>;
  contentType?: string;
};

type ResultEnvelope = {
  code?: string | number;
  msg?: string;
  message?: string;
  data?: unknown;
  error?: {
    message?: string;
  };
};

function hasOwn(value: object, key: string): boolean {
  return Object.prototype.hasOwnProperty.call(value, key);
}

export class HttpClient extends BaseHttpClient {
  constructor(config: ImAdminBackendConfig) {
    super(config as any);
  }

  private buildRequestHeaders(
    headers?: Record<string, string>,
    contentType?: string,
  ): Record<string, string> | undefined {
    const mergedHeaders = {
      ...(headers ?? {}),
    };

    if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {
      mergedHeaders['Content-Type'] = contentType;
    }

    return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;
  }

  private isResultEnvelope(value: unknown): value is ResultEnvelope {
    return typeof value === 'object'
      && value !== null
      && (hasOwn(value, 'code') || hasOwn(value, 'data') || hasOwn(value, 'msg') || hasOwn(value, 'message'));
  }

  private hasSuccessCode(code: unknown): boolean {
    return SUCCESS_CODES.includes(code as never) || SUCCESS_CODES.includes(String(code) as never);
  }

  private async handleErrorResponse(response: Response, requestConfig: unknown): Promise<never> {
    let payload: unknown = null;

    try {
      payload = await response.json();
    } catch {
      payload = null;
    }

    let message = \`HTTP \${response.status}: \${response.statusText}\`;
    if (typeof payload === 'object' && payload !== null) {
      const candidate = payload as ResultEnvelope;
      message = candidate.error?.message?.trim()
        || candidate.msg?.trim()
        || candidate.message?.trim()
        || message;
    }

    const error = SdkError.fromHttpStatus(response.status, message);
    const applyErrorInterceptors = (this as any).applyErrorInterceptors;
    if (typeof applyErrorInterceptors === 'function') {
      await applyErrorInterceptors.call(this, error, requestConfig);
    }

    throw error;
  }

  async processResponse<T>(response: Response, requestConfig: unknown): Promise<T> {
    if (!response.ok) {
      return this.handleErrorResponse(response, requestConfig);
    }

    if (response.status === 204) {
      return undefined as T;
    }

    const contentType = response.headers.get('content-type') ?? '';
    if (contentType.includes('application/json')) {
      const result = await response.json();
      if (this.isResultEnvelope(result) && this.hasSuccessCode(result.code)) {
        return result.data as T;
      }
      if (this.isResultEnvelope(result) && hasOwn(result, 'code')) {
        throw SdkError.fromApiResult(result as never, response.status);
      }
      return result as T;
    }

    if (contentType.includes('text/')) {
      return await response.text() as T;
    }

    return await response.json() as T;
  }

  setAuthToken(token: string): void {
    super.setAuthToken(token);
  }

  setTokenManager(manager: AuthTokenManager): void {
    const baseProto = Object.getPrototypeOf(HttpClient.prototype) as {
      setTokenManager?: (this: HttpClient, m: AuthTokenManager) => void;
    };
    if (typeof baseProto.setTokenManager === 'function') {
      baseProto.setTokenManager.call(this, manager);
      return;
    }
    (this as any).authConfig = (this as any).authConfig || {};
    (this as any).authConfig.tokenManager = manager;
  }

  async request<T>(path: string, options: HttpRequestOptions = {}): Promise<T> {
    const execute = (this as any).execute;
    if (typeof execute !== 'function') {
      throw new Error('BaseHttpClient execute method is not available');
    }
    const { body, headers, contentType, method = 'GET', ...rest } = options;
    return withRetry(
      () =>
        execute.call(this, {
          url: path,
          method,
          ...rest,
          body,
          headers: this.buildRequestHeaders(headers, body == null ? undefined : contentType),
        }),
      { maxRetries: 3 },
    );
  }

  async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'GET', params, headers });
  }

  async post<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'POST', body, params, headers, contentType });
  }

  async put<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PUT', body, params, headers, contentType });
  }

  async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'DELETE', params, headers });
  }

  async patch<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PATCH', body, params, headers, contentType });
  }
}

export function createHttpClient(config: ImAdminBackendConfig): HttpClient {
  return new HttpClient(config);
}
`;
}

function materializeGeneratedWorkspace() {
  writeFile(path.join(generatedRoot, 'package.json'), renderGeneratedPackageJson());
  writeFile(path.join(generatedRoot, 'README.md'), renderGeneratedReadme());
  writeFile(path.join(generatedRoot, 'sdkwork-sdk.json'), renderGeneratedSdkworkConfig());
  writeFile(path.join(generatedRoot, 'tsconfig.json'), `{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "declarationMap": true,
    "outDir": "./dist",
    "rootDir": "./src",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
`);
  writeFile(path.join(generatedRoot, 'vite.config.ts'), `import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import { resolve } from 'path';

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'ImAdminBackendSdk',
      formats: ['es', 'cjs'],
      fileName: (format) => \`index.\${format === 'es' ? 'js' : 'cjs'}\`,
    },
    rollupOptions: {
      external: ['@sdkwork/sdk-common'],
      output: {
        globals: {
          '@sdkwork/sdk-common': 'SdkworkSdkCommon',
        },
      },
    },
    sourcemap: true,
  },
  plugins: [
    dts({ include: ['src'], outDir: 'dist' }),
  ],
});
`);
  writeFile(path.join(generatedRoot, 'src', 'index.ts'), `export { ImAdminBackendClient, createImAdminBackendClient } from './sdk';
export * from './types';
export * from './api';
export * from './http';
export * from './auth';
`);
  writeFile(path.join(generatedRoot, 'src', 'sdk.ts'), renderGeneratedSdk());
  writeFile(path.join(generatedRoot, 'src', 'auth', 'index.ts'), `export { DefaultAuthTokenManager, createTokenManager } from '@sdkwork/sdk-common';
export type { AuthTokenManager, AuthTokens } from '@sdkwork/sdk-common';
`);
  writeFile(path.join(generatedRoot, 'src', 'http', 'index.ts'), `export { HttpClient, createHttpClient } from './client';
`);
  writeFile(path.join(generatedRoot, 'src', 'http', 'client.ts'), renderGeneratedHttpClient());
  writeFile(path.join(generatedRoot, 'src', 'types', 'common.ts'), `export interface BasePlusVO {
  id?: string | number;
  createdAt?: string;
  updatedAt?: string;
  createdBy?: string;
  updatedBy?: string;
}

export interface BasePlusEntity extends BasePlusVO {
  deleted?: boolean;
}

export interface QueryListForm {
  keyword?: string;
  status?: string | number;
  startTime?: string;
  endTime?: string;
  orderBy?: string;
  orderDirection?: 'asc' | 'desc';
}

export type LooseJsonValue = unknown;
export type LooseJsonObject = Record<string, unknown>;

export type { Page, PageResult, RequestConfig, RequestOptions, QueryParams } from '@sdkwork/sdk-common';
export { DEFAULT_TIMEOUT, SUCCESS_CODES } from '@sdkwork/sdk-common';
import type { AuthTokenManager, AuthTokens } from '@sdkwork/sdk-common';
export type { AuthTokenManager, AuthTokens };

export interface ImAdminBackendConfig {
  baseUrl: string;
  authToken?: string;
  tokenManager?: AuthTokenManager;
  timeout?: number;
  headers?: Record<string, string>;
}
`);
  writeFile(path.join(generatedRoot, 'src', 'types', 'index.ts'), `export * from './common';
`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'paths.ts'), `export const BACKEND_API_PREFIX = '';

export function backendApiPath(path: string): string {
  if (!path) {
    return BACKEND_API_PREFIX;
  }
  if (/^https?:\\/\\//i.test(path)) {
    return path;
  }
  const normalizedPrefixRaw = (BACKEND_API_PREFIX || '').trim();
  const normalizedPrefix = normalizedPrefixRaw ? \`/\${normalizedPrefixRaw.replace(/^\\/+|\\/+$/g, '')}\` : '';
  const normalizedPath = path.startsWith('/') ? path : \`/\${path}\`;
  if (!normalizedPrefix || normalizedPrefix === '/') {
    return normalizedPath;
  }
  if (normalizedPath === normalizedPrefix || normalizedPath.startsWith(\`\${normalizedPrefix}/\`)) {
    return normalizedPath;
  }
  return \`\${normalizedPrefix}\${normalizedPath}\`;
}
`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'base.ts'), `import type { QueryParams } from '../types/common';
import type { HttpClient } from '../http/client';

export abstract class BaseApi {
  constructor(protected http: HttpClient, protected basePath: string) {}

  protected async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.http.get<T>(\`\${this.basePath}\${path}\`, params, headers);
  }

  protected async post<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {
    return this.http.post<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);
  }

  protected async put<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {
    return this.http.put<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);
  }

  protected async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.http.delete<T>(\`\${this.basePath}\${path}\`, params, headers);
  }

  protected async patch<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {
    return this.http.patch<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);
  }
}
`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'index.ts'), renderApiIndex());
  for (const groupName of Object.keys(routeGroups)) {
    writeFile(path.join(generatedRoot, 'src', 'api', `${groupName}.ts`), renderApiFile(groupName));
  }
}

function materializeComposedWorkspace() {
  writeFile(path.join(composedRoot, 'package.json'), renderComposedPackageJson());
  writeFile(path.join(composedRoot, 'README.md'), `# @sdkwork/im-admin-sdk

Composed IM admin TypeScript SDK built on the generated admin backend transport package.

## Client Surface

The preferred consumer entrypoint is \`ImAdminSdkClient\`, which exposes the admin domains
published by the generated backend transport package.

## Package Boundary

- Consume generated transport symbols only through \`@sdkwork/im-admin-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private source paths from manual or downstream code.
`);
  writeFile(path.join(composedRoot, 'tsconfig.build.json'), `{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ES2022",
    "moduleResolution": "Bundler",
    "rootDir": "src",
    "declaration": true,
    "declarationMap": true,
    "outDir": "dist",
    "strict": true,
    "skipLibCheck": true,
    "verbatimModuleSyntax": true,
    "isolatedModules": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "paths": {
      "@sdkwork/im-admin-backend-sdk": [
        "../generated/server-openapi/dist/index.d.ts"
      ]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.d.ts"],
  "exclude": ["dist", "test"]
}
`);
  writeFile(path.join(composedRoot, 'bin', 'clean-dist.mjs'), `#!/usr/bin/env node
import { existsSync, readdirSync, renameSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const distRoot = path.join(packageRoot, 'dist');
const promotedBuildRoot = path.join(distRoot, 'composed', 'src');

if (existsSync(promotedBuildRoot)) {
  for (const entry of readdirSync(promotedBuildRoot, { withFileTypes: true })) {
    const sourcePath = path.join(promotedBuildRoot, entry.name);
    const targetPath = path.join(distRoot, entry.name);
    rmSync(targetPath, { recursive: true, force: true });
    renameSync(sourcePath, targetPath);
  }
}

for (const relativePath of ['dist/composed', 'dist/generated']) {
  const absolutePath = path.join(packageRoot, relativePath);
  if (existsSync(absolutePath)) {
    rmSync(absolutePath, { recursive: true, force: true });
  }
}
`);
  writeFile(path.join(composedRoot, 'bin', 'run-tsc.mjs'), renderComposedRunTsc());
  writeFile(path.join(composedRoot, 'bin', 'package-task.mjs'), renderComposedPackageTask());
  writeFile(path.join(composedRoot, 'src', 'generated-backend-types.ts'), `import type { ImAdminBackendConfig } from '@sdkwork/im-admin-backend-sdk';

export type { ImAdminBackendConfig } from '@sdkwork/im-admin-backend-sdk';
`);
  writeFile(path.join(composedRoot, 'src', 'types.ts'), renderComposedTypes());
  writeFile(path.join(composedRoot, 'src', 'sdk-context.ts'), renderComposedSdkContext());
  writeFile(path.join(composedRoot, 'src', 'sdk.ts'), renderComposedSdk());
  writeFile(path.join(composedRoot, 'src', 'index.ts'), `export * from './generated-backend-types.js';
export * from './sdk-context.js';
export { ImAdminSdkClient, createImAdminSdkClient } from './sdk.js';
export * from './types.js';
`);
  writeFile(path.join(composedRoot, 'test', 'im-admin-sdk-client.test.mjs'), renderComposedTest());
}

function materializeWorkspaceReadme() {
  writeFile(path.join(typescriptRoot, 'README.md'), renderTypeScriptWorkspaceReadme());
}

materializeWorkspaceReadme();
materializeGeneratedWorkspace();
materializeComposedWorkspace();

console.log(`Materialized IM admin TypeScript workspace at ${typescriptRoot}`);
