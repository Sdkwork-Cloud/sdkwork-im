declare module '@sdkwork/sdk-common' {
  export type QueryParams = Record<string, string | number | boolean | undefined>;

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
    timeout?: number;
    headers?: Record<string, string>;
  }

  export interface RequestOptions extends RequestConfig {}

  export interface AuthTokens {
    authToken?: string;
    refreshToken?: string;
  }

  export interface AuthTokenManager {
    getTokens?: () => AuthTokens | Promise<AuthTokens>;
    refreshTokens?: () => AuthTokens | Promise<AuthTokens>;
  }

  export const DEFAULT_TIMEOUT: number;
  export const SUCCESS_CODES: number[];
}
