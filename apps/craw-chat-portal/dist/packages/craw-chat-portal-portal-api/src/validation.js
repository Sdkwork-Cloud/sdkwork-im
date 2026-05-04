import {
  assertUserCenterValidationPreflightCompatibility,
  USER_CENTER_VALIDATION_SOURCE_PACKAGE_NAME,
  createUserCenterServerValidationPluginDefinition,
  createUserCenterValidationInteropContract,
  createUserCenterValidationPluginDefinition,
  createUserCenterValidationPreflightReport,
  createUserCenterValidationSnapshot,
  requireUserCenterProtectedToken,
  resolveUserCenterProtectedToken,
} from '../../../../../../sdkwork-appbase/packages/pc-react/identity/sdkwork-user-center-validation-pc-react/src/index.ts';
import {
  createCrawChatPortalUserCenterConfig,
  createCrawChatPortalUserCenterPluginDefinition,
  createCrawChatPortalUserCenterServerPluginDefinition,
} from './userCenter.js';

export const CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_SOURCE_PACKAGE =
  USER_CENTER_VALIDATION_SOURCE_PACKAGE_NAME;
export const CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_PLUGIN_PACKAGES = Object.freeze([
  'craw-chat-portal-validation',
]);

export function createCrawChatPortalUserCenterValidationSnapshot(options = {}) {
  return createUserCenterValidationSnapshot(createCrawChatPortalUserCenterConfig(options));
}

export function createCrawChatPortalUserCenterValidationInteropContract(options = {}) {
  return createUserCenterValidationInteropContract(
    createCrawChatPortalUserCenterValidationSnapshot(options),
  );
}

export function createCrawChatPortalUserCenterValidationPluginDefinition(options = {}) {
  return createUserCenterValidationPluginDefinition({
    ...options,
    packageNames: options.packageNames ?? [...CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_PLUGIN_PACKAGES],
    title: options.title ?? 'Craw Chat Portal',
    userCenterPlugin: createCrawChatPortalUserCenterPluginDefinition(options),
  });
}

export function createCrawChatPortalUserCenterServerValidationPluginDefinition(options = {}) {
  return createUserCenterServerValidationPluginDefinition({
    packageNames: options.packageNames ?? [...CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_PLUGIN_PACKAGES],
    title: options.title ?? 'Craw Chat Portal Server Validation',
    userCenterServerPlugin: createCrawChatPortalUserCenterServerPluginDefinition(options),
  });
}

export function createCrawChatPortalUserCenterValidationPreflightReport(options) {
  const { peerContract, ...configOptions } = options;
  return createUserCenterValidationPreflightReport({
    peerContract,
    snapshot: createCrawChatPortalUserCenterValidationSnapshot(configOptions),
  });
}

export function assertCrawChatPortalUserCenterValidationPreflight(options) {
  const { peerContract, ...configOptions } = options;
  return assertUserCenterValidationPreflightCompatibility({
    peerContract,
    snapshot: createCrawChatPortalUserCenterValidationSnapshot(configOptions),
  });
}

export function resolveCrawChatPortalProtectedToken(options) {
  return resolveUserCenterProtectedToken(options);
}

export function requireCrawChatPortalProtectedToken(options) {
  return requireUserCenterProtectedToken(options);
}
