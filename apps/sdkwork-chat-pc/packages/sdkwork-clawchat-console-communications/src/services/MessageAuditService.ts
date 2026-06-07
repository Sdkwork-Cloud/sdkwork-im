import { getBackendSdkClientWithSession } from '@sdkwork/clawchat-pc-core';

export interface AuditMessage {
  id: string;
  time: string;
  sender: string;
  receiver: string;
  snippet: string;
  alert: boolean;
}

export interface GetAuditMessagesResponse {
  data: AuditMessage[];
  total: number;
}

type UnknownRecord = Record<string, unknown>;

function asRecord(value: unknown): UnknownRecord {
  return value && typeof value === 'object' && !Array.isArray(value) ? value as UnknownRecord : {};
}

function asRecordArray(value: unknown): UnknownRecord[] {
  return Array.isArray(value) ? value.map(asRecord).filter((item) => Object.keys(item).length > 0) : [];
}

function readString(record: UnknownRecord, keys: string[], fallback = ''): string {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim()) {
      return value.trim();
    }
    if (typeof value === 'number' && Number.isFinite(value)) {
      return String(value);
    }
  }
  return fallback;
}

function readBoolean(record: UnknownRecord, keys: string[], fallback = false): boolean {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'boolean') {
      return value;
    }
    if (typeof value === 'string' && value.trim()) {
      return ['1', 'true', 'yes', 'warning', 'critical', 'failed', 'error']
        .includes(value.trim().toLowerCase());
    }
  }
  return fallback;
}

function normalizeAuditMessage(record: UnknownRecord, index: number): AuditMessage {
  const payload = readString(record, ['payload', 'summary', 'message', 'details']);
  const action = readString(record, ['action', 'eventType', 'type'], 'audit.record');
  const aggregateType = readString(record, ['aggregateType'], 'system');
  const aggregateId = readString(record, ['aggregateId'], '');
  return {
    alert: readBoolean(record, ['alert', 'sensitive', 'violated'], action.toLowerCase().includes('fail')),
    id: readString(record, ['recordId', 'id'], `audit-${index + 1}`),
    receiver: aggregateId ? `${aggregateType}:${aggregateId}` : aggregateType,
    sender: readString(record, ['actorId', 'createdBy', 'userId', 'tenantId'], 'system'),
    snippet: payload || action,
    time: readString(record, ['recordedAt', 'createdAt', 'time'], ''),
  };
}

class MessageAuditService {
  async getMessages(params: { page: number; pageSize: number; search?: string }): Promise<GetAuditMessagesResponse> {
    const backend = getBackendSdkClientWithSession();
    const response = await backend.audit.records.list();
    let filtered = asRecordArray(asRecord(response).items)
      .map(normalizeAuditMessage);

    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter((message) =>
        message.sender.toLowerCase().includes(q)
        || message.receiver.toLowerCase().includes(q)
        || message.snippet.toLowerCase().includes(q),
      );
    }

    const start = (params.page - 1) * params.pageSize;
    const end = start + params.pageSize;

    return {
      data: filtered.slice(start, end),
      total: filtered.length,
    };
  }
}

export const messageAuditService = new MessageAuditService();
