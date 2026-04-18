import type {
  CrawChatAdminBackendClientLike,
  CrawChatSdkAdminClientCreateOptions,
  SdkworkBackendConfig,
} from './types.js';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

async function dynamicImportModule(moduleName: string): Promise<unknown> {
  const dynamicImport = new Function(
    'name',
    'return import(name);',
  ) as (name: string) => Promise<unknown>;
  return dynamicImport(moduleName);
}

export async function createGeneratedBackendClient(
  backendConfig: SdkworkBackendConfig,
): Promise<CrawChatAdminBackendClientLike> {
  const moduleExport = await dynamicImportModule('@sdkwork/craw-chat-admin-backend-sdk');
  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error(
      'Unable to resolve @sdkwork/craw-chat-admin-backend-sdk createClient factory',
    );
  }
  return createClient(backendConfig) as Promise<CrawChatAdminBackendClientLike>;
}

export async function resolveBackendClient(
  options: CrawChatSdkAdminClientCreateOptions,
): Promise<CrawChatAdminBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }
  if (options.backendConfig) {
    return createGeneratedBackendClient(options.backendConfig);
  }
  throw new Error('backendClient or backendConfig is required');
}

export class CrawChatSdkAdminContext {
  constructor(readonly backendClient: CrawChatAdminBackendClientLike) {}

  setAuthToken(token: string): void {
    this.backendClient.setAuthToken?.(token);
  }

  setAccessToken(token: string): void {
    this.backendClient.setAccessToken?.(token);
  }

  setApiKey(apiKey: string): void {
    this.backendClient.setApiKey?.(apiKey);
  }
}
