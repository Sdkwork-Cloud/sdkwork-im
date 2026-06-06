import type { ImSdkClient } from '@sdkwork/im-sdk';
import { getImSdkClientWithSession } from '@sdkwork/clawchat-pc-core';
import { callService, type CallService, type SdkworkCallSnapshot, type SdkworkCallType } from './CallService';
import { chatService, type ChatDeviceSyncResult, type ChatService } from './ChatService';
import { contactService, type ContactDeviceSyncResult, type ContactService } from './ContactService';
import {
  parseDeviceSyncPayload,
  resolveSdkworkChatPcDeviceId,
  retrieveDeviceSyncFeedWindow,
  toDeviceSyncRecord,
  type DeviceSyncFeedEntry,
} from './DeviceSyncFeedService';
import { groupService, type GroupMemberSyncChange, type GroupService } from './GroupService';

export interface ImStartupSyncOptions {
  deviceId?: string;
}

export interface RecoveredRtcSessionResult {
  rtcSessionId: string;
  snapshot: SdkworkCallSnapshot;
}

export interface ImStartupSyncError {
  stage: 'chat' | 'contacts' | 'groups' | 'rtc';
  message: string;
}

export interface ImStartupSyncResult {
  chat?: ChatDeviceSyncResult;
  contacts?: ContactDeviceSyncResult;
  deviceId: string;
  errors: ImStartupSyncError[];
  groups?: GroupMemberSyncChange[];
  recoveredRtcSessions: RecoveredRtcSessionResult[];
}

export interface ImSyncCoordinatorService {
  syncStartup(options?: ImStartupSyncOptions): Promise<ImStartupSyncResult>;
}

export interface ImSyncCoordinatorServiceDependencies {
  callService?: Pick<CallService, 'recoverRtcSession'>;
  chatService?: Pick<ChatService, 'syncOfflineMessages'>;
  contactService?: Pick<ContactService, 'syncContactsFromDeviceFeed'>;
  getClient?: () => ImSdkClient;
  groupService?: Pick<GroupService, 'syncGroupMembersFromDeviceFeed'>;
}

const RTC_DEVICE_SYNC_NAMESPACE = 'rtc';

function toErrorMessage(error: unknown): string {
  if (error instanceof Error && error.message) {
    return error.message;
  }
  return typeof error === 'string' && error.trim().length > 0 ? error : 'IM startup sync failed';
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function parseJsonRecord(value: unknown): Record<string, unknown> {
  if (typeof value !== 'string' || value.trim().length === 0) {
    return {};
  }

  try {
    return toDeviceSyncRecord(JSON.parse(value));
  } catch {
    return {};
  }
}

function resolveCallType(value: unknown): SdkworkCallType | undefined {
  return value === 'video' || value === 'video_call' ? 'video' : value === 'voice' ? 'voice' : undefined;
}

function extractRtcHintsFromPayload(payload: Record<string, unknown>): Array<{
  rtcMode?: string;
  rtcSessionId: string;
}> {
  const directRtcSessionId = pickString(payload.rtcSessionId, payload.rtc_session_id);
  const directRtcMode = pickString(payload.rtcMode, payload.rtc_mode);
  const results = new Map<string, { rtcMode?: string; rtcSessionId: string }>();
  if (directRtcSessionId) {
    results.set(directRtcSessionId, { rtcMode: directRtcMode, rtcSessionId: directRtcSessionId });
  }

  const body = toDeviceSyncRecord(payload.body);
  const parts = Array.isArray(body.parts) ? body.parts : [];
  for (const partValue of parts) {
    const part = toDeviceSyncRecord(partValue);
    if (part.kind !== 'signal') {
      continue;
    }
    const signalPayload = parseJsonRecord(part.payload);
    const rtcSessionId = pickString(
      signalPayload.rtcSessionId,
      signalPayload.rtc_session_id,
      part.rtcSessionId,
    );
    if (!rtcSessionId) {
      continue;
    }
    const rtcMode = pickString(signalPayload.rtcMode, signalPayload.rtc_mode, directRtcMode);
    results.set(rtcSessionId, {
      ...results.get(rtcSessionId),
      ...(rtcMode ? { rtcMode } : {}),
      rtcSessionId,
    });
  }

  return [...results.values()];
}

function extractRecoverableRtcSessions(entry: DeviceSyncFeedEntry): Array<{
  rtcMode?: string;
  rtcSessionId: string;
}> {
  if (entry.originEventType !== 'message.posted') {
    return [];
  }

  const payload = parseDeviceSyncPayload(entry);
  const hints = extractRtcHintsFromPayload(payload);
  if (hints.length > 0) {
    return hints;
  }

  return [];
}

class SdkworkImSyncCoordinatorService implements ImSyncCoordinatorService {
  private readonly callService: Pick<CallService, 'recoverRtcSession'>;
  private readonly chatService: Pick<ChatService, 'syncOfflineMessages'>;
  private readonly contactService: Pick<ContactService, 'syncContactsFromDeviceFeed'>;
  private readonly getClient: () => ImSdkClient;
  private readonly groupService: Pick<GroupService, 'syncGroupMembersFromDeviceFeed'>;
  private readonly rtcDeviceSyncAfterSeq = new Map<string, number>();

  constructor(dependencies: ImSyncCoordinatorServiceDependencies = {}) {
    this.callService = dependencies.callService ?? callService;
    this.chatService = dependencies.chatService ?? chatService;
    this.contactService = dependencies.contactService ?? contactService;
    this.getClient = dependencies.getClient ?? getImSdkClientWithSession;
    this.groupService = dependencies.groupService ?? groupService;
  }

  async syncStartup(options: ImStartupSyncOptions = {}): Promise<ImStartupSyncResult> {
    const deviceId = options.deviceId ?? resolveSdkworkChatPcDeviceId();
    const result: ImStartupSyncResult = {
      deviceId,
      errors: [],
      recoveredRtcSessions: [],
    };

    try {
      result.chat = await this.chatService.syncOfflineMessages(deviceId);
    } catch (error) {
      result.errors.push({ stage: 'chat', message: toErrorMessage(error) });
    }

    try {
      result.contacts = await this.contactService.syncContactsFromDeviceFeed(deviceId);
    } catch (error) {
      result.errors.push({ stage: 'contacts', message: toErrorMessage(error) });
    }

    try {
      result.groups = await this.groupService.syncGroupMembersFromDeviceFeed(deviceId);
    } catch (error) {
      result.errors.push({ stage: 'groups', message: toErrorMessage(error) });
    }

    try {
      result.recoveredRtcSessions = await this.recoverRtcSessionsFromDeviceFeed(deviceId);
    } catch (error) {
      result.errors.push({ stage: 'rtc', message: toErrorMessage(error) });
    }

    return result;
  }

  private async recoverRtcSessionsFromDeviceFeed(deviceId: string): Promise<RecoveredRtcSessionResult[]> {
    const window = await retrieveDeviceSyncFeedWindow(
      this.getClient(),
      RTC_DEVICE_SYNC_NAMESPACE,
      deviceId,
      this.rtcDeviceSyncAfterSeq,
    );
    const recovered: RecoveredRtcSessionResult[] = [];
    const seenRtcSessionIds = new Set<string>();

    for (const entry of window.entries) {
      for (const hint of extractRecoverableRtcSessions(entry)) {
        if (seenRtcSessionIds.has(hint.rtcSessionId)) {
          continue;
        }
        seenRtcSessionIds.add(hint.rtcSessionId);
        const snapshot = await this.callService.recoverRtcSession(hint.rtcSessionId, {
          ...(resolveCallType(hint.rtcMode) ? { type: resolveCallType(hint.rtcMode) } : {}),
        });
        recovered.push({
          rtcSessionId: hint.rtcSessionId,
          snapshot,
        });
      }
    }

    return recovered;
  }
}

export function createSdkworkImSyncCoordinatorService(
  dependencies?: ImSyncCoordinatorServiceDependencies,
): ImSyncCoordinatorService {
  return new SdkworkImSyncCoordinatorService(dependencies);
}

export const imSyncCoordinatorService = createSdkworkImSyncCoordinatorService();
