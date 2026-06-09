import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function sliceBetween(source, startMarker, endMarker, description) {
  const start = source.indexOf(startMarker);
  assert.notEqual(start, -1, `${description} must keep ${startMarker} auditable`);
  const end = source.indexOf(endMarker, start);
  assert.notEqual(end, -1, `${description} must end before ${endMarker}`);
  return source.slice(start, end);
}

const createGroupModalSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/CreateGroupModal.tsx');
const allContactsSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/AllContactsContainer.tsx');
const groupsSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/GroupsContainer.tsx');
const newFriendsSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/NewFriendsContainer.tsx');
const contactDetailSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/ContactDetailPane.tsx');

assert.match(
  createGroupModalSource,
  /onCreated\?:\s*\(group:\s*Chat\)\s*=>\s*void\s*\|\s*Promise<void>/u,
  'CreateGroupModal must allow async onCreated callbacks so backend chat hydration can be awaited',
);
assert.match(
  createGroupModalSource,
  /await\s+onCreated\?\.\(group\)/u,
  'CreateGroupModal must await parent backend hydration before showing success and closing',
);
assert.match(
  createGroupModalSource,
  /contactService\.getContacts\(\)[\s\S]*?\.catch\s*\(/u,
  'CreateGroupModal contact loading must fail-close instead of leaving the modal in a loading state',
);
assert.match(
  createGroupModalSource,
  /contactService\.getContacts\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'CreateGroupModal contact loading must always clear loading after success or failure',
);

assert.match(
  allContactsSource,
  /contactService\.getContacts\(\)[\s\S]*?\.catch\s*\(/u,
  'All contacts loading must surface backend failures and stop loading',
);
assert.match(
  allContactsSource,
  /contactService\.getContacts\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'All contacts loading must always clear loading after success or failure',
);

assert.match(
  groupsSource,
  /groupService\.getGroups\(\)[\s\S]*?\.catch\s*\(/u,
  'Groups loading must surface backend failures and stop loading',
);
assert.match(
  groupsSource,
  /groupService\.getGroups\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'Groups loading must always clear loading after success or failure',
);
assert.match(
  groupsSource,
  /<CreateGroupModal[\s\S]*?onCreated=\{async\s*\(group\)\s*=>\s*\{[\s\S]*?setGroups\(\s*\(\s*previousGroups\s*\)\s*=>\s*\[group,\s*\.\.\.previousGroups\]\s*\)[\s\S]*?onOpenGroup\?\.\(\s*group\s*\)/u,
  'GroupsContainer must reuse CreateGroupModal and append through a functional state update so rapid group creation does not drop previous groups',
);
assert.doesNotMatch(
  groupsSource,
  /customPrompt\s*\(|groupService\.createGroup\s*\([^)]*\[\s*\]\s*\)/u,
  'GroupsContainer must not bypass CreateGroupModal with prompt-based empty group creation',
);

assert.match(
  newFriendsSource,
  /contactService\.getFriendRequests\(\)[\s\S]*?\.catch\s*\(/u,
  'New friends loading must surface backend failures and stop loading',
);
assert.match(
  newFriendsSource,
  /contactService\.getFriendRequests\(\)[\s\S]*?\.finally\s*\(\s*\(\)\s*=>\s*setLoading\(false\)\s*\)/u,
  'New friends loading must always clear loading after success or failure',
);
assert.match(
  newFriendsSource,
  /await\s+contactService\.handleFriendRequest\(req\.id,\s*'reject'\)/u,
  'Rejecting a friend request must await the real SDK mutation before local state changes',
);
assert.match(
  newFriendsSource,
  /await\s+contactService\.handleFriendRequest\(req\.id,\s*'accept'\)/u,
  'Accepting a friend request must await the real SDK mutation before local state changes',
);
assert.match(
  newFriendsSource,
  /catch\s*(?:\(|\{)/u,
  'Friend request actions must surface backend failures instead of creating unhandled promises',
);

assert.doesNotMatch(
  contactDetailSource,
  /toast\s*\(\s*`[^`]*\$\{user\.phone\}[^`]*`\s*,\s*['"]success['"]\s*\)/u,
  'Contact phone row must not show a fake successful call toast before the RTC call path is available',
);
assert.match(
  contactDetailSource,
  /const\s+startVoiceCall\s*=\s*\(\)\s*=>\s*\{[\s\S]*?if\s*\(onStartCall\)\s*\{[\s\S]*?onStartCall\('voice',\s*user\);[\s\S]*?return;[\s\S]*?toast\(t\('contacts\.detail\.toast\.voiceUnavailable'\),\s*'error'\);[\s\S]*?\};/u,
  'Contact phone row must fail-close when RTC start-call wiring is unavailable',
);
assert.match(
  contactDetailSource,
  /await\s+contactService\.recommendToFriend\(user\.id\)[\s\S]*catch\s*(?:\(|\{)/u,
  'Recommend-contact action must handle SDK failures',
);
assert.match(
  contactDetailSource,
  /await\s+contactService\.addToBlacklist\(user\.id\)[\s\S]*catch\s*(?:\(|\{)/u,
  'Blacklist-contact action must handle SDK failures',
);

console.log('sdkwork-chat-pc chat UI failure handling contract passed');
