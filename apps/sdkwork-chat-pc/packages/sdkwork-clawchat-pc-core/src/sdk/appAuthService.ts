import {
  appbasePackageMeta,
  createSdkworkAppCapabilityPresetManifest,
} from '@sdkwork/appbase-pc-react';
import {
  authPackageMeta,
  createAuthRouteCatalog,
  resolveAuthStatus,
  type SdkworkAuthSessionLike,
} from '@sdkwork/auth-pc-react/auth';
import {
  createIamAppSdkAdapter,
  unwrapIamSdkResponse,
} from '@sdkwork/iam-sdk-adapter';
import type {
  AuthSession,
  QrAuthSession,
} from '@sdkwork-internal/im-app-api-generated';
import {
  getAppSdkClientWithSession,
  resetAppSdkClient,
  type SdkworkImAppClient,
} from './appSdkClient';
import { resetAgentAppSdkClient } from './agentAppSdkClient';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  normalizeSdkworkChatSessionUser,
  readAppSdkSessionTokens,
  type SdkworkChatSession,
} from './session';
import { resetImSdkClient } from './imSdkClient';

export interface AppAuthLoginInput {
  password: string;
  remember?: boolean;
  username: string;
}

export interface AppAuthRegisterInput {
  confirmPassword?: string;
  email?: string;
  name?: string;
  password: string;
  phone?: string;
  username: string;
  verificationCode?: string;
}

export interface AppAuthVerifyCodeInput {
  code: string;
  scene: string;
  target: string;
  verifyType: string;
}

export interface AppAuthSendVerifyCodeInput {
  scene: string;
  target: string;
  verifyType: string;
}

export interface AppAuthQrSessionCreateInput {
  purpose?: 'login' | 'register';
}

export interface AppAuthQrScanInput {
  accountId?: string;
  entryId?: string;
  externalUserId?: string;
  ipHash?: string;
  scanSource?: string;
  userAgent?: string;
}

export interface AppAuthQrPasswordInput extends AppAuthRegisterInput {
  channel?: string;
}

export interface AppAuthService {
  createQrAuthPassword(sessionKey: string, input: AppAuthQrPasswordInput): Promise<QrAuthSession>;
  createQrAuthScan(sessionKey: string, input?: AppAuthQrScanInput): Promise<QrAuthSession>;
  createQrAuthSession(input?: AppAuthQrSessionCreateInput): Promise<QrAuthSession>;
  getCurrentSession(): Promise<SdkworkChatSession | null>;
  login(input: AppAuthLoginInput): Promise<SdkworkChatSession>;
  logout(): Promise<void>;
  refreshToken(refreshToken?: string): Promise<SdkworkChatSession>;
  register(input: AppAuthRegisterInput): Promise<SdkworkChatSession>;
  retrieveQrAuthSession(sessionKey: string): Promise<QrAuthSession>;
  sendVerifyCode(input: AppAuthSendVerifyCodeInput): Promise<void>;
  verifyCode(input: AppAuthVerifyCodeInput): Promise<boolean>;
}

export const sdkworkChatAuthAppbaseManifest = createSdkworkAppCapabilityPresetManifest(
  'collaboration-desktop',
  {
    id: 'sdkwork-chat-pc',
    title: 'SDKWork Chat PC',
    extraPackageNames: [
      '@sdkwork/auth-pc-react',
      '@sdkwork/im-pc-react',
    ],
  },
);

export const sdkworkChatAuthRoutes = createAuthRouteCatalog('/auth');

function toSession(data: AuthSession): SdkworkChatSession {
  return {
    accessToken: data.accessToken,
    authToken: data.authToken,
    refreshToken: data.refreshToken,
    ...(data.context ? { context: data.context } : {}),
    ...(data.expiresAt ? { expiresAt: Date.parse(data.expiresAt) } : {}),
    ...(data.sessionId ?? data.context?.sessionId ? { sessionId: data.sessionId ?? data.context.sessionId } : {}),
    ...(normalizeSdkworkChatSessionUser(data.user ?? data.userInfo)
      ? { user: normalizeSdkworkChatSessionUser(data.user ?? data.userInfo) }
      : {}),
  };
}

function persistAuthSession(result: unknown): SdkworkChatSession {
  const session = toSession(result as AuthSession);
  applyAppSdkSessionTokens(session);
  resetAppSdkClient();
  resetAgentAppSdkClient();
  resetImSdkClient();
  return session;
}

function createClient(): SdkworkImAppClient {
  return getAppSdkClientWithSession(readAppSdkSessionTokens());
}

export function createAppAuthService(getClient: () => SdkworkImAppClient = createClient): AppAuthService {
  const getIam = () => createIamAppSdkAdapter(getClient());

  return {
    async getCurrentSession() {
      const session = readAppSdkSessionTokens();
      const authSession: SdkworkAuthSessionLike | null = session
        ? {
            accessToken: session.accessToken,
            authToken: session.authToken,
            refreshToken: session.refreshToken,
          }
        : null;
      if (resolveAuthStatus(authSession) !== 'authenticated') {
        return null;
      }

      try {
        return persistAuthSession(await getIam().auth.sessions.current.retrieve());
      } catch {
        return session;
      }
    },

    async login(input) {
      return persistAuthSession(await getIam().auth.sessions.create({
        password: input.password,
        remember: input.remember,
        username: input.username,
      }));
    },

    async register(input) {
      return persistAuthSession(await getIam().auth.registrations.create({
        confirmPassword: input.confirmPassword ?? input.password,
        email: input.email,
        name: input.name,
        password: input.password,
        phone: input.phone,
        type: input.phone ? 'PHONE' : input.email ? 'EMAIL' : 'DEFAULT',
        username: input.username,
        verificationCode: input.verificationCode,
      }));
    },

    async logout() {
      try {
        await getIam().auth.sessions.current.delete();
      } finally {
        clearAppSdkSessionTokens();
        resetAppSdkClient();
        resetAgentAppSdkClient();
        resetImSdkClient();
      }
    },

    async refreshToken(refreshToken = readAppSdkSessionTokens()?.refreshToken) {
      if (!refreshToken) {
        throw new Error('refreshToken is required.');
      }
      return persistAuthSession(await getIam().auth.sessions.refresh({ refreshToken }));
    },

    async sendVerifyCode(input) {
      await getIam().auth.verificationCodes.create({
        scene: input.scene,
        target: input.target,
        verifyType: input.verifyType,
      });
    },

    async verifyCode(input) {
      const result = await getIam().auth.verificationCodes.verify({
        code: input.code,
        scene: input.scene,
        target: input.target,
        verifyType: input.verifyType,
      }) as { valid?: boolean; verified?: boolean };
      return Boolean(result.verified ?? result.valid);
    },

    async createQrAuthSession(input = {}) {
      return unwrapIamSdkResponse<QrAuthSession>(await getIam().openPlatform.qrAuth.sessions.create({
        purpose: input.purpose ?? 'login',
      }));
    },

    async retrieveQrAuthSession(sessionKey) {
      const result = unwrapIamSdkResponse<QrAuthSession>(
        await getIam().openPlatform.qrAuth.sessions.retrieve(sessionKey),
      );
      if (result.session) {
        persistAuthSession(result.session);
      }
      return result;
    },

    async createQrAuthScan(sessionKey, input = {}) {
      return unwrapIamSdkResponse<QrAuthSession>(
        await getIam().openPlatform.qrAuth.sessions.scans.create(sessionKey, input),
      );
    },

    async createQrAuthPassword(sessionKey, input) {
      const result = unwrapIamSdkResponse<QrAuthSession>(
        await getIam().openPlatform.qrAuth.sessions.passwords.create(sessionKey, {
          channel: input.channel,
          confirmPassword: input.confirmPassword ?? input.password,
          email: input.email,
          name: input.name,
          password: input.password,
          phone: input.phone,
          username: input.username,
          verificationCode: input.verificationCode,
        }),
      );
      if (result.session) {
        persistAuthSession(result.session);
      }
      return result;
    },
  };
}

export const appAuthService = createAppAuthService();
export const sdkworkChatAuthAppbaseMeta = {
  appbasePackageMeta,
  authPackageMeta,
  manifest: sdkworkChatAuthAppbaseManifest,
};
