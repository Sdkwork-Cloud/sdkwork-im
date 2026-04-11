import type {
  CrawChatBackendClientLike,
  CrawChatClientCreateOptions,
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
): Promise<CrawChatBackendClientLike> {
  const moduleExport = await dynamicImportModule('@sdkwork/craw-chat-backend-sdk');
  const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
  if (typeof createClient !== 'function') {
    throw new Error(
      'Unable to resolve @sdkwork/craw-chat-backend-sdk createClient factory',
    );
  }
  return createClient(backendConfig) as Promise<CrawChatBackendClientLike>;
}

export async function resolveBackendClient(
  options: CrawChatClientCreateOptions,
): Promise<CrawChatBackendClientLike> {
  if (options.backendClient) {
    return options.backendClient;
  }
  if (options.backendConfig) {
    return createGeneratedBackendClient(options.backendConfig);
  }
  throw new Error('backendClient or backendConfig is required');
}

export class CrawChatSdkContext {
  constructor(readonly backendClient: CrawChatBackendClientLike) {}

  setAuthToken(token: string): void {
    this.backendClient.setAuthToken?.(token);
  }
}
