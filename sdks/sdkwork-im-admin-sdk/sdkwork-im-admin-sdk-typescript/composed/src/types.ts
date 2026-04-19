import type {
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
  | 'auth'
  | 'users'
  | 'marketing'
  | 'tenants'
  | 'access'
  | 'routing'
  | 'catalog'
  | 'usage'
  | 'billing'
  | 'operations'
  | 'storage'
  | 'setAuthToken'
  | 'setTokenManager'
  | 'http'
>;
