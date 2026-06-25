import { createSdkworkAuthAppbaseIntegration } from '@sdkwork/auth-pc-react';
import {
  clearSdkworkChatIamRuntimeSession,
  getSdkworkChatIamRuntime,
  resetSdkworkChatAuthenticatedSdkClients,
  resetSdkworkChatIamRuntime,
} from './appAuthRuntime';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  isAppSdkSessionAuthenticated,
  normalizeSdkworkChatSessionUser,
  readAppSdkSessionTokens,
  type SdkworkChatSession,
  type SdkworkChatSessionUser,
} from './session';

export interface AppAuthService {
  getCurrentSession(): Promise<SdkworkChatSession | null>;
  logout(): Promise<void>;
}

interface RuntimeSessionPayload {
  accessToken?: string;
  authToken?: string;
  context?: unknown;
  expiresAt?: number | string;
  refreshToken?: string;
  sessionId?: string;
  user?: SdkworkChatSessionUser;
  userInfo?: SdkworkChatSessionUser;
}

type RuntimeSessionContext = NonNullable<SdkworkChatSession['context']>;

const sdkworkChatAuthIntegration = createSdkworkAuthAppbaseIntegration({
  app: {
    id: 'sdkwork-im-pc',
    title: 'Sdkwork IM PC',
  },
  basePath: '/auth',
  extraPackageNames: [
    '@sdkwork/im-pc-react',
  ],
});

export const sdkworkChatAuthAppbaseManifest = sdkworkChatAuthIntegration.manifest;

export const sdkworkChatAuthRoutes = sdkworkChatAuthIntegration.routes;

export const sdkworkChatAuthAppbaseMeta = sdkworkChatAuthIntegration.appbaseMeta;

function normalizeContextString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeContextStringArray(value: unknown): string[] {
  if (Array.isArray(value)) {
    return value
      .map((item) => normalizeContextString(item))
      .filter(Boolean) as string[];
  }

  const normalized = normalizeContextString(value);
  if (!normalized) {
    return [];
  }

  return normalized
    .split(normalized.includes(',') ? /,/u : /\s+/u)
    .map((item) => item.trim())
    .filter(Boolean);
}

function normalizeRuntimeSessionContext(value: unknown): SdkworkChatSession['context'] | undefined {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return undefined;
  }

  const context = value as Record<string, unknown>;
  const appId = normalizeContextString(context.appId ?? context.app_id);
  const tenantId = normalizeContextString(context.tenantId ?? context.tenant_id);
  const userId = normalizeContextString(context.userId ?? context.user_id);
  const sessionId = normalizeContextString(context.sessionId ?? context.session_id);
  if (!appId || !tenantId || !userId || !sessionId) {
    return undefined;
  }

  return {
    appId,
    authLevel: (normalizeContextString(context.authLevel ?? context.auth_level) ?? 'password') as RuntimeSessionContext['authLevel'],
    dataScope: normalizeContextStringArray(context.dataScope ?? context.data_scope),
    deploymentMode: (normalizeContextString(context.deploymentMode ?? context.deployment_mode) ?? 'saas') as RuntimeSessionContext['deploymentMode'],
    environment: (normalizeContextString(context.environment ?? context.env) ?? 'dev') as RuntimeSessionContext['environment'],
    ...(normalizeContextString(context.organizationId ?? context.organization_id)
      ? { organizationId: normalizeContextString(context.organizationId ?? context.organization_id) }
      : {}),
    permissionScope: normalizeContextStringArray(context.permissionScope ?? context.permission_scope),
    sessionId,
    tenantId,
    userId,
  };
}

function toSession(data: RuntimeSessionPayload): SdkworkChatSession {
  const expiresAt = typeof data.expiresAt === 'string' ? Date.parse(data.expiresAt) : data.expiresAt;
  const context = normalizeRuntimeSessionContext(data.context);
  return {
    accessToken: data.accessToken,
    authToken: data.authToken,
    refreshToken: data.refreshToken,
    ...(context ? { context } : {}),
    ...(expiresAt ? { expiresAt } : {}),
    ...(data.sessionId ?? context?.sessionId ? { sessionId: data.sessionId ?? context?.sessionId } : {}),
    ...(normalizeSdkworkChatSessionUser(data.user ?? data.userInfo)
      ? { user: normalizeSdkworkChatSessionUser(data.user ?? data.userInfo) }
      : {}),
  };
}

export const appAuthService: AppAuthService = {
  async getCurrentSession() {
    const storedSession = readAppSdkSessionTokens();
    if (!isAppSdkSessionAuthenticated(storedSession)) {
      clearSdkworkChatIamRuntimeSession();
      return null;
    }

    try {
      const session = await getSdkworkChatIamRuntime().service.auth.sessions.current.retrieve();
      return applyAppSdkSessionTokens(toSession(session as unknown as RuntimeSessionPayload));
    } catch {
      clearSdkworkChatIamRuntimeSession();
      resetSdkworkChatIamRuntime();
      return null;
    }
  },

  async logout() {
    try {
      await getSdkworkChatIamRuntime().service.auth.sessions.current.delete();
    } finally {
      clearAppSdkSessionTokens();
      resetSdkworkChatAuthenticatedSdkClients();
      resetSdkworkChatIamRuntime();
    }
  },
};
