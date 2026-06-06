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

function flattenStrings(value, prefix = '') {
  if (typeof value === 'string') {
    return [[prefix, value]];
  }
  if (!value || typeof value !== 'object') {
    return [];
  }
  return Object.entries(value).flatMap(([key, nested]) =>
    flattenStrings(nested, prefix ? `${prefix}.${key}` : key)
  );
}

function assertNoMojibake(label, payload) {
  const suspicious = /[锟�]|(?:鎼|鍙|宸|涓|杩|榛|瀹|鏅|绋|缁|閫|瑙|璇|鐢|绠|妗|搴|浣|娣|鏈|鈱|鸿|绯|鍔|彴|亰|瘽|滅|妯|撳|愬|戞|湁|垚|枃|炕|栬|潰|樿|姞|煡|枫|棌|厠|煶|绘)/u;
  const matches = flattenStrings(payload).filter(([, value]) => suspicious.test(value));
  assert.deepEqual(
    matches,
    [],
    `${label} must not contain mojibake strings`,
  );
}

const chatZh = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/i18n/locales/zh-CN.json');
const chatEn = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/i18n/locales/en-US.json');
const workspaceZh = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-workspace/src/i18n/locales/zh-CN.json');
const workspaceEn = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-workspace/src/i18n/locales/en-US.json');
const rootPackage = readJson('package.json');

assert.equal(
  rootPackage.scripts?.['test:sdkwork-chat-pc-i18n'],
  'node scripts/dev/sdkwork-chat-pc-i18n.test.mjs',
  'root package must expose the sdkwork chat pc i18n regression test',
);

assertNoMojibake('chat zh-CN i18n', chatZh);
assertNoMojibake('workspace zh-CN i18n', workspaceZh);

assert.deepEqual(Object.keys(chatZh.sidebar).sort(), Object.keys(chatEn.sidebar).sort(), 'chat sidebar locale keys must match');
assert.equal(chatZh.sidebar.chat, '聊天');
assert.equal(chatZh.sidebar.workspace, '工作台');
assert.equal(chatZh.sidebar.settings, '设置');
assert.equal(chatZh.sidebar.settingsLocked, '当前应用设置已锁定');
assert.match(chatZh.agent.greeting, /^你好[！,，]我是 \{\{name\}\}，有什么.*帮你的吗？$/u);

for (const [key, expected] of Object.entries({
  loading: '加载中...',
  startWork: '开始高效工作吧',
  commonApps: '常用应用',
  appManagement: '应用管理',
  addApp: '添加应用',
  recentDocs: '最近文档',
  openInNewTab: '在新标签页打开',
  more: '更多',
  defaultCenter: '默认中心',
  loadAppFailed: '加载工作台数据失败',
})) {
  assert.equal(workspaceZh[key], expected, `workspace zh-CN ${key}`);
}
assert.equal(workspaceZh.apps.notary, '公证业务');
assert.equal(workspaceZh.apps.drive, '云盘');
assert.match(workspaceZh.apps.writing, /^AI\s*智能写作$/u);
assert.deepEqual(Object.keys(workspaceZh.apps).sort(), Object.keys(workspaceEn.apps).sort(), 'workspace app locale keys must match');
assert.deepEqual(Object.keys(workspaceZh.docs).sort(), Object.keys(workspaceEn.docs).sort(), 'workspace doc locale keys must match');

const sidebarSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/Sidebar.tsx');
const sidebarHoverTitleKeys = Object.keys(chatEn.sidebar).filter((key) => key !== 'settingsLocked');
for (const key of sidebarHoverTitleKeys) {
  assert.match(
    sidebarSource,
    new RegExp(`title=\\{t\\(['"]sidebar\\.${key}['"]\\)\\}`, 'u'),
    `Sidebar hover title for ${key} must use i18n`,
  );
}
assert.match(sidebarSource, /toast\(t\(['"]sidebar\.settingsLocked['"]\), "success"\)/u);
assert.doesNotMatch(
  sidebarSource,
  /title=["'][^"']*["']/u,
  'Sidebar must not use hard-coded hover title strings',
);

const settingsSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/SettingsModal.tsx');
assert.match(settingsSource, /useTranslation/u, 'Settings modal must use react-i18next');
assert.match(settingsSource, /i18n\.changeLanguage/u, 'Settings language selector must update the chat i18next instance');
assert.match(settingsSource, /sdkwork-chat-pc:language-changed/u, 'Settings language selector must notify nested workspace i18n instances');
assert.doesNotMatch(settingsSource, /value="ja-JP"/u, 'Settings language selector must not offer unsupported locales');

for (const relativePath of [
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/i18n/index.ts',
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-workspace/src/i18n/index.ts',
]) {
  const source = read(relativePath);
  assert.match(source, /resolveInitialLanguage/u, `${relativePath} must initialize from persisted chat settings language`);
  assert.match(source, /sdkwork-chat-pc:language-changed/u, `${relativePath} must subscribe to app language changes`);
  assert.match(source, /changeLanguage/u, `${relativePath} must update i18next when the app language changes`);
}

console.log('sdkwork-chat-pc i18n contract passed');
