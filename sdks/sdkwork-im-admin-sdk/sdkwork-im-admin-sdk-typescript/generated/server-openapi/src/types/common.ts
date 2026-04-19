export interface BasePlusVO {
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

export type LooseJsonValue = unknown;
export type LooseJsonObject = Record<string, unknown>;

export type { Page, PageResult, RequestConfig, RequestOptions, QueryParams } from '@sdkwork/sdk-common';
export { DEFAULT_TIMEOUT, SUCCESS_CODES } from '@sdkwork/sdk-common';
import type { AuthTokenManager, AuthTokens } from '@sdkwork/sdk-common';
export type { AuthTokenManager, AuthTokens };

export interface ImAdminBackendConfig {
  baseUrl: string;
  authToken?: string;
  tokenManager?: AuthTokenManager;
  timeout?: number;
  headers?: Record<string, string>;
}
