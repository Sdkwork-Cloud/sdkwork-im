import { AdminApiError } from './generated-backend-types.js';

const adminSessionTokenKey = 'sdkwork.router.admin.session-token';
const adminProxyPrefix = '/api/admin';

type TauriWindowLike = Window & {
  __TAURI__?: unknown;
  __TAURI_INTERNALS__?: TauriInternalsLike;
  isTauri?: boolean;
};

type TauriInternalsLike = {
  invoke?: <T>(command: string, args?: Record<string, unknown>) => Promise<T>;
};

let cachedAdminDesktopBaseUrl: string | null = null;

export function adminBaseUrl(): string {
  return cachedAdminDesktopBaseUrl ?? adminProxyPrefix;
}

function resolveWindow(): TauriWindowLike | null {
  if (typeof window === 'undefined') {
    return null;
  }

  return window as TauriWindowLike;
}

function isDesktopRuntime(): boolean {
  const currentWindow = resolveWindow();
  return Boolean(
    currentWindow?.isTauri
      || currentWindow?.__TAURI__
      || currentWindow?.__TAURI_INTERNALS__,
  );
}

function trimTrailingSlash(value: string): string {
  return value.replace(/\/+$/g, '');
}

function joinUrl(baseUrl: string, requestPath: string): string {
  const normalizedBase = trimTrailingSlash(baseUrl);
  const normalizedPath = requestPath.startsWith('/') ? requestPath : `/${requestPath}`;
  return `${normalizedBase}${normalizedPath}`;
}

async function invokeDesktopCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const invoke = resolveWindow()?.__TAURI_INTERNALS__?.invoke;
  if (typeof invoke !== 'function') {
    throw new Error('Tauri invoke bridge is unavailable.');
  }

  return invoke<T>(command, args);
}

async function resolveAdminBaseUrl(): Promise<string> {
  if (cachedAdminDesktopBaseUrl) {
    return cachedAdminDesktopBaseUrl;
  }

  if (!isDesktopRuntime()) {
    return adminProxyPrefix;
  }

  try {
    const runtimeBaseUrl = await invokeDesktopCommand<string>('runtime_base_url');
    const normalizedBaseUrl = runtimeBaseUrl?.trim();
    if (normalizedBaseUrl) {
      cachedAdminDesktopBaseUrl = joinUrl(normalizedBaseUrl, adminProxyPrefix);
      return cachedAdminDesktopBaseUrl;
    }
  } catch {
    // Fall back to the browser-style relative proxy path when the desktop bridge is unavailable.
  }

  return adminProxyPrefix;
}

export function readAdminSessionToken(): string | null {
  return globalThis.localStorage?.getItem(adminSessionTokenKey) ?? null;
}

export function persistAdminSessionToken(token: string): void {
  globalThis.localStorage?.setItem(adminSessionTokenKey, token);
}

export function clearAdminSessionToken(): void {
  globalThis.localStorage?.removeItem(adminSessionTokenKey);
}

function resolveAdminErrorMessage(status: number, payload: unknown): string {
  if (typeof payload === 'object' && payload !== null) {
    const candidate = payload as {
      error?: { message?: string };
      message?: string;
    };
    const message =
      candidate.error?.message?.trim()
      || candidate.message?.trim();
    if (message) {
      return message;
    }
  }

  return `Admin request failed with status ${status}`;
}

async function readJson<T>(response: Response): Promise<T> {
  let payload: unknown = null;

  try {
    payload = await response.json();
  } catch {
    payload = null;
  }

  if (!response.ok) {
    throw new AdminApiError(
      response.status,
      payload,
      resolveAdminErrorMessage(response.status, payload),
    );
  }

  return payload as T;
}

export function requiredToken(token?: string): string {
  const sessionToken = token ?? readAdminSessionToken();
  if (!sessionToken) {
    throw new AdminApiError(401, null, 'Admin session token not found');
  }
  return sessionToken;
}

export async function getJson<T>(requestPath: string, token?: string): Promise<T> {
  const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
    headers: {
      authorization: `Bearer ${requiredToken(token)}`,
    },
  });
  return readJson<T>(response);
}

export async function postJson<TRequest, TResponse>(
  requestPath: string,
  body: TRequest,
  token?: string,
): Promise<TResponse> {
  const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
      ...(token ? { authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(body),
  });
  return readJson<TResponse>(response);
}

export async function patchJson<TRequest, TResponse>(
  requestPath: string,
  body: TRequest,
  token?: string,
): Promise<TResponse> {
  const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
    method: 'PATCH',
    headers: {
      'content-type': 'application/json',
      ...(token ? { authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(body),
  });
  return readJson<TResponse>(response);
}

export async function putJson<TRequest, TResponse>(
  requestPath: string,
  body: TRequest,
  token?: string,
): Promise<TResponse> {
  const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
    method: 'PUT',
    headers: {
      'content-type': 'application/json',
      ...(token ? { authorization: `Bearer ${token}` } : {}),
    },
    body: JSON.stringify(body),
  });
  return readJson<TResponse>(response);
}

export async function deleteEmpty(requestPath: string, token?: string): Promise<void> {
  const response = await fetch(`${await resolveAdminBaseUrl()}${requestPath}`, {
    method: 'DELETE',
    headers: {
      authorization: `Bearer ${requiredToken(token)}`,
    },
  });

  if (!response.ok) {
    await readJson<never>(response);
  }
}
