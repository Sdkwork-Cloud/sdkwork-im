import { StatusBadge } from '@sdkwork/ui-pc-react/components/ui';
import { useAdminI18n } from 'sdkwork-craw-chat-admin-core';

const ACTION_PROGRESS_KEYWORDS = [
  'saving',
  'updating',
  'creating',
  'deleting',
  'issuing',
  'reloading',
  'provisioning',
  'restoring',
  'revoking',
  'disabling',
  'activating',
];

const ACTION_SUCCESS_KEYWORDS = [
  'saved',
  'updated',
  'created',
  'deleted',
  'issued',
  'finished',
  'restored',
  'revoked',
  'disabled',
];

const ACTION_FAILURE_KEYWORDS = [
  'failed',
  'error',
  'expired',
  'unavailable',
  'not permitted',
  'rate limited',
];

function includesAny(normalized: string, keywords: string[]) {
  return keywords.some((keyword) => normalized.includes(keyword));
}

function compactStatusLabel(status: string) {
  const normalized = status.toLowerCase();

  if (normalized.includes('synchronized')) {
    return 'Live sync';
  }

  if (normalized.includes('refresh')) {
    return 'Refreshing';
  }

  if (normalized.includes('authenticate') || normalized.includes('awaiting') || normalized.includes('signed out')) {
    return 'Awaiting sign-in';
  }

  if (includesAny(normalized, ACTION_FAILURE_KEYWORDS)) {
    return 'Action required';
  }

  if (includesAny(normalized, ACTION_PROGRESS_KEYWORDS)) {
    return 'Applying change';
  }

  if (includesAny(normalized, ACTION_SUCCESS_KEYWORDS)) {
    return 'Change applied';
  }

  return status;
}

function resolveStatusToken(status: string) {
  const normalized = status.toLowerCase();

  if (includesAny(normalized, ACTION_FAILURE_KEYWORDS)) {
    return 'pending';
  }

  if (normalized.includes('synchronized') || includesAny(normalized, ACTION_SUCCESS_KEYWORDS)) {
    return 'live';
  }

  if (
    normalized.includes('refresh')
    || normalized.includes('loading')
    || includesAny(normalized, ACTION_PROGRESS_KEYWORDS)
  ) {
    return 'syncing';
  }

  if (normalized.includes('authenticate') || normalized.includes('awaiting') || normalized.includes('signed out')) {
    return 'pending';
  }

  return 'active';
}

export function ShellStatus({ status }: { status: string }) {
  const { t } = useAdminI18n();

  return (
    <div className="whitespace-nowrap">
      <StatusBadge
        label={t(compactStatusLabel(status))}
        showIcon
        status={resolveStatusToken(status)}
      />
    </div>
  );
}
