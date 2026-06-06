import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const callServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/CallService.ts');
const callOverlaySource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/CallOverlay.tsx');
const chatLayoutSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/ChatLayout.tsx');

assert.match(
  callServiceSource,
  /watchIncomingCalls\s*\(/u,
  'CallService must expose watchIncomingCalls so the PC app subscribes to conversation RTC invitations.',
);
assert.match(
  callServiceSource,
  /acceptIncomingCall\s*\(/u,
  'CallService must expose acceptIncomingCall for incoming RTC voice/video calls.',
);
assert.match(
  callServiceSource,
  /rejectIncomingCall\s*\(/u,
  'CallService must expose rejectIncomingCall for incoming RTC voice/video calls.',
);
assert.match(
  callServiceSource,
  /\.callController\.acceptIncoming\s*\(/u,
  'acceptIncomingCall must delegate to the standard RTC call controller acceptIncoming method.',
);
assert.match(
  callServiceSource,
  /\.callController\.rejectIncoming\s*\(/u,
  'rejectIncomingCall must delegate to the standard RTC call controller rejectIncoming method.',
);
assert.match(
  callServiceSource,
  /controllerSnapshot\.activeInvitation\?\.initiatorDisplayName/u,
  'CallService snapshots must map incoming invitation display name into the product call target.',
);
assert.match(
  callServiceSource,
  /direction:\s*controllerSnapshot\.direction/u,
  'CallService snapshots must expose incoming/outgoing direction from the RTC controller.',
);

assert.match(
  callOverlaySource,
  /mode\?\s*:\s*['"]incoming['"]\s*\|\s*['"]outgoing['"]/u,
  'CallOverlay must support incoming and outgoing modes.',
);
assert.match(
  callOverlaySource,
  /mode\s*===\s*['"]outgoing['"][\s\S]*startOutgoingCall/u,
  'CallOverlay must start SDK-backed outgoing calls only in outgoing mode.',
);
assert.match(
  callOverlaySource,
  /callService\.acceptIncomingCall\s*\(/u,
  'CallOverlay must accept incoming calls through CallService.',
);
assert.match(
  callOverlaySource,
  /callService\.rejectIncomingCall\s*\(/u,
  'CallOverlay must reject incoming calls through CallService.',
);

assert.match(
  chatLayoutSource,
  /callService\.subscribe/u,
  'ChatLayout must subscribe to CallService so incoming invitations can open the call overlay.',
);
assert.match(
  chatLayoutSource,
  /direction\s*===\s*['"]incoming['"]/u,
  'ChatLayout must detect incoming call snapshots.',
);
assert.match(
  chatLayoutSource,
  /mode=\{callMode\}/u,
  'ChatLayout must pass incoming/outgoing mode into CallOverlay.',
);

console.log('sdkwork-chat-pc RTC incoming call contract passed');
