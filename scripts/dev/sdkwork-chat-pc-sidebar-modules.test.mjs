import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const rootPackage = readJson('package.json');
const sidebarSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/Sidebar.tsx');
const settingsServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/SettingsService.ts');
const settingsModalSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/SettingsModal.tsx');
const chatLayoutSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/ChatLayout.tsx');
const newFriendsContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/NewFriendsContainer.tsx');
const contactServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService.ts');
const messageListSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/MessageList.tsx');
const chatListSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ChatList.tsx');
const toastSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/Toast.tsx');
const zhLocale = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/i18n/locales/zh-CN.json');
const enLocale = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/i18n/locales/en-US.json');

const expectedDefaultSidebarModules = [
  'chat',
  'workspace',
  'contacts',
  'knowledge',
  'drive',
  'agent',
  'favorites',
];

function extractStringArray(source, exportName) {
  const match = source.match(new RegExp(`export\\s+const\\s+${exportName}\\s*=\\s*\\[([\\s\\S]*?)\\]`, 'u'));
  assert.ok(match, `${exportName} must be exported as a string array`);
  return [...match[1].matchAll(/"([^"]+)"/gu)].map((item) => item[1]);
}

assert.equal(
  rootPackage.scripts?.['test:sdkwork-chat-pc-sidebar-modules'],
  'node scripts/dev/sdkwork-chat-pc-sidebar-modules.test.mjs',
  'root package must expose the sdkwork chat pc sidebar modules regression test',
);

assert.doesNotMatch(
  sidebarSource,
  /\bsetInterval\s*\(/u,
  'Sidebar must not poll /api/config/modules with setInterval',
);
assert.doesNotMatch(
  sidebarSource,
  /\bclearInterval\s*\(/u,
  'Sidebar should not own an interval cleanup for module config refresh',
);

assert.match(
  sidebarSource,
  /addEventListener\(["']focus["']/u,
  'Sidebar must refresh module config when the app window regains focus',
);
assert.match(
  sidebarSource,
  /addEventListener\(["']visibilitychange["']/u,
  'Sidebar must refresh module config when the app returns from hidden to visible',
);
assert.match(
  sidebarSource,
  /document\.visibilityState\s*===\s*["']visible["']/u,
  'Sidebar visibility handler must only refresh after the document becomes visible',
);

assert.match(
  sidebarSource,
  /sdkwork-chat-pc:settings-changed/u,
  'Sidebar must listen for settings changes instead of relying on polling',
);
assert.match(
  settingsServiceSource,
  /sdkwork-chat-pc:settings-changed/u,
  'SettingsService must notify same-window consumers after settings updates',
);
assert.match(
  settingsServiceSource,
  /dispatchEvent\(new CustomEvent/u,
  'SettingsService must dispatch a typed settings changed event after updates',
);

assert.deepEqual(
  extractStringArray(settingsServiceSource, 'DEFAULT_SIDEBAR_MODULES'),
  expectedDefaultSidebarModules,
  'default sidebar modules must only keep chat, workspace, contacts, knowledge, drive, agent, and favorites visible',
);

for (const moduleId of expectedDefaultSidebarModules) {
  assert.match(
    settingsServiceSource,
    new RegExp(`"(${moduleId})"`, 'u'),
    `SettingsService module catalog must include ${moduleId}`,
  );
}

assert.match(
  settingsServiceSource,
  /sidebarModules:\s*\[\.\.\.DEFAULT_SIDEBAR_MODULES\]/u,
  'SettingsService default settings must use DEFAULT_SIDEBAR_MODULES instead of all business modules',
);
assert.doesNotMatch(
  settingsServiceSource,
  /ALL_APP_MODULES\.forEach\(\(feature\)[\s\S]*?parsed\.sidebarModules\.push\(feature\)/u,
  'loading saved settings must not append every unfinished business module back into sidebarModules',
);
assert.match(
  settingsServiceSource,
  /normalizeSidebarModules/u,
  'SettingsService must normalize persisted sidebar modules through an explicit helper',
);

assert.match(
  sidebarSource,
  /DEFAULT_SIDEBAR_MODULES/u,
  'Sidebar must initialize and fall back to the default visible module list',
);
assert.doesNotMatch(
  sidebarSource,
  /m\s*===\s*["']course["']/u,
  'Sidebar must not treat course as a pinned module',
);
assert.doesNotMatch(
  sidebarSource,
  /setSidebarModules\(Array\.from\(new Set\(\["chat",\s*\.\.\.actualModules,\s*"course"\]\)\)\)/u,
  'Sidebar must not force-add course to the visible module set',
);
assert.match(
  sidebarSource,
  /modId\s*===\s*["']drive["']/u,
  'Sidebar must render a visible drive entry',
);
assert.match(
  sidebarSource,
  /onTabChange\(["']drive["']\)/u,
  'Sidebar drive entry must route to the existing drive view',
);
assert.match(
  sidebarSource,
  /title=\{t\(["']sidebar\.drive["']\)\}/u,
  'Sidebar drive entry must use the standard sidebar.drive i18n title',
);
assert.equal(zhLocale.sidebar.drive, '网盘', 'zh-CN sidebar locale must name drive as 网盘');
assert.equal(enLocale.sidebar.drive, 'Cloud Drive', 'en-US sidebar locale must name drive as Cloud Drive');

assert.match(
  settingsModalSource,
  /DEFAULT_SIDEBAR_MODULES/u,
  'SettingsModal must fall back to the product-ready default sidebar modules',
);
assert.match(
  settingsModalSource,
  /\{\s*id:\s*["']drive["'][\s\S]*?icon:\s*Cloud/u,
  'SettingsModal module list must include the drive module with the standard cloud icon',
);
assert.match(
  settingsModalSource,
  /DEFAULT_SIDEBAR_MODULES\.includes\(mod\.id\)/u,
  'SettingsModal must keep the product-ready default modules visible even when the server module list is incomplete',
);
assert.match(
  chatLayoutSource,
  /case\s+["']drive["']:\s*[\r\n\s]*return\s+<DriveView\s*\/>/u,
  'ChatLayout must route drive to the existing DriveView',
);
assert.match(
  chatLayoutSource,
  /\[[\s\S]*["']drive["'][\s\S]*\]\.includes\(activeTab\)/u,
  'ChatLayout must treat drive as a full-screen module page instead of rendering an empty unified header',
);

assert.match(
  chatLayoutSource,
  /subscribePendingFriendRequestCount/u,
  'ChatLayout must subscribe to pending friend request counts for the contacts red dot',
);
assert.match(
  chatLayoutSource,
  /friendRequestUnreadCount=\{friendRequestUnreadCount\}/u,
  'ChatLayout must pass the pending friend request count into Sidebar',
);
assert.match(
  sidebarSource,
  /friendRequestUnreadCount\?:\s*number/u,
  'Sidebar must accept the pending friend request count without owning SDK calls',
);
assert.match(
  sidebarSource,
  /modId\s*===\s*["']contacts["'][\s\S]*friendRequestUnreadCount\s*>\s*0[\s\S]*bg-red-500/u,
  'Sidebar contacts icon must reuse the existing red badge style for pending friend requests',
);
assert.match(
  newFriendsContainerSource,
  /subscribePendingFriendRequestCount/u,
  'NewFriendsContainer must refresh when friend request count changes so accepted/rejected requests disappear promptly',
);
assert.match(
  contactServiceSource,
  /startPendingFriendRequestRealtime/u,
  'ContactService must start realtime friend request subscriptions when red dot listeners are active',
);
assert.match(
  contactServiceSource,
  /friend_request\.submitted/u,
  'ContactService must listen for submitted friend request realtime events instead of relying only on polling',
);
assert.match(
  toastSource,
  /placement\?:\s*['"]top['"]\s*\|\s*['"]bottom-right['"]/u,
  'Toast API must support an optional bottom-right placement for desktop notification-style reminders',
);
assert.match(
  toastSource,
  /bottom-6\s+right-6/u,
  'ToastContainer must render bottom-right notifications as an overlay without changing the main layout',
);
assert.match(
  chatLayoutSource,
  /previousFriendRequestUnreadCountRef/u,
  'ChatLayout must track the previous friend request count so first load does not spam notifications',
);
assert.match(
  chatLayoutSource,
  /toast\([^)]*friendRequest[^)]*\{[^}]*placement:\s*["']bottom-right["']/u,
  'ChatLayout must show a bottom-right notification when pending friend requests increase',
);
assert.match(
  messageListSource,
  /formatVideoCallMessageContent/u,
  'MessageList must format RTC call content through a display-name resolver instead of rendering raw participant ids',
);
assert.match(
  messageListSource,
  /resolveDisplayName/u,
  'MessageList must resolve RTC call participant ids to current-user/contact display names before rendering',
);
assert.match(
  chatListSource,
  /formatChatListLastMessage/u,
  'ChatList must format RTC call previews before rendering the last message so raw participant ids are not shown in the conversation list',
);
assert.match(
  chatListSource,
  /replaceRtcPreviewParticipantId/u,
  'ChatList RTC call previews must replace known participant ids with the chat display name without changing the sidebar layout',
);

console.log('sdkwork-chat-pc sidebar modules refresh contract passed');
