import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';

import { routeGroups, routes } from './materialize-management-authority.mjs';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const typescriptRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-management-typescript',
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
  const callArgs = [];

  for (const paramName of params) {
    callArgs.push(paramName);
  }

  if (method !== 'get' && method !== 'delete') {
    signatureParts.push('body: LooseJsonObject');
    callArgs.push('body');
  }

  const signature = signatureParts.join(', ');
  const pathExpression = `backendApiPath(\`${renderInterpolatedPath(routePath)}\`)`;
  const invocation = `this.client.${method}<LooseJsonValue>(${pathExpression}${callArgs.length > params.length ? ', body' : ''})`;

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

  return `import { backendApiPath } from './paths';\nimport type { HttpClient } from '../http/client';\nimport type { LooseJsonObject, LooseJsonValue } from '../types/common';\n\nexport class ${className} {\n  private client: HttpClient;\n\n  constructor(client: HttpClient) {\n    this.client = client;\n  }\n\n${methods}\n}\n\nexport function create${className}(client: HttpClient): ${className} {\n  return new ${className}(client);\n}\n`;
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

  return `import { HttpClient, createHttpClient } from './http/client';\nimport type { SdkworkBackendConfig } from './types/common';\nimport type { AuthTokenManager } from '@sdkwork/sdk-common';\n\n${imports}\n\nexport class SdkworkBackendClient {\n  private httpClient: HttpClient;\n\n${fields}\n\n  constructor(config: SdkworkBackendConfig) {\n    this.httpClient = createHttpClient(config);\n${assignments}\n  }\n\n  setApiKey(apiKey: string): this {\n    this.httpClient.setApiKey(apiKey);\n    return this;\n  }\n\n  setAuthToken(token: string): this {\n    this.httpClient.setAuthToken(token);\n    return this;\n  }\n\n  setAccessToken(token: string): this {\n    this.httpClient.setAccessToken(token);\n    return this;\n  }\n\n  setTokenManager(manager: AuthTokenManager): this {\n    this.httpClient.setTokenManager(manager);\n    return this;\n  }\n\n  get http(): HttpClient {\n    return this.httpClient;\n  }\n}\n\nexport function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {\n  return new SdkworkBackendClient(config);\n}\n\nexport default SdkworkBackendClient;\n`;
}

function renderGeneratedPackageJson() {
  const stableBuildCommand = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';

  return `${JSON.stringify({
    name: '@sdkwork/craw-chat-management-backend-sdk',
    version: '0.1.0',
    description: 'Generated TypeScript transport package for the Craw Chat operator-console management backend',
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
    keywords: ['sdk', 'api', 'backend', 'sdkwork', 'craw-chat', 'management', 'operator-console'],
  }, null, 2)}\n`;
}

function renderGeneratedReadme() {
  return `# @sdkwork/craw-chat-management-backend-sdk

Generated TypeScript transport package for the Craw Chat operator-console management backend.

## Package Role

This package is the generator-owned transport layer for the checked-in management OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing management integrations, prefer the composed package
\`@sdkwork/craw-chat-sdk-management\`, which keeps the transport layer behind a stable management
client facade.

## Installation

\`\`\`bash
npm install @sdkwork/craw-chat-management-backend-sdk
# or
yarn add @sdkwork/craw-chat-management-backend-sdk
# or
pnpm add @sdkwork/craw-chat-management-backend-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { SdkworkBackendClient } from '@sdkwork/craw-chat-management-backend-sdk';

const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
  timeout: 30000,
});

client.setApiKey('your-management-api-key');

const tenantPage = await client.tenants.getApiAdminTenants();
\`\`\`

## Authentication Modes

Choose exactly one authentication mode per client instance.

### Mode A: API Key

Recommended for service-to-service management automation.

\`\`\`typescript
const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
});

client.setApiKey('your-management-api-key');
// Sends: Authorization: Bearer <apiKey>
\`\`\`

### Mode B: Dual Token

Use this when the target deployment expects a bearer token plus a delegated access token.

\`\`\`typescript
const client = new SdkworkBackendClient({
  baseUrl: 'https://your-management-origin.example.com',
});

client.setAuthToken('your-auth-token');
client.setAccessToken('your-access-token');
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
\`\`\`

Do not combine \`setApiKey(...)\` with \`setAuthToken(...)\` and \`setAccessToken(...)\` on the
same client instance.

## Endpoint Targeting

- Configure \`baseUrl\` to the origin that serves the checked-in \`/api/admin/*\` contract for the
  current environment.
- In packaged installs, that target is the unified \`craw-chat-server\` or \`web-gateway\` public
  origin.
- In direct backend development, point \`baseUrl\` to the management backend origin that already
  owns the \`/api/admin/*\` surface for that environment.

## Surface Groups

${Object.keys(routeGroups).map((groupName) => `- \`client.${groupName}\``).join('\n')}

## Package Boundary

- Use only the package root entrypoint: \`@sdkwork/craw-chat-management-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private generator paths from downstream code.
- Keep business orchestration in the composed package \`@sdkwork/craw-chat-sdk-management\`
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
    name: 'sdkwork-craw-chat-sdk-management',
    version: '0.1.0',
    sdkType: 'backend',
    packageName: '@sdkwork/craw-chat-management-backend-sdk',
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
  return `${JSON.stringify({
    name: '@sdkwork/craw-chat-sdk-management',
    version: '0.1.0',
    description: 'Composed Craw Chat management TypeScript SDK built on the generated management backend SDK',
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
      '@sdkwork/craw-chat-management-backend-sdk': 'file:../generated/server-openapi',
    },
    scripts: {
      typecheck: 'node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit',
      build: 'node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json && node ./bin/clean-dist.mjs',
      test: 'node ./test/craw-chat-sdk-management-client.test.mjs',
    },
  }, null, 2)}\n`;
}

function renderComposedTypes() {
  const groups = Object.keys(routeGroups);
  const clientGroupKeys = groups.map((groupName) => `  | '${groupName}'`).join('\n');

  return `import type {\n  SdkworkBackendClient,\n  SdkworkBackendConfig,\n} from '@sdkwork/craw-chat-management-backend-sdk';\n\nexport type { SdkworkBackendConfig } from '@sdkwork/craw-chat-management-backend-sdk';\n\nexport interface CrawChatSdkManagementClientCreateOptions {\n  backendClient?: CrawChatManagementBackendClientLike;\n  backendConfig?: SdkworkBackendConfig;\n}\n\nexport interface CrawChatSdkManagementClientOptions {\n  backendClient: CrawChatManagementBackendClientLike;\n}\n\nexport type CrawChatManagementBackendClientLike = Pick<\n  SdkworkBackendClient,\n${clientGroupKeys}\n  | 'setApiKey'\n  | 'setAuthToken'\n  | 'setAccessToken'\n  | 'setTokenManager'\n  | 'http'\n>;\n`;
}

function renderComposedSdkContext() {
  return `import type {\n  CrawChatManagementBackendClientLike,\n  CrawChatSdkManagementClientCreateOptions,\n  SdkworkBackendConfig,\n} from './types.js';\n\nfunction isRecord(value: unknown): value is Record<string, unknown> {\n  return typeof value === 'object' && value !== null;\n}\n\nasync function dynamicImportModule(moduleName: string): Promise<unknown> {\n  const dynamicImport = new Function('name', 'return import(name);') as (name: string) => Promise<unknown>;\n  return dynamicImport(moduleName);\n}\n\nexport async function createGeneratedBackendClient(\n  backendConfig: SdkworkBackendConfig,\n): Promise<CrawChatManagementBackendClientLike> {\n  const moduleExport = await dynamicImportModule('@sdkwork/craw-chat-management-backend-sdk');\n  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;\n  if (typeof createClient !== 'function') {\n    throw new Error('Unable to resolve @sdkwork/craw-chat-management-backend-sdk createClient factory');\n  }\n  return createClient(backendConfig) as Promise<CrawChatManagementBackendClientLike>;\n}\n\nexport async function resolveBackendClient(\n  options: CrawChatSdkManagementClientCreateOptions,\n): Promise<CrawChatManagementBackendClientLike> {\n  if (options.backendClient) {\n    return options.backendClient;\n  }\n  if (options.backendConfig) {\n    return createGeneratedBackendClient(options.backendConfig);\n  }\n  throw new Error('backendClient or backendConfig is required');\n}\n\nexport class CrawChatSdkManagementContext {\n  constructor(readonly backendClient: CrawChatManagementBackendClientLike) {}\n\n  setAuthToken(token: string): void {\n    this.backendClient.setAuthToken?.(token);\n  }\n\n  setAccessToken(token: string): void {\n    this.backendClient.setAccessToken?.(token);\n  }\n\n  setApiKey(apiKey: string): void {\n    this.backendClient.setApiKey?.(apiKey);\n  }\n}\n`;
}

function renderComposedSdk() {
  const groups = Object.keys(routeGroups);
  const fields = groups.map((groupName) => `  readonly ${groupName}: CrawChatManagementBackendClientLike['${groupName}'];`).join('\n');
  const assignments = groups.map((groupName) => `    this.${groupName} = options.backendClient.${groupName};`).join('\n');

  return `import { CrawChatSdkManagementContext, resolveBackendClient } from './sdk-context.js';\nimport type {\n  CrawChatManagementBackendClientLike,\n  CrawChatSdkManagementClientCreateOptions,\n  CrawChatSdkManagementClientOptions,\n} from './types.js';\n\nexport class CrawChatSdkManagementClient {\n  private readonly context: CrawChatSdkManagementContext;\n\n  readonly backendClient: CrawChatManagementBackendClientLike;\n${fields}\n\n  constructor(options: CrawChatSdkManagementClientOptions) {\n    this.context = new CrawChatSdkManagementContext(options.backendClient);\n    this.backendClient = options.backendClient;\n${assignments}\n  }\n\n  static async create(\n    options: CrawChatSdkManagementClientCreateOptions,\n  ): Promise<CrawChatSdkManagementClient> {\n    return new CrawChatSdkManagementClient({\n      backendClient: await resolveBackendClient(options),\n    });\n  }\n\n  setAuthToken(token: string): this {\n    this.context.setAuthToken(token);\n    return this;\n  }\n\n  setAccessToken(token: string): this {\n    this.context.setAccessToken(token);\n    return this;\n  }\n\n  setApiKey(apiKey: string): this {\n    this.context.setApiKey(apiKey);\n    return this;\n  }\n}\n\nexport async function createCrawChatSdkManagementClient(\n  options: CrawChatSdkManagementClientCreateOptions,\n): Promise<CrawChatSdkManagementClient> {\n  return CrawChatSdkManagementClient.create(options);\n}\n`;
}

function renderComposedTest() {
  const groups = Object.keys(routeGroups);
  const groupStubs = groups.map((groupName) => `  const ${groupName} = { marker: '${groupName}' };`).join('\n');
  const backendFields = groups.map((groupName) => `    ${groupName},`).join('\n');
  const assertions = groups.map((groupName) => `  assert.equal(sdk.${groupName}, backendClient.${groupName});`).join('\n');

  return `import assert from 'node:assert/strict';\n\nimport { CrawChatSdkManagementClient } from '../dist/index.js';\n\nfunction createBackendClientStub() {\n  const calls = [];\n${groupStubs}\n\n  const backendClient = {\n${backendFields}\n    http: {\n      async request() {\n        calls.push({ method: 'http.request' });\n        return {};\n      },\n    },\n    setAuthToken(token) {\n      calls.push({ method: 'setAuthToken', token });\n      return backendClient;\n    },\n    setAccessToken(token) {\n      calls.push({ method: 'setAccessToken', token });\n      return backendClient;\n    },\n    setApiKey(apiKey) {\n      calls.push({ method: 'setApiKey', apiKey });\n      return backendClient;\n    },\n    setTokenManager(manager) {\n      calls.push({ method: 'setTokenManager', manager });\n      return backendClient;\n    },\n  };\n\n  return { backendClient, calls };\n}\n\nasync function testCreateFactoryAndTokenHelpers() {\n  const { backendClient, calls } = createBackendClientStub();\n  const sdk = await CrawChatSdkManagementClient.create({ backendClient });\n\n  sdk.setAuthToken('auth-token');\n  sdk.setAccessToken('access-token');\n  sdk.setApiKey('api-key');\n\n  assert.deepEqual(calls.slice(-3), [\n    { method: 'setAuthToken', token: 'auth-token' },\n    { method: 'setAccessToken', token: 'access-token' },\n    { method: 'setApiKey', apiKey: 'api-key' },\n  ]);\n}\n\nfunction testConstructorSurface() {\n  const { backendClient } = createBackendClientStub();\n  const sdk = new CrawChatSdkManagementClient({ backendClient });\n\n  assert.equal(sdk.backendClient, backendClient);\n${assertions}\n}\n\nawait testCreateFactoryAndTokenHelpers();\ntestConstructorSurface();\n\nconsole.log('craw-chat management composed sdk smoke tests passed');\n`;
}

function materializeGeneratedWorkspace() {
  writeFile(path.join(generatedRoot, 'package.json'), renderGeneratedPackageJson());
  writeFile(path.join(generatedRoot, 'README.md'), renderGeneratedReadme());
  writeFile(path.join(generatedRoot, 'sdkwork-sdk.json'), renderGeneratedSdkworkConfig());
  writeFile(path.join(generatedRoot, 'tsconfig.json'), `{\n  "compilerOptions": {\n    "target": "ES2020",\n    "module": "ESNext",\n    "lib": ["ES2020", "DOM", "DOM.Iterable"],\n    "strict": true,\n    "esModuleInterop": true,\n    "skipLibCheck": true,\n    "forceConsistentCasingInFileNames": true,\n    "declaration": true,\n    "declarationMap": true,\n    "outDir": "./dist",\n    "rootDir": "./src",\n    "moduleResolution": "bundler",\n    "resolveJsonModule": true,\n    "isolatedModules": true\n  },\n  "include": ["src/**/*"],\n  "exclude": ["node_modules", "dist"]\n}\n`);
  writeFile(path.join(generatedRoot, 'vite.config.ts'), `import { defineConfig } from 'vite';\nimport dts from 'vite-plugin-dts';\nimport { resolve } from 'path';\n\nexport default defineConfig({\n  build: {\n    lib: {\n      entry: resolve(__dirname, 'src/index.ts'),\n      name: 'SdkworkManagementBackend',\n      formats: ['es', 'cjs'],\n      fileName: (format) => \`index.\${format === 'es' ? 'js' : 'cjs'}\`,\n    },\n    rollupOptions: {\n      external: ['@sdkwork/sdk-common'],\n      output: {\n        globals: {\n          '@sdkwork/sdk-common': 'SdkworkSdkCommon',\n        },\n      },\n    },\n    sourcemap: true,\n  },\n  plugins: [\n    dts({ include: ['src'], outDir: 'dist' }),\n  ],\n});\n`);
  writeFile(path.join(generatedRoot, 'src', 'index.ts'), `export { SdkworkBackendClient, createClient } from './sdk';\nexport * from './types';\nexport * from './api';\nexport * from './http';\nexport * from './auth';\n`);
  writeFile(path.join(generatedRoot, 'src', 'sdk.ts'), renderGeneratedSdk());
  writeFile(path.join(generatedRoot, 'src', 'auth', 'index.ts'), `export { DefaultAuthTokenManager, createTokenManager } from '@sdkwork/sdk-common';\nexport type { AuthTokenManager, AuthTokens, AuthMode } from '@sdkwork/sdk-common';\n`);
  writeFile(path.join(generatedRoot, 'src', 'http', 'index.ts'), `export { HttpClient, createHttpClient } from './client';\n`);
  writeFile(path.join(generatedRoot, 'src', 'http', 'client.ts'), `import type { SdkworkBackendConfig } from '../types/common';\nimport type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';\nimport type { AuthTokenManager } from '@sdkwork/sdk-common';\nimport { BaseHttpClient, withRetry } from '@sdkwork/sdk-common';\n\ntype HttpRequestOptions = RequestOptions & {\n  body?: unknown;\n  headers?: Record<string, string>;\n  contentType?: string;\n};\n\nexport class HttpClient extends BaseHttpClient {\n  private static readonly API_KEY_HEADER = 'Authorization';\n  private static readonly API_KEY_USE_BEARER = true;\n\n  constructor(config: SdkworkBackendConfig) {\n    super(config as any);\n  }\n\n  private getInternalAuthConfig(): any {\n    const self = this as any;\n    self.authConfig = self.authConfig || {};\n    return self.authConfig;\n  }\n\n  private getInternalHeaders(): Record<string, string> {\n    const self = this as any;\n    self.config = self.config || {};\n    self.config.headers = self.config.headers || {};\n    return self.config.headers;\n  }\n\n  private buildRequestHeaders(headers?: Record<string, string>, contentType?: string): Record<string, string> | undefined {\n    const mergedHeaders = { ...(headers ?? {}) };\n    if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {\n      mergedHeaders['Content-Type'] = contentType;\n    }\n    return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;\n  }\n\n  setApiKey(apiKey: string): void {\n    const authConfig = this.getInternalAuthConfig();\n    const headers = this.getInternalHeaders();\n    authConfig.apiKey = apiKey;\n    authConfig.tokenManager?.clearTokens?.();\n    if (HttpClient.API_KEY_HEADER === 'Authorization' && HttpClient.API_KEY_USE_BEARER) {\n      authConfig.authMode = 'apikey';\n      delete headers['Access-Token'];\n      return;\n    }\n    authConfig.authMode = 'dual-token';\n    headers[HttpClient.API_KEY_HEADER] = HttpClient.API_KEY_USE_BEARER ? \`Bearer \${apiKey}\` : apiKey;\n  }\n\n  setAuthToken(token: string): void {\n    const headers = this.getInternalHeaders();\n    if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'authorization') {\n      delete headers[HttpClient.API_KEY_HEADER];\n    }\n    super.setAuthToken(token);\n  }\n\n  setAccessToken(token: string): void {\n    const headers = this.getInternalHeaders();\n    if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'access-token') {\n      delete headers[HttpClient.API_KEY_HEADER];\n    }\n    super.setAccessToken(token);\n  }\n\n  setTokenManager(manager: AuthTokenManager): void {\n    const baseProto = Object.getPrototypeOf(HttpClient.prototype) as { setTokenManager?: (this: HttpClient, m: AuthTokenManager) => void };\n    if (typeof baseProto.setTokenManager === 'function') {\n      baseProto.setTokenManager.call(this, manager);\n      return;\n    }\n    this.getInternalAuthConfig().tokenManager = manager;\n  }\n\n  async request<T>(path: string, options: HttpRequestOptions = {}): Promise<T> {\n    const execute = (this as any).execute;\n    if (typeof execute !== 'function') {\n      throw new Error('BaseHttpClient execute method is not available');\n    }\n    const { body, headers, contentType, method = 'GET', ...rest } = options;\n    return withRetry(\n      () => execute.call(this, {\n        url: path,\n        method,\n        ...rest,\n        body,\n        headers: this.buildRequestHeaders(headers, body == null ? undefined : contentType),\n      }),\n      { maxRetries: 3 },\n    );\n  }\n\n  async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {\n    return this.request<T>(path, { method: 'GET', params, headers });\n  }\n\n  async post<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.request<T>(path, { method: 'POST', body, params, headers, contentType });\n  }\n\n  async put<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.request<T>(path, { method: 'PUT', body, params, headers, contentType });\n  }\n\n  async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {\n    return this.request<T>(path, { method: 'DELETE', params, headers });\n  }\n\n  async patch<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.request<T>(path, { method: 'PATCH', body, params, headers, contentType });\n  }\n}\n\nexport function createHttpClient(config: SdkworkBackendConfig): HttpClient {\n  return new HttpClient(config);\n}\n`);
  writeFile(path.join(generatedRoot, 'src', 'types', 'common.ts'), `export interface BasePlusVO {\n  id?: string | number;\n  createdAt?: string;\n  updatedAt?: string;\n  createdBy?: string;\n  updatedBy?: string;\n}\n\nexport interface BasePlusEntity extends BasePlusVO {\n  deleted?: boolean;\n}\n\nexport interface QueryListForm {\n  keyword?: string;\n  status?: string | number;\n  startTime?: string;\n  endTime?: string;\n  orderBy?: string;\n  orderDirection?: 'asc' | 'desc';\n}\n\nexport type LooseJsonValue = unknown;\nexport type LooseJsonObject = Record<string, unknown>;\n\nexport type { Page, PageResult, RequestConfig, RequestOptions, QueryParams } from '@sdkwork/sdk-common';\nexport { DEFAULT_TIMEOUT, SUCCESS_CODES } from '@sdkwork/sdk-common';\nimport type { AuthTokenManager, AuthMode, AuthTokens } from '@sdkwork/sdk-common';\nexport type { AuthTokenManager, AuthMode, AuthTokens };\n\nexport interface SdkworkBackendConfig {\n  baseUrl: string;\n  apiKey?: string;\n  authToken?: string;\n  accessToken?: string;\n  tenantId?: string;\n  organizationId?: string;\n  platform?: string;\n  tokenManager?: AuthTokenManager;\n  timeout?: number;\n  authMode?: AuthMode;\n  headers?: Record<string, string>;\n}\n`);
  writeFile(path.join(generatedRoot, 'src', 'types', 'index.ts'), `export * from './common';\n`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'paths.ts'), `export const BACKEND_API_PREFIX = '';\n\nexport function backendApiPath(path: string): string {\n  if (!path) {\n    return BACKEND_API_PREFIX;\n  }\n  if (/^https?:\\/\\//i.test(path)) {\n    return path;\n  }\n  const normalizedPrefixRaw = (BACKEND_API_PREFIX || '').trim();\n  const normalizedPrefix = normalizedPrefixRaw ? \`/\${normalizedPrefixRaw.replace(/^\\/+|\\/+$/g, '')}\` : '';\n  const normalizedPath = path.startsWith('/') ? path : \`/\${path}\`;\n  if (!normalizedPrefix || normalizedPrefix === '/') {\n    return normalizedPath;\n  }\n  if (normalizedPath === normalizedPrefix || normalizedPath.startsWith(\`\${normalizedPrefix}/\`)) {\n    return normalizedPath;\n  }\n  return \`\${normalizedPrefix}\${normalizedPath}\`;\n}\n`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'base.ts'), `import type { QueryParams } from '../types/common';\nimport type { HttpClient } from '../http/client';\n\nexport abstract class BaseApi {\n  constructor(protected http: HttpClient, protected basePath: string) {}\n\n  protected async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {\n    return this.http.get<T>(\`\${this.basePath}\${path}\`, params, headers);\n  }\n\n  protected async post<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.http.post<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);\n  }\n\n  protected async put<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.http.put<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);\n  }\n\n  protected async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {\n    return this.http.delete<T>(\`\${this.basePath}\${path}\`, params, headers);\n  }\n\n  protected async patch<T>(path: string, body?: unknown, params?: QueryParams, headers?: Record<string, string>, contentType?: string): Promise<T> {\n    return this.http.patch<T>(\`\${this.basePath}\${path}\`, body, params, headers, contentType);\n  }\n}\n`);
  writeFile(path.join(generatedRoot, 'src', 'api', 'index.ts'), renderApiIndex());
  for (const groupName of Object.keys(routeGroups)) {
    writeFile(path.join(generatedRoot, 'src', 'api', `${groupName}.ts`), renderApiFile(groupName));
  }
}

function materializeComposedWorkspace() {
  writeFile(path.join(composedRoot, 'package.json'), renderComposedPackageJson());
  writeFile(path.join(composedRoot, 'README.md'), `# @sdkwork/craw-chat-sdk-management\n\nComposed management TypeScript SDK built on the generated management backend transport package.\n\n## Client Surface\n\nThe preferred consumer entrypoint is \`CrawChatSdkManagementClient\`, which exposes the management domains published by the generated backend transport package.\nUse the composed package by default for operator-console and management integrations.\n\n## Package Boundary\n\n- Consume generated transport symbols only through \`@sdkwork/craw-chat-management-backend-sdk\`.\n- Do not import \`generated/server-openapi/src/*\` private source paths from manual or downstream code.\n`);
  writeFile(path.join(composedRoot, 'tsconfig.build.json'), `{\n  "compilerOptions": {\n    "target": "ES2022",\n    "module": "ES2022",\n    "moduleResolution": "Bundler",\n    "declaration": true,\n    "declarationMap": true,\n    "outDir": "dist",\n    "strict": true,\n    "skipLibCheck": true,\n    "verbatimModuleSyntax": true,\n    "isolatedModules": true,\n    "esModuleInterop": true,\n    "forceConsistentCasingInFileNames": true,\n    "baseUrl": ".",\n    "paths": {\n      "@sdkwork/craw-chat-management-backend-sdk": [\n        "../generated/server-openapi/src/index.ts"\n      ]\n    }\n  },\n  "include": [\"src/**/*.ts\", \"src/**/*.d.ts\"],\n  "exclude": [\"dist\", \"test\"]\n}\n`);
  writeFile(path.join(composedRoot, 'bin', 'clean-dist.mjs'), `#!/usr/bin/env node\nimport { existsSync, readdirSync, renameSync, rmSync } from 'node:fs';\nimport path from 'node:path';\nimport { fileURLToPath } from 'node:url';\n\nconst scriptDir = path.dirname(fileURLToPath(import.meta.url));\nconst packageRoot = path.resolve(scriptDir, '..');\nconst distRoot = path.join(packageRoot, 'dist');\nconst promotedBuildRoot = path.join(distRoot, 'composed', 'src');\n\nif (existsSync(promotedBuildRoot)) {\n  for (const entry of readdirSync(promotedBuildRoot, { withFileTypes: true })) {\n    const sourcePath = path.join(promotedBuildRoot, entry.name);\n    const targetPath = path.join(distRoot, entry.name);\n    rmSync(targetPath, { recursive: true, force: true });\n    renameSync(sourcePath, targetPath);\n  }\n}\n\nfor (const relativePath of ['dist/composed', 'dist/generated']) {\n  const absolutePath = path.join(packageRoot, relativePath);\n  if (existsSync(absolutePath)) {\n    rmSync(absolutePath, { recursive: true, force: true });\n  }\n}\n`);
  writeFile(path.join(composedRoot, 'src', 'generated-backend-types.ts'), `import type { SdkworkBackendConfig } from '@sdkwork/craw-chat-management-backend-sdk';\n\nexport type { SdkworkBackendConfig } from '@sdkwork/craw-chat-management-backend-sdk';\n`);
  writeFile(path.join(composedRoot, 'src', 'types.ts'), renderComposedTypes());
  writeFile(path.join(composedRoot, 'src', 'sdk-context.ts'), renderComposedSdkContext());
  writeFile(path.join(composedRoot, 'src', 'sdk.ts'), renderComposedSdk());
  writeFile(path.join(composedRoot, 'src', 'index.ts'), `export * from './generated-backend-types.js';\nexport * from './sdk-context.js';\nexport { CrawChatSdkManagementClient, createCrawChatSdkManagementClient } from './sdk.js';\nexport * from './types.js';\n`);
  writeFile(path.join(composedRoot, 'test', 'craw-chat-sdk-management-client.test.mjs'), renderComposedTest());
}

materializeGeneratedWorkspace();
materializeComposedWorkspace();

console.log(`Materialized management TypeScript workspace at ${typescriptRoot}`);
