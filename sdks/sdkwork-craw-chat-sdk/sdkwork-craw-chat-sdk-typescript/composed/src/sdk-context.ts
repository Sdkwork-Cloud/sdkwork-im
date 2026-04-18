import type {
  CrawChatBackendClientLike,
  CrawChatSdkClientCreateOptions,
  SdkworkBackendConfig,
} from './types.js';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function isModuleNotFoundError(
  error: unknown,
  moduleName: string,
): boolean {
  return (
    isRecord(error) &&
    error.code === 'ERR_MODULE_NOT_FOUND' &&
    typeof error.message === 'string' &&
    error.message.includes(moduleName)
  );
}

async function dynamicImportModule(moduleName: string): Promise<unknown> {
  const dynamicImport = new Function(
    'name',
    'return import(name);',
  ) as (name: string) => Promise<unknown>;
  return dynamicImport(moduleName);
}

async function loadGeneratedBackendModule(): Promise<unknown> {
  try {
    return await dynamicImportModule('@sdkwork/craw-chat-backend-sdk');
  } catch (error) {
    if (!isModuleNotFoundError(error, '@sdkwork/craw-chat-backend-sdk')) {
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
  backendConfig: SdkworkBackendConfig,
): Promise<CrawChatBackendClientLike> {
  const moduleExport = await loadGeneratedBackendModule();
  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error(
      'Unable to resolve @sdkwork/craw-chat-backend-sdk createClient factory',
    );
  }
  return createClient(backendConfig) as Promise<CrawChatBackendClientLike>;
}

function resolveBackendConfig(
  options: CrawChatSdkClientCreateOptions,
): SdkworkBackendConfig | undefined {
  if (options.baseUrl) {
    return {
      baseUrl: options.baseUrl,
      authToken: options.authToken,
      tokenManager: options.tokenManager,
      timeout: options.timeout,
      headers: options.headers,
    };
  }
  return undefined;
}

export async function resolveBackendClient(
  options: CrawChatSdkClientCreateOptions,
): Promise<CrawChatBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }
  const backendConfig = resolveBackendConfig(options);
  if (backendConfig) {
    return createGeneratedBackendClient(backendConfig);
  }
  throw new Error('backendClient or baseUrl is required');
}

export class CrawChatSdkContext {
  constructor(readonly backendClient: CrawChatBackendClientLike) {}

  setAuthToken(token: string): void {
    this.backendClient.setAuthToken?.(token);
  }
}
