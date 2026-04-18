import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

import { CrawChatSdkClient } from '../dist/index.js';

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
    auth: {
      async login(body) {
        calls.push({ method: 'auth.login', body });
        return {
          accessToken: 'portal-token',
          refreshToken: 'portal-refresh-token',
          expiresAt: 1760000000,
          user: {
            id: 'ops_demo',
            login: 'ops_demo',
            name: 'Lin Tao',
            role: 'Tenant Operations Lead',
            email: 'lin.tao@nebula-commerce.example',
            actorKind: 'user',
            clientKind: 'portal_operator',
            permissions: ['portal.access'],
          },
          workspace: {
            name: 'Nebula Commerce IM',
            slug: 'nebula-commerce-im',
          },
        };
      },
      async me() {
        calls.push({ method: 'auth.me' });
        return {
          tenantId: 't_demo',
          user: {
            id: 'ops_demo',
            login: 'ops_demo',
            name: 'Lin Tao',
            role: 'Tenant Operations Lead',
            email: 'lin.tao@nebula-commerce.example',
            actorKind: 'user',
            clientKind: 'portal_operator',
            permissions: ['portal.access'],
          },
          workspace: {
            name: 'Nebula Commerce IM',
            slug: 'nebula-commerce-im',
          },
        };
      },
    },
    portal: {
      async getHome() {
        calls.push({ method: 'portal.getHome' });
        return { hero: { title: 'Craw Chat Tenant Portal' } };
      },
      async getAuth() {
        calls.push({ method: 'portal.getAuth' });
        return { title: 'Sign in to Nebula Commerce IM' };
      },
      async getWorkspace() {
        calls.push({ method: 'portal.getWorkspace' });
        return { name: 'Nebula Commerce IM' };
      },
      async getDashboard() {
        calls.push({ method: 'portal.getDashboard' });
        return { hero: { title: 'Tenant operations overview' } };
      },
      async getConversations() {
        calls.push({ method: 'portal.getConversations' });
        return { hero: { title: 'Conversation operations' } };
      },
      async getRealtime() {
        calls.push({ method: 'portal.getRealtime' });
        return { hero: { title: 'Realtime posture' } };
      },
      async getMedia() {
        calls.push({ method: 'portal.getMedia' });
        return { hero: { title: 'Media and RTC' } };
      },
      async getAutomation() {
        calls.push({ method: 'portal.getAutomation' });
        return { hero: { title: 'Automation and notifications' } };
      },
      async getGovernance() {
        calls.push({ method: 'portal.getGovernance' });
        return { hero: { title: 'Governance and compliance' } };
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
        calls.push({
          method: 'rtc.issueRtcParticipantCredential',
          rtcSessionId,
          body,
        });
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

function createRealtimeEvent(overrides = {}) {
  return {
    tenantId: 'tenant-1',
    principalId: 'user-1',
    deviceId: 'device-1',
    realtimeSeq: 1,
    scopeType: 'conversation',
    scopeId: 'conversation-1',
    eventType: 'message.created',
    deliveryClass: 'durable',
    payload: JSON.stringify({}),
    occurredAt: '2026-04-15T12:00:00.000Z',
    ...overrides,
  };
}

function createFakeWebSocket() {
  const listeners = {
    open: new Set(),
    message: new Set(),
    close: new Set(),
    error: new Set(),
  };
  const sent = [];
  let readyState = 0;

  return {
    sent,
    get readyState() {
      return readyState;
    },
    addEventListener(type, listener) {
      listeners[type]?.add(listener);
    },
    removeEventListener(type, listener) {
      listeners[type]?.delete(listener);
    },
    send(data) {
      sent.push(typeof data === 'string' ? JSON.parse(data) : data);
    },
    close(code, reason) {
      readyState = 3;
      this.emit('close', { code, reason });
    },
    open() {
      readyState = 1;
      this.emit('open', { type: 'open' });
    },
    receiveJson(value) {
      this.emit('message', {
        data: JSON.stringify(value),
      });
    },
    emit(type, event) {
      for (const listener of listeners[type] ?? []) {
        listener(event);
      }
    },
  };
}

async function flushMicrotasks(times = 2) {
  for (let index = 0; index < times; index += 1) {
    await Promise.resolve();
  }
}

async function testPublicExportsHideLegacySurface() {
  const sdkModule = await import('../dist/index.js');

  assert.equal('CrawChatSdkClient' in sdkModule, true);
  assert.equal('generated' in sdkModule, true);
  assert.equal('SdkworkBackendClient' in sdkModule, true);
  assert.equal('CrawChatClient' in sdkModule, false);
  assert.equal('createCrawChatClient' in sdkModule, false);
  assert.equal('CrawChatGeneratedClient' in sdkModule, false);
  assert.equal('CrawChatReceiver' in sdkModule, false);
  assert.equal('CrawChatWebSocketReceiver' in sdkModule, false);
  assert.equal('CrawChatAuthModule' in sdkModule, false);
  assert.equal('CrawChatConversationsModule' in sdkModule, false);
  assert.equal('CrawChatLiveModule' in sdkModule, false);
  assert.equal('CrawChatMediaModule' in sdkModule, false);
  assert.equal('CrawChatMessagesModule' in sdkModule, false);
  assert.equal('CrawChatRtcModule' in sdkModule, false);
  assert.equal('CrawChatSyncModule' in sdkModule, false);
}

function testDeclarationSurfaceUsesSdkNaming() {
  const typesDeclaration = readFileSync(new URL('../dist/types.d.ts', import.meta.url), 'utf8');
  const authModuleDeclaration = readFileSync(
    new URL('../dist/auth-module.d.ts', import.meta.url),
    'utf8',
  );
  const messagesModuleDeclaration = readFileSync(
    new URL('../dist/messages-module.d.ts', import.meta.url),
    'utf8',
  );
  const sdkDeclaration = readFileSync(new URL('../dist/sdk.d.ts', import.meta.url), 'utf8');
  const sdkContextDeclaration = readFileSync(
    new URL('../dist/sdk-context.d.ts', import.meta.url),
    'utf8',
  );

  assert.match(typesDeclaration, /\bCrawChatSdkClientOptions\b/);
  assert.match(typesDeclaration, /\bCrawChatAuthLoginRequest\b/);
  assert.match(typesDeclaration, /\bCrawChatAuthLoginResult\b/);
  assert.match(typesDeclaration, /\bCrawChatAuthSession\b/);
  assert.match(typesDeclaration, /\bCrawChatSubscription\b/);
  assert.match(typesDeclaration, /\bCrawChatCreateAgentInput\b/);
  assert.match(typesDeclaration, /\bai_image_generation\b/);
  assert.match(typesDeclaration, /\bai_video_generation\b/);

  assert.doesNotMatch(typesDeclaration, /\bCrawChatClientOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatClientCreateOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatBootstrapSessionOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatBootstrapSessionResult\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalLoginRequest\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalLoginResponse\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalMeResponse\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalSnapshot\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalUserView\b/);
  assert.doesNotMatch(typesDeclaration, /\bPortalWorkspaceView\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatBackendClientLike\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverEvent\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverMessageHandler\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverDataHandler\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverRtcSignalHandler\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverBatch\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverPullAckResult\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatRealtimeWebSocketReceiverOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatRealtimeWebSocketMode\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatReceiverSubscription\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatSdkContextOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatResolvedSdkClientOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatTransportConfig\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatCreateRawMessageOptions\b/);
  assert.doesNotMatch(typesDeclaration, /\bCrawChatRtcSignalEnvelope\b/);
  assert.doesNotMatch(typesDeclaration, /\bbackendConfig\??:\b/);
  assert.doesNotMatch(typesDeclaration, /\bai_image\b/);
  assert.doesNotMatch(typesDeclaration, /\bai_video\b/);
  const connectOptionsDeclarationMatch = typesDeclaration.match(
    /export interface CrawChatConnectOptions \{([\s\S]*?)\n\}/,
  );
  assert.ok(connectOptionsDeclarationMatch, 'Expected CrawChatConnectOptions declaration.');
  assert.doesNotMatch(connectOptionsDeclarationMatch[0], /\bsessionId\??:/);
  assert.doesNotMatch(connectOptionsDeclarationMatch[0], /\bmode\??:/);
  const sdkClientOptionsDeclarationMatch = typesDeclaration.match(
    /export interface CrawChatSdkClientOptions[\s\S]*?\n\}/,
  );
  assert.ok(sdkClientOptionsDeclarationMatch, 'Expected CrawChatSdkClientOptions declaration.');
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\bbackendClient\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\bheaders\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\btimeout\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\btokenManager\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\bfetch\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\blogger\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\bretry\??:/);
  assert.doesNotMatch(sdkClientOptionsDeclarationMatch[0], /\bextends Partial<SdkworkBackendConfig>\b/);
  const webSocketFactoryRequestDeclarationMatch = typesDeclaration.match(
    /export interface CrawChatRealtimeWebSocketFactoryRequest \{([\s\S]*?)\n\}/,
  );
  assert.ok(
    webSocketFactoryRequestDeclarationMatch,
    'Expected CrawChatRealtimeWebSocketFactoryRequest declaration.',
  );
  assert.doesNotMatch(webSocketFactoryRequestDeclarationMatch[0], /\bmode\??:/);

  assert.match(authModuleDeclaration, /\bCrawChatAuthLoginRequest\b/);
  assert.match(authModuleDeclaration, /\bCrawChatAuthLoginResult\b/);
  assert.match(authModuleDeclaration, /\bCrawChatAuthSession\b/);
  assert.doesNotMatch(authModuleDeclaration, /\bPortalLoginRequest\b/);
  assert.doesNotMatch(authModuleDeclaration, /\bPortalLoginResponse\b/);
  assert.doesNotMatch(authModuleDeclaration, /\bPortalMeResponse\b/);

  assert.match(messagesModuleDeclaration, /\bcreateAiImageGeneration\b/);
  assert.match(messagesModuleDeclaration, /\bcreateAiVideoGeneration\b/);
  assert.match(messagesModuleDeclaration, /\bcreateAgent\b/);
  assert.doesNotMatch(messagesModuleDeclaration, /\bcreateAiImage\b/);
  assert.doesNotMatch(messagesModuleDeclaration, /\bcreateAiVideo\b/);

  assert.match(sdkDeclaration, /\bconstructor\(options: CrawChatSdkClientOptions\)/);
  assert.match(sdkDeclaration, /\breadonly generated: SdkworkBackendClient;/);
  assert.doesNotMatch(sdkDeclaration, /\bCrawChatClientCreateOptions\b/);
  assert.doesNotMatch(sdkDeclaration, /\breadonly backendClient:/);

  assert.match(sdkContextDeclaration, /\bCrawChatSdkClientOptions\b/);
  assert.doesNotMatch(sdkContextDeclaration, /\bCrawChatClientOptions\b/);
  assert.doesNotMatch(sdkContextDeclaration, /\bCrawChatClientCreateOptions\b/);
}

function testInternalRealtimeDeclarationsUseEventOnlyShape() {
  const readDeclaration = (...relativePaths) => {
    for (const relativePath of relativePaths) {
      try {
        return readFileSync(new URL(relativePath, import.meta.url), 'utf8');
      } catch (error) {
        if (error?.code !== 'ENOENT') {
          throw error;
        }
      }
    }

    throw new Error(`Missing declaration file for any of: ${relativePaths.join(', ')}`);
  };

  const internalTypesDeclaration = readDeclaration('../dist/receiver-internal-types.d.ts');
  const internalReceiverDeclaration = readDeclaration('../dist/receiver.d.ts');
  const internalWebSocketReceiverDeclaration = readDeclaration('../dist/websocket-receiver.d.ts');
  const assembledInternalTypesDeclaration = readDeclaration(
    '../../dist/receiver-internal-types.d.ts',
    '../composed/dist/receiver-internal-types.d.ts',
  );
  const assembledInternalReceiverDeclaration = readDeclaration(
    '../../dist/receiver.d.ts',
    '../composed/dist/receiver.d.ts',
  );
  const assembledInternalWebSocketReceiverDeclaration = readDeclaration(
    '../../dist/websocket-receiver.d.ts',
    '../composed/dist/websocket-receiver.d.ts',
  );

  for (const declaration of [
    internalTypesDeclaration,
    assembledInternalTypesDeclaration,
  ]) {
    assert.doesNotMatch(declaration, /\bCrawChatInternalReceiverMessageHandler\b/);
    assert.doesNotMatch(declaration, /\bCrawChatInternalReceiverDataHandler\b/);
    assert.doesNotMatch(declaration, /\bCrawChatInternalReceiverRtcSignalHandler\b/);
  }

  for (const declaration of [
    internalReceiverDeclaration,
    assembledInternalReceiverDeclaration,
    internalWebSocketReceiverDeclaration,
    assembledInternalWebSocketReceiverDeclaration,
  ]) {
    assert.doesNotMatch(declaration, /\bonMessage\(/);
    assert.doesNotMatch(declaration, /\bonData\(/);
    assert.doesNotMatch(declaration, /\bonRtcSignal\(/);
    assert.doesNotMatch(declaration, /\bonConversationMessage\(/);
  }
}

function testSynchronousClientConstructionAndDomains() {
  const sdk = new CrawChatSdkClient({
    baseUrl: 'https://api.example.test',
    websocketBaseUrl: 'wss://socket.example.test',
    authToken: 'token-1',
  });

  assert.equal(typeof CrawChatSdkClient.create, 'undefined');
  assert.equal(sdk.getApiBaseUrl(), 'https://api.example.test');
  assert.equal(sdk.getWebSocketBaseUrl(), 'wss://socket.example.test');
  assert.equal(
    sdk.resolveRealtimeWebSocketUrl(),
    'wss://socket.example.test/api/v1/realtime/ws',
  );
  assert.equal(typeof sdk.auth.login, 'function');
  assert.equal(typeof sdk.auth.useToken, 'function');
  assert.equal(typeof sdk.auth.clearToken, 'function');
  assert.equal(typeof sdk.messages.createText, 'function');
  assert.equal(typeof sdk.messages.send, 'function');
  assert.equal(typeof sdk.messages.uploadAndSend, 'function');
  assert.equal(typeof sdk.createTextMessage, 'function');
  assert.equal(typeof sdk.send, 'function');
  assert.equal(typeof sdk.uploadAndSendMessage, 'function');
  assert.equal(typeof sdk.decodeMessage, 'function');
  assert.equal(typeof sdk.createAiImageGenerationMessage, 'function');
  assert.equal(typeof sdk.createAiVideoGenerationMessage, 'function');
  assert.equal(typeof sdk.createAgentMessage, 'function');
  assert.equal(typeof sdk.sync.catchUp, 'function');
  assert.equal(typeof sdk.sync.ack, 'function');
  assert.equal(typeof sdk.connect, 'function');
  assert.equal(typeof sdk.messages.createAiImageGeneration, 'function');
  assert.equal(typeof sdk.messages.createAiVideoGeneration, 'function');
  assert.equal(typeof sdk.messages.createAgent, 'function');
  assert.equal(typeof sdk.messages.createAiImage, 'undefined');
  assert.equal(typeof sdk.messages.createAiVideo, 'undefined');
  assert.equal(typeof sdk.portal.getWorkspace, 'function');
  assert.equal(typeof sdk.portal.getDashboard, 'function');
  assert.equal(typeof sdk.backendClient, 'undefined');
  assert.equal(typeof sdk.bootstrapSession, 'undefined');
  assert.equal(typeof sdk.createReceiver, 'undefined');
  assert.equal(typeof sdk.createWebSocketReceiver, 'undefined');
}

function testAuthModuleManagesTokens() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
  });

  sdk.auth.useToken('auth-token');
  sdk.auth.clearToken();

  assert.deepEqual(calls.slice(-2), [
    { method: 'setAuthToken', token: 'auth-token' },
    { method: 'setAuthToken', token: '' },
  ]);
}

function testAuthModulePrefersClearAuthTokenWhenBackendSupportsIt() {
  const { backendClient, calls } = createBackendClientStub();
  backendClient.clearAuthToken = () => {
    calls.push({ method: 'clearAuthToken' });
    return backendClient;
  };
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
  });

  sdk.auth.useToken('auth-token');
  sdk.auth.clearToken();

  assert.deepEqual(calls.slice(-2), [
    { method: 'setAuthToken', token: 'auth-token' },
    { method: 'clearAuthToken' },
  ]);
}

async function testPortalDomainProxiesGeneratedPortalHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
  });

  const workspace = await sdk.portal.getWorkspace();
  const dashboard = await sdk.portal.getDashboard();

  assert.deepEqual(workspace, { name: 'Nebula Commerce IM' });
  assert.deepEqual(dashboard, { hero: { title: 'Tenant operations overview' } });
  assert.deepEqual(calls.slice(-2), [
    { method: 'portal.getWorkspace' },
    { method: 'portal.getDashboard' },
  ]);
}

async function testTopLevelMessageApisCreateSendAndUpload() {
  const { backendClient, calls } = createBackendClientStub();
  backendClient.media.createMediaUpload = async (body) => {
    calls.push({ method: 'media.createMediaUpload', body });
    return {
      mediaAsset: {
        mediaAssetId: body.mediaAssetId,
        processingState: 'pendingUpload',
        resource: body.resource,
      },
      mediaAssetId: body.mediaAssetId,
      bucket: body.bucket,
      objectKey: body.objectKey,
      storageProvider: 'object-storage-volcengine',
      uploadMethod: 'PUT',
      uploadUrl: `https://upload.example.test/${body.bucket}/${body.objectKey}`,
      uploadHeaders: {
        'content-type': body.resource.mimeType ?? 'application/octet-stream',
      },
      uploadExpiresInSeconds: 900,
    };
  };
  backendClient.media.completeMediaUpload = async (mediaAssetId, body) => {
    calls.push({ method: 'media.completeMediaUpload', mediaAssetId, body });
    return {
      mediaAssetId,
      processingState: 'ready',
      storageProvider: body.storageProvider,
      bucket: body.bucket,
      objectKey: body.objectKey,
      resource: {
        type: 'image',
        mimeType: 'image/png',
        url: `https://cdn.example.test/${mediaAssetId}.png`,
      },
    };
  };

  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
  });

  const text = sdk.createTextMessage({
    conversationId: 'conversation-1',
    text: 'hello world',
    summary: 'Greeting',
    renderHints: { tone: 'friendly' },
  });

  assert.deepEqual(text, {
    kind: 'text',
    target: {
      conversationId: 'conversation-1',
      channel: 'conversation',
    },
    body: {
      text: 'hello world',
      summary: 'Greeting',
      renderHints: { tone: 'friendly' },
    },
  });

  const delivery = await sdk.send(text);
  assert.deepEqual(delivery, { messageId: 'msg-1', conversationId: 'conversation-1' });

  const uploadRequests = [];
  const originalFetch = globalThis.fetch;
  globalThis.fetch = async (input, init = {}) => {
    uploadRequests.push({ input, init });
    return new Response(null, {
      status: 200,
      headers: {
        etag: 'etag-showcase-upload',
      },
    });
  };

  try {
    const uploaded = await sdk.uploadAndSendMessage({
      upload: {
        mediaAssetId: 'asset-image-1',
        bucket: 'tenant-demo-media',
        objectKey: 'showcase/showcase.png',
        resource: {
          type: 'image',
          name: 'showcase.png',
          mimeType: 'image/png',
          size: 4,
        },
        body: new Uint8Array([1, 2, 3, 4]),
      },
      createMessage: (preparedUpload) =>
        sdk.createImageMessage({
          conversationId: 'conversation-1',
          mediaAssetId: preparedUpload.mediaAssetId,
          text: 'Uploaded showcase image',
          summary: 'Showcase image',
        }),
    });

    assert.equal(uploaded.mediaAssetId, 'asset-image-1');
    assert.equal(uploaded.url, 'https://cdn.example.test/asset-image-1.png');
    assert.equal(uploaded.etag, 'etag-showcase-upload');
    assert.equal(uploaded.message.kind, 'image');
    assert.equal(uploaded.delivery.messageId, 'msg-1');
    assert.equal(uploadRequests.length, 1);
    assert.equal(
      uploadRequests[0].input,
      'https://upload.example.test/tenant-demo-media/showcase/showcase.png',
    );
    assert.equal(uploadRequests[0].init.method, 'PUT');
    assert.deepEqual(uploadRequests[0].init.headers, {
      'content-type': 'image/png',
    });
    assert.deepEqual(calls.slice(-3), [
      {
        method: 'media.createMediaUpload',
        body: {
          mediaAssetId: 'asset-image-1',
          bucket: 'tenant-demo-media',
          objectKey: 'showcase/showcase.png',
          resource: {
            type: 'image',
            name: 'showcase.png',
            mimeType: 'image/png',
            size: 4,
          },
        },
      },
      {
        method: 'media.completeMediaUpload',
        mediaAssetId: 'asset-image-1',
        body: {
          bucket: 'tenant-demo-media',
          objectKey: 'showcase/showcase.png',
          url: 'https://upload.example.test/tenant-demo-media/showcase/showcase.png',
          storageProvider: 'object-storage-volcengine',
          checksum: undefined,
          etag: 'etag-showcase-upload',
        },
      },
      {
        method: 'conversation.postConversationMessage',
        conversationId: 'conversation-1',
        body: {
          text: 'Uploaded showcase image',
          summary: 'Showcase image',
          parts: [
            {
              kind: 'media',
              mediaAssetId: 'asset-image-1',
              resource: {
                type: 'image',
              },
            },
          ],
        },
      },
    ]);
  } finally {
    globalThis.fetch = originalFetch;
  }
}

async function testStandaloneMediaUploadHelpersUsePresignedUploadSession() {
  const { backendClient, calls } = createBackendClientStub();
  backendClient.media.createMediaUpload = async (body) => {
    calls.push({ method: 'media.createMediaUpload', body });
    return {
      mediaAsset: {
        mediaAssetId: body.mediaAssetId,
        processingState: 'pendingUpload',
        resource: body.resource,
      },
      mediaAssetId: body.mediaAssetId,
      bucket: body.bucket,
      objectKey: body.objectKey,
      storageProvider: 'object-storage-volcengine',
      uploadMethod: 'PUT',
      uploadUrl: `https://upload.example.test/${body.bucket}/${body.objectKey}`,
      uploadHeaders: {
        'content-type': body.resource.mimeType ?? 'application/octet-stream',
      },
      uploadExpiresInSeconds: 900,
    };
  };
  backendClient.media.completeMediaUpload = async (mediaAssetId, body) => {
    calls.push({ method: 'media.completeMediaUpload', mediaAssetId, body });
    return {
      mediaAssetId,
      processingState: 'ready',
      storageProvider: body.storageProvider,
      bucket: body.bucket,
      objectKey: body.objectKey,
      resource: {
        type: 'image',
        mimeType: 'image/png',
        url: `https://cdn.example.test/${mediaAssetId}.png`,
      },
    };
  };

  const uploadRequests = [];
  const originalFetch = globalThis.fetch;
  globalThis.fetch = async (input, init = {}) => {
    uploadRequests.push({ input, init });
    return new Response(null, {
      status: 200,
      headers: {
        etag: 'etag-demo-upload',
      },
    });
  };

  try {
    const sdk = new CrawChatSdkClient({
      backendClient,
      baseUrl: 'https://api.example.test',
    });

    assert.equal(typeof sdk.media.upload, 'function');
    assert.equal(typeof sdk.media.uploadAndComplete, 'function');
    assert.equal(typeof sdk.upload, 'function');

    const prepared = await sdk.media.uploadAndComplete({
      mediaAssetId: 'asset-upload-0',
      bucket: 'tenant-demo-media',
      objectKey: 'tenant/t_demo/asset-upload-0/demo.png',
      resource: {
        type: 'image',
        name: 'demo.png',
        mimeType: 'image/png',
        size: 4,
      },
      body: new Uint8Array([9, 8, 7, 6]),
    });

    const uploaded = await sdk.upload({
      mediaAssetId: 'asset-upload-1',
      bucket: 'tenant-demo-media',
      objectKey: 'tenant/t_demo/asset-upload-1/demo.png',
      resource: {
        type: 'image',
        name: 'demo.png',
        mimeType: 'image/png',
        size: 4,
      },
      body: new Uint8Array([1, 2, 3, 4]),
    });

    assert.equal(prepared.mediaAssetId, 'asset-upload-0');
    assert.equal(prepared.url, 'https://cdn.example.test/asset-upload-0.png');
    assert.equal(prepared.etag, 'etag-demo-upload');
    assert.equal(uploaded.mediaAssetId, 'asset-upload-1');
    assert.equal(uploaded.url, 'https://cdn.example.test/asset-upload-1.png');
    assert.equal(uploaded.asset.processingState, 'ready');
    assert.equal(uploaded.etag, 'etag-demo-upload');
    assert.equal(uploadRequests.length, 2);
    assert.equal(
      uploadRequests[0].input,
      'https://upload.example.test/tenant-demo-media/tenant/t_demo/asset-upload-0/demo.png',
    );
    assert.equal(uploadRequests[0].init.method, 'PUT');
    assert.deepEqual(uploadRequests[0].init.headers, {
      'content-type': 'image/png',
    });
    assert.equal(
      uploadRequests[1].input,
      'https://upload.example.test/tenant-demo-media/tenant/t_demo/asset-upload-1/demo.png',
    );
    assert.equal(uploadRequests[1].init.method, 'PUT');
    assert.deepEqual(uploadRequests[1].init.headers, {
      'content-type': 'image/png',
    });
    assert.deepEqual(calls.slice(-4), [
      {
        method: 'media.createMediaUpload',
        body: {
          mediaAssetId: 'asset-upload-0',
          bucket: 'tenant-demo-media',
          objectKey: 'tenant/t_demo/asset-upload-0/demo.png',
          resource: {
            type: 'image',
            name: 'demo.png',
            mimeType: 'image/png',
            size: 4,
          },
        },
      },
      {
        method: 'media.completeMediaUpload',
        mediaAssetId: 'asset-upload-0',
        body: {
          bucket: 'tenant-demo-media',
          objectKey: 'tenant/t_demo/asset-upload-0/demo.png',
          url: 'https://upload.example.test/tenant-demo-media/tenant/t_demo/asset-upload-0/demo.png',
          storageProvider: 'object-storage-volcengine',
          checksum: undefined,
          etag: 'etag-demo-upload',
        },
      },
      {
        method: 'media.createMediaUpload',
        body: {
          mediaAssetId: 'asset-upload-1',
          bucket: 'tenant-demo-media',
          objectKey: 'tenant/t_demo/asset-upload-1/demo.png',
          resource: {
            type: 'image',
            name: 'demo.png',
            mimeType: 'image/png',
            size: 4,
          },
        },
      },
      {
        method: 'media.completeMediaUpload',
        mediaAssetId: 'asset-upload-1',
        body: {
          bucket: 'tenant-demo-media',
          objectKey: 'tenant/t_demo/asset-upload-1/demo.png',
          url: 'https://upload.example.test/tenant-demo-media/tenant/t_demo/asset-upload-1/demo.png',
          storageProvider: 'object-storage-volcengine',
          checksum: undefined,
          etag: 'etag-demo-upload',
        },
      },
    ]);
  } finally {
    globalThis.fetch = originalFetch;
  }
}

function testExpandedMessageFamilies() {
  const { backendClient } = createBackendClientStub();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
  });

  const agent = sdk.createAgentMessage({
    conversationId: 'conversation-ai',
    text: 'Primary support agent joined',
    agentId: 'assistant-1',
    agentName: 'Assistant',
    stage: 'active',
    status: 'online',
    capabilities: ['summarize', 'route'],
  });
  const agentDecoded = sdk.decodeMessage(agent.body);
  assert.equal(agentDecoded.type, 'agent');
  assert.equal(agentDecoded.content.agentId, 'assistant-1');
  assert.deepEqual(agentDecoded.content.capabilities, ['summarize', 'route']);

  const aiText = sdk.createAiTextMessage({
    conversationId: 'conversation-ai',
    text: 'Assistant answer',
    prompt: 'summarize the last order',
    model: 'gpt-5.4',
    status: 'completed',
  });
  const aiTextDecoded = sdk.decodeMessage(aiText.body);
  assert.equal(aiTextDecoded.type, 'ai_text');
  assert.equal(aiTextDecoded.content.prompt, 'summarize the last order');

  const aiImageGeneration = sdk.createAiImageGenerationMessage({
    conversationId: 'conversation-ai',
    text: 'Generated product mockup',
    mediaAssetId: 'asset-ai-image-1',
    prompt: 'generate a matte product hero shot',
    model: 'gpt-image-1',
    status: 'completed',
  });
  const aiImageDecoded = sdk.decodeMessage(aiImageGeneration.body);
  assert.equal(aiImageDecoded.type, 'ai_image_generation');
  assert.equal(aiImageDecoded.content.model, 'gpt-image-1');

  const aiVideoGeneration = sdk.createAiVideoGenerationMessage({
    conversationId: 'conversation-ai',
    text: 'Generated promo teaser',
    mediaAssetId: 'asset-ai-video-1',
    prompt: 'generate a 10 second cinematic teaser',
    model: 'sora',
    status: 'processing',
    durationSeconds: 10,
  });
  const aiVideoDecoded = sdk.decodeMessage(aiVideoGeneration.body);
  assert.equal(aiVideoDecoded.type, 'ai_video_generation');
  assert.equal(aiVideoDecoded.content.durationSeconds, 10);

  const handoff = sdk.createAgentHandoffMessage({
    conversationId: 'conversation-ai',
    text: 'Escalating to billing specialist',
    fromAgentId: 'router',
    toAgentId: 'billing-specialist',
    reason: 'invoice_exception',
    status: 'pending',
  });
  const handoffDecoded = sdk.decodeMessage(handoff.body);
  assert.equal(handoffDecoded.type, 'agent_handoff');
  assert.equal(handoffDecoded.content.toAgentId, 'billing-specialist');

  const toolResult = sdk.createToolResultMessage({
    conversationId: 'conversation-ai',
    text: 'Fetched order totals',
    toolName: 'get-order-total',
    invocationId: 'tool-1',
    status: 'success',
    output: {
        total: 128.5,
      },
  });
  const toolResultDecoded = sdk.decodeMessage(toolResult.body);
  assert.equal(toolResultDecoded.type, 'tool_result');
  assert.equal(toolResultDecoded.content.toolName, 'get-order-total');

  const workflowEvent = sdk.createWorkflowEventMessage({
    conversationId: 'conversation-ai',
    text: 'Workflow advanced to fulfillment',
    workflowId: 'wf-1',
    eventName: 'state.changed',
    stage: 'fulfillment',
    status: 'success',
    data: {
      orderId: 'order-1001',
    },
  });
  const workflowDecoded = sdk.decodeMessage(workflowEvent.body);
  assert.equal(workflowDecoded.type, 'workflow_event');
  assert.equal(workflowDecoded.content.stage, 'fulfillment');
}

async function testSyncCatchUpProvidesContextAndAck() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
    authToken: 'token-1',
  });

  backendClient.realtime.listRealtimeEvents = async (params) => {
    calls.push({ method: 'realtime.listRealtimeEvents', params });
    return {
      deviceId: 'device-1',
      items: [
        createRealtimeEvent({
          realtimeSeq: 31,
          scopeId: 'conversation-1',
          eventType: 'message.created',
          payload: JSON.stringify({
            messageId: 'msg-31',
            conversationId: 'conversation-1',
            body: {
              text: 'hello catch up',
            },
          }),
        }),
      ],
      hasMore: false,
      nextAfterSeq: 31,
      ackedThroughSeq: 0,
      trimmedThroughSeq: 0,
    };
  };

  const batch = await sdk.sync.catchUp({ limit: 20 });
  assert.equal(batch.highestSequence, 31);
  assert.equal(batch.items.length, 1);
  assert.equal(batch.items[0].message.type, 'text');
  assert.equal(batch.items[0].conversationId, 'conversation-1');
  assert.equal(batch.items[0].sequence, 31);
  assert.equal(batch.items[0].source, 'catch_up');
  assert.equal(batch.items[0].receivedAt, '2026-04-15T12:00:00.000Z');
  assert.deepEqual(batch.items[0].sender, {
    principalId: 'user-1',
    deviceId: 'device-1',
  });
  assert.equal(typeof batch.items[0].ack, 'function');

  await batch.items[0].ack();

  assert.deepEqual(calls.slice(-2), [
    {
      method: 'realtime.listRealtimeEvents',
      params: {
        limit: 20,
      },
    },
    {
      method: 'realtime.ackRealtimeEvents',
      body: {
        ackedSeq: 31,
      },
    },
  ]);
}

async function testConnectProvidesContextFirstLiveApi() {
  const { backendClient } = createBackendClientStub();
  const socket = createFakeWebSocket();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
    authToken: 'token-1',
    webSocketFactory: () => socket,
  });
  const observed = [];

  const connectPromise = sdk.connect({
    deviceId: 'device-1',
    subscriptions: {
      conversations: ['conversation-1'],
      rtcSessions: ['rtc-1'],
    },
  });

  await flushMicrotasks();
  socket.open();
  socket.receiveJson({
    type: 'realtime.connected',
    tenantId: 'tenant-1',
    principalId: 'user-1',
    deviceId: 'device-1',
    actor: {
      id: 'user-1',
      kind: 'user',
    },
    sender: {
      principalId: 'user-1',
      deviceId: 'device-1',
      sessionId: 'session-1',
      senderId: 'user-1:device-1',
    },
    ackedThroughSeq: 0,
    trimmedThroughSeq: 0,
    latestRealtimeSeq: 0,
  });

  const live = await connectPromise;

  assert.equal(typeof live.messages?.on, 'function');
  assert.equal(typeof live.messages?.onConversation, 'function');
  assert.equal(typeof live.data?.on, 'function');
  assert.equal(typeof live.signals?.on, 'function');
  assert.equal(typeof live.events?.on, 'function');
  assert.equal(typeof live.lifecycle?.onStateChange, 'function');
  assert.equal(typeof live.lifecycle?.onError, 'function');

  live.messages.onConversation('conversation-1', (message, ctx) => {
    observed.push({
      type: message.type,
      sequence: ctx.sequence,
      source: ctx.source,
      conversationId: ctx.conversationId,
      summary: message.summary,
      receivedAt: ctx.receivedAt,
      sender: ctx.sender,
    });
  });

  socket.receiveJson({
    type: 'event.window',
    reason: 'push',
    requestId: null,
    window: {
      deviceId: 'device-1',
      items: [
        createRealtimeEvent({
          realtimeSeq: 41,
          eventType: 'message.created',
          payload: JSON.stringify({
            messageId: 'msg-41',
            conversationId: 'conversation-1',
            body: {
              text: 'hello live push',
              summary: 'Live greeting',
            },
          }),
        }),
      ],
      hasMore: false,
      nextAfterSeq: 41,
      ackedThroughSeq: 0,
      trimmedThroughSeq: 0,
    },
  });
  await Promise.resolve();

  assert.deepEqual(observed, [
    {
      type: 'text',
      sequence: 41,
      source: 'live',
      conversationId: 'conversation-1',
      summary: 'Live greeting',
      receivedAt: '2026-04-15T12:00:00.000Z',
      sender: {
        principalId: 'user-1',
        deviceId: 'device-1',
      },
    },
  ]);
}

async function testConnectExposesStateRawEventAndErrorHooks() {
  const { backendClient } = createBackendClientStub();
  const socket = createFakeWebSocket();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
    authToken: 'token-1',
    webSocketFactory: () => socket,
  });
  const states = [];
  const rawEvents = [];
  const errors = [];

  const connectPromise = sdk.connect({
    deviceId: 'device-1',
    subscriptions: {
      conversations: ['conversation-1'],
    },
  });

  await flushMicrotasks();
  socket.open();
  socket.receiveJson({
    type: 'realtime.connected',
    tenantId: 'tenant-1',
    principalId: 'user-1',
    deviceId: 'device-1',
    actor: {
      id: 'user-1',
      kind: 'user',
    },
    sender: {
      principalId: 'user-1',
      deviceId: 'device-1',
      sessionId: 'session-1',
      senderId: 'user-1:device-1',
    },
    ackedThroughSeq: 0,
    trimmedThroughSeq: 0,
    latestRealtimeSeq: 0,
  });

  const live = await connectPromise;

  assert.equal(typeof live.onMessage, 'undefined');
  assert.equal(typeof live.onConversationMessage, 'undefined');
  assert.equal(typeof live.onData, 'undefined');
  assert.equal(typeof live.onSignal, 'undefined');
  assert.equal(typeof live.onRawEvent, 'undefined');
  assert.equal(typeof live.onStateChange, 'undefined');
  assert.equal(typeof live.onError, 'undefined');

  live.lifecycle.onStateChange((state) => {
    states.push(state.status);
  });
  live.events.on((context) => {
    rawEvents.push({
      kind: context.kind,
      sequence: context.sequence,
      source: context.source,
    });
  });
  live.lifecycle.onError((context) => {
    errors.push({
      code: context.code,
      message:
        context.error instanceof Error
          ? context.error.message
          : String(context.error),
    });
  });

  socket.receiveJson({
    type: 'event.window',
    reason: 'push',
    requestId: null,
    window: {
      deviceId: 'device-1',
      items: [
        createRealtimeEvent({
          realtimeSeq: 52,
          eventType: 'message.created',
          payload: JSON.stringify({
            messageId: 'msg-52',
            conversationId: 'conversation-1',
            body: {
              text: 'observe raw event',
              summary: 'Raw event payload',
            },
          }),
        }),
      ],
      hasMore: false,
      nextAfterSeq: 52,
      ackedThroughSeq: 0,
      trimmedThroughSeq: 0,
    },
  });
  socket.receiveJson({
    type: 'error',
    requestId: 'req-1',
    code: 'pull_only',
    message: 'switch to pull only',
  });
  await Promise.resolve();

  live.disconnect(4001, 'manual-close');
  await Promise.resolve();

  assert.deepEqual(states, ['connected', 'error', 'closed']);
  assert.deepEqual(rawEvents, [
    {
      kind: 'message',
      sequence: 52,
      source: 'live',
    },
  ]);
  assert.deepEqual(errors, [
    {
      code: 'pull_only',
      message: 'switch to pull only',
    },
  ]);
}

async function testConnectFailsFastBeforeSocketWhenSessionResumePreflightFails() {
  const { backendClient } = createBackendClientStub();
  const socket = createFakeWebSocket();
  const sdk = new CrawChatSdkClient({
    backendClient,
    baseUrl: 'https://api.example.test',
    authToken: 'token-1',
    webSocketFactory: () => socket,
  });
  const resumeError = new Error('resume failed');

  backendClient.session.resume = async () => {
    throw resumeError;
  };

  const result = await Promise.race([
    sdk.connect({
      deviceId: 'device-1',
      subscriptions: {
        conversations: ['conversation-1'],
      },
    }).then(
      () => 'resolved',
      (error) => error,
    ),
    new Promise((resolve) => {
      setTimeout(() => resolve('timeout'), 20);
    }),
  ]);

  assert.equal(result, resumeError);
  assert.equal(socket.readyState, 0);
  assert.deepEqual(socket.sent, []);
}

function testConstructorErrorUsesPublicConfigLanguage() {
  assert.throws(
    () => {
      new CrawChatSdkClient({});
    },
    (error) => {
      assert.equal(error?.name, 'CrawChatSdkError');
      assert.equal(error?.code, 'backend_client_missing');
      assert.equal(error?.message, 'baseUrl or apiBaseUrl is required');
      assert.doesNotMatch(error?.message ?? '', /\bbackendClient\b/);
      return true;
    },
  );
}

async function testConnectMissingWebSocketUrlUsesPublicLanguage() {
  const { backendClient } = createBackendClientStub();
  const sdk = new CrawChatSdkClient({
    backendClient,
    authToken: 'token-1',
  });

  await assert.rejects(
    () => sdk.connect(),
    (error) => {
      assert.equal(error?.name, 'CrawChatSdkError');
      assert.equal(error?.code, 'websocket_url_required');
      assert.equal(
        error?.message,
        'websocketBaseUrl or connect({ url }) is required to establish realtime connectivity.',
      );
      assert.doesNotMatch(error?.message ?? '', /\breceiver\b/);
      return true;
    },
  );
}

async function testConnectDefaultWebSocketFactoryErrorUsesPublicLanguage() {
  const sdk = new CrawChatSdkClient({
    baseUrl: 'https://api.example.test',
    authToken: 'token-1',
  });

  await assert.rejects(
    () => sdk.connect(),
    (error) => {
      assert.equal(error?.name, 'CrawChatSdkError');
      assert.equal(error?.code, 'websocket_factory_required');
      assert.equal(
        error?.message,
        'The default WebSocket implementation cannot attach Authorization headers. Provide webSocketFactory for authenticated realtime connections.',
      );
      assert.doesNotMatch(error?.message ?? '', /\bcreateSocket\(/);
      assert.doesNotMatch(error?.message ?? '', /\breceiver\b/);
      return true;
    },
  );
}

async function testConnectMissingGlobalWebSocketUsesPublicLanguage() {
  const originalWebSocket = globalThis.WebSocket;
  const sdk = new CrawChatSdkClient({
    baseUrl: 'https://api.example.test',
  });

  try {
    globalThis.WebSocket = undefined;

    await assert.rejects(
      () => sdk.connect(),
      (error) => {
        assert.equal(error?.name, 'CrawChatSdkError');
        assert.equal(error?.code, 'websocket_factory_required');
        assert.equal(
          error?.message,
          'No global WebSocket implementation is available. Provide webSocketFactory to establish realtime connectivity in this runtime.',
        );
        assert.doesNotMatch(error?.message ?? '', /\bcreateSocket\(/);
        assert.doesNotMatch(error?.message ?? '', /\breceiver\b/);
        return true;
      },
    );
  } finally {
    globalThis.WebSocket = originalWebSocket;
  }
}

await testPublicExportsHideLegacySurface();
testDeclarationSurfaceUsesSdkNaming();
testInternalRealtimeDeclarationsUseEventOnlyShape();
testSynchronousClientConstructionAndDomains();
testAuthModuleManagesTokens();
testAuthModulePrefersClearAuthTokenWhenBackendSupportsIt();
await testPortalDomainProxiesGeneratedPortalHelpers();
await testTopLevelMessageApisCreateSendAndUpload();
await testStandaloneMediaUploadHelpersUsePresignedUploadSession();
testExpandedMessageFamilies();
await testSyncCatchUpProvidesContextAndAck();
await testConnectProvidesContextFirstLiveApi();
await testConnectExposesStateRawEventAndErrorHooks();
await testConnectFailsFastBeforeSocketWhenSessionResumePreflightFails();
testConstructorErrorUsesPublicConfigLanguage();
await testConnectMissingWebSocketUrlUsesPublicLanguage();
await testConnectDefaultWebSocketFactoryErrorUsesPublicLanguage();
await testConnectMissingGlobalWebSocketUsesPublicLanguage();

console.log('craw-chat composed sdk smoke tests passed');
