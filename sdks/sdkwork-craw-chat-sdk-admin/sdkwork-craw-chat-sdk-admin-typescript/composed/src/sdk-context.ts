import type {
  CrawChatAdminBackendClientLike,
  CrawChatAdminSdkClientCreateOptions,
  CrawChatAdminBackendConfig,
} from './types.js';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function isModuleNotFoundError(error: unknown, moduleName: string): boolean {
  return (
    isRecord(error) &&
    error.code === 'ERR_MODULE_NOT_FOUND' &&
    typeof error.message === 'string' &&
    error.message.includes(moduleName)
  );
}

async function dynamicImportModule(moduleName: string): Promise<unknown> {
  const dynamicImport = new Function('name', 'return import(name);') as (
    name: string,
  ) => Promise<unknown>;
  return dynamicImport(moduleName);
}

async function loadGeneratedBackendModule(): Promise<unknown> {
  try {
    return await dynamicImportModule('@sdkwork/craw-chat-admin-backend-sdk');
  } catch (error) {
    if (!isModuleNotFoundError(error, '@sdkwork/craw-chat-admin-backend-sdk')) {
      throw error;
    }
  }

  const workspaceFallbackHref = new URL(
    '../../generated/server-openapi/dist/index.js',
    import.meta.url,
  ).href;
  return dynamicImportModule(workspaceFallbackHref);
}

export async function createGeneratedBackendClient(
  backendConfig: CrawChatAdminBackendConfig,
): Promise<CrawChatAdminBackendClientLike> {
  const moduleExport = await loadGeneratedBackendModule();
  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error(
      'Unable to resolve @sdkwork/craw-chat-admin-backend-sdk createClient factory.',
    );
  }
  return createClient(backendConfig) as CrawChatAdminBackendClientLike;
}

function resolveBackendConfig(
  options: CrawChatAdminSdkClientCreateOptions,
): CrawChatAdminBackendConfig | undefined {
  if (options.baseUrl) {
    return {
      baseUrl: options.baseUrl,
      authToken: options.authToken,
      headers: options.headers,
      timeout: options.timeout,
      fetch: options.fetch,
    };
  }
  return undefined;
}

export async function resolveBackendClient(
  options: CrawChatAdminSdkClientCreateOptions,
): Promise<CrawChatAdminBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }

  const backendConfig = resolveBackendConfig(options);
  if (backendConfig) {
    return createGeneratedBackendClient(backendConfig);
  }

  throw new Error('backendClient or baseUrl is required.');
}

export class CrawChatAdminSdkContext {
  constructor(readonly backendClient: CrawChatAdminBackendClientLike) {}

  setAuthToken(token: string): void {
    this.backendClient.setAuthToken?.(token);
  }
}
