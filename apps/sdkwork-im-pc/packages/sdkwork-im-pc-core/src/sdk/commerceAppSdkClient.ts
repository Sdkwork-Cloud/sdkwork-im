import {
  createCommerceAppSdkClient,
  type CommerceAppSdkClient as GeneratedCommerceAppSdkClient,
  type SdkworkAppConfig,
} from '@sdkwork/commerce-app-sdk';
import type { Interceptors } from '@sdkwork/sdk-common';
import { resolveAppSdkBaseUrl } from './appSdkClient';
import {
  createSdkworkChatRequestContextInterceptors,
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';

export type CommerceAppSdkClient = GeneratedCommerceAppSdkClient;
export type CommerceAppSdkClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let commerceAppSdkClient: CommerceAppSdkClient | null = null;

export function createCommerceAppSdkClientConfig(
  session?: SdkworkChatSession | null,
): CommerceAppSdkClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenManager: getSdkworkChatGlobalTokenManager(),
  };
}

export function initCommerceAppSdkClient(
  config: CommerceAppSdkClientConfig = createCommerceAppSdkClientConfig(),
): CommerceAppSdkClient {
  commerceAppSdkClient = createCommerceAppSdkClient(config);
  return commerceAppSdkClient;
}

export function getCommerceAppSdkClient(): CommerceAppSdkClient {
  return commerceAppSdkClient ?? initCommerceAppSdkClient();
}

export function getCommerceAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): CommerceAppSdkClient {
  return initCommerceAppSdkClient(createCommerceAppSdkClientConfig(session));
}

export function resetCommerceAppSdkClient(): void {
  commerceAppSdkClient = null;
}
