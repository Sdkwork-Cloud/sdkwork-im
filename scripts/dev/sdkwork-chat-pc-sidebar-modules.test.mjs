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

console.log('sdkwork-chat-pc sidebar modules refresh contract passed');
