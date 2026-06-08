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
assert.deepEqual(Object.keys(chatZh.profile).sort(), Object.keys(chatEn.profile).sort(), 'profile locale keys must match');
assert.deepEqual(
  Object.keys(chatZh.contacts.organizationDirectory).sort(),
  Object.keys(chatEn.contacts.organizationDirectory).sort(),
  'contact organization directory locale keys must match',
);
assert.deepEqual(
  Object.keys(chatZh.contacts.menu).sort(),
  Object.keys(chatEn.contacts.menu).sort(),
  'contact menu locale keys must match',
);
assert.deepEqual(
  Object.keys(chatZh.contacts.starred).sort(),
  Object.keys(chatEn.contacts.starred).sort(),
  'contact starred locale keys must match',
);
assert.deepEqual(
  Object.keys(chatZh.contacts.addFriend).sort(),
  Object.keys(chatEn.contacts.addFriend).sort(),
  'contact add friend locale keys must match',
);
assert.deepEqual(
  Object.keys(chatZh.contacts.groups).sort(),
  Object.keys(chatEn.contacts.groups).sort(),
  'contact groups locale keys must match',
);
assert.deepEqual(
  Object.keys(chatZh.contacts.detail).sort(),
  Object.keys(chatEn.contacts.detail).sort(),
  'contact detail locale keys must match',
);
assert.equal(chatZh.sidebar.chat, '聊天');
assert.equal(chatZh.sidebar.workspace, '工作台');
assert.equal(chatZh.sidebar.settings, '设置');
assert.equal(chatZh.sidebar.settingsLocked, '当前应用设置已锁定');
assert.match(chatZh.agent.greeting, /^你好[！,，]我是 \{\{name\}\}，有什么.*帮你的吗？$/u);
assert.equal(chatZh.profile.actions.logout, '退出登录');
assert.equal(chatZh.contacts.organizationDirectory.treeTitle, '组织架构');
assert.equal(chatZh.contacts.organizationDirectory.searchPlaceholder, '搜索组织、部门、成员');
assert.equal(chatZh.contacts.menu.organization, '组织架构');
assert.equal(chatZh.contacts.starred.title, '星标联系人');
assert.equal(chatZh.contacts.addFriend.title, '添加好友');
assert.equal(chatZh.contacts.groups.title, '我的群组');
assert.equal(chatZh.contacts.detail.basicInfo, '基本信息');
assert.equal(chatEn.profile.actions.logout, 'Log out');
assert.equal(chatEn.contacts.organizationDirectory.treeTitle, 'Organization');
assert.equal(chatEn.contacts.organizationDirectory.searchPlaceholder, 'Search organizations, departments, members');
assert.equal(chatEn.contacts.menu.organization, 'Organization');
assert.equal(chatEn.contacts.starred.title, 'Starred contacts');
assert.equal(chatEn.contacts.addFriend.title, 'Add Friend');
assert.equal(chatEn.contacts.groups.title, 'My groups');
assert.equal(chatEn.contacts.detail.basicInfo, 'Basic Info');

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

const orgContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/OrgContainer.tsx');
assert.match(orgContainerSource, /useTranslation/u, 'OrgContainer must use react-i18next');
assert.match(
  orgContainerSource,
  /contacts\.organizationDirectory\.treeTitle/u,
  'OrgContainer organization tree title must use i18n',
);
assert.match(
  orgContainerSource,
  /contacts\.organizationDirectory\.searchPlaceholder/u,
  'OrgContainer search placeholder must use i18n',
);

const contactsViewSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/ContactsView.tsx');
assert.match(contactsViewSource, /useTranslation/u, 'ContactsView must use react-i18next');
assert.match(
  contactsViewSource,
  /contacts\.menu\.organization/u,
  'ContactsView organization menu item must use i18n',
);
assert.match(
  contactsViewSource,
  /contacts\.starred\.title/u,
  'ContactsView starred section must use i18n',
);

const groupsContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/GroupsContainer.tsx');
assert.match(groupsContainerSource, /useTranslation/u, 'GroupsContainer must use react-i18next');
assert.match(groupsContainerSource, /contacts\.groups\.title/u, 'GroupsContainer title must use i18n');
assert.doesNotMatch(
  groupsContainerSource,
  /加载群组失败|我的群组|请输入新群组名称|创建群组成功|创建群组失败|发起群聊|加载中|人活跃/u,
  'GroupsContainer must not keep hard-coded Chinese contact group copy',
);

const contactDetailSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/ContactDetailPane.tsx');
assert.match(contactDetailSource, /useTranslation/u, 'ContactDetailPane must use react-i18next');
assert.match(contactDetailSource, /contacts\.detail\.basicInfo/u, 'ContactDetailPane basic info heading must use i18n');
assert.doesNotMatch(
  contactDetailSource,
  /Contact starred|Contact unstarred|Contact update failed|Chat ID is not ready|Chat ID copied|Voice calling is unavailable|Video calling is unavailable|Set remark|Remark updated|Remark update failed|Recommendation sent|Recommendation failed|Added to blacklist|Blacklist update failed|Contact deleted|Delete contact failed|Messaging is unavailable|Unknown position|Basic Info|Department|Company|Location|Signature|Mail app selected|Voice call|Video call|Copy Chat ID|Add to Blacklist/u,
  'ContactDetailPane must not keep hard-coded English contact detail copy',
);

const addFriendModalSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/AddFriendModal.tsx');
assert.match(addFriendModalSource, /useTranslation/u, 'AddFriendModal must use react-i18next');
assert.match(addFriendModalSource, /contacts\.addFriend\.title/u, 'AddFriendModal title must use i18n');
assert.doesNotMatch(
  addFriendModalSource,
  /['"]Add Friend['"]|['"]Searching contacts|`No contact found|['"]Search failed['"]|`Friend request sent|['"]Friend request failed['"]|['"]Email, Chat ID, or phone['"]|['"]Searching\.\.\.['"]|>\s*Search\s*<|>\s*Add\s*</u,
  'AddFriendModal must not keep hard-coded add-friend copy',
);

const profileMenuSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ProfileMenuModal.tsx');
assert.match(profileMenuSource, /useTranslation/u, 'ProfileMenuModal must use react-i18next');
assert.match(profileMenuSource, /profile\.copyChatId/u, 'ProfileMenuModal copy Chat ID title must use i18n');
assert.doesNotMatch(
  profileMenuSource,
  /['"]Chat ID is not ready|['"]Chat ID copied['"]|['"]Copy Chat ID failed['"]|>\s*Online\s*<|>\s*Favorites\s*<|>\s*Settings\s*<|>\s*Busy\s*<|>\s*Away\s*<|>\s*Log out\s*</u,
  'ProfileMenuModal must not keep hard-coded profile copy',
);

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
