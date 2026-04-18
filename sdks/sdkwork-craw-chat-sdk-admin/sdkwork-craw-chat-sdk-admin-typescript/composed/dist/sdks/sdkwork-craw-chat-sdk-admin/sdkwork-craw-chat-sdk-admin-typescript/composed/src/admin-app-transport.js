import { AdminApiError } from './generated-backend-types.js';
const adminSessionTokenKey = 'sdkwork.router.admin.session-token';
const adminProxyPrefix = '/api/admin';
let cachedAdminDesktopBaseUrl = null;
export function adminBaseUrl() {
    return cachedAdminDesktopBaseUrl ?? adminProxyPrefix;
}
function resolveWindow() {
    if (typeof window === 'undefined') {
        return null;
    }
    return window;
}
function isDesktopRuntime() {
    const currentWindow = resolveWindow();
    return Boolean(currentWindow?.isTauri
        || currentWindow?.__TAURI__
        || currentWindow?.__TAURI_INTERNALS__);
}
function trimTrailingSlash(value) {
    return value.replace(/\/+$/g, '');
}
function joinUrl(baseUrl, requestPath) {
    const normalizedBase = trimTrailingSlash(baseUrl);
    const normalizedPath = requestPath.startsWith('/') ? requestPath : `/${requestPath}`;
    return `${normalizedBase}${normalizedPath}`;
}
async function invokeDesktopCommand(command, args) {
    const invoke = resolveWindow()?.__TAURI_INTERNALS__?.invoke;
    if (typeof invoke !== 'function') {
        throw new Error('Tauri invoke bridge is unavailable.');
    }
    return invoke(command, args);
}
async function resolveAdminBaseUrl() {
    if (cachedAdminDesktopBaseUrl) {
        return cachedAdminDesktopBaseUrl;
    }
    if (!isDesktopRuntime()) {
        return adminProxyPrefix;
    }
    try {
        const runtimeBaseUrl = await invokeDesktopCommand('runtime_base_url');
        const normalizedBaseUrl = runtimeBaseUrl?.trim();
        if (normalizedBaseUrl) {
            cachedAdminDesktopBaseUrl = joinUrl(normalizedBaseUrl, adminProxyPrefix);
            return cachedAdminDesktopBaseUrl;
        }
    }
    catch {
        // Fall back to the browser-style relative proxy path when the desktop bridge is unavailable.
    }
    return adminProxyPrefix;
}
export function readAdminSessionToken() {
    return globalThis.localStorage?.getItem(adminSessionTokenKey) ?? null;
}
export function persistAdminSessionToken(token) {
    globalThis.localStorage?.setItem(adminSessionTokenKey, token);
}
export function clearAdminSessionToken() {
    globalThis.localStorage?.removeItem(adminSessionTokenKey);
}
function resolveAdminErrorMessage(status, payload) {
    if (typeof payload === 'object' && payload !== null) {
        const candidate = payload;
        const message = candidate.error?.message?.trim()
            || candidate.message?.trim();
        if (message) {
            return message;
        }
    }
    return `Admin request failed with status ${status}`;
}
async function readJson(response) {
    let payload = null;
    try {
        payload = await response.json();
    }
    catch {
        payload = null;
    }
    if (!response.ok) {
        throw new AdminApiError(response.status, payload, resolveAdminErrorMessage(response.status, payload));
    }
    return payload;
}
export function requiredToken(token) {
    const sessionToken = token ?? readAdminSessionToken();
    if (!sessionToken) {
        throw new AdminApiError(401, null, 'Admin session token not found');
    }
    return sessionToken;
}
export async function getJson(requestPath, token) {
    const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
        headers: {
            authorization: `Bearer ${requiredToken(token)}`,
        },
    });
    return readJson(response);
}
export async function postJson(requestPath, body, token) {
    const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
        method: 'POST',
        headers: {
            'content-type': 'application/json',
            ...(token ? { authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify(body),
    });
    return readJson(response);
}
export async function patchJson(requestPath, body, token) {
    const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
        method: 'PATCH',
        headers: {
            'content-type': 'application/json',
            ...(token ? { authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify(body),
    });
    return readJson(response);
}
export async function putJson(requestPath, body, token) {
    const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
        method: 'PUT',
        headers: {
            'content-type': 'application/json',
            ...(token ? { authorization: `Bearer ${token}` } : {}),
        },
        body: JSON.stringify(body),
    });
    return readJson(response);
}
export async function deleteEmpty(requestPath, token) {
    const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
        method: 'DELETE',
        headers: {
            authorization: `Bearer ${requiredToken(token)}`,
        },
    });
    if (!response.ok) {
        await readJson(response);
    }
}
