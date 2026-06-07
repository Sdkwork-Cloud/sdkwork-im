import type { IamAppContext } from '@sdkwork/iam-contracts';
import type { AuthTokenManager, AuthTokens, Interceptors, RequestConfig } from '@sdkwork/sdk-common';

export interface SdkworkChatSessionUser {
  avatar?: string;
  displayName?: string;
  email?: string;
  id?: string | number;
  name?: string;
  nickname?: string;
  phone?: string;
  userId?: string;
  username?: string;
}

export interface SdkworkChatSessionTokens {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
}

export interface SdkworkChatAppContext extends IamAppContext {
  actorId?: string;
  actorKind?: string;
  contextSignature?: string;
  deviceId?: string;
}

export interface SdkworkChatSession extends SdkworkChatSessionTokens {
  context?: SdkworkChatAppContext;
  expiresAt?: number;
  sessionId?: string;
  user?: SdkworkChatSessionUser;
}

export interface SdkworkChatSessionChangedDetail {
  session: SdkworkChatSession | null;
}

export type SdkworkChatRequestContext = Partial<SdkworkChatAppContext>;

const SDKWORK_CHAT_SESSION_KEY = 'sdkwork-chat-pc:session:v1';
export const SDKWORK_CHAT_SESSION_CHANGED_EVENT = 'sdkwork-chat-pc:auth-session-changed';
const SDKWORK_CHAT_CONTEXT_HEADER_NAMES = new Set([
  'x-sdkwork-app-id',
  'x-sdkwork-tenant-id',
  'x-sdkwork-organization-id',
  'x-sdkwork-user-id',
  'x-sdkwork-session-id',
  'x-sdkwork-environment',
  'x-sdkwork-deployment-mode',
  'x-sdkwork-auth-level',
  'x-sdkwork-actor-id',
  'x-sdkwork-actor-kind',
  'x-sdkwork-device-id',
  'x-sdkwork-data-scope',
  'x-sdkwork-permission-scope',
  'x-sdkwork-context-signature',
  'x-tenant-id',
  'x-organization-id',
  'x-user-id',
]);

let sdkworkChatGlobalTokenManager: AuthTokenManager | null = null;
let sdkworkChatGlobalTokenManagerSession: SdkworkChatSession | null = null;

function getStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.localStorage;
}

function dispatchAppSdkSessionChanged(session: SdkworkChatSession | null): void {
  if (typeof window === 'undefined') {
    return;
  }
  window.dispatchEvent(new CustomEvent(
    SDKWORK_CHAT_SESSION_CHANGED_EVENT,
    { detail: { session } },
  ));
}

function normalizeToken(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeStringArray(value: unknown): string[] {
  if (Array.isArray(value)) {
    return value
      .map((item) => normalizeString(item))
      .filter(Boolean) as string[];
  }

  const normalized = normalizeString(value);
  if (!normalized) {
    return [];
  }
  const separator = normalized.includes(',') ? /,/u : /\s+/u;
  return normalized
    .split(separator)
    .map((item) => item.trim())
    .filter(Boolean);
}

function normalizeExpiresAt(value: unknown): number | undefined {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }
  if (typeof value !== 'string' || !value.trim()) {
    return undefined;
  }
  const timestamp = Date.parse(value);
  return Number.isFinite(timestamp) ? timestamp : undefined;
}

function decodeBase64UrlJson(value: string): Record<string, unknown> | undefined {
  try {
    const normalized = value.replace(/-/gu, '+').replace(/_/gu, '/');
    const padded = normalized.padEnd(Math.ceil(normalized.length / 4) * 4, '=');
    const binary = atob(padded);
    const bytes = Uint8Array.from(binary, (char) => char.charCodeAt(0));
    const text = new TextDecoder().decode(bytes);
    const parsed = JSON.parse(text);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

function decodeJwtClaims(token?: string): Record<string, unknown> | undefined {
  const normalized = normalizeToken(token);
  if (!normalized) {
    return undefined;
  }
  const [, payload] = normalized.split('.');
  if (!payload) {
    return undefined;
  }
  return decodeBase64UrlJson(payload);
}

function readSessionJwtClaims(session?: SdkworkChatSession | null): Record<string, unknown>[] {
  return [
    decodeJwtClaims(session?.accessToken),
    decodeJwtClaims(session?.authToken),
  ].filter(Boolean) as Record<string, unknown>[];
}

function pickClaimString(
  session: SdkworkChatSession | null | undefined,
  claimKeys: string[],
  ...fallbacks: unknown[]
): string | undefined {
  for (const claims of readSessionJwtClaims(session)) {
    for (const key of claimKeys) {
      const value = normalizeString(claims[key]);
      if (value) {
        return value;
      }
    }
  }
  return normalizeString(fallbacks.find((value) => normalizeString(value)));
}

function pickClaimStringArray(
  session: SdkworkChatSession | null | undefined,
  claimKeys: string[],
  fallback?: unknown,
): string[] {
  for (const claims of readSessionJwtClaims(session)) {
    for (const key of claimKeys) {
      const values = normalizeStringArray(claims[key]);
      if (values.length > 0) {
        return values;
      }
    }
  }
  return normalizeStringArray(fallback);
}

function normalizeContext(value: unknown): SdkworkChatAppContext | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }
  const context = value as Partial<IamAppContext>;
  if (!normalizeString(context.appId) || !normalizeString(context.tenantId) || !normalizeString(context.userId)) {
    return undefined;
  }
  return {
    ...context,
    appId: normalizeString(context.appId) ?? '',
    tenantId: normalizeString(context.tenantId) ?? '',
    userId: normalizeString(context.userId) ?? '',
    sessionId: normalizeString(context.sessionId) ?? normalizeString((value as { sessionId?: unknown }).sessionId) ?? '',
    environment: context.environment ?? 'dev',
    deploymentMode: context.deploymentMode ?? 'local',
    authLevel: context.authLevel ?? 'password',
    dataScope: Array.isArray(context.dataScope) ? context.dataScope : [],
    permissionScope: Array.isArray(context.permissionScope) ? context.permissionScope : [],
  };
}

export function normalizeSdkworkChatSessionUser(value: unknown): SdkworkChatSessionUser | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }
  const user = value as Partial<SdkworkChatSessionUser>;
  const id = normalizeString(user.userId) ?? normalizeString(user.id);
  const avatar = normalizeString(user.avatar);
  const normalized: SdkworkChatSessionUser = {
    ...(avatar ? { avatar } : {}),
    ...(normalizeString(user.displayName) ? { displayName: normalizeString(user.displayName) } : {}),
    ...(normalizeString(user.email) ? { email: normalizeString(user.email) } : {}),
    ...(id ? { id } : {}),
    ...(normalizeString(user.name) ? { name: normalizeString(user.name) } : {}),
    ...(normalizeString(user.nickname) ? { nickname: normalizeString(user.nickname) } : {}),
    ...(normalizeString(user.phone) ? { phone: normalizeString(user.phone) } : {}),
    ...(normalizeString(user.userId) ? { userId: normalizeString(user.userId) } : {}),
    ...(normalizeString(user.username) ? { username: normalizeString(user.username) } : {}),
  };
  return Object.keys(normalized).length > 0 ? normalized : undefined;
}

function normalizeSession(value: unknown): SdkworkChatSession | null {
  if (!value || typeof value !== 'object') {
    return null;
  }

  const candidate = value as SdkworkChatSession & { expiresAt?: unknown };
  const context = normalizeContext(candidate.context);
  const sessionId = normalizeString(candidate.sessionId) ?? normalizeString(context?.sessionId);
  const session: SdkworkChatSession = {
    accessToken: normalizeToken(candidate.accessToken),
    authToken: normalizeToken(candidate.authToken),
    refreshToken: normalizeToken(candidate.refreshToken),
    ...(context ? { context } : {}),
    ...(typeof candidate.expiresAt !== 'undefined'
      ? { expiresAt: normalizeExpiresAt(candidate.expiresAt) }
      : {}),
    ...(sessionId ? { sessionId } : {}),
    ...(normalizeSdkworkChatSessionUser(candidate.user) ? { user: normalizeSdkworkChatSessionUser(candidate.user) } : {}),
  };

  return session.authToken || session.accessToken ? session : null;
}

export function readAppSdkSessionTokens(): SdkworkChatSession | null {
  const storage = getStorage();
  if (!storage) {
    return null;
  }

  const rawValue = storage.getItem(SDKWORK_CHAT_SESSION_KEY);
  if (!rawValue) {
    return null;
  }

  try {
    return normalizeSession(JSON.parse(rawValue));
  } catch {
    storage.removeItem(SDKWORK_CHAT_SESSION_KEY);
    return null;
  }
}

export function persistAppSdkSessionTokens(session: SdkworkChatSession): SdkworkChatSession {
  const normalizedSession = normalizeSession(session);
  if (!normalizedSession) {
    clearAppSdkSessionTokens();
    throw new Error('SDKWork Chat session requires authToken or accessToken.');
  }

  getStorage()?.setItem(SDKWORK_CHAT_SESSION_KEY, JSON.stringify(normalizedSession));
  syncSdkworkChatGlobalTokenManager(normalizedSession);
  dispatchAppSdkSessionChanged(normalizedSession);
  return normalizedSession;
}

export function applyAppSdkSessionTokens(session: SdkworkChatSession): SdkworkChatSession {
  return persistAppSdkSessionTokens(session);
}

export function clearAppSdkSessionTokens(): void {
  getStorage()?.removeItem(SDKWORK_CHAT_SESSION_KEY);
  syncSdkworkChatGlobalTokenManager(null);
  dispatchAppSdkSessionChanged(null);
}

export function resolveAppSdkAccessToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.accessToken ?? session?.authToken;
}

export function resolveAppSdkAuthToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.authToken ?? session?.accessToken;
}

export function resolveAppSdkRefreshToken(session = readAppSdkSessionTokens()): string | undefined {
  return session?.refreshToken;
}

export function resolveAppSdkTenantId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(session, ['tenantId', 'tenant_id', 'tid'], session?.context?.tenantId);
}

export function resolveAppSdkOrganizationId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['organizationId', 'organization_id', 'orgId', 'org_id', 'oid'],
    session?.context?.organizationId,
  );
}

export function resolveAppSdkUserId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['userId', 'user_id', 'uid', 'sub', 'principalId', 'principal_id', 'accountId', 'account_id'],
    session?.context?.userId,
    session?.user?.userId,
    session?.user?.id,
  );
}

export function resolveAppSdkSessionId(session = readAppSdkSessionTokens()): string | undefined {
  return pickClaimString(
    session,
    ['sessionId', 'session_id', 'sid'],
    session?.sessionId,
    session?.context?.sessionId,
  );
}

function resolveAppSdkAppId(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['appId', 'app_id', 'azp', 'aud'], session?.context?.appId);
}

function resolveAppSdkEnvironment(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['environment', 'env'], session?.context?.environment);
}

function resolveAppSdkDeploymentMode(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['deploymentMode', 'deployment_mode'], session?.context?.deploymentMode);
}

function resolveAppSdkAuthLevel(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['authLevel', 'auth_level', 'acr'], session?.context?.authLevel);
}

function resolveAppSdkActorId(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['actorId', 'actor_id'], session?.context?.actorId);
}

function resolveAppSdkActorKind(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['actorKind', 'actor_kind'], session?.context?.actorKind);
}

function resolveAppSdkDeviceId(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['deviceId', 'device_id'], session?.context?.deviceId);
}

function resolveAppSdkContextSignature(session?: SdkworkChatSession | null): string | undefined {
  return pickClaimString(session, ['contextSignature', 'context_signature'], session?.context?.contextSignature);
}

export function createSdkworkChatRequestContext(
  session = readAppSdkSessionTokens(),
): SdkworkChatRequestContext | undefined {
  const context = session?.context;
  const appId = resolveAppSdkAppId(session);
  const tenantId = resolveAppSdkTenantId(session);
  const organizationId = resolveAppSdkOrganizationId(session);
  const userId = resolveAppSdkUserId(session);
  const sessionId = resolveAppSdkSessionId(session);
  const environment = resolveAppSdkEnvironment(session) as IamAppContext['environment'] | undefined;
  const deploymentMode = resolveAppSdkDeploymentMode(session) as IamAppContext['deploymentMode'] | undefined;
  const authLevel = resolveAppSdkAuthLevel(session) as IamAppContext['authLevel'] | undefined;
  const actorId = resolveAppSdkActorId(session);
  const actorKind = resolveAppSdkActorKind(session);
  const deviceId = resolveAppSdkDeviceId(session);
  const dataScope = pickClaimStringArray(session, ['dataScope', 'data_scope'], context?.dataScope);
  const permissionScope = pickClaimStringArray(
    session,
    ['permissionScope', 'permission_scope', 'scope', 'scp'],
    context?.permissionScope,
  );
  const contextSignature = resolveAppSdkContextSignature(session);
  const requestContext: SdkworkChatRequestContext = {
    ...(appId ? { appId } : {}),
    ...(tenantId ? { tenantId } : {}),
    ...(organizationId ? { organizationId } : {}),
    ...(userId ? { userId } : {}),
    ...(sessionId ? { sessionId } : {}),
    ...(environment ? { environment } : {}),
    ...(deploymentMode ? { deploymentMode } : {}),
    ...(authLevel ? { authLevel } : {}),
    ...(dataScope.length ? { dataScope } : {}),
    ...(permissionScope.length ? { permissionScope } : {}),
    ...(actorId ? { actorId } : {}),
    ...(actorKind ? { actorKind } : {}),
    ...(deviceId ? { deviceId } : {}),
    ...(contextSignature ? { contextSignature } : {}),
  };
  return Object.keys(requestContext).length > 0 ? requestContext : undefined;
}

function buildSdkworkChatAppContextHeadersFromContext(
  context?: SdkworkChatRequestContext,
): Record<string, string> | undefined {
  const headers = {
    ...(context?.appId ? { 'X-Sdkwork-App-Id': context.appId } : {}),
    ...(context?.tenantId ? { 'X-Sdkwork-Tenant-Id': context.tenantId } : {}),
    ...(context?.organizationId ? { 'X-Sdkwork-Organization-Id': context.organizationId } : {}),
    ...(context?.userId ? { 'X-Sdkwork-User-Id': context.userId } : {}),
    ...(context?.sessionId ? { 'X-Sdkwork-Session-Id': context.sessionId } : {}),
    ...(context?.environment ? { 'X-Sdkwork-Environment': context.environment } : {}),
    ...(context?.deploymentMode ? { 'X-Sdkwork-Deployment-Mode': context.deploymentMode } : {}),
    ...(context?.authLevel ? { 'X-Sdkwork-Auth-Level': context.authLevel } : {}),
    ...(context?.actorId ? { 'X-Sdkwork-Actor-Id': context.actorId } : {}),
    ...(context?.actorKind ? { 'X-Sdkwork-Actor-Kind': context.actorKind } : {}),
    ...(context?.deviceId ? { 'X-Sdkwork-Device-Id': context.deviceId } : {}),
    ...(context?.dataScope?.length ? { 'X-Sdkwork-Data-Scope': context.dataScope.join(',') } : {}),
    ...(context?.permissionScope?.length ? { 'X-Sdkwork-Permission-Scope': context.permissionScope.join(',') } : {}),
    ...(context?.contextSignature ? { 'X-Sdkwork-Context-Signature': context.contextSignature } : {}),
  };
  return Object.keys(headers).length > 0 ? headers : undefined;
}

export function buildSdkworkChatAppContextHeaders(
  session?: SdkworkChatSession | null,
): Record<string, string> | undefined {
  return buildSdkworkChatAppContextHeadersFromContext(createSdkworkChatRequestContext(session));
}

function removeSdkworkChatContextHeaders(headers?: Record<string, string>): Record<string, string> {
  const cleanHeaders: Record<string, string> = {};
  for (const [key, value] of Object.entries(headers ?? {})) {
    if (!SDKWORK_CHAT_CONTEXT_HEADER_NAMES.has(key.toLowerCase())) {
      cleanHeaders[key] = value;
    }
  }
  return cleanHeaders;
}

function resolveRequestContextSession(
  sessionOrReader?: SdkworkChatSession | null | (() => SdkworkChatSession | null),
): SdkworkChatSession | null {
  if (typeof sessionOrReader === 'function') {
    return sessionOrReader();
  }
  return sessionOrReader ?? readAppSdkSessionTokens();
}

export function createSdkworkChatRequestContextInterceptors(
  sessionOrReader?: SdkworkChatSession | null | (() => SdkworkChatSession | null),
): Interceptors {
  return {
    request: [
      (config: RequestConfig): RequestConfig => {
        const requestContext = createSdkworkChatRequestContext(resolveRequestContextSession(sessionOrReader));
        const contextHeaders = buildSdkworkChatAppContextHeadersFromContext(requestContext) ?? {};
        return {
          ...config,
          headers: {
            ...removeSdkworkChatContextHeaders(config.headers),
            ...contextHeaders,
          },
        };
      },
    ],
    response: [],
    error: [],
  };
}

export function createSdkworkChatSessionTokenManager(
  sessionOrReader?: SdkworkChatSession | null | (() => SdkworkChatSession | null),
): AuthTokenManager {
  let currentSession = typeof sessionOrReader === 'function' ? null : sessionOrReader ?? null;
  const readConfiguredSession = () => (
    typeof sessionOrReader === 'function'
      ? sessionOrReader()
      : currentSession
  );
  const readCurrentSession = () => readConfiguredSession() ?? readAppSdkSessionTokens();
  const isExpired = () => {
    const expiresAt = readCurrentSession()?.expiresAt;
    return typeof expiresAt === 'number' && Number.isFinite(expiresAt) && Date.now() >= expiresAt;
  };
  const updateTokens = (tokens: AuthTokens): void => {
    const existing = readCurrentSession() ?? {};
    const expiresAt = typeof tokens.expiresAt === 'number' && Number.isFinite(tokens.expiresAt)
      ? tokens.expiresAt
      : typeof tokens.expiresIn === 'number' && Number.isFinite(tokens.expiresIn)
        ? Date.now() + tokens.expiresIn * 1000
        : existing.expiresAt;
    currentSession = applyAppSdkSessionTokens({
      ...existing,
      accessToken: tokens.accessToken ?? existing.accessToken,
      authToken: tokens.authToken ?? existing.authToken,
      refreshToken: tokens.refreshToken ?? existing.refreshToken,
      ...(expiresAt ? { expiresAt } : {}),
    });
  };
  const patchTokens = (tokens: Partial<SdkworkChatSessionTokens>): void => {
    const existing = readCurrentSession() ?? {};
    const next = {
      ...existing,
      ...tokens,
    };
    currentSession = applyAppSdkSessionTokens(next);
  };

  return {
    getAuthToken: () => resolveAppSdkAuthToken(readCurrentSession()),
    getAccessToken: () => resolveAppSdkAccessToken(readCurrentSession()),
    getRefreshToken: () => resolveAppSdkRefreshToken(readCurrentSession()),
    getTokens: () => {
      const current = readCurrentSession();
      return {
        ...(current?.accessToken ? { accessToken: current.accessToken } : {}),
        ...(current?.authToken ? { authToken: current.authToken } : {}),
        ...(current?.refreshToken ? { refreshToken: current.refreshToken } : {}),
        ...(current?.expiresAt ? { expiresAt: current.expiresAt } : {}),
      };
    },
    setTokens: updateTokens,
    setAccessToken: (token: string) => patchTokens({ accessToken: normalizeToken(token) }),
    setAuthToken: (token: string) => patchTokens({ authToken: normalizeToken(token) }),
    setRefreshToken: (token: string) => patchTokens({ refreshToken: normalizeToken(token) }),
    clearTokens: () => {
      currentSession = null;
      clearAppSdkSessionTokens();
    },
    clearAuthToken: () => patchTokens({ authToken: undefined }),
    clearAccessToken: () => patchTokens({ accessToken: undefined }),
    isExpired,
    isValid: () => Boolean(resolveAppSdkAccessToken(readCurrentSession()) || resolveAppSdkAuthToken(readCurrentSession())) && !isExpired(),
    hasToken: () => Boolean(resolveAppSdkAccessToken(readCurrentSession()) || resolveAppSdkAuthToken(readCurrentSession())),
    hasAuthToken: () => Boolean(resolveAppSdkAuthToken(readCurrentSession())),
    hasAccessToken: () => Boolean(resolveAppSdkAccessToken(readCurrentSession())),
    willExpireIn: (seconds: number) => {
      const expiresAt = readCurrentSession()?.expiresAt;
      return typeof expiresAt === 'number' && Number.isFinite(expiresAt) && Date.now() + seconds * 1000 >= expiresAt;
    },
  };
}

export function syncSdkworkChatGlobalTokenManager(session: SdkworkChatSession | null = readAppSdkSessionTokens()): void {
  sdkworkChatGlobalTokenManagerSession = session ? normalizeSession(session) : null;
}

export function getSdkworkChatGlobalTokenManager(): AuthTokenManager {
  syncSdkworkChatGlobalTokenManager(readAppSdkSessionTokens());
  if (!sdkworkChatGlobalTokenManager) {
    sdkworkChatGlobalTokenManager = createSdkworkChatSessionTokenManager(
      () => sdkworkChatGlobalTokenManagerSession,
    );
  }
  return sdkworkChatGlobalTokenManager;
}

export function isAppSdkSessionAuthenticated(session = readAppSdkSessionTokens()): boolean {
  return Boolean(resolveAppSdkAccessToken(session));
}
