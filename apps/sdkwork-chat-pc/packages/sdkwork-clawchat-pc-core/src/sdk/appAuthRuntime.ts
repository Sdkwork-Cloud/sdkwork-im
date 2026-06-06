import type {
  SdkworkAuthAppearanceConfig,
  SdkworkAuthRuntimeConfig,
  SdkworkIamRuntimeAuthRuntimeLike,
} from '@sdkwork/auth-pc-react';
import { createAppAuthService } from './appAuthService';
import { getAppSdkClientWithSession, resetAppSdkClient } from './appSdkClient';
import { resetAgentAppSdkClient } from './agentAppSdkClient';
import { resetImSdkClient } from './imSdkClient';
import {
  applyAppSdkSessionTokens,
  clearAppSdkSessionTokens,
  readAppSdkSessionTokens,
  type SdkworkChatSession,
  type SdkworkChatSessionUser,
} from './session';

const AUTH_METHOD_UNAVAILABLE_MESSAGE =
  'This SDKWork Chat IAM runtime auth method is not available in the current integration.';

const SDKWORK_CHAT_VERIFICATION_POLICY = {
  emailCodeLoginEnabled: false,
  emailRegistrationVerificationRequired: false,
  phoneCodeLoginEnabled: false,
  phoneRegistrationVerificationRequired: false,
} as const;

let sdkworkChatIamRuntime: SdkworkIamRuntimeAuthRuntimeLike | null = null;

function readEnvValue(...keys: string[]): string | undefined {
  const meta = import.meta as ImportMeta & {
    env?: Record<string, string | boolean | undefined>;
  };

  for (const key of keys) {
    const value = meta.env?.[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }

  return undefined;
}

function parseBoolean(value: string | undefined): boolean | undefined {
  const normalized = (value ?? '').trim().toLowerCase();
  if (!normalized) {
    return undefined;
  }

  if (['1', 'on', 'true', 'yes'].includes(normalized)) {
    return true;
  }

  if (['0', 'false', 'no', 'off'].includes(normalized)) {
    return false;
  }

  return undefined;
}

function toRuntimeUser(user?: SdkworkChatSessionUser) {
  if (!user) {
    return undefined;
  }

  return {
    avatar: user.avatar,
    displayName: user.displayName ?? user.name ?? user.nickname,
    email: user.email,
    id: typeof user.id === 'number' ? String(user.id) : user.id,
    name: user.name,
    nickname: user.nickname,
    phone: user.phone,
    userId: user.userId,
    username: user.username,
  };
}

function toRuntimeSession(session: SdkworkChatSession) {
  const accessToken = session.accessToken ?? session.authToken;
  const authToken = session.authToken ?? session.accessToken;

  if (!accessToken || !authToken) {
    throw new Error('SDKWork Chat IAM session requires accessToken or authToken.');
  }

  const user = toRuntimeUser(session.user);
  return {
    accessToken,
    authToken,
    context: session.context,
    expiresAt: session.expiresAt ? new Date(session.expiresAt).toISOString() : undefined,
    refreshToken: session.refreshToken,
    sessionId: session.sessionId ?? session.context?.sessionId,
    user,
    userInfo: user,
  };
}

function readStoredRuntimeSession() {
  const session = readAppSdkSessionTokens();
  if (!session) {
    return {};
  }

  return {
    accessToken: session.accessToken ?? session.authToken,
    authToken: session.authToken ?? session.accessToken,
    refreshToken: session.refreshToken,
  };
}

function persistRuntimeSession(session: {
  accessToken?: string;
  authToken?: string;
  context?: SdkworkChatSession['context'];
  refreshToken?: string;
  sessionId?: string;
  user?: SdkworkChatSessionUser;
  userInfo?: SdkworkChatSessionUser;
}): void {
  const existingSession = readAppSdkSessionTokens();
  applyAppSdkSessionTokens({
    accessToken: session.accessToken ?? session.authToken,
    authToken: session.authToken ?? session.accessToken,
    context: session.context ?? existingSession?.context,
    refreshToken: session.refreshToken ?? existingSession?.refreshToken,
    sessionId: session.sessionId ?? existingSession?.sessionId,
    user: session.user ?? session.userInfo ?? existingSession?.user,
  });
  resetAppSdkClient();
  resetAgentAppSdkClient();
  resetImSdkClient();
}

function clearRuntimeSession() {
  clearAppSdkSessionTokens();
  resetAppSdkClient();
  resetAgentAppSdkClient();
  resetImSdkClient();
}

async function unavailable(): Promise<never> {
  throw new Error(AUTH_METHOD_UNAVAILABLE_MESSAGE);
}

function createSdkworkChatIamRuntime(): SdkworkIamRuntimeAuthRuntimeLike {
  const service = createAppAuthService(() => getAppSdkClientWithSession(readAppSdkSessionTokens()));
  return {
    contextStore: {
      clear: clearRuntimeSession,
    },
    service: {
      auth: {
        oauthAuthorizationUrls: {
          retrieve: unavailable,
        },
        oauthSessions: {
          create: unavailable,
        },
        passwordResetRequests: {
          create: unavailable,
        },
        passwordResets: {
          create: unavailable,
        },
        sessions: {
          create: async (payload) => toRuntimeSession(await service.login({
            password: String(payload.password ?? ''),
            remember: Boolean(payload.remember),
            username: String(payload.username ?? '').trim(),
          })),
          current: {
            delete: () => service.logout(),
            retrieve: async () => {
              const session = await service.getCurrentSession();
              if (!session) {
                throw new Error('SDKWork Chat IAM session is not authenticated.');
              }
              return toRuntimeSession(session);
            },
            update: async () => {
              const session = await service.getCurrentSession();
              if (!session) {
                throw new Error('SDKWork Chat IAM session is not authenticated.');
              }
              return toRuntimeSession(session);
            },
          },
          refresh: async (payload) => toRuntimeSession(await service.refreshToken(
            typeof payload.refreshToken === 'string' ? payload.refreshToken : undefined,
          )),
        },
        registrations: {
          create: async (payload) => toRuntimeSession(await service.register({
            confirmPassword: typeof payload.confirmPassword === 'string' ? payload.confirmPassword : undefined,
            email: typeof payload.email === 'string' ? payload.email : undefined,
            name: typeof payload.name === 'string' ? payload.name : undefined,
            password: String(payload.password ?? ''),
            phone: typeof payload.phone === 'string' ? payload.phone : undefined,
            username: String(payload.username ?? payload.email ?? payload.phone ?? '').trim(),
            verificationCode: typeof payload.verificationCode === 'string' ? payload.verificationCode : undefined,
          })),
        },
        verificationCodes: {
          create: (payload) => service.sendVerifyCode({
            scene: String(payload.scene ?? 'REGISTER'),
            target: String(payload.target ?? ''),
            verifyType: String(payload.verifyType ?? 'EMAIL'),
          }),
          verify: async (payload) => {
            const verified = await service.verifyCode({
              code: String(payload.code ?? ''),
              scene: String(payload.scene ?? 'REGISTER'),
              target: String(payload.target ?? ''),
              verifyType: String(payload.verifyType ?? 'EMAIL'),
            });
            return {
              valid: verified,
              verified,
            };
          },
        },
      },
      iam: {
        users: {
          current: {
            retrieve: async () => {
              const session = await service.getCurrentSession();
              return toRuntimeUser(session?.user) ?? {};
            },
          },
        },
      },
      openPlatform: {
        qrAuth: {
          sessions: {
            create: (payload) => service.createQrAuthSession({
              purpose: payload?.purpose === 'register' ? 'register' : 'login',
            }),
            retrieve: (sessionKey) => service.retrieveQrAuthSession(sessionKey),
            scans: {
              create: (sessionKey, payload = {}) => service.createQrAuthScan(sessionKey, payload),
            },
            passwords: {
              create: (sessionKey, payload) => service.createQrAuthPassword(sessionKey, {
                confirmPassword: typeof payload.confirmPassword === 'string' ? payload.confirmPassword : undefined,
                email: typeof payload.email === 'string' ? payload.email : undefined,
                password: String(payload.password ?? ''),
                phone: typeof payload.phone === 'string' ? payload.phone : undefined,
                username: String(payload.username ?? payload.email ?? payload.phone ?? '').trim(),
                verificationCode: typeof payload.verificationCode === 'string' ? payload.verificationCode : undefined,
              }),
            },
          },
        },
      },
      system: {
        iam: {
          verificationPolicy: {
            retrieve: async () => SDKWORK_CHAT_VERIFICATION_POLICY,
          },
        },
      },
    },
    tokenStore: {
      clear: clearRuntimeSession,
      get: readStoredRuntimeSession,
      set: persistRuntimeSession,
    },
  };
}

function resolveDevelopmentPrefill(): SdkworkAuthRuntimeConfig['developmentPrefill'] {
  const account = readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_DEFAULT_ACCOUNT',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_ACCOUNT',
  );
  const email = readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_DEFAULT_EMAIL',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_EMAIL',
  );
  const phone = readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_DEFAULT_PHONE',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PHONE',
  );
  const password = readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_DEFAULT_PASSWORD',
    'VITE_SDKWORK_AUTH_DEV_DEFAULT_PASSWORD',
  );
  const verificationCode = readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_VERIFICATION_CODE',
    'VITE_SDKWORK_AUTH_DEV_VERIFICATION_CODE',
  );
  const verificationCodeBypassEnabled = parseBoolean(readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_VERIFICATION_CODE_ENABLED',
    'VITE_SDKWORK_AUTH_DEV_VERIFICATION_CODE_ENABLED',
  ));
  const enabled = parseBoolean(readEnvValue(
    'VITE_SDKWORK_CHAT_AUTH_DEV_PREFILL_ENABLED',
    'VITE_SDKWORK_AUTH_DEV_PREFILL_ENABLED',
  ));
  const shouldEnable = enabled ?? Boolean(account || email || phone || password || verificationCode);

  if (!shouldEnable) {
    return undefined;
  }

  return {
    account: account || email || phone,
    email,
    enabled: true,
    loginMethod: 'password',
    password,
    phone,
    verificationCode,
    ...(typeof verificationCodeBypassEnabled === 'boolean'
      ? { verificationCodeBypassEnabled }
      : {}),
  };
}

export function getSdkworkChatIamRuntime(): SdkworkIamRuntimeAuthRuntimeLike {
  if (!sdkworkChatIamRuntime) {
    sdkworkChatIamRuntime = createSdkworkChatIamRuntime();
  }

  return sdkworkChatIamRuntime;
}

export function resetSdkworkChatIamRuntime(): void {
  sdkworkChatIamRuntime = null;
}

export function resolveSdkworkChatAuthRuntimeConfig(): SdkworkAuthRuntimeConfig {
  const developmentPrefill = resolveDevelopmentPrefill();
  return {
    leftRailMode: 'qr-only',
    loginMethods: ['password'],
    oauthLoginEnabled: false,
    oauthProviders: [],
    qrLoginEnabled: true,
    recoveryMethods: [],
    registerMethods: ['email', 'phone'],
    verificationPolicy: {
      emailCodeLoginEnabled: false,
      emailRegistrationVerificationRequired: false,
      phoneCodeLoginEnabled: false,
      phoneRegistrationVerificationRequired: false,
    },
    ...(developmentPrefill ? { developmentPrefill } : {}),
  };
}

export function resolveSdkworkChatAuthAppearance(): SdkworkAuthAppearanceConfig {
  return {
    asidePanelClassName: 'sdkwork-chat-auth-aside-panel',
    bodyClassName: 'sdkwork-chat-auth-body',
    contentContainerClassName: 'sdkwork-chat-auth-content',
    pageClassName: 'sdkwork-chat-auth-page',
    qrFrameClassName: 'sdkwork-chat-auth-qr-frame',
    shellClassName: 'sdkwork-chat-auth-card-shell',
    slotProps: {
      background: {
        className: 'sdkwork-chat-auth-background',
      },
      page: {
        className: 'sdkwork-chat-auth-page',
      },
      shell: {
        className: 'sdkwork-chat-auth-card-shell',
      },
    },
    theme: {
      asideCardBackgroundColor: 'var(--sdkwork-chat-auth-aside-card-bg)',
      asideCardBorderColor: 'var(--sdkwork-chat-auth-aside-card-border)',
      asidePanelBackgroundColor: 'var(--sdkwork-chat-auth-aside-bg)',
      asidePanelBorderColor: 'var(--sdkwork-chat-auth-aside-border)',
      asidePanelColor: 'var(--sdkwork-chat-auth-aside-text)',
      badgeBackgroundColor: 'var(--sdkwork-chat-auth-aside-badge-bg)',
      badgeTextColor: 'var(--sdkwork-chat-auth-aside-badge-text)',
      contentBackgroundColor: 'var(--sdkwork-chat-auth-content-bg)',
      contentBorderColor: 'var(--sdkwork-chat-auth-content-border)',
      contentTextColor: 'var(--sdkwork-chat-auth-content-text)',
      descriptionColor: 'var(--sdkwork-chat-auth-muted-text)',
      dividerColor: 'var(--sdkwork-chat-auth-divider)',
      fieldBackgroundColor: 'var(--sdkwork-chat-auth-field-bg)',
      fieldBorderColor: 'var(--sdkwork-chat-auth-field-border)',
      fieldPlaceholderColor: '#9ca3af',
      fieldTextColor: 'var(--sdkwork-chat-auth-content-text)',
      formMutedTextColor: 'var(--sdkwork-chat-auth-muted-text)',
      iconMutedColor: 'var(--sdkwork-chat-auth-muted-text)',
      labelColor: 'var(--sdkwork-chat-auth-content-text)',
      pageBackgroundColor: 'var(--sdkwork-chat-auth-bg)',
      qrFrameBackgroundColor: 'var(--sdkwork-chat-auth-qr-bg)',
      qrFrameBorderColor: 'var(--sdkwork-chat-auth-qr-border)',
      shellBackdropFilter: 'blur(16px)',
      shellBackgroundColor: 'var(--sdkwork-chat-auth-content-bg)',
      shellBorderColor: 'var(--sdkwork-chat-auth-content-border)',
      tabActiveBackgroundColor: 'var(--sdkwork-chat-auth-tab-active-bg)',
      tabActiveTextColor: 'var(--sdkwork-chat-auth-content-text)',
      tabBackgroundColor: 'var(--sdkwork-chat-auth-tab-bg)',
      tabInactiveTextColor: 'var(--sdkwork-chat-auth-muted-text)',
      titleColor: 'var(--sdkwork-chat-auth-content-text)',
    },
  };
}
