declare module '@sdkwork/sdk-common' {
  export type QueryParamScalar = string | number | boolean | undefined | null;
  export type QueryParamValue = QueryParamScalar | Array<string | number | boolean>;
  export type QueryParams = Record<string, QueryParamValue>;

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
    url: string;
    method: string;
    headers?: Record<string, string>;
    params?: QueryParams;
    body?: unknown;
    timeout?: number;
    signal?: AbortSignal;
    skipAuth?: boolean;
    retryCount?: number;
    metadata?: Record<string, unknown>;
  }

  export interface RequestOptions {
    method?: string;
    headers?: Record<string, string>;
    body?: unknown;
    params?: QueryParams;
    signal?: AbortSignal;
    skipAuth?: boolean;
    requiresAuth?: boolean;
    timeout?: number;
    retry?: {
      maxRetries?: number;
      retryDelay?: number;
      retryBackoff?: 'fixed' | 'linear' | 'exponential';
      maxRetryDelay?: number;
    };
    cache?: boolean | number;
    metadata?: Record<string, unknown>;
  }

  export interface AuthTokens {
    authToken?: string;
    refreshToken?: string;
    expiresIn?: number;
    expiresAt?: number;
    tokenType?: string;
    scope?: string;
  }

  export interface AuthTokenManager {
    getAuthToken(): string | undefined;
    getRefreshToken(): string | undefined;
    getTokens(): AuthTokens;
    setTokens(tokens: AuthTokens): void;
    setAuthToken(token: string): void;
    setRefreshToken(token: string): void;
    clearTokens(): void;
    clearAuthToken(): void;
    isExpired(): boolean;
    isValid(): boolean;
    hasToken(): boolean;
    hasAuthToken(): boolean;
    willExpireIn(seconds: number): boolean;
  }

  export const DEFAULT_TIMEOUT: number;
  export const SUCCESS_CODES: Array<number | string>;

  export class DefaultAuthTokenManager implements AuthTokenManager {
    constructor(initialTokens?: AuthTokens);
    getAuthToken(): string | undefined;
    getRefreshToken(): string | undefined;
    getTokens(): AuthTokens;
    setTokens(tokens: AuthTokens): void;
    setAuthToken(token: string): void;
    setRefreshToken(token: string): void;
    clearTokens(): void;
    clearAuthToken(): void;
    isExpired(): boolean;
    isValid(): boolean;
    hasToken(): boolean;
    hasAuthToken(): boolean;
    willExpireIn(seconds: number): boolean;
  }

  export function createTokenManager(tokens?: AuthTokens): AuthTokenManager;

  export abstract class BaseHttpClient {
    constructor(config: Record<string, unknown>);
    setAuthToken(token: string): void;
    setTokenManager(manager: AuthTokenManager): void;
    execute<T>(config: RequestConfig): Promise<T>;
    abstract request<T>(path: string, options?: RequestOptions): Promise<T>;
    abstract get<T>(path: string, params?: QueryParams): Promise<T>;
    abstract post<T>(path: string, body?: unknown): Promise<T>;
    abstract put<T>(path: string, body?: unknown): Promise<T>;
    abstract delete<T>(path: string, body?: unknown): Promise<T>;
    abstract patch<T>(path: string, body?: unknown): Promise<T>;
  }

  export function withRetry<T>(
    fn: () => Promise<T>,
    config?: {
      maxRetries?: number;
      retryDelay?: number;
      retryBackoff?: 'fixed' | 'linear' | 'exponential';
      maxRetryDelay?: number;
    },
  ): Promise<T>;
}
