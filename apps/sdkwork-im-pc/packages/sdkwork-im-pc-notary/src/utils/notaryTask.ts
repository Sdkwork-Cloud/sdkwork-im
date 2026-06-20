import type { NotaryTask } from '@sdkwork/im-pc-types';

export function getNotaryTaskDisplayNo(task: NotaryTask): string {
  return task.caseNo ?? task.caseId ?? task.id;
}

export function generateClientId(prefix = 'notary'): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return `${prefix}-${crypto.randomUUID()}`;
  }
  return `${prefix}-${Date.now().toString(36)}`;
}
