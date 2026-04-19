import type {
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
