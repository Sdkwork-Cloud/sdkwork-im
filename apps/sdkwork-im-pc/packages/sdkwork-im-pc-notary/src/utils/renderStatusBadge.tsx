import React from 'react';
import type { TFunction } from 'i18next';
import type { NotaryTask } from '@sdkwork/im-pc-types';

export function renderStatusBadge(status: NotaryTask['status'], t: TFunction): React.ReactNode {
  switch (status) {
    case 'PENDING_REVIEW':
      return (
        <span className="px-2 py-1 rounded bg-orange-500/20 text-orange-400 text-xs font-medium border border-orange-500/20">
          {t('status.pendingReview')}
        </span>
      );
    case 'PROCESSING':
      return (
        <span className="px-2 py-1 rounded bg-indigo-500/20 text-indigo-400 text-xs font-medium border border-indigo-500/20">
          {t('status.processing')}
        </span>
      );
    case 'COMPLETED':
      return (
        <span className="px-2 py-1 rounded bg-green-500/20 text-green-400 text-xs font-medium border border-green-500/20">
          {t('status.completed')}
        </span>
      );
    case 'REJECTED':
      return (
        <span className="px-2 py-1 rounded bg-red-500/20 text-red-400 text-xs font-medium border border-red-500/20">
          {t('status.rejected')}
        </span>
      );
    default:
      return null;
  }
}
