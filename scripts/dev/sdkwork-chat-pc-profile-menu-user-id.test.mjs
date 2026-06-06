import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const profileMenuSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ProfileMenuModal.tsx');

assert.match(
  profileMenuSource,
  /currentUser\.id/u,
  'ProfileMenuModal must render the current user id in the profile header',
);

assert.match(
  profileMenuSource,
  /navigator\.clipboard\.writeText\(\s*currentUser\.id\s*\)/u,
  'ProfileMenuModal user id must support copying the current user id',
);

assert.match(
  profileMenuSource,
  /title=["']复制ID["']/u,
  'ProfileMenuModal user id copy control must expose a copy-ID title',
);

assert.match(
  profileMenuSource,
  /toast\(["']已复制ID["'],\s*["']success["']\)/u,
  'ProfileMenuModal must show an in-app success toast after copying the user id',
);

console.log('sdkwork-chat-pc profile menu user id contract passed');
