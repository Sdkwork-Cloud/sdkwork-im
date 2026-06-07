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
  context?: SdkworkChatSession['context'];
  expiresAt?: number | string;
  refreshToken?: string;
  sessionId?: string;
  user?: SdkworkChatSessionUser;
  userInfo?: SdkworkChatSessionUser;
}

const sdkworkChatAuthIntegration = createSdkworkAuthAppbaseIntegration({
  app: {
    id: 'sdkwork-chat-pc',
    title: 'SDKWork Chat PC',
  },
  basePath: '/auth',
  extraPackageNames: [
    '@sdkwork/im-pc-react',
  ],
});

export const sdkworkChatAuthAppbaseManifest = sdkworkChatAuthIntegration.manifest;

export const sdkworkChatAuthRoutes = sdkworkChatAuthIntegration.routes;

export const sdkworkChatAuthAppbaseMeta = sdkworkChatAuthIntegration.appbaseMeta;

function toSession(data: RuntimeSessionPayload): SdkworkChatSession {
  const expiresAt = typeof data.expiresAt === 'string' ? Date.parse(data.expiresAt) : data.expiresAt;
  const accessToken = data.accessToken ?? data.authToken;
  const authToken = data.authToken ?? data.accessToken;
  return {
    accessToken,
    authToken,
    refreshToken: data.refreshToken,
    ...(data.context ? { context: data.context } : {}),
    ...(expiresAt ? { expiresAt } : {}),
    ...(data.sessionId ?? data.context?.sessionId ? { sessionId: data.sessionId ?? data.context.sessionId } : {}),
    ...(normalizeSdkworkChatSessionUser(data.user ?? data.userInfo)
      ? { user: normalizeSdkworkChatSessionUser(data.user ?? data.userInfo) }
      : {}),
  };
}

export const appAuthService: AppAuthService = {
  async getCurrentSession() {
    if (!readAppSdkSessionTokens()) {
      return null;
    }

    try {
      const session = await getSdkworkChatIamRuntime().service.auth.sessions.current.retrieve();
      return applyAppSdkSessionTokens(toSession(session as RuntimeSessionPayload));
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
