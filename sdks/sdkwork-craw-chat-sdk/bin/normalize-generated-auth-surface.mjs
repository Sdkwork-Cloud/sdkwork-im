#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    languages: [],
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
    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

function normalizeNewlines(value) {
  return value.replace(/\r?\n/g, '\n');
}

function ensureParentDirectory(filePath) {
  mkdirSync(path.dirname(filePath), { recursive: true });
}

function writeIfChanged(filePath, content) {
  const nextContent = normalizeNewlines(content);
  const currentContent = existsSync(filePath) ? normalizeNewlines(readFileSync(filePath, 'utf8')) : null;
  if (currentContent === nextContent) {
    return;
  }
  ensureParentDirectory(filePath);
  writeFileSync(filePath, nextContent, 'utf8');
}

function normalizeTypeScriptPackageJson(filePath) {
  const currentPackage = existsSync(filePath)
    ? JSON.parse(readFileSync(filePath, 'utf8'))
    : {};
  const stableBuildCommand = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';

  const nextPackage = {
    ...currentPackage,
    description: 'Generated TypeScript transport package for the Craw Chat app API',
    scripts: {
      ...currentPackage.scripts,
      build: stableBuildCommand,
      prepublishOnly: 'npm run build',
    },
    keywords: [
      'sdk',
      'api',
      'backend',
      'sdkwork',
      'craw-chat',
      'app',
      'chat',
    ],
  };

  writeIfChanged(filePath, `${JSON.stringify(nextPackage, null, 2)}\n`);
}

function normalizeFlutterPubspec(filePath) {
  const currentSource = existsSync(filePath) ? readFileSync(filePath, 'utf8') : '';
  let nextSource = currentSource;

  if (/^description:.*$/m.test(nextSource)) {
    nextSource = nextSource.replace(
      /^description:.*$/m,
      'description: Generated Flutter transport package for the Craw Chat app API',
    );
  } else {
    nextSource = `${nextSource.trimEnd()}\ndescription: Generated Flutter transport package for the Craw Chat app API\n`;
  }

  writeIfChanged(filePath, nextSource);
}

function removeIfExists(targetPath) {
  if (!existsSync(targetPath)) {
    return;
  }
  rmSync(targetPath, { recursive: true, force: true });
}

function renderTypeScriptIndex() {
  return `export { SdkworkBackendClient, createClient } from './sdk';
export * from './types';
export * from './api';
`;
}

function renderTypeScriptSdk() {
  return `import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { SessionApi, createSessionApi } from './api/session';
import { PresenceApi, createPresenceApi } from './api/presence';
import { RealtimeApi, createRealtimeApi } from './api/realtime';
import { DeviceApi, createDeviceApi } from './api/device';
import { InboxApi, createInboxApi } from './api/inbox';
import { ConversationApi, createConversationApi } from './api/conversation';
import { MessageApi, createMessageApi } from './api/message';
import { MediaApi, createMediaApi } from './api/media';
import { StreamApi, createStreamApi } from './api/stream';
import { RtcApi, createRtcApi } from './api/rtc';

export class SdkworkBackendClient {
  private readonly httpClient: HttpClient;

  public readonly session: SessionApi;
  public readonly presence: PresenceApi;
  public readonly realtime: RealtimeApi;
  public readonly device: DeviceApi;
  public readonly inbox: InboxApi;
  public readonly conversation: ConversationApi;
  public readonly message: MessageApi;
  public readonly media: MediaApi;
  public readonly stream: StreamApi;
  public readonly rtc: RtcApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.session = createSessionApi(this.httpClient);
    this.presence = createPresenceApi(this.httpClient);
    this.realtime = createRealtimeApi(this.httpClient);
    this.device = createDeviceApi(this.httpClient);
    this.inbox = createInboxApi(this.httpClient);
    this.conversation = createConversationApi(this.httpClient);
    this.message = createMessageApi(this.httpClient);
    this.media = createMediaApi(this.httpClient);
    this.stream = createStreamApi(this.httpClient);
    this.rtc = createRtcApi(this.httpClient);
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }
}

export function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {
  return new SdkworkBackendClient(config);
}

export default SdkworkBackendClient;
`;
}

function renderTypeScriptHttpClient() {
  return `import type { SdkworkBackendConfig } from '../types/common';
import type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { BaseHttpClient, withRetry } from '@sdkwork/sdk-common';

type HttpRequestOptions = RequestOptions & {
  body?: unknown;
  headers?: Record<string, string>;
  contentType?: string;
};

export class HttpClient extends BaseHttpClient {
  constructor(config: SdkworkBackendConfig) {
    super(config as any);
  }

  private getInternalAuthConfig(): any {
    const self = this as any;
    self.authConfig = self.authConfig || {};
    return self.authConfig;
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

  private buildRequestBody(body: unknown, contentType?: string): unknown {
    if (body == null) {
      return body;
    }

    const normalizedContentType = (contentType ?? '').toLowerCase();
    if (normalizedContentType === 'application/x-www-form-urlencoded') {
      return this.encodeFormBody(body);
    }

    return body;
  }

  private encodeFormBody(body: unknown): string {
    if (body instanceof URLSearchParams) {
      return body.toString();
    }
    if (typeof body === 'string') {
      return body;
    }

    const params = new URLSearchParams();
    if (body instanceof Map) {
      for (const [key, value] of body.entries()) {
        this.appendFormValue(params, String(key), value);
      }
      return params.toString();
    }
    if (typeof body === 'object') {
      for (const [key, value] of Object.entries(body as Record<string, unknown>)) {
        this.appendFormValue(params, key, value);
      }
      return params.toString();
    }

    params.append('value', String(body));
    return params.toString();
  }

  private appendFormValue(params: URLSearchParams, key: string, value: unknown): void {
    if (value == null) {
      return;
    }
    if (Array.isArray(value)) {
      value.forEach((item) => this.appendFormValue(params, key, item));
      return;
    }
    if (value instanceof Date) {
      params.append(key, value.toISOString());
      return;
    }
    if (typeof value === 'object') {
      params.append(key, JSON.stringify(value));
      return;
    }
    params.append(key, String(value));
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
    this.getInternalAuthConfig().tokenManager = manager;
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
          body: this.buildRequestBody(body, contentType),
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

export function createHttpClient(config: SdkworkBackendConfig): HttpClient {
  return new HttpClient(config);
}
`;
}

function renderTypeScriptCommonTypes() {
  return `export interface BasePlusVO {
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

export type { Page, PageResult, RequestConfig, RequestOptions, QueryParams } from '@sdkwork/sdk-common';
export { DEFAULT_TIMEOUT, SUCCESS_CODES } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
export type { AuthTokenManager };

export interface SdkworkBackendConfig {
  baseUrl: string;
  authToken?: string;
  tokenManager?: AuthTokenManager;
  timeout?: number;
  headers?: Record<string, string>;
}
`;
}

function renderTypeScriptCommonShim() {
  return `declare module '@sdkwork/sdk-common' {
  export type QueryParamScalar = string | number | boolean | undefined | null;
  export type QueryParamValue = QueryParamScalar | Array<string | number | boolean>;
  export type QueryParams = Record<string, QueryParamValue>;

  export interface Page<T = unknown> {
    records?: T[];
    total?: number;
    current?: number;
    size?: number;
  }

  export interface PageResult<T = unknown> {
    records?: T[];
    total?: number;
    current?: number;
    size?: number;
  }

  export interface RequestConfig {
    url: string;
    method: string;
    headers?: Record<string, string>;
    params?: QueryParams;
    body?: unknown;
    timeout?: number;
    signal?: AbortSignal;
    skipAuth?: boolean;
    retryCount?: number;
    metadata?: Record<string, unknown>;
  }

  export interface RequestOptions {
    method?: string;
    headers?: Record<string, string>;
    body?: unknown;
    params?: QueryParams;
    signal?: AbortSignal;
    skipAuth?: boolean;
    requiresAuth?: boolean;
    timeout?: number;
    retry?: {
      maxRetries?: number;
      retryDelay?: number;
      retryBackoff?: 'fixed' | 'linear' | 'exponential';
      maxRetryDelay?: number;
    };
    cache?: boolean | number;
    metadata?: Record<string, unknown>;
  }

  export interface AuthTokens {
    authToken?: string;
    refreshToken?: string;
    expiresIn?: number;
    expiresAt?: number;
    tokenType?: string;
    scope?: string;
  }

  export interface AuthTokenManager {
    getAuthToken(): string | undefined;
    getRefreshToken(): string | undefined;
    getTokens(): AuthTokens;
    setTokens(tokens: AuthTokens): void;
    setAuthToken(token: string): void;
    setRefreshToken(token: string): void;
    clearTokens(): void;
    clearAuthToken(): void;
    isExpired(): boolean;
    isValid(): boolean;
    hasToken(): boolean;
    hasAuthToken(): boolean;
    willExpireIn(seconds: number): boolean;
  }

  export const DEFAULT_TIMEOUT: number;
  export const SUCCESS_CODES: Array<number | string>;

  export class DefaultAuthTokenManager implements AuthTokenManager {
    constructor(initialTokens?: AuthTokens);
    getAuthToken(): string | undefined;
    getRefreshToken(): string | undefined;
    getTokens(): AuthTokens;
    setTokens(tokens: AuthTokens): void;
    setAuthToken(token: string): void;
    setRefreshToken(token: string): void;
    clearTokens(): void;
    clearAuthToken(): void;
    isExpired(): boolean;
    isValid(): boolean;
    hasToken(): boolean;
    hasAuthToken(): boolean;
    willExpireIn(seconds: number): boolean;
  }

  export function createTokenManager(tokens?: AuthTokens): AuthTokenManager;

  export abstract class BaseHttpClient {
    constructor(config: Record<string, unknown>);
    setAuthToken(token: string): void;
    setTokenManager(manager: AuthTokenManager): void;
    execute<T>(config: RequestConfig): Promise<T>;
    abstract request<T>(path: string, options?: RequestOptions): Promise<T>;
    abstract get<T>(path: string, params?: QueryParams): Promise<T>;
    abstract post<T>(path: string, body?: unknown): Promise<T>;
    abstract put<T>(path: string, body?: unknown): Promise<T>;
    abstract delete<T>(path: string, body?: unknown): Promise<T>;
    abstract patch<T>(path: string, body?: unknown): Promise<T>;
  }

  export function withRetry<T>(
    fn: () => Promise<T>,
    config?: {
      maxRetries?: number;
      retryDelay?: number;
      retryBackoff?: 'fixed' | 'linear' | 'exponential';
      maxRetryDelay?: number;
    },
  ): Promise<T>;
}
`;
}

function renderTypeScriptReadme() {
  return `# @sdkwork/craw-chat-backend-sdk

Generated TypeScript transport package for the Craw Chat app API.

## Package Role

This package is the generator-owned transport layer for the checked-in app OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing chat integrations, prefer the composed package \`@sdkwork/craw-chat-sdk\`,
which adds the higher-level chat client and manual orchestration layer above this transport package.

## Installation

\`\`\`bash
npm install @sdkwork/craw-chat-backend-sdk
# or
yarn add @sdkwork/craw-chat-backend-sdk
# or
pnpm add @sdkwork/craw-chat-backend-sdk
\`\`\`

## Quick Start

\`\`\`typescript
import { SdkworkBackendClient } from '@sdkwork/craw-chat-backend-sdk';

const client = new SdkworkBackendClient({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'your-bearer-token',
  timeout: 30000,
});

const result = await client.inbox.getInbox();
\`\`\`

## Authentication

Craw Chat app routes use bearer authentication only.

\`\`\`typescript
const client = new SdkworkBackendClient({
  baseUrl: 'http://127.0.0.1:18090',
});

client.setAuthToken('your-bearer-token');
// Sends: Authorization: Bearer <token>
\`\`\`

If token ownership lives outside the SDK, provide a custom \`tokenManager\` in the constructor instead.

## Endpoint Targeting

- In direct local development, point \`baseUrl\` to the app-facing service origin, typically the
  local \`local-minimal-node\` HTTP endpoint such as \`http://127.0.0.1:18090\`.
- In packaged installs, point \`baseUrl\` to the unified \`craw-chat-server\` or \`web-gateway\`
  public origin.
- Keep one deployment model per client configuration. Do not mix direct local service and unified
  gateway assumptions in the same client instance.

## Configuration

\`\`\`typescript
const client = new SdkworkBackendClient({
  baseUrl: 'http://127.0.0.1:18090',
  timeout: 30000,
  headers: {
    'X-Custom-Header': 'value',
  },
});
\`\`\`

## Surface Groups

- \`client.session\` - session API
- \`client.presence\` - presence API
- \`client.realtime\` - realtime API
- \`client.device\` - device API
- \`client.inbox\` - inbox API
- \`client.conversation\` - conversation API
- \`client.message\` - message API
- \`client.media\` - media API
- \`client.stream\` - stream API
- \`client.rtc\` - rtc API

## Publishing

This SDK includes cross-platform publish scripts in \`bin/\`:

- \`bin/publish-core.mjs\`
- \`bin/publish.sh\`
- \`bin/publish.ps1\`

## License

MIT

## Package Boundary

- Use only the package root entrypoint: \`@sdkwork/craw-chat-backend-sdk\`.
- Do not import \`generated/server-openapi/src/*\` private generator paths from downstream code.
- Keep business orchestration in the composed package \`@sdkwork/craw-chat-sdk\` instead of
  re-exporting generated internals.
- The workspace normalization wrapper strips generator-only auth scaffolding and source-tree build
  residue before verification and packaging.

## Regeneration Contract

- Generator-owned files are tracked in \`.sdkwork/sdkwork-generator-manifest.json\`.
- Each run also writes \`.sdkwork/sdkwork-generator-changes.json\` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes \`.sdkwork/sdkwork-generator-report.json\` with the full execution report, including \`schemaVersion\`, \`generator\`, stable artifact paths, and the execution handoff commands that match CLI \`--json\` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in \`custom/\`.
- Files scaffolded under \`custom/\` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to \`.sdkwork/manual-backups/\` before overwrite or removal.
`;
}

function renderFlutterBackendClient() {
  return `import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/session.dart';
import 'src/api/presence.dart';
import 'src/api/realtime.dart';
import 'src/api/device.dart';
import 'src/api/inbox.dart';
import 'src/api/conversation.dart';
import 'src/api/message.dart';
import 'src/api/media.dart';
import 'src/api/stream.dart';
import 'src/api/rtc.dart';

class SdkworkBackendConfig {
  final String baseUrl;
  final String? authToken;
  final Map<String, String> headers;
  final int timeout;

  const SdkworkBackendConfig({
    required this.baseUrl,
    this.authToken,
    this.headers = const <String, String>{},
    this.timeout = 30000,
  });

  SdkConfig toSdkConfig() {
    return SdkConfig(
      baseUrl: baseUrl,
      timeout: timeout,
      headers: headers,
      authToken: authToken,
    );
  }
}

class SdkworkBackendClient {
  final HttpClient _httpClient;

  late final SessionApi session;
  late final PresenceApi presence;
  late final RealtimeApi realtime;
  late final DeviceApi device;
  late final InboxApi inbox;
  late final ConversationApi conversation;
  late final MessageApi message;
  late final MediaApi media;
  late final StreamApi stream;
  late final RtcApi rtc;

  SdkworkBackendClient({
    required SdkworkBackendConfig config,
  }) : _httpClient = HttpClient(config: config.toSdkConfig()) {
    session = SessionApi(_httpClient);
    presence = PresenceApi(_httpClient);
    realtime = RealtimeApi(_httpClient);
    device = DeviceApi(_httpClient);
    inbox = InboxApi(_httpClient);
    conversation = ConversationApi(_httpClient);
    message = MessageApi(_httpClient);
    media = MediaApi(_httpClient);
    stream = StreamApi(_httpClient);
    rtc = RtcApi(_httpClient);
  }

  factory SdkworkBackendClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkBackendClient(
      config: SdkworkBackendConfig(
        baseUrl: baseUrl,
        authToken: authToken,
        headers: headers ?? const <String, String>{},
        timeout: timeout,
      ),
    );
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }
}
`;
}

function renderFlutterReadme() {
  return `# backend_sdk

Generated Flutter transport package for the Craw Chat app API.

## Package Role

This package is the generator-owned transport layer for the checked-in app OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing chat integrations, prefer the composed Flutter layers under
\`sdkwork-craw-chat-sdk-flutter/composed\`, where the manual \`craw_chat_sdk\` package wraps this
transport package with the higher-level chat client surface.

## Installation

Add to \`pubspec.yaml\`:

\`\`\`yaml
dependencies:
  backend_sdk: ^0.1.0
\`\`\`

## Quick Start

\`\`\`dart
import 'package:backend_sdk/backend_sdk.dart';

final client = SdkworkBackendClient(
  config: const SdkworkBackendConfig(
    baseUrl: 'http://127.0.0.1:18090',
    authToken: 'your-bearer-token',
  ),
);

final result = await client.inbox.getInbox();
print(result);
\`\`\`

## Authentication

Craw Chat app routes use bearer authentication only.

\`\`\`dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18090',
);

client.setAuthToken('your-bearer-token');
// Sends: Authorization: Bearer <token>
\`\`\`

## Endpoint Targeting

- In direct local development, point \`baseUrl\` to the app-facing service origin, typically the
  local \`local-minimal-node\` HTTP endpoint such as \`http://127.0.0.1:18090\`.
- In packaged installs, point \`baseUrl\` to the unified \`craw-chat-server\` or \`web-gateway\`
  public origin.
- Keep one deployment model per client configuration. Do not mix direct local service and unified
  gateway assumptions in the same client instance.

## Configuration

\`\`\`dart
final client = SdkworkBackendClient(
  config: const SdkworkBackendConfig(
    baseUrl: 'http://127.0.0.1:18090',
    timeout: 30000,
    headers: <String, String>{
      'X-Custom-Header': 'value',
    },
  ),
);
\`\`\`

## Surface Groups

- \`client.session\` - session API
- \`client.presence\` - presence API
- \`client.realtime\` - realtime API
- \`client.device\` - device API
- \`client.inbox\` - inbox API
- \`client.conversation\` - conversation API
- \`client.message\` - message API
- \`client.media\` - media API
- \`client.stream\` - stream API
- \`client.rtc\` - rtc API

## Publishing

This SDK includes cross-platform publish scripts in \`bin/\`:

- \`bin/publish-core.mjs\`
- \`bin/publish.sh\`
- \`bin/publish.ps1\`

## License

MIT

## Package Boundary

- Use only the package root entrypoint: \`package:backend_sdk/backend_sdk.dart\`.
- Do not import generated \`lib/src/\` imports from downstream code.
- Keep business orchestration in the composed Flutter layers under
  \`sdkwork-craw-chat-sdk-flutter/composed\` instead of re-exporting generated internals.
- The workspace normalization wrapper strips generator-only auth scaffolding and source-tree build
  residue before verification and packaging.

## Regeneration Contract

- Generator-owned files are tracked in \`.sdkwork/sdkwork-generator-manifest.json\`.
- Each run also writes \`.sdkwork/sdkwork-generator-changes.json\` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes \`.sdkwork/sdkwork-generator-report.json\` with the full execution report, including \`schemaVersion\`, \`generator\`, stable artifact paths, and the execution handoff commands that match CLI \`--json\` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in \`custom/\`.
- Files scaffolded under \`custom/\` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to \`.sdkwork/manual-backups/\` before overwrite or removal.
`;
}

function normalizeTypeScript(workspaceRoot) {
  const generatedRoot = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
  );
  const composedRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'composed');

  normalizeTypeScriptPackageJson(path.join(generatedRoot, 'package.json'));
  writeIfChanged(path.join(generatedRoot, 'src', 'index.ts'), renderTypeScriptIndex());
  writeIfChanged(path.join(generatedRoot, 'src', 'sdk.ts'), renderTypeScriptSdk());
  writeIfChanged(path.join(generatedRoot, 'src', 'http', 'client.ts'), renderTypeScriptHttpClient());
  writeIfChanged(path.join(generatedRoot, 'src', 'types', 'common.ts'), renderTypeScriptCommonTypes());
  writeIfChanged(path.join(generatedRoot, 'README.md'), renderTypeScriptReadme());
  writeIfChanged(path.join(composedRoot, 'src', 'shims-sdk-common.d.ts'), renderTypeScriptCommonShim());
  removeIfExists(path.join(generatedRoot, 'src', 'auth'));
  removeIfExists(path.join(generatedRoot, 'dist', 'auth'));
  removeIfExists(path.join(generatedRoot, 'src', 'index.js'));
  removeIfExists(path.join(generatedRoot, 'src', 'index.d.ts'));
}

function normalizeFlutter(workspaceRoot) {
  const generatedRoot = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-flutter',
    'generated',
    'server-openapi',
  );

  normalizeFlutterPubspec(path.join(generatedRoot, 'pubspec.yaml'));
  writeIfChanged(path.join(generatedRoot, 'lib', 'backend_client.dart'), renderFlutterBackendClient());
  writeIfChanged(path.join(generatedRoot, 'README.md'), renderFlutterReadme());
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

if (languageSet.has('typescript')) {
  normalizeTypeScript(workspaceRoot);
}

if (languageSet.has('flutter')) {
  normalizeFlutter(workspaceRoot);
}

console.log(
  `[sdkwork-craw-chat-sdk] Normalized generated transport packages for ${[...languageSet].sort().join(', ')}.`,
);
