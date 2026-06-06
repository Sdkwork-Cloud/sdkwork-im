import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkCallService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/CallService';

const calls: Array<{
  method: string;
  rtcSessionId?: string;
}> = [];

const fakeClient = {
  rtc: {
    async retrieve(rtcSessionId: string) {
      calls.push({ method: 'rtc.retrieve', rtcSessionId });
      return {
        tenantId: 'tenant-1',
        rtcSessionId,
        conversationId: 'conversation-rtc-1',
        rtcMode: 'voice',
        initiatorId: 'u_bob',
        providerPluginId: 'volcengine',
        providerSessionId: 'room-provider-1',
        startedAt: '2026-06-04T10:00:00.000Z',
        state: 'accepted',
      };
    },
    async create() {
      calls.push({ method: 'rtc.create' });
      throw new Error('recoverRtcSession must not create a duplicate RTC session');
    },
    async invite() {
      calls.push({ method: 'rtc.invite' });
      throw new Error('recoverRtcSession must not invite a duplicate RTC session');
    },
  },
} as unknown as ImSdkClient;

async function main(): Promise<void> {
  const service = createSdkworkCallService({
    getClient: () => fakeClient,
    readSession: () => ({
      authToken: 'auth-token-1',
      sessionId: 'session-1',
      context: {
        appId: 'app-1',
        tenantId: 'tenant-1',
        userId: 'u_alice',
        sessionId: 'session-1',
        environment: 'dev',
        deploymentMode: 'local',
        authLevel: 'password',
        dataScope: [],
        permissionScope: [],
      },
      user: {
        userId: 'u_alice',
        displayName: 'Alice',
      },
    }),
    createStack: async (options) => {
      calls.push({ method: 'rtc.stack.create' });
      assert.deepEqual(options.watchConversationIds, ['conversation-rtc-1']);
      return {
        callController: {
          getSnapshot() {
            return {
              controllerState: 'watching',
              state: 'idle',
              watchedConversationIds: ['conversation-rtc-1'],
            };
          },
          onSnapshot() {
            calls.push({ method: 'rtc.stack.onSnapshot' });
            return () => undefined;
          },
        },
        mediaClient: {
          async muteAudio() {
            return undefined;
          },
          async muteVideo() {
            return undefined;
          },
        },
        async close() {
          calls.push({ method: 'rtc.stack.close' });
        },
      };
    },
  });

  assert.equal(typeof service.recoverRtcSession, 'function');

  const snapshot = await service.recoverRtcSession('rtc-recover-1', {
    targetName: 'Bob',
  });

  assert.deepEqual(calls, [
    { method: 'rtc.retrieve', rtcSessionId: 'rtc-recover-1' },
    { method: 'rtc.stack.create' },
    { method: 'rtc.stack.onSnapshot' },
  ]);
  assert.deepEqual(snapshot, {
    state: 'connected',
    conversationId: 'conversation-rtc-1',
    isAudioMuted: false,
    isVideoMuted: true,
    participantId: 'u_alice',
    providerKey: 'volcengine',
    roomId: 'room-provider-1',
    rtcMode: 'voice',
    rtcSessionId: 'rtc-recover-1',
    targetName: 'Bob',
    type: 'voice',
  });

  console.log('sdkwork-chat-pc RTC session recovery contract passed');
}

void main();
