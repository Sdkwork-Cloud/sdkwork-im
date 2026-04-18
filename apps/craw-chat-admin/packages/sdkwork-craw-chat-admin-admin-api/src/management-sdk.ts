import { createClient } from '@sdkwork/craw-chat-management-backend-sdk';
import { CrawChatSdkManagementClient } from '@sdkwork/craw-chat-sdk-management';

import { AdminApiError, requiredToken, resolveAdminSdkBaseUrl } from './transport';

export type AdminManagementModuleName =
  | 'auth'
  | 'users'
  | 'marketing'
  | 'tenants'
  | 'access'
  | 'routing'
  | 'catalog'
  | 'usage'
  | 'billing'
  | 'operations';

type AdminManagementMethod = (...args: unknown[]) => Promise<unknown>;
type AdminManagementModule = Record<string, AdminManagementMethod>;

interface AdminManagementCallOptions {
  token?: string;
  requireAuth?: boolean;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

function resolveAdminErrorStatus(error: unknown): number {
  if (error instanceof AdminApiError) {
    return error.status;
  }

  if (isRecord(error) && typeof error.httpStatus === 'number') {
    return error.httpStatus;
  }

  if (isRecord(error) && typeof error.status === 'number') {
    return error.status;
  }

  return 500;
}

function resolveAdminErrorMessage(error: unknown, status: number): string {
  if (error instanceof Error && error.message.trim()) {
    return error.message.trim();
  }

  if (isRecord(error) && typeof error.message === 'string' && error.message.trim()) {
    return error.message.trim();
  }

  return `Admin request failed with status ${status}`;
}

function toAdminApiError(error: unknown): AdminApiError {
  if (error instanceof AdminApiError) {
    return error;
  }

  return new AdminApiError(
    resolveAdminErrorMessage(error, resolveAdminErrorStatus(error)),
    resolveAdminErrorStatus(error),
  );
}

async function createAdminManagementSdk(
  token: string | undefined,
  requireAuth: boolean,
): Promise<CrawChatSdkManagementClient> {
  const authToken = requireAuth ? requiredToken(token) : token;
  const backendClient = createClient({
    baseUrl: await resolveAdminSdkBaseUrl(),
    ...(authToken ? { authToken } : {}),
  });

  return new CrawChatSdkManagementClient({ backendClient });
}

export async function callAdminManagementMethod<T>(
  moduleName: AdminManagementModuleName,
  methodName: string,
  args: unknown[] = [],
  options: AdminManagementCallOptions = {},
): Promise<T> {
  try {
    const sdk = await createAdminManagementSdk(
      options.token,
      options.requireAuth ?? true,
    );
    const moduleCandidate = sdk[moduleName] as unknown;
    if (!isRecord(moduleCandidate)) {
      throw new Error(`Management SDK module "${moduleName}" is unavailable.`);
    }

    const methodCandidate = moduleCandidate[methodName];
    if (typeof methodCandidate !== 'function') {
      throw new Error(
        `Management SDK method "${moduleName}.${methodName}" is unavailable.`,
      );
    }

    return (await methodCandidate.apply(moduleCandidate, args)) as T;
  } catch (error) {
    throw toAdminApiError(error);
  }
}
