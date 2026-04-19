export type JsonPrimitive = string | number | boolean | null;
export type JsonValue = JsonPrimitive | JsonObject | JsonValue[];
export interface JsonObject {
    [key: string]: JsonValue | undefined;
}
export type QueryValue = JsonPrimitive | undefined;
export type QueryParams = Record<string, QueryValue>;
export type Identifier = string | number;
export interface FetchRequestInitLike {
    method?: string;
    headers?: Record<string, string>;
    body?: string;
}
export interface FetchResponseLike {
    ok: boolean;
    status: number;
    json(): Promise<unknown>;
    text(): Promise<string>;
}
export type FetchLike = (input: string, init?: FetchRequestInitLike) => Promise<FetchResponseLike>;
export interface ControlPlaneBackendConfig {
    baseUrl: string;
    authToken?: string;
    headers?: Record<string, string>;
    timeout?: number;
    fetch?: FetchLike;
}
export interface ControlPlaneErrorResponse extends JsonObject {
    status?: string;
    code?: string;
    message?: string;
}
export declare const DEFAULT_TIMEOUT = 15000;
export declare class AdminApiError extends Error {
    readonly status: number;
    readonly payload: unknown;
    constructor(status: number, payload: unknown, message?: string);
}
//# sourceMappingURL=common.d.ts.map