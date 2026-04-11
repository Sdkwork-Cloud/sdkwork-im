import assert from 'node:assert/strict';

import { CrawChatClient } from '../dist/index.js';

function createBackendClientStub() {
  const calls = [];
  const backendClient = {
    session: {
      async resume(body) {
        calls.push({ method: 'session.resume', body });
        return { sessionId: 'session-1', deviceId: body.deviceId };
      },
      async disconnect(body) {
        calls.push({ method: 'session.disconnect', body });
        return { currentDeviceId: body.deviceId };
      },
    },
    presence: {
      async heartbeat(body) {
        calls.push({ method: 'presence.heartbeat', body });
        return { currentDeviceId: body.deviceId };
      },
      async getPresenceMe() {
        calls.push({ method: 'presence.getPresenceMe' });
        return { currentDeviceId: 'device-1' };
      },
    },
    realtime: {
      async syncRealtimeSubscriptions(body) {
        calls.push({ method: 'realtime.syncRealtimeSubscriptions', body });
        return { items: body.items ?? [] };
      },
      async listRealtimeEvents(params) {
        calls.push({ method: 'realtime.listRealtimeEvents', params });
        return { items: [] };
      },
      async ackRealtimeEvents(body) {
        calls.push({ method: 'realtime.ackRealtimeEvents', body });
        return { ackedSeq: body.ackedSeq };
      },
    },
    device: {
      async register(body) {
        calls.push({ method: 'device.register', body });
        return { deviceId: body.deviceId };
      },
      async getDeviceSyncFeed(deviceId, params) {
        calls.push({ method: 'device.getDeviceSyncFeed', deviceId, params });
        return { items: [] };
      },
    },
    inbox: {
      async getInbox() {
        calls.push({ method: 'inbox.getInbox' });
        return { items: [] };
      },
    },
    conversation: {
      async createConversation(body) {
        calls.push({ method: 'conversation.createConversation', body });
        return { conversationId: body.conversationId };
      },
      async createAgentDialog(body) {
        calls.push({ method: 'conversation.createAgentDialog', body });
        return { conversationId: body.conversationId };
      },
      async createAgentHandoff(body) {
        calls.push({ method: 'conversation.createAgentHandoff', body });
        return { conversationId: body.conversationId };
      },
      async createSystemChannel(body) {
        calls.push({ method: 'conversation.createSystemChannel', body });
        return { conversationId: body.conversationId };
      },
      async getConversationSummary(conversationId) {
        calls.push({ method: 'conversation.getConversationSummary', conversationId });
        return { conversationId };
      },
      async getAgentHandoffState(conversationId) {
        calls.push({ method: 'conversation.getAgentHandoffState', conversationId });
        return { conversationId };
      },
      async acceptAgentHandoff(conversationId) {
        calls.push({ method: 'conversation.acceptAgentHandoff', conversationId });
        return { conversationId };
      },
      async resolveAgentHandoff(conversationId) {
        calls.push({ method: 'conversation.resolveAgentHandoff', conversationId });
        return { conversationId };
      },
      async closeAgentHandoff(conversationId) {
        calls.push({ method: 'conversation.closeAgentHandoff', conversationId });
        return { conversationId };
      },
      async listConversationMembers(conversationId) {
        calls.push({ method: 'conversation.listConversationMembers', conversationId });
        return { items: [] };
      },
      async addConversationMember(conversationId, body) {
        calls.push({ method: 'conversation.addConversationMember', conversationId, body });
        return { memberId: body.principalId };
      },
      async removeConversationMember(conversationId, body) {
        calls.push({ method: 'conversation.removeConversationMember', conversationId, body });
        return { memberId: body.memberId };
      },
      async transferConversationOwner(conversationId, body) {
        calls.push({ method: 'conversation.transferConversationOwner', conversationId, body });
        return { memberId: body.memberId };
      },
      async changeConversationMemberRole(conversationId, body) {
        calls.push({ method: 'conversation.changeConversationMemberRole', conversationId, body });
        return { memberId: body.memberId };
      },
      async leave(conversationId) {
        calls.push({ method: 'conversation.leave', conversationId });
        return { conversationId };
      },
      async getConversationReadCursor(conversationId) {
        calls.push({ method: 'conversation.getConversationReadCursor', conversationId });
        return { lastReadMessageId: 'msg-1' };
      },
      async updateConversationReadCursor(conversationId, body) {
        calls.push({ method: 'conversation.updateConversationReadCursor', conversationId, body });
        return { readSeq: body.readSeq };
      },
      async listConversationMessages(conversationId) {
        calls.push({ method: 'conversation.listConversationMessages', conversationId });
        return { items: [] };
      },
      async postConversationMessage(conversationId, body) {
        calls.push({ method: 'conversation.postConversationMessage', conversationId, body });
        return { messageId: 'msg-1', conversationId };
      },
      async publishSystemChannelMessage(conversationId, body) {
        calls.push({ method: 'conversation.publishSystemChannelMessage', conversationId, body });
        return { messageId: 'system-1', conversationId };
      },
    },
    message: {
      async edit(messageId, body) {
        calls.push({ method: 'message.edit', messageId, body });
        return { messageId };
      },
      async recall(messageId) {
        calls.push({ method: 'message.recall', messageId });
        return { messageId };
      },
    },
    media: {
      async createMediaUpload(body) {
        calls.push({ method: 'media.createMediaUpload', body });
        return { mediaAssetId: 'asset-1' };
      },
      async completeMediaUpload(mediaAssetId, body) {
        calls.push({ method: 'media.completeMediaUpload', mediaAssetId, body });
        return { mediaAssetId };
      },
      async getMediaDownloadUrl(mediaAssetId, params) {
        calls.push({ method: 'media.getMediaDownloadUrl', mediaAssetId, params });
        return { url: 'https://example.test/file' };
      },
      async getMediaAsset(mediaAssetId) {
        calls.push({ method: 'media.getMediaAsset', mediaAssetId });
        return { mediaAssetId };
      },
      async attachMediaAsset(mediaAssetId, body) {
        calls.push({ method: 'media.attachMediaAsset', mediaAssetId, body });
        return { messageId: 'msg-2' };
      },
    },
    stream: {
      async open(body) {
        calls.push({ method: 'stream.open', body });
        return { streamId: body.streamId };
      },
      async listStreamFrames(streamId, params) {
        calls.push({ method: 'stream.listStreamFrames', streamId, params });
        return { items: [] };
      },
      async appendStreamFrame(streamId, body) {
        calls.push({ method: 'stream.appendStreamFrame', streamId, body });
        return { streamId, frameSeq: body.frameSeq, payload: body.payload };
      },
      async checkpoint(streamId, body) {
        calls.push({ method: 'stream.checkpoint', streamId, body });
        return { streamId, lastCheckpointSeq: body.frameSeq };
      },
      async complete(streamId, body) {
        calls.push({ method: 'stream.complete', streamId, body });
        return { streamId, resultMessageId: body.resultMessageId };
      },
      async abort(streamId, body) {
        calls.push({ method: 'stream.abort', streamId, body });
        return { streamId };
      },
    },
    rtc: {
      async createRtcSession(body) {
        calls.push({ method: 'rtc.createRtcSession', body });
        return { rtcSessionId: body.rtcSessionId };
      },
      async inviteRtcSession(rtcSessionId, body) {
        calls.push({ method: 'rtc.inviteRtcSession', rtcSessionId, body });
        return { rtcSessionId };
      },
      async acceptRtcSession(rtcSessionId, body) {
        calls.push({ method: 'rtc.acceptRtcSession', rtcSessionId, body });
        return { rtcSessionId };
      },
      async rejectRtcSession(rtcSessionId, body) {
        calls.push({ method: 'rtc.rejectRtcSession', rtcSessionId, body });
        return { rtcSessionId };
      },
      async endRtcSession(rtcSessionId, body) {
        calls.push({ method: 'rtc.endRtcSession', rtcSessionId, body });
        return { rtcSessionId };
      },
      async postRtcSignal(rtcSessionId, body) {
        calls.push({ method: 'rtc.postRtcSignal', rtcSessionId, body });
        return { rtcSessionId, signalType: body.signalType, payload: body.payload };
      },
      async issueRtcParticipantCredential(rtcSessionId, body) {
        calls.push({ method: 'rtc.issueRtcParticipantCredential', rtcSessionId, body });
        return { rtcSessionId, participantId: body.participantId };
      },
      async getRtcRecordingArtifact(rtcSessionId) {
        calls.push({ method: 'rtc.getRtcRecordingArtifact', rtcSessionId });
        return { rtcSessionId, playbackUrl: 'https://example.test/recording' };
      },
    },
    setAuthToken(token) {
      calls.push({ method: 'setAuthToken', token });
      return backendClient;
    },
  };

  return { backendClient, calls };
}

async function testConversationsPostText() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatClient({ backendClient });

  const result = await sdk.conversations.postText('conversation-1', 'hello world', {
    clientMsgId: 'client-1',
    summary: 'Greeting',
    renderHints: { tone: 'friendly' },
  });

  assert.deepEqual(result, { messageId: 'msg-1', conversationId: 'conversation-1' });
  assert.deepEqual(calls.at(-1), {
    method: 'conversation.postConversationMessage',
    conversationId: 'conversation-1',
    body: {
      clientMsgId: 'client-1',
      summary: 'Greeting',
      text: 'hello world',
      renderHints: { tone: 'friendly' },
    },
  });
}

async function testStreamsAppendTextFrame() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatClient({ backendClient });

  const result = await sdk.streams.appendTextFrame('stream-1', {
    frameSeq: 7,
    text: 'partial chunk',
    schemaRef: 'urn:craw-chat:stream:text',
    attributes: { role: 'assistant' },
  });

  assert.equal(result.frameSeq, 7);
  assert.deepEqual(calls.at(-1), {
    method: 'stream.appendStreamFrame',
    streamId: 'stream-1',
    body: {
      frameSeq: 7,
      frameType: 'text',
      schemaRef: 'urn:craw-chat:stream:text',
      encoding: 'text/plain; charset=utf-8',
      payload: 'partial chunk',
      attributes: { role: 'assistant' },
    },
  });
}

async function testRtcPostJsonSignal() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatClient({ backendClient });

  const result = await sdk.rtc.postJsonSignal('rtc-1', 'offer', {
    schemaRef: 'urn:craw-chat:rtc:signal',
    signalingStreamId: 'signal-stream-1',
    payload: {
      sdp: 'v=0',
      type: 'offer',
    },
  });

  assert.equal(result.rtcSessionId, 'rtc-1');
  assert.deepEqual(calls.at(-1), {
    method: 'rtc.postRtcSignal',
    rtcSessionId: 'rtc-1',
    body: {
      signalType: 'offer',
      schemaRef: 'urn:craw-chat:rtc:signal',
      signalingStreamId: 'signal-stream-1',
      payload: JSON.stringify({
        sdp: 'v=0',
        type: 'offer',
      }),
    },
  });
}

function testTokenHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatClient({ backendClient });

  sdk.setAuthToken('auth-token');
  assert.equal(typeof sdk.setAccessToken, 'undefined');
  assert.equal(typeof sdk.setApiKey, 'undefined');

  assert.deepEqual(
    calls.slice(-1),
    [
      { method: 'setAuthToken', token: 'auth-token' },
    ],
  );
}

await testConversationsPostText();
await testStreamsAppendTextFrame();
await testRtcPostJsonSignal();
testTokenHelpers();

console.log('craw-chat composed sdk smoke tests passed');
