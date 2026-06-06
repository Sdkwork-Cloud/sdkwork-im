#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

const authGateSource = readText('src', 'AuthGate.tsx');
const appSource = readText('src', 'App.tsx');
const authStylesSource = readText('src', 'index.css');
const appAuthServiceSource = readText('packages', 'sdkwork-clawchat-pc-core', 'src', 'sdk', 'appAuthService.ts');
const authRuntimeSource = readText('packages', 'sdkwork-clawchat-pc-core', 'src', 'sdk', 'appAuthRuntime.ts');
const chatLayoutSource = readText('packages', 'sdkwork-clawchat-pc-chat', 'src', 'pages', 'ChatLayout.tsx');
const sidebarSource = readText('packages', 'sdkwork-clawchat-pc-chat', 'src', 'components', 'Sidebar.tsx');
const profileMenuSource = readText('packages', 'sdkwork-clawchat-pc-chat', 'src', 'components', 'ProfileMenuModal.tsx');
const settingsModalSource = readText('packages', 'sdkwork-clawchat-pc-chat', 'src', 'components', 'SettingsModal.tsx');
const chatWindowControlsSource = readText('packages', 'sdkwork-clawchat-pc-chat', 'src', 'components', 'WindowControls.tsx');
const desktopRustSource = readText('packages', 'sdkwork-clawchat-pc-desktop', 'src-tauri', 'src', 'lib.rs');
const tauriWindowControlPermissionSource = readText('packages', 'sdkwork-clawchat-pc-desktop', 'src-tauri', 'permissions', 'window-control.toml');
const tauriConfig = readJson('packages', 'sdkwork-clawchat-pc-desktop', 'src-tauri', 'tauri.conf.json');
const tauriDefaultCapability = readJson('packages', 'sdkwork-clawchat-pc-desktop', 'src-tauri', 'capabilities', 'default.json');
const viteConfigSource = readText('vite.config.ts');
const tsconfig = readJson('tsconfig.json');
const packageJson = readJson('package.json');

assert.match(
  authGateSource,
  /import\s+\{\s*SdkworkIamAuthRoutes[\s\S]*\}\s+from\s+['"]@sdkwork\/auth-pc-react['"]/u,
  'AuthGate must render the SDKWork Appbase PC React IAM auth routes.',
);

assert.match(
  authGateSource,
  /<SdkworkIamAuthRoutes[\s\S]*basePath=["']\/auth["'][\s\S]*homePath=["']\/["']/u,
  'AuthGate must mount the appbase auth routes at /auth and return to the current chat PC home path.',
);

assert.match(
  authGateSource,
  /getRuntime=\{getSdkworkChatIamRuntime\}/u,
  'AuthGate must inject the sdkwork-chat generated-SDK backed IAM runtime.',
);

assert.match(
  authGateSource,
  /runtimeConfig=\{resolveSdkworkChatAuthRuntimeConfig\(\)\}/u,
  'AuthGate must pass the sdkwork-chat auth runtime config into the appbase auth UI.',
);

assert.match(
  authGateSource,
  /appearance=\{resolveSdkworkChatAuthAppearance\(\)\}/u,
  'AuthGate must pass a product-owned appearance override so login and verification backgrounds are controlled by the current app.',
);

assert.match(
  authGateSource,
  /viewportMode=["']flow["']/u,
  'AuthGate must render appbase auth routes in flow mode inside the desktop login shell with an app header.',
);

for (const marker of [
  'SdkworkChatAuthShell',
  'SDKWork Chat',
  'MessageSquare',
  'Sun',
  'Moon',
  'isLightMode',
  'toggleAuthTheme',
  'data-tauri-drag-region',
  'drag-region',
  'no-drag',
  'data-no-drag',
  'WindowControlMinimizeIcon',
  'WindowControlMaximizeIcon',
  'WindowControlRestoreIcon',
  'X',
  'handleWindowControl',
  'invokeDesktopWindowControl',
  'sdkwork_chat_pc_window_control',
  'startDragging',
]) {
  assert.match(
    authGateSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `AuthGate must include login app header/window control marker ${marker}`,
  );
}

assert.doesNotMatch(
  authGateSource,
  /\/auth\/qr-login/u,
  'QR login must stay inside the standard /auth/login page instead of introducing a separate default QR login route.',
);

assert.doesNotMatch(
  authGateSource,
  /<form\b|appAuthService\.login|appAuthService\.register|appAuthService\.sendVerifyCode/u,
  'AuthGate must not keep a bespoke login/register form or direct auth service calls once appbase auth UI is mounted.',
);

assert.doesNotMatch(
  authGateSource,
  /appWindow\.close\?\(/u,
  'AuthGate close button must not directly close the Tauri window; desktop close must hide to tray through the native command.',
);

for (const marker of [
  'invokeDesktopWindowControl',
  'sdkwork_chat_pc_window_control',
  'closeToTray',
  'minimize',
  'toggleMaximize',
]) {
  assert.match(
    chatWindowControlsSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `Main chat AppHeader window controls must use native desktop command marker ${marker}`,
  );
}

assert.match(
  chatLayoutSource,
  /import\s+\{[\s\S]*isSdkworkChatDesktopRuntime[\s\S]*\}\s+from\s+['"]@sdkwork\/clawchat-pc-core['"]/u,
  'ChatLayout must import the shared desktop-runtime guard instead of deciding web/desktop locally.',
);

assert.match(
  chatLayoutSource,
  /const\s+shouldRenderDesktopAppHeader\s*=\s*isSdkworkChatDesktopRuntime\(\)/u,
  'ChatLayout must compute AppHeader visibility from the shared desktop-runtime guard.',
);

assert.match(
  chatLayoutSource,
  /\{shouldRenderDesktopAppHeader\s*&&\s*\([\s\S]*?<WindowControls\s*\/>[\s\S]*?\)\}/u,
  'ChatLayout must render the native AppHeader only in the desktop runtime.',
);

assert.match(
  authGateSource,
  /import\s+\{[\s\S]*isSdkworkChatDesktopRuntime[\s\S]*\}\s+from\s+['"]@sdkwork\/clawchat-pc-core['"]/u,
  'AuthGate must import the shared desktop-runtime guard so web login does not show the desktop AppHeader.',
);

assert.match(
  authGateSource,
  /const\s+shouldRenderDesktopAppHeader\s*=\s*isSdkworkChatDesktopRuntime\(\)/u,
  'AuthGate must compute auth AppHeader visibility from the shared desktop-runtime guard.',
);

assert.match(
  authGateSource,
  /\{shouldRenderDesktopAppHeader\s*&&\s*\([\s\S]*?<header[\s\S]*?sdkwork-chat-auth-header[\s\S]*?<\/header>[\s\S]*?\)\}/u,
  'AuthGate must render the auth AppHeader only in the desktop runtime.',
);

assert.doesNotMatch(
  chatWindowControlsSource,
  /window\.close\(|requestFullscreen\(|exitFullscreen\(/u,
  'Main chat AppHeader must not use browser close/fullscreen APIs for desktop window controls; it must use the Tauri native command.',
);

assert.match(
  authRuntimeSource,
  /export function getSdkworkChatIamRuntime\(\)/u,
  'pc-core must expose a reusable SDKWork Chat IAM runtime for appbase auth routes.',
);

assert.match(
  readText('packages', 'sdkwork-clawchat-pc-core', 'src', 'index.ts'),
  /export \* from ['"]\.\/runtime\/desktopEnvironment['"]/u,
  'pc-core must export the shared desktop runtime guard for auth and chat shells.',
);

const desktopEnvironmentSource = readText(
  'packages',
  'sdkwork-clawchat-pc-core',
  'src',
  'runtime',
  'desktopEnvironment.ts',
);

assert.match(
  desktopEnvironmentSource,
  /export function isSdkworkChatDesktopRuntime\(\): boolean/u,
  'pc-core must define a shared isSdkworkChatDesktopRuntime guard.',
);

assert.match(
  desktopEnvironmentSource,
  /__TAURI__[\s\S]*core[\s\S]*invoke/u,
  'The desktop runtime guard must detect the Tauri invoke bridge used by SDKWork Chat window controls.',
);

assert.match(
  desktopEnvironmentSource,
  /window[\s\S]*getCurrentWindow|window[\s\S]*appWindow/u,
  'The desktop runtime guard must also recognize the Tauri window bridge.',
);

assert.match(
  authRuntimeSource,
  /verificationPolicy[\s\S]*emailCodeLoginEnabled:\s*false[\s\S]*emailRegistrationVerificationRequired:\s*false[\s\S]*phoneCodeLoginEnabled:\s*false[\s\S]*phoneRegistrationVerificationRequired:\s*false/u,
  'SDKWork Chat IAM runtime must disable email/phone code login and registration verification codes by default.',
);

assert.match(
  authRuntimeSource,
  /loginMethods:\s*\[\s*['"]password['"]\s*\]/u,
  'SDKWork Chat IAM runtime must default to password login only.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /loginMethods:\s*\[[^\]]*(?:email-code|phone-code)/u,
  'SDKWork Chat IAM runtime must not enable verification-code login by default.',
);

assert.match(
  authRuntimeSource,
  /leftRailMode:\s*['"]qr-only['"][\s\S]*qrLoginEnabled:\s*true/u,
  'SDKWork Chat IAM runtime must render the appbase QR scan panel inside the normal login page rail.',
);

assert.match(
  authRuntimeSource,
  /recoveryMethods:\s*\[\s*\]/u,
  'SDKWork Chat IAM runtime must hide forgot-password email/phone verification-code recovery by default.',
);

assert.match(
  authRuntimeSource,
  /auth:[\s\S]*sessions:[\s\S]*create[\s\S]*registrations:[\s\S]*create[\s\S]*verificationCodes:[\s\S]*create[\s\S]*verify/u,
  'SDKWork Chat IAM runtime must adapt sessions, registrations, and verificationCodes to the generated app SDK backed service.',
);

assert.match(
  authRuntimeSource,
  /openPlatform:[\s\S]*qrAuth:[\s\S]*sessions:[\s\S]*create[\s\S]*retrieve[\s\S]*scans:[\s\S]*create[\s\S]*passwords:[\s\S]*create/u,
  'SDKWork Chat IAM runtime must expose appbase-standard openPlatform.qrAuth.sessions methods.',
);

assert.match(
  authRuntimeSource,
  /createAppAuthService\(\s*\(\)\s*=>\s*getAppSdkClientWithSession/u,
  'SDKWork Chat IAM runtime must inject the current sdkwork-im-app-sdk client into the auth service boundary.',
);

assert.match(
  appAuthServiceSource,
  /createIamAppSdkAdapter\(\s*getClient\(\)\s*\)/u,
  'SDKWork Chat auth service must pass the generated app SDK directly to the shared appbase IAM adapter.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /function\s+createBoundIamAppSdkClient\(/u,
  'SDKWork Chat auth service must not keep app-local generated SDK binding glue; appbase IAM adapter owns generated SDK method binding.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /resolveQrAuthSessionKey/u,
  'SDKWork Chat auth service must not keep app-local QR sessionKey path normalization; appbase IAM adapter owns QR path parameter normalization.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /createIamAppSdkAdapter\(\s*createBoundIamAppSdkClient/u,
  'SDKWork Chat auth service must not wrap the generated SDK before appbase IAM adapter.',
);

assert.match(
  appAuthServiceSource,
  /createQrAuthPassword\([^)]*\):\s*Promise<\s*QrAuthSession\s*>/u,
  'QR password completion must return the canonical QrAuthSession envelope instead of an app-local session-only DTO.',
);

assert.doesNotMatch(
  appAuthServiceSource,
  /resolveQrAuthPasswordSession/u,
  'SDKWork Chat auth service must not keep app-local QR password response shape fallback; the OpenAPI contract is QrAuthSession.',
);

assert.match(
  appAuthServiceSource,
  /unwrapIamSdkResponse<\s*QrAuthSession\s*>[\s\S]*openPlatform\.qrAuth\.sessions\.passwords\.create/u,
  'QR password completion must pass through the shared appbase IAM envelope helper and preserve the QrAuthSession DTO.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /toRuntimeSession\(\s*await\s+service\.createQrAuthPassword/u,
  'SDKWork Chat IAM runtime must return the canonical QrAuthSession to appbase instead of converting QR password completion into a session-only DTO.',
);

assert.match(
  appAuthServiceSource,
  /import\s*\{[^}]*unwrapIamSdkResponse[^}]*\}\s*from\s*['"]@sdkwork\/iam-sdk-adapter['"]/u,
  'SDKWork Chat auth service must consume the shared appbase IAM response envelope helper instead of defining app-local envelope logic.',
);

assert.match(
  appAuthServiceSource,
  /unwrapIamSdkResponse<\s*QrAuthSession\s*>[\s\S]*openPlatform\.qrAuth\.sessions\.create/u,
  'QR session creation must pass through the shared appbase IAM envelope helper before appbase auth components consume the session DTO.',
);

assert.match(
  appAuthServiceSource,
  /unwrapIamSdkResponse<\s*QrAuthSession\s*>[\s\S]*openPlatform\.qrAuth\.sessions\.retrieve/u,
  'QR session polling must pass through the shared appbase IAM envelope helper before appbase auth components consume the status DTO.',
);

for (const [methodMarker, message] of [
  ['getIam().auth.sessions.create', 'password login must go through the shared appbase IAM adapter'],
  ['getIam().auth.registrations.create', 'registration must go through the shared appbase IAM adapter'],
  ['getIam().auth.sessions.current.retrieve', 'session restore must go through the shared appbase IAM adapter'],
  ['getIam().auth.sessions.current.delete', 'logout must go through the shared appbase IAM adapter'],
  ['getIam().auth.sessions.refresh', 'token refresh must go through the shared appbase IAM adapter'],
  ['getIam().auth.verificationCodes.create', 'verification code creation must go through the shared appbase IAM adapter'],
  ['getIam().auth.verificationCodes.verify', 'verification code verification must go through the shared appbase IAM adapter'],
  ['getIam().openPlatform.qrAuth.sessions.create', 'QR session creation must go through the shared appbase IAM adapter'],
  ['getIam().openPlatform.qrAuth.sessions.retrieve', 'QR session polling must go through the shared appbase IAM adapter'],
  ['getIam().openPlatform.qrAuth.sessions.scans.create', 'QR scan recording must go through the shared appbase IAM adapter'],
  ['getIam().openPlatform.qrAuth.sessions.passwords.create', 'QR password confirmation must go through the shared appbase IAM adapter'],
]) {
  assert.match(
    appAuthServiceSource,
    new RegExp(methodMarker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    message,
  );
}

assert.match(
  authRuntimeSource,
  /context[\s\S]*sessionId/u,
  'SDKWork Chat IAM runtime must preserve AuthSession.context/sessionId for login state continuity.',
);

assert.match(
  authRuntimeSource,
  /user:\s*session\.user\s*\?\?\s*session\.userInfo\s*\?\?\s*existingSession\?\.user/u,
  'SDKWork Chat IAM runtime tokenStore must preserve appbase user/userInfo when token updates merge into the stored session.',
);

assert.match(
  authGateSource,
  /readAppSdkSessionTokens/u,
  'AuthGate must re-read persisted IAM session state so logout navigation cannot reuse a stale in-memory session.',
);

assert.match(
  authGateSource,
  /isAuthenticatedSession\(\s*session\s*\)\s*&&\s*isAuthenticatedSession\(\s*readAppSdkSessionTokens\(\)\s*\)/u,
  'AuthGate must treat the user as authenticated only when both in-memory state and persisted session storage are still authenticated.',
);

assert.match(
  chatLayoutSource,
  /import\s+\{\s*useNavigate\s*\}\s+from\s+['"]react-router-dom['"]/u,
  'ChatLayout must use router navigation for logout instead of reloading the whole page.',
);

assert.match(
  chatLayoutSource,
  /import\s+\{[\s\S]*appAuthService[\s\S]*\}\s+from\s+['"]@sdkwork\/clawchat-pc-core['"]/u,
  'ChatLayout logout must call the generated-SDK backed product auth service.',
);

assert.match(
  chatLayoutSource,
  /const\s+handleLogout\s*=\s*async\s*\(\)\s*=>\s*\{[\s\S]*await\s+appAuthService\.logout\(\)[\s\S]*navigate\(\s*["']\/auth\/login\?redirect=%2F["']\s*,\s*\{\s*replace:\s*true\s*\}\s*\)/u,
  'ChatLayout must clear the IAM session through appAuthService.logout and then replace the current route with the login screen.',
);

assert.match(
  chatLayoutSource,
  /<Sidebar[\s\S]*onLogout=\{handleLogout\}/u,
  'ChatLayout must pass the real logout handler into the profile menu path.',
);

assert.match(
  chatLayoutSource,
  /<SettingsModal[\s\S]*onLogout=\{handleLogout\}/u,
  'ChatLayout must pass the real logout handler into the settings modal path.',
);

assert.match(
  sidebarSource,
  /onLogout:\s*\(\)\s*=>\s*void\s*\|\s*Promise<void>/u,
  'Sidebar must accept the real logout handler and pass it to profile actions.',
);

assert.match(
  sidebarSource,
  /<ProfileMenuModal[\s\S]*onLogout=\{onLogout\}/u,
  'Sidebar must wire profile-menu logout to the real logout handler.',
);

for (const [source, label] of [
  [profileMenuSource, 'Profile menu'],
  [settingsModalSource, 'Settings modal'],
]) {
  assert.match(
    source,
    /onLogout:\s*\(\)\s*=>\s*void\s*\|\s*Promise<void>/u,
    `${label} must require a real logout handler instead of implementing app-local reload behavior.`,
  );
  assert.match(
    source,
    /onClick=\{\(\)\s*=>\s*\{[\s\S]*void\s+onLogout\(\);?[\s\S]*\}\}/u,
    `${label} logout button must invoke the real logout handler.`,
  );
  assert.doesNotMatch(
    source,
    /window\.location\.reload|location\.reload|setTimeout\([\s\S]*reload/u,
    `${label} logout must not rely on page reload because persisted tokens can keep the user authenticated.`,
  );
}

assert.doesNotMatch(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*?min-height:\s*100vh/u,
  'Auth shell must not use min-height: 100vh because the app header makes the login page overflow vertically.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*?height:\s*100dvh[\s\S]*?overflow:\s*hidden/u,
  'Auth shell must lock to the viewport and hide page-level overflow so the login screen does not show a scrollbar.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-page\s*\{[\s\S]*?min-height:\s*100%[\s\S]*?overflow:\s*hidden/u,
  'Appbase auth page must be constrained to the auth main viewport instead of adding its own viewport-height scroll surface.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-main\s+\.sdkwork-iam-auth-routes\s*\{[\s\S]*?height:\s*100%[\s\S]*?min-height:\s*0/u,
  'Flow auth routes must inherit the desktop auth main height instead of keeping their default 100dvh surface.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-card-shell\s*\{[\s\S]*?height:\s*auto[\s\S]*?max-height:\s*100%[\s\S]*?min-height:\s*0/u,
  'Auth card shell must size itself from the active login/register content within the available auth viewport.',
);

assert.doesNotMatch(
  authStylesSource,
  /sdkwork-chat-auth-card-shell\s*\{[\s\S]*?(?:height|min-height):\s*min\(680px,\s*calc\(100dvh\s*-\s*88px\)\)/u,
  'Auth card shell must not pin every login/register mode to the old 680px desktop height.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-content\s*\{[\s\S]*?overflow:\s*visible/u,
  'Auth content must remain visible and let the card adapt instead of creating an internal scrollbar.',
);

assert.doesNotMatch(
  authStylesSource,
  /sdkwork-chat-auth-content\s*\{[\s\S]*?overflow-y:\s*auto/u,
  'Auth content must not use overflow-y:auto because it creates the login/register card scrollbar.',
);

const lightAuthShellBlock = authStylesSource.match(
  /html\.light-mode \.sdkwork-chat-auth-shell\s*\{(?<body>[\s\S]*?)\n\}/u,
)?.groups?.body ?? '';

assert.match(
  authRuntimeSource,
  /asidePanelBackgroundColor:\s*['"]var\(--sdkwork-chat-auth-aside-bg\)['"]/u,
  'Appbase auth QR rail background must use the product-owned auth theme variable so light/dark mode can switch it.',
);

assert.match(
  authRuntimeSource,
  /asidePanelClassName:\s*['"]sdkwork-chat-auth-aside-panel['"]/u,
  'Appbase auth QR rail must receive a product-owned class so the app can tune nested QR panel utility colors without editing appbase.',
);

assert.match(
  authRuntimeSource,
  /qrFrameClassName:\s*['"]sdkwork-chat-auth-qr-frame['"]/u,
  'Appbase auth QR frame must receive a product-owned class for mode-specific QR frame polish.',
);

assert.match(
  authRuntimeSource,
  /asideCardBackgroundColor:\s*['"]var\(--sdkwork-chat-auth-aside-card-bg\)['"]/u,
  'Appbase auth QR rail cards must use the product-owned auth theme variable so light/dark mode can switch them.',
);

assert.doesNotMatch(
  authRuntimeSource,
  /asidePanelBackgroundColor:\s*['"]#101114['"]|asidePanelColor:\s*['"]#f5f5f5['"]/u,
  'Appbase auth appearance must not hard-code a dark QR rail because it prevents light mode from switching cleanly.',
);

assert.match(
  authStylesSource,
  /sdkwork-chat-auth-shell\s*\{[\s\S]*--sdkwork-chat-auth-aside-bg:/u,
  'Auth shell must define QR rail theme tokens next to the rest of the auth palette.',
);

assert.match(
  authStylesSource,
  /html\.light-mode \.sdkwork-chat-auth-aside-panel \.text-white\s*\{[\s\S]*color:\s*var\(--sdkwork-chat-auth-aside-text\)\s*!important/u,
  'Light auth mode must override nested QR rail text-white utility classes so QR rail text changes with the app theme.',
);

assert.match(
  authStylesSource,
  /html\.light-mode \.sdkwork-chat-auth-aside-panel \.text-zinc-300/u,
  'Light auth mode must override nested QR rail muted zinc utility classes so QR rail copy remains readable on the light surface.',
);

assert.match(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-qr-bg:\s*#fff(?:fff)?;/u,
  'Light auth mode must use a white QR frame background so the QR area switches away from the dark theme.',
);

assert.match(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-aside-bg:\s*#f8fbff;/u,
  'Light auth mode must use a quiet commercial light-blue QR rail surface.',
);

assert.doesNotMatch(
  lightAuthShellBlock,
  /--sdkwork-chat-auth-qr-bg:\s*rgba\(17,\s*24,\s*39,\s*0\.88\)|--sdkwork-chat-auth-aside-bg:\s*#101114/u,
  'Light auth mode must not keep the dark slate QR frame or rail background.',
);

assert.equal(
  tauriConfig.app.windows[0].decorations,
  false,
  'Desktop login header expects the Tauri window to keep native decorations disabled.',
);

assert.equal(
  tauriConfig.app.withGlobalTauri,
  true,
  'Desktop AppHeader controls use the Tauri global invoke bridge so live Vite code can call the native window-control command without adding @tauri-apps/api to the web package.',
);

for (const permission of [
  'core:default',
  'shell:allow-open',
  'core:window:allow-start-dragging',
  'core:window:allow-is-maximized',
  'core:window:allow-minimize',
  'core:window:allow-maximize',
  'core:window:allow-toggle-maximize',
  'core:window:allow-unmaximize',
  'core:window:allow-close',
]) {
  assert.ok(
    tauriDefaultCapability.permissions.includes(permission),
    `Tauri default capability must grant ${permission} for the custom auth AppHeader window controls.`,
  );
}

assert.ok(
  tauriDefaultCapability.permissions.includes('allow-sdkwork-chat-pc-window-control'),
  'Tauri default capability must grant the SDKWork Chat PC native window-control command.',
);

assert.match(
  tauriWindowControlPermissionSource,
  /identifier\s*=\s*["']allow-sdkwork-chat-pc-window-control["'][\s\S]*commands\.allow\s*=\s*\[\s*["']sdkwork_chat_pc_window_control["']\s*\]/u,
  'Tauri custom app command permission must allow only the SDKWork Chat PC native window-control command.',
);

assert.match(
  desktopRustSource,
  /enum\s+SdkworkChatPcWindowControlAction[\s\S]*Minimize[\s\S]*ToggleMaximize[\s\S]*CloseToTray[\s\S]*Show/u,
  'Desktop Rust shell must define cross-platform minimize, maximize, close-to-tray, and show actions.',
);

assert.match(
  desktopRustSource,
  /#\[tauri::command\][\s\S]*sdkwork_chat_pc_window_control/u,
  'Desktop Rust shell must expose a Tauri command for AppHeader window controls.',
);

assert.match(
  desktopRustSource,
  /CloseToTray[\s\S]*window\.hide\(\)/u,
  'Desktop close action must hide the main window to tray instead of terminating the process.',
);

assert.match(
  desktopRustSource,
  /WindowEvent::CloseRequested[\s\S]*api\.prevent_close\(\)[\s\S]*window\.hide\(\)/u,
  'Desktop native window close requests must be intercepted and hidden to tray so the backend remains running.',
);

assert.match(
  desktopRustSource,
  /TrayIconBuilder[\s\S]*on_tray_icon_event[\s\S]*handle_tray_icon_event[\s\S]*fn\s+handle_tray_icon_event[\s\S]*show_main_window\(app\)/u,
  'Desktop shell must install a tray icon that can restore the hidden main window.',
);

assert.match(
  desktopRustSource,
  /use\s+std::sync::atomic::\{AtomicBool,\s*Ordering\}/u,
  'Desktop shell must keep an explicit exiting flag so tray Quit can bypass close-to-tray interception.',
);

assert.match(
  desktopRustSource,
  /static\s+IS_EXITING:\s*AtomicBool\s*=\s*AtomicBool::new\(false\)/u,
  'Desktop shell must initialize the exiting flag to false.',
);

assert.match(
  desktopRustSource,
  /const\s+TRAY_MENU_CHAT_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_chat["'][\s\S]*const\s+TRAY_MENU_SETTINGS_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_settings["'][\s\S]*const\s+TRAY_MENU_QUIT_ID:\s*&str\s*=\s*["']sdkwork_chat_pc_tray_quit["']/u,
  'Desktop tray menu must define stable IDs for Chat, Settings, and Quit.',
);

assert.match(
  desktopRustSource,
  /const\s+TRAY_EVENT_OPEN_SETTINGS:\s*&str\s*=\s*["']sdkwork-chat-pc:\/\/tray\/open-settings["']/u,
  'Desktop tray settings action must emit a stable frontend event name.',
);

assert.match(
  desktopRustSource,
  /MenuBuilder::new\(app\)[\s\S]*\.text\(TRAY_MENU_CHAT_ID,\s*["']聊天["']\)[\s\S]*\.text\(TRAY_MENU_SETTINGS_ID,\s*["']设置["']\)[\s\S]*\.separator\(\)[\s\S]*\.text\(TRAY_MENU_QUIT_ID,\s*["']退出["']\)[\s\S]*\.build\(\)/u,
  'Desktop tray menu must expose right-click entries: 聊天, 设置, 退出.',
);

assert.match(
  desktopRustSource,
  /TrayIconBuilder::with_id\(TRAY_ICON_ID\)[\s\S]*\.menu\(&menu\)[\s\S]*\.show_menu_on_left_click\(false\)[\s\S]*\.on_menu_event\(\|app,\s*event\|\s*handle_tray_menu_event\(app,\s*event\)\)/u,
  'Desktop tray icon must bind the native menu and handle menu events while keeping left click as window restore.',
);

assert.match(
  desktopRustSource,
  /fn\s+handle_tray_menu_event<R:\s*Runtime>\(app:\s*&AppHandle<R>,\s*event:\s*tauri::menu::MenuEvent\)[\s\S]*TRAY_MENU_CHAT_ID[\s\S]*show_main_window\(app\)[\s\S]*TRAY_MENU_SETTINGS_ID[\s\S]*show_main_window\(app\)[\s\S]*emit\(TRAY_EVENT_OPEN_SETTINGS/u,
  'Desktop tray Chat must restore the main window and Settings must restore it before emitting the frontend settings event.',
);

assert.match(
  desktopRustSource,
  /fn\s+quit_app<R:\s*Runtime>\(app:\s*&AppHandle<R>\)[\s\S]*IS_EXITING\.store\(true,\s*Ordering::SeqCst\)[\s\S]*main_window\(app\)[\s\S]*window\.close\(\)[\s\S]*app\.exit\(0\)/u,
  'Desktop tray Quit must set the exiting flag, close the main window, and exit the Tauri app process.',
);

assert.match(
  desktopRustSource,
  /WindowEvent::CloseRequested[\s\S]*IS_EXITING\.load\(Ordering::SeqCst\)[\s\S]*return;[\s\S]*api\.prevent_close\(\)[\s\S]*window\.hide\(\)/u,
  'Desktop native close requests must only hide to tray when the app is not executing the tray Quit action.',
);

assert.match(
  appSource,
  /useTauriTrayNavigationBridge/u,
  'App must install a Tauri tray navigation bridge at the router level.',
);

assert.match(
  appSource,
  /const\s+TRAY_PENDING_SETTINGS_STORAGE_KEY\s*=\s*['"]sdkwork-chat-pc:pending-open-settings['"]/u,
  'Tray Settings must persist a pending settings intent so it survives navigation from console/admin routes.',
);

assert.match(
  appSource,
  /sdkwork-chat-pc:\/\/tray\/open-chat[\s\S]*navigate\(['"]\/['"],\s*\{\s*replace:\s*false\s*\}\)/u,
  'Tray Chat action must navigate the frontend back to the main chat route.',
);

assert.match(
  appSource,
  /sdkwork-chat-pc:\/\/tray\/open-settings[\s\S]*sessionStorage\.setItem\(TRAY_PENDING_SETTINGS_STORAGE_KEY,\s*['"]1['"]\)[\s\S]*navigate\(['"]\/['"],\s*\{\s*replace:\s*false\s*\}\)[\s\S]*window\.dispatchEvent\(new\s+CustomEvent\(['"]sdkwork-chat-pc:open-settings['"]\)\)/u,
  'Tray Settings action must record a pending settings intent, navigate to the chat route, and dispatch the in-app open-settings event.',
);

assert.match(
  chatLayoutSource,
  /window\.addEventListener\(['"]sdkwork-chat-pc:open-settings['"],\s*openSettingsFromTray\)/u,
  'ChatLayout must listen for the tray settings event.',
);

assert.match(
  chatLayoutSource,
  /const\s+openSettingsFromTray\s*=\s*\(\)\s*=>\s*\{[\s\S]*setActiveTab\(["']chat["']\)[\s\S]*setIsSettingsOpen\(true\)[\s\S]*\}/u,
  'ChatLayout tray settings handler must switch back to chat and open the settings modal.',
);

assert.match(
  chatLayoutSource,
  /sessionStorage\.getItem\(['"]sdkwork-chat-pc:pending-open-settings['"]\)[\s\S]*sessionStorage\.removeItem\(['"]sdkwork-chat-pc:pending-open-settings['"]\)[\s\S]*openSettingsFromTray\(\)/u,
  'ChatLayout must consume the pending tray settings intent after route navigation mounts the chat layout.',
);

assert.match(
  viteConfigSource,
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]iam[\\/]sdkwork-auth-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/auth-pc-react from canonical sdkwork-appbase so the verification-code register flow is available.',
);

assert.match(
  viteConfigSource,
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/i18n-pc-react for canonical appbase auth routes.',
);

assert.match(
  viteConfigSource,
  /sdkwork-ui[\\/]sdkwork-ui-pc-react[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/ui-pc-react from source so appbase auth UI does not load browser-unsafe dist chunks.',
);

assert.match(
  viteConfigSource,
  /\^@sdkwork\\\/ui-pc-react\\\/\(\.\+\)\$/u,
  'Vite must resolve @sdkwork/ui-pc-react subpaths from source for appbase PC React components.',
);

assert.doesNotMatch(
  viteConfigSource,
  /sdkwork-ui[\\/]sdkwork-ui-pc-react[\\/]dist[\\/]index\.js/u,
  'Vite must not resolve @sdkwork/ui-pc-react to dist/index.js because that bundle can emit browser runtime require("react") calls.',
);

assert.match(
  viteConfigSource,
  /dedupe:\s*\[[\s\S]*['"]react['"][\s\S]*['"]react-dom['"]/u,
  'Vite must dedupe React packages when composing appbase and sdkwork UI source packages.',
);

assert.match(
  viteConfigSource,
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-adapter[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/iam-sdk-adapter from canonical sdkwork-appbase source.',
);

assert.match(
  viteConfigSource,
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-ports[\\/]src[\\/]index\.ts/u,
  'Vite must resolve @sdkwork/iam-sdk-ports from canonical sdkwork-appbase source.',
);

assert.match(
  String(packageJson.dependencies?.['@sdkwork/auth-pc-react'] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]iam[\\/]sdkwork-auth-pc-react/u,
  'package.json must link @sdkwork/auth-pc-react to canonical sdkwork-appbase.',
);

assert.match(
  String(packageJson.dependencies?.['@sdkwork/i18n-pc-react'] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react/u,
  'package.json must include the canonical appbase i18n PC React package.',
);

assert.match(
  String(packageJson.dependencies?.['@sdkwork/iam-sdk-adapter'] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-adapter/u,
  'package.json must include the canonical appbase IAM SDK adapter package.',
);

assert.match(
  String(packageJson.dependencies?.['@sdkwork/iam-sdk-ports'] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-ports/u,
  'package.json must include the canonical appbase IAM SDK ports package.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/auth-pc-react']?.[0] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]iam[\\/]sdkwork-auth-pc-react[\\/]src[\\/]index\.ts/u,
  'tsconfig must resolve @sdkwork/auth-pc-react from canonical sdkwork-appbase.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/i18n-pc-react']?.[0] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]foundation[\\/]sdkwork-i18n-pc-react[\\/]src[\\/]index\.ts/u,
  'tsconfig must resolve @sdkwork/i18n-pc-react for canonical appbase auth routes.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/iam-sdk-adapter']?.[0] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-adapter[\\/]src[\\/]index\.ts/u,
  'tsconfig must resolve @sdkwork/iam-sdk-adapter from canonical sdkwork-appbase.',
);

assert.match(
  String(tsconfig.compilerOptions?.paths?.['@sdkwork/iam-sdk-ports']?.[0] ?? ''),
  /sdkwork-appbase[\\/]packages[\\/]common[\\/]iam[\\/]sdkwork-iam-sdk-ports[\\/]src[\\/]index\.ts/u,
  'tsconfig must resolve @sdkwork/iam-sdk-ports from canonical sdkwork-appbase.',
);

console.log('sdkwork chat appbase auth UI contract passed.');
