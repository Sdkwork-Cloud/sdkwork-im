import {
  createRtcAppHttpClient,
  createStandardRtcCallControllerStack,
  type RtcCallControllerSnapshot,
  type RtcDataSourceConfig,
  type RtcSignalingTransportLike,
} from '@sdkwork/rtc-sdk';
import type { ImSdkClient, RtcSession } from '@sdkwork/im-sdk';
import {
  buildSdkworkChatAppContextHeaders,
  getImSdkClientWithSession,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  resolveImSdkApiBaseUrl,
  type SdkworkChatSession,
} from '@sdkwork/clawchat-pc-core';

export type SdkworkCallType = 'voice' | 'video';

export type SdkworkCallState =
  | 'idle'
  | 'ringing'
  | 'connecting'
  | 'connected'
  | 'ended'
  | 'rejected'
  | 'errored';

export interface SdkworkCallSnapshot {
  state: SdkworkCallState;
  controllerState?: RtcCallControllerSnapshot['controllerState'];
  conversationId?: string;
  direction?: RtcCallControllerSnapshot['direction'];
  errorMessage?: string;
  isAudioMuted: boolean;
  isVideoMuted: boolean;
  participantId?: string;
  providerKey?: string;
  roomId?: string;
  rtcMode?: string;
  rtcSessionId?: string;
  targetName?: string;
  type?: SdkworkCallType;
}

export interface StartOutgoingCallOptions {
  conversationId: string;
  targetName: string;
  type: SdkworkCallType;
}

export interface RecoverRtcSessionOptions {
  targetName?: string;
  type?: SdkworkCallType;
}

export interface EndCallOptions {
  reason?: string;
}

export interface WatchIncomingCallsOptions {
  conversationIds: string[];
}

export interface CallService {
  acceptIncomingCall(): Promise<SdkworkCallSnapshot>;
  endCall(options?: EndCallOptions): Promise<void>;
  getSnapshot(): SdkworkCallSnapshot;
  recoverRtcSession(rtcSessionId: string, options?: RecoverRtcSessionOptions): Promise<SdkworkCallSnapshot>;
  rejectIncomingCall(options?: EndCallOptions): Promise<SdkworkCallSnapshot>;
  setAudioMuted(muted: boolean): Promise<SdkworkCallSnapshot>;
  setVideoMuted(muted: boolean): Promise<SdkworkCallSnapshot>;
  startOutgoingCall(options: StartOutgoingCallOptions): Promise<SdkworkCallSnapshot>;
  subscribe(handler: (snapshot: SdkworkCallSnapshot) => void): () => void;
  watchIncomingCalls(conversationIds: string[]): Promise<SdkworkCallSnapshot>;
}

const DEFAULT_RTC_PROVIDER_KEY = 'volcengine';

type SdkworkRtcStackLike = {
  callController: {
    acceptIncoming?(options: {
      autoPublish?: {
        audio?: boolean;
        video?: boolean;
      };
      conversationId?: string;
      participantId: string;
      roomId?: string;
      rtcMode?: string;
      rtcSessionId: string;
    }): Promise<RtcCallControllerSnapshot> | RtcCallControllerSnapshot;
    end?(options?: EndCallOptions): Promise<RtcCallControllerSnapshot> | RtcCallControllerSnapshot;
    getSnapshot(): RtcCallControllerSnapshot;
    onSnapshot(handler: (snapshot: RtcCallControllerSnapshot) => void): () => void;
    rejectIncoming?(options: {
      reason?: string;
      rtcSessionId: string;
    }): Promise<RtcCallControllerSnapshot> | RtcCallControllerSnapshot;
    replaceWatchedConversations?(conversationIds: readonly string[]): Promise<RtcCallControllerSnapshot> | RtcCallControllerSnapshot;
    startOutgoing?(options: {
      autoPublish?: {
        audio?: boolean;
        video?: boolean;
      };
      conversationId: string;
      initiatorDisplayName?: string;
      participantId: string;
      roomId?: string;
      rtcMode: string;
      rtcSessionId: string;
      signalingStreamId?: string;
    }): Promise<RtcCallControllerSnapshot>;
  };
  close(): Promise<void> | void;
  mediaClient: {
    muteAudio?(muted: boolean): Promise<unknown> | unknown;
    muteVideo?(muted: boolean): Promise<unknown> | unknown;
  };
};

type SdkworkCreateRtcStack = (
  options: Parameters<typeof createStandardRtcCallControllerStack>[0],
) => Promise<SdkworkRtcStackLike>;

export interface SdkworkCallServiceDependencies {
  createStack?: SdkworkCreateRtcStack;
  getClient?: (session: SdkworkChatSession | null) => ImSdkClient;
  readSession?: () => SdkworkChatSession | null;
}

function createIdleSnapshot(): SdkworkCallSnapshot {
  return {
    state: 'idle',
    isAudioMuted: false,
    isVideoMuted: false,
  };
}

function readEnvValue(key: string): string | undefined {
  const value = import.meta.env?.[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeIdSegment(value: string): string {
  return value
    .trim()
    .replace(/[^a-zA-Z0-9_-]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .slice(0, 48);
}

function createRuntimeId(prefix: string, stablePart: string): string {
  const randomPart =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `${prefix}-${normalizeIdSegment(stablePart) || 'conversation'}-${randomPart}`;
}

function resolveParticipantId(session: SdkworkChatSession | null): string {
  const candidate =
    session?.context?.userId
    ?? session?.user?.userId
    ?? session?.user?.id
    ?? session?.sessionId
    ?? session?.context?.sessionId;
  if (candidate) {
    return String(candidate);
  }
  throw new Error('SDKWork Chat login session does not include a user id for RTC.');
}

function resolveDeviceId(session: SdkworkChatSession | null, participantId: string): string {
  const sessionPart = session?.sessionId ?? session?.context?.sessionId ?? participantId;
  return `sdkwork-chat-pc-${normalizeIdSegment(String(sessionPart)) || normalizeIdSegment(participantId) || 'device'}`;
}

function toRtcMode(type: SdkworkCallType): string {
  return type === 'video' ? 'video' : 'voice';
}

function toCallType(rtcMode: string | undefined): SdkworkCallType {
  return rtcMode === 'video' || rtcMode === 'video_call' ? 'video' : 'voice';
}

function toRecoveredServiceState(state: RtcSession['state']): SdkworkCallState {
  switch (state) {
    case 'accepted':
      return 'connected';
    case 'rejected':
      return 'rejected';
    case 'ended':
      return 'ended';
    case 'started':
    default:
      return 'ringing';
  }
}

function isActiveRtcSession(session: RtcSession): boolean {
  return session.state === 'started' || session.state === 'accepted';
}

function toServiceState(controllerState: RtcCallControllerSnapshot['controllerState']): SdkworkCallState {
  switch (controllerState) {
    case 'connected':
      return 'connected';
    case 'connecting':
      return 'connecting';
    case 'ended':
      return 'ended';
    case 'rejected':
      return 'rejected';
    case 'errored':
      return 'errored';
    case 'incoming_ringing':
    case 'outgoing_ringing':
      return 'ringing';
    case 'watching':
    case 'idle':
    default:
      return 'idle';
  }
}

function toErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return typeof error === 'string' && error.trim().length > 0 ? error : 'RTC call failed.';
}

function createRtcDataSourceConfig(): RtcDataSourceConfig {
  const providerKey =
    readEnvValue('VITE_CRAW_CHAT_RTC_PROVIDER_KEY') ?? DEFAULT_RTC_PROVIDER_KEY;
  const providerUrl = readEnvValue('VITE_CRAW_CHAT_RTC_PROVIDER_URL');
  const volcengineAppId = readEnvValue('VITE_CRAW_CHAT_RTC_VOLCENGINE_APP_ID');
  const nativeConfig = volcengineAppId
    ? {
        appId: volcengineAppId,
      }
    : undefined;

  return {
    ...(providerKey ? { providerKey } : {}),
    ...(providerUrl ? { providerUrl } : {}),
    ...(nativeConfig ? { nativeConfig } : {}),
  };
}

function createRtcSignalingTransport(session: SdkworkChatSession | null): RtcSignalingTransportLike {
  const currentSession = session ?? readAppSdkSessionTokens();
  return createRtcAppHttpClient({
    baseUrl: resolveImSdkApiBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    headerProvider: () => buildSdkworkChatAppContextHeaders(readAppSdkSessionTokens() ?? currentSession),
  });
}

class SdkworkRtcCallService implements CallService {
  private readonly listeners = new Set<(snapshot: SdkworkCallSnapshot) => void>();
  private readonly createStack: SdkworkCreateRtcStack;
  private readonly getClient: (session: SdkworkChatSession | null) => ImSdkClient;
  private readonly readSession: () => SdkworkChatSession | null;
  private snapshot: SdkworkCallSnapshot = createIdleSnapshot();
  private stack: SdkworkRtcStackLike | null = null;
  private unsubscribeControllerSnapshot: (() => void) | undefined;
  private sequence = 0;

  constructor(dependencies: SdkworkCallServiceDependencies = {}) {
    this.createStack = dependencies.createStack ?? (async (options) => createStandardRtcCallControllerStack(options));
    this.getClient = dependencies.getClient ?? ((session) => getImSdkClientWithSession(session));
    this.readSession = dependencies.readSession ?? readAppSdkSessionTokens;
  }

  getSnapshot(): SdkworkCallSnapshot {
    return {
      ...this.snapshot,
    };
  }

  subscribe(handler: (snapshot: SdkworkCallSnapshot) => void): () => void {
    this.listeners.add(handler);
    handler(this.getSnapshot());
    return () => {
      this.listeners.delete(handler);
    };
  }

  async startOutgoingCall(options: StartOutgoingCallOptions): Promise<SdkworkCallSnapshot> {
    const sequence = ++this.sequence;
    const session = this.readSession();
    const participantId = resolveParticipantId(session);
    const rtcMode = toRtcMode(options.type);
    const rtcSessionId = createRuntimeId('rtc-pc', options.conversationId);
    const signalingStreamId = createRuntimeId('signal', rtcSessionId);
    const roomId = rtcSessionId;

    this.applySnapshot({
      ...createIdleSnapshot(),
      state: 'ringing',
      conversationId: options.conversationId,
      direction: 'outgoing',
      isAudioMuted: this.snapshot.isAudioMuted,
      isVideoMuted: options.type === 'voice' ? true : this.snapshot.isVideoMuted,
      participantId,
      roomId,
      rtcMode,
      rtcSessionId,
      targetName: options.targetName,
      type: options.type,
    });

    try {
      await this.closeStack();
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }

      const stack = await this.createStack({
        transport: createRtcSignalingTransport(session),
        deviceId: resolveDeviceId(session, participantId),
        watchConversationIds: [options.conversationId],
        dataSourceConfig: createRtcDataSourceConfig(),
      });

      if (sequence !== this.sequence) {
        await stack.close();
        return this.getSnapshot();
      }

      this.stack = stack;
      this.unsubscribeControllerSnapshot = stack.callController.onSnapshot((controllerSnapshot) => {
        this.applyControllerSnapshot(controllerSnapshot);
      });

      if (!stack.callController.startOutgoing) {
        throw new Error('RTC call controller does not support outgoing calls.');
      }

      const controllerSnapshot = await stack.callController.startOutgoing({
        rtcSessionId,
        conversationId: options.conversationId,
        rtcMode,
        roomId,
        participantId,
        signalingStreamId,
        initiatorDisplayName: session?.user?.displayName ?? session?.user?.name,
        autoPublish: {
          audio: !this.snapshot.isAudioMuted,
          video: options.type === 'video' && !this.snapshot.isVideoMuted,
        },
      });
      this.applyControllerSnapshot(controllerSnapshot);
      return this.getSnapshot();
    } catch (error) {
      if (sequence === this.sequence) {
        await this.closeStack();
        this.applySnapshot({
          ...this.snapshot,
          state: 'errored',
          errorMessage: toErrorMessage(error),
        });
      }
      return this.getSnapshot();
    }
  }

  async setAudioMuted(muted: boolean): Promise<SdkworkCallSnapshot> {
    if (this.stack?.mediaClient.muteAudio && this.snapshot.state === 'connected') {
      await this.stack.mediaClient.muteAudio(muted);
    }
    this.applySnapshot({
      ...this.snapshot,
      isAudioMuted: muted,
    });
    return this.getSnapshot();
  }

  async setVideoMuted(muted: boolean): Promise<SdkworkCallSnapshot> {
    if (this.stack?.mediaClient.muteVideo && this.snapshot.state === 'connected') {
      await this.stack.mediaClient.muteVideo(muted);
    }
    this.applySnapshot({
      ...this.snapshot,
      isVideoMuted: muted,
    });
    return this.getSnapshot();
  }

  async watchIncomingCalls(conversationIds: string[]): Promise<SdkworkCallSnapshot> {
    const normalizedConversationIds = [...new Set(
      conversationIds
        .map((conversationId) => String(conversationId).trim())
        .filter((conversationId) => conversationId.length > 0),
    )];

    if (this.hasActiveCall()) {
      return this.getSnapshot();
    }

    if (normalizedConversationIds.length === 0) {
      await this.closeStack();
      this.applySnapshot(createIdleSnapshot());
      return this.getSnapshot();
    }

    const session = this.readSession();
    const participantId = resolveParticipantId(session);

    try {
      if (this.stack?.callController.replaceWatchedConversations) {
        const controllerSnapshot = await this.stack.callController.replaceWatchedConversations(normalizedConversationIds);
        this.applyControllerSnapshot(controllerSnapshot);
        return this.getSnapshot();
      }

      await this.closeStack();
      const stack = await this.createStack({
        transport: createRtcSignalingTransport(session),
        deviceId: resolveDeviceId(session, participantId),
        watchConversationIds: normalizedConversationIds,
        dataSourceConfig: createRtcDataSourceConfig(),
      });
      this.stack = stack;
      this.unsubscribeControllerSnapshot = stack.callController.onSnapshot((controllerSnapshot) => {
        this.applyControllerSnapshot(controllerSnapshot);
      });
      this.applySnapshot({
        ...createIdleSnapshot(),
        controllerState: stack.callController.getSnapshot().controllerState,
        participantId,
      });
    } catch (error) {
      await this.closeStack();
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        errorMessage: toErrorMessage(error),
      });
    }

    return this.getSnapshot();
  }

  async acceptIncomingCall(): Promise<SdkworkCallSnapshot> {
    const stack = this.stack;
    const rtcSessionId = this.snapshot.rtcSessionId;
    if (!stack?.callController.acceptIncoming || !rtcSessionId) {
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        errorMessage: 'RTC incoming call is not available.',
      });
      return this.getSnapshot();
    }

    const session = this.readSession();
    const participantId = resolveParticipantId(session);
    const controllerSnapshot = await stack.callController.acceptIncoming({
      rtcSessionId,
      conversationId: this.snapshot.conversationId,
      rtcMode: this.snapshot.rtcMode,
      roomId: this.snapshot.roomId,
      participantId,
      autoPublish: {
        audio: !this.snapshot.isAudioMuted,
        video: this.snapshot.type === 'video' && !this.snapshot.isVideoMuted,
      },
    });
    this.applyControllerSnapshot(controllerSnapshot);
    return this.getSnapshot();
  }

  async rejectIncomingCall(options: EndCallOptions = {}): Promise<SdkworkCallSnapshot> {
    const stack = this.stack;
    const rtcSessionId = this.snapshot.rtcSessionId;
    if (!stack?.callController.rejectIncoming || !rtcSessionId) {
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        errorMessage: 'RTC incoming call is not available.',
      });
      return this.getSnapshot();
    }

    const controllerSnapshot = await stack.callController.rejectIncoming({
      rtcSessionId,
      reason: options.reason,
    });
    this.applyControllerSnapshot(controllerSnapshot);
    return this.getSnapshot();
  }

  async endCall(options: EndCallOptions = {}): Promise<void> {
    const stack = this.stack;
    ++this.sequence;
    if (!stack) {
      this.applySnapshot({
        ...this.snapshot,
        state: this.snapshot.state === 'idle' ? 'idle' : 'ended',
      });
      return;
    }

    try {
      const controllerSnapshot = stack.callController.getSnapshot();
      const hasActiveSession = Boolean(
        controllerSnapshot.rtcSessionId ?? controllerSnapshot.activeInvitation?.rtcSessionId,
      );
      if (
        hasActiveSession
        && controllerSnapshot.controllerState !== 'ended'
        && controllerSnapshot.controllerState !== 'rejected'
        && controllerSnapshot.controllerState !== 'idle'
        && stack.callController.end
      ) {
        await stack.callController.end({
          reason: options.reason,
        });
      }
    } catch (error) {
      this.applySnapshot({
        ...this.snapshot,
        state: 'errored',
        errorMessage: toErrorMessage(error),
      });
    } finally {
      await this.closeStack();
      this.applySnapshot({
        ...this.snapshot,
        state: this.snapshot.state === 'errored' ? 'errored' : 'ended',
      });
    }
  }

  async recoverRtcSession(
    rtcSessionId: string,
    options: RecoverRtcSessionOptions = {},
  ): Promise<SdkworkCallSnapshot> {
    const sequence = ++this.sequence;
    const session = this.readSession();
    const participantId = resolveParticipantId(session);

    try {
      const imClient = this.getClient(session);
      const rtcSession = await imClient.rtc.retrieve(rtcSessionId);
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }

      await this.closeStack();
      if (sequence !== this.sequence) {
        return this.getSnapshot();
      }

      const callType = options.type ?? toCallType(rtcSession.rtcMode);
      this.applySnapshot({
        ...createIdleSnapshot(),
        state: toRecoveredServiceState(rtcSession.state),
        ...(rtcSession.conversationId ? { conversationId: rtcSession.conversationId } : {}),
        isAudioMuted: this.snapshot.isAudioMuted,
        isVideoMuted: callType === 'voice' ? true : this.snapshot.isVideoMuted,
        participantId,
        ...(rtcSession.providerPluginId ? { providerKey: rtcSession.providerPluginId } : {}),
        roomId: rtcSession.providerSessionId ?? rtcSession.rtcSessionId,
        rtcMode: rtcSession.rtcMode,
        rtcSessionId: rtcSession.rtcSessionId,
        ...(options.targetName ? { targetName: options.targetName } : {}),
        type: callType,
      });

      if (!isActiveRtcSession(rtcSession) || !rtcSession.conversationId) {
        return this.getSnapshot();
      }

      const stack = await this.createStack({
        transport: createRtcSignalingTransport(session),
        deviceId: resolveDeviceId(session, participantId),
        watchConversationIds: [rtcSession.conversationId],
        dataSourceConfig: createRtcDataSourceConfig(),
      });

      if (sequence !== this.sequence) {
        await stack.close();
        return this.getSnapshot();
      }

      this.stack = stack;
      this.unsubscribeControllerSnapshot = stack.callController.onSnapshot((controllerSnapshot) => {
        this.applyControllerSnapshot(controllerSnapshot);
      });
      return this.getSnapshot();
    } catch (error) {
      if (sequence === this.sequence) {
        await this.closeStack();
        this.applySnapshot({
          ...this.snapshot,
          state: 'errored',
          errorMessage: toErrorMessage(error),
        });
      }
      return this.getSnapshot();
    }
  }

  private applyControllerSnapshot(controllerSnapshot: RtcCallControllerSnapshot): void {
    const activeInvitation = controllerSnapshot.activeInvitation;
    const rtcMode = controllerSnapshot.rtcMode ?? activeInvitation?.rtcMode ?? this.snapshot.rtcMode;
    const callType = toCallType(rtcMode);
    this.applySnapshot({
      ...this.snapshot,
      state: toServiceState(controllerSnapshot.controllerState),
      controllerState: controllerSnapshot.controllerState,
      conversationId:
        controllerSnapshot.conversationId
        ?? activeInvitation?.conversationId
        ?? this.snapshot.conversationId,
      direction: controllerSnapshot.direction ?? this.snapshot.direction,
      errorMessage:
        controllerSnapshot.lastError ? toErrorMessage(controllerSnapshot.lastError) : undefined,
      participantId: controllerSnapshot.participantId ?? this.snapshot.participantId,
      providerKey: controllerSnapshot.providerKey ?? this.snapshot.providerKey,
      roomId: controllerSnapshot.roomId ?? this.snapshot.roomId,
      rtcMode,
      rtcSessionId:
        controllerSnapshot.rtcSessionId
        ?? activeInvitation?.rtcSessionId
        ?? this.snapshot.rtcSessionId,
      targetName:
        controllerSnapshot.activeInvitation?.initiatorDisplayName
        ?? controllerSnapshot.activeInvitation?.initiatorId
        ?? this.snapshot.targetName,
      type: callType,
    });
  }

  private hasActiveCall(): boolean {
    return Boolean(
      this.snapshot.rtcSessionId
        && this.snapshot.controllerState !== 'watching'
        && this.snapshot.state !== 'idle'
        && this.snapshot.state !== 'ended'
        && this.snapshot.state !== 'rejected'
        && this.snapshot.state !== 'errored',
    );
  }

  private applySnapshot(snapshot: SdkworkCallSnapshot): void {
    this.snapshot = {
      ...snapshot,
    };
    for (const listener of this.listeners) {
      listener(this.getSnapshot());
    }
  }

  private async closeStack(): Promise<void> {
    const stack = this.stack;
    this.stack = null;
    this.unsubscribeControllerSnapshot?.();
    this.unsubscribeControllerSnapshot = undefined;
    if (stack) {
      await stack.close();
    }
  }
}

export function createSdkworkCallService(dependencies?: SdkworkCallServiceDependencies): CallService {
  return new SdkworkRtcCallService(dependencies);
}

export const callService = createSdkworkCallService();
