import type { NotaryTask } from '@sdkwork/im-pc-types';

import { SYSTEM_ASSIGNED_NOTARY_LABEL } from '../constants/notary';

const UNASSIGNED_NOTARY_LABELS = new Set([
  SYSTEM_ASSIGNED_NOTARY_LABEL,
  '系统分配',
  '未分配',
  'Unassigned',
]);

export function isNotaryAssigned(
  task: NotaryTask,
  localizedLabels: string[] = [],
): boolean {
  if (task.primaryNotaryMembershipId?.trim()) {
    return true;
  }

  const notaryName = task.notary?.trim();
  if (!notaryName) {
    return false;
  }

  const unassignedLabels = new Set([
    ...UNASSIGNED_NOTARY_LABELS,
    ...localizedLabels.filter(Boolean),
  ]);

  return !unassignedLabels.has(notaryName);
}
