export declare function adminBaseUrl(): string;
export declare function readAdminSessionToken(): string | null;
export declare function persistAdminSessionToken(token: string): void;
export declare function clearAdminSessionToken(): void;
export declare function requiredToken(token?: string): string;
export declare function getJson<T>(requestPath: string, token?: string): Promise<T>;
export declare function postJson<TRequest, TResponse>(requestPath: string, body: TRequest, token?: string): Promise<TResponse>;
export declare function patchJson<TRequest, TResponse>(requestPath: string, body: TRequest, token?: string): Promise<TResponse>;
export declare function putJson<TRequest, TResponse>(requestPath: string, body: TRequest, token?: string): Promise<TResponse>;
export declare function deleteEmpty(requestPath: string, token?: string): Promise<void>;
//# sourceMappingURL=admin-app-transport.d.ts.map