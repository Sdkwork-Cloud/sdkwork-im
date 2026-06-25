import type { CourseOperationResult } from '@sdkwork/course-app-sdk';

export function asRecord(value: unknown): Record<string, unknown> | null {
  if (value == null || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }
  return value as Record<string, unknown>;
}

export function readOptionalString(
  record: Record<string, unknown>,
  ...keys: string[]
): string | undefined {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

export function readString(
  record: Record<string, unknown>,
  ...keys: string[]
): string {
  return readOptionalString(record, ...keys) ?? '';
}

export function readNumber(
  record: Record<string, unknown>,
  ...keys: string[]
): number | undefined {
  for (const key of keys) {
    const value = record[key];
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return undefined;
}

export function extractCoursePayload(payload: unknown): Record<string, unknown> | null {
  const record = asRecord(payload);
  if (!record) {
    return null;
  }
  if ('data' in record) {
    const data = record.data;
    const dataRecord = asRecord(data);
    if (dataRecord) {
      return dataRecord;
    }
    if (Array.isArray(data)) {
      return { items: data };
    }
  }
  return record;
}

export function extractCourseItems(payload: unknown): Record<string, unknown>[] {
  const dataRecord = extractCoursePayload(payload);
  if (!dataRecord) {
    return [];
  }
  if (Array.isArray(dataRecord.items)) {
    return dataRecord.items as Record<string, unknown>[];
  }
  if (Array.isArray(dataRecord.courses)) {
    return dataRecord.courses as Record<string, unknown>[];
  }
  if (Array.isArray(dataRecord.sections)) {
    return dataRecord.sections as Record<string, unknown>[];
  }
  if (Array.isArray(dataRecord.lessons)) {
    return dataRecord.lessons as Record<string, unknown>[];
  }
  if (Array.isArray(dataRecord.comments)) {
    return dataRecord.comments as Record<string, unknown>[];
  }
  if (Array.isArray(dataRecord.liveSessions)) {
    return dataRecord.liveSessions as Record<string, unknown>[];
  }
  return [];
}

export function extractCourseEntity(payload: unknown): Record<string, unknown> | null {
  const dataRecord = extractCoursePayload(payload);
  if (!dataRecord) {
    return null;
  }
  if (Array.isArray(dataRecord.items) || Array.isArray(dataRecord.courses)) {
    return null;
  }
  return dataRecord;
}

export function courseOperationData(
  payload: CourseOperationResult | unknown,
): Record<string, unknown> | null {
  return extractCoursePayload(payload);
}
