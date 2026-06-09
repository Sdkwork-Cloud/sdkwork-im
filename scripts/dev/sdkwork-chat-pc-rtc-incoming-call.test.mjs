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
  /\.calls\.accept\s*\(\s*rtcSessionId\s*\)/u,
  'acceptIncomingCall must synchronize accept signaling through the composed IM calls facade.',
);
assert.match(
  callServiceSource,
  /\.calls\.reject\s*\(\s*rtcSessionId\s*\)/u,
  'rejectIncomingCall must synchronize reject signaling through the composed IM calls facade.',
);
assert.match(
  callServiceSource,
  /\.calls\.end\s*\(\s*rtcSessionId\s*\)/u,
  'endCall must synchronize hangup signaling through the composed IM calls facade.',
);
assert.match(
  callServiceSource,
  /targetName:\s*[\s\S]*\?\?\s*this\.snapshot\.targetName/u,
  'CallService snapshots must not expose raw initiator ids as target names when ChatLayout can resolve friendly chat/contact names.',
);
assert.match(
  callServiceSource,
  /peerUserId\??:\s*string/u,
  'CallService snapshots must expose a peerUserId so the call modal can resolve the friendly contact name instead of showing a raw user id.',
);
assert.match(
  callServiceSource,
  /incomingState !== ['"]ringing['"]/u,
  'CallService must ignore closing-only call signals for unknown sessions instead of opening a false incoming overlay.',
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
  callOverlaySource,
  /isIncomingRinging\s*=\s*mode\s*===\s*['"]incoming['"][\s\S]*callState\s*===\s*['"]ringing['"]/u,
  'CallOverlay must render accept/reject for a newly opened incoming ringing modal even when the previous global call snapshot was terminal.',
);
assert.match(
  callOverlaySource,
  /isOutgoingRinging/u,
  'CallOverlay must distinguish outgoing ringing from connected calls so the caller only sees the cancel/hangup action before the peer answers.',
);
assert.match(
  callOverlaySource,
  /isConnectedCall/u,
  'CallOverlay must distinguish connected calls so mute, camera, and screen-share controls are not shown during ringing.',
);
assert.match(
  callOverlaySource,
  /canControlLocalMedia\s*=\s*callOverlayPhase\s*===\s*['"]connected['"]\s*\|\|\s*callOverlayPhase\s*===\s*['"]outgoing-ringing['"]/u,
  'CallOverlay must let the caller control local microphone/camera state while an outgoing call is ringing through the explicit phase matrix.',
);
assert.match(
  callOverlaySource,
  /canToggleVideo\s*=\s*canControlLocalMedia\s*&&\s*type\s*===\s*['"]video['"]/u,
  'CallOverlay must show camera controls for connected video calls and outgoing video calls before the peer answers.',
);
assert.match(
  callOverlaySource,
  /canShareScreen\s*=\s*callOverlayPhase\s*===\s*['"]connected['"]\s*&&\s*type\s*===\s*['"]video['"]/u,
  'CallOverlay must show screen sharing only for connected video calls.',
);
assert.match(
  callOverlaySource,
  /localMediaStatusText/u,
  'CallOverlay must display the caller local microphone/camera status instead of only hiding controls.',
);
assert.match(
  callOverlaySource,
  /type\s+CallOverlayPhase\s*=/u,
  'CallOverlay button visibility must be derived from an explicit call phase matrix instead of scattered role checks.',
);
assert.match(
  callOverlaySource,
  /callOverlayPhase\s*===\s*['"]incoming-ringing['"][\s\S]*callService\.acceptIncomingCall/u,
  'CallOverlay incoming ringing controls must show accept/reject actions for the receiver.',
);
assert.match(
  callOverlaySource,
  /callOverlayPhase\s*===\s*['"]outgoing-ringing['"][\s\S]*callService\.endCall/u,
  'CallOverlay outgoing ringing controls must show caller local media controls and a cancel action.',
);
assert.match(
  callOverlaySource,
  /callOverlayPhase\s*===\s*['"]connected['"][\s\S]*canShareScreen/u,
  'CallOverlay connected controls must enable in-call media controls and video screen sharing.',
);
assert.match(
  callOverlaySource,
  /callOverlayPhase\s*===\s*['"]finished['"][\s\S]*closeOverlayWithMediaRelease\s*\(\s*\)/u,
  'CallOverlay finished calls must release local media and close the modal instead of sending duplicate hangup signaling.',
);
assert.match(
  callOverlaySource,
  /autoClosedTerminalSessionRef/u,
  'CallOverlay must remember terminal RTC sessions it has auto-closed so repeated end/reject/error signals do not close the modal more than once.',
);
assert.match(
  callOverlaySource,
  /snapshotIsTerminal[\s\S]*autoClosedTerminalSessionRef[\s\S]*closeOverlayWithMediaRelease\s*\(\s*\)/u,
  'CallOverlay must release local media and automatically close when the current RTC session receives a remote ended/rejected/errored snapshot.',
);
assert.match(
  callOverlaySource,
  /truncate/u,
  'CallOverlay must keep long peer names on one line with ellipsis instead of wrapping and breaking the modal layout.',
);
assert.match(
  callOverlaySource,
  /getUserMedia\s*\(/u,
  'CallOverlay must request browser camera/microphone media for real voice/video call controls.',
);
assert.match(
  callOverlaySource,
  /localMediaStreamRef/u,
  'CallOverlay must retain the local media stream so mute/camera controls affect real tracks and cleanup can stop them.',
);
assert.match(
  callOverlaySource,
  /screenShareStreamRef/u,
  'CallOverlay must retain the screen-share stream so hangup, remote terminal signals, and unmount can release it.',
);
assert.match(
  callOverlaySource,
  /getTracks\s*\(\s*\)\.forEach[\s\S]*\.stop\s*\(/u,
  'CallOverlay must stop all local camera, microphone, and screen-share tracks when a call ends or closes.',
);
assert.match(
  callOverlaySource,
  /\.enabled\s*=/u,
  'CallOverlay mute/camera toggles must update MediaStreamTrack.enabled instead of only updating UI state.',
);
assert.match(
  callOverlaySource,
  /localPreviewVideoRef/u,
  'CallOverlay must keep a local video element ref so the existing picture-in-picture area can show the real camera preview.',
);
assert.match(
  callOverlaySource,
  /\.srcObject\s*=\s*stream/u,
  'CallOverlay must bind the browser local media stream to a video element instead of showing only a static placeholder.',
);
assert.match(
  callOverlaySource,
  /autoPlay[\s\S]*playsInline[\s\S]*muted/u,
  'CallOverlay local camera preview must be autoplaying, inline, and muted to avoid echo.',
);
assert.match(
  callOverlaySource,
  /releaseCallMedia\s*\(\s*\)[\s\S]*onClose\s*\(\s*\)/u,
  'CallOverlay must release browser media before closing after terminal call actions or remote terminal snapshots.',
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
assert.match(
  chatLayoutSource,
  /rtcSessionId=\{callTarget\.rtcSessionId\}/u,
  'ChatLayout must pass the active RTC session id into CallOverlay so stale terminal snapshots from older calls do not suppress incoming accept/reject controls.',
);
assert.match(
  chatLayoutSource,
  /contactService\.getUserById\s*\(\s*snapshot\.peerUserId/u,
  'ChatLayout must resolve incoming call peerUserId through ContactService before rendering the call modal name.',
);

console.log('sdkwork-chat-pc RTC incoming call contract passed');
