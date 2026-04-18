import type {
  CrawChatManagementBackendClientLike,
  CrawChatSdkManagementClientCreateOptions,
  SdkworkBackendConfig,
} from './types.js';

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

async function dynamicImportModule(moduleName: string): Promise<unknown> {
  const dynamicImport = new Function('name', 'return import(name);') as (name: string) => Promise<unknown>;
  return dynamicImport(moduleName);
}

export async function createGeneratedBackendClient(
  backendConfig: SdkworkBackendConfig,
): Promise<CrawChatManagementBackendClientLike> {
  const moduleExport = await dynamicImportModule('@sdkwork/craw-chat-management-backend-sdk');
  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error('Unable to resolve @sdkwork/craw-chat-management-backend-sdk createClient factory');
  }
  return createClient(backendConfig) as Promise<CrawChatManagementBackendClientLike>;
}

export async function resolveBackendClient(
  options: CrawChatSdkManagementClientCreateOptions,
): Promise<CrawChatManagementBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }
  if (options.backendConfig) {
    return createGeneratedBackendClient(options.backendConfig);
  }
  throw new Error('backendClient or backendConfig is required');
}

export class CrawChatSdkManagementContext {
  constructor(readonly backendClient: CrawChatManagementBackendClientLike) {}

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
