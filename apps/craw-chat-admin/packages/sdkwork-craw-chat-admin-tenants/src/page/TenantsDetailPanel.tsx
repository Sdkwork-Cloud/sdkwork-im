import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  InlineAlert,
  StatusBadge,
} from '@sdkwork/ui-pc-react';
import { useAdminI18n } from 'sdkwork-craw-chat-admin-core';

import type { TenantDirectoryRow } from './shared';

type TenantsDetailPanelProps = {
  selectedTenant: TenantDirectoryRow;
};

export function TenantsDetailPanel({ selectedTenant }: TenantsDetailPanelProps) {
  const { formatNumber, t } = useAdminI18n();

  return (
    <div className="space-y-4">
      <div className="grid gap-3 text-sm text-[var(--sdk-color-text-secondary)] sm:grid-cols-2 xl:grid-cols-4">
        <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-4">
          <div className="text-xs uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {t('Projects')}
          </div>
          <div className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">
            {formatNumber(selectedTenant.projectCount)}
          </div>
        </div>
        <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-4">
          <div className="text-xs uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {t('Portal users')}
          </div>
          <div className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">
            {formatNumber(selectedTenant.portalUserCount)}
          </div>
        </div>
        <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-4">
          <div className="text-xs uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {t('Active keys')}
          </div>
          <div className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">
            {formatNumber(selectedTenant.activeApiKeyCount)}
          </div>
        </div>
        <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-4">
          <div className="text-xs uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {t('Requests')}
          </div>
          <div className="mt-2 text-xl font-semibold text-[var(--sdk-color-text-primary)]">
            {formatNumber(selectedTenant.requestCount)}
          </div>
        </div>
      </div>

      <Card>
        <CardHeader>
          <div className="flex items-start justify-between gap-3">
            <CardTitle className="text-base">{t('Workspace posture')}</CardTitle>
            <StatusBadge
              showIcon
              status={selectedTenant.canIssueApiKey ? t('Ready') : t('Incomplete')}
              variant={selectedTenant.canIssueApiKey ? 'success' : 'warning'}
            />
          </div>
          <CardDescription>
            {t('Active projects, portal users, and live access key coverage for the selected tenant.')}
          </CardDescription>
        </CardHeader>
        <CardContent className="grid gap-4 text-sm sm:grid-cols-2">
          <div className="space-y-1">
            <div className="text-[var(--sdk-color-text-muted)]">{t('Tenant')}</div>
            <div className="font-medium text-[var(--sdk-color-text-primary)]">
              {selectedTenant.name}
            </div>
            <div className="text-[var(--sdk-color-text-secondary)]">{selectedTenant.id}</div>
          </div>
          <div className="space-y-1">
            <div className="text-[var(--sdk-color-text-muted)]">{t('Project footprint')}</div>
            <div className="font-medium text-[var(--sdk-color-text-primary)]">
              {selectedTenant.projectSummary}
            </div>
            <div className="text-[var(--sdk-color-text-secondary)]">
              {t('{count} projects attached', {
                count: formatNumber(selectedTenant.projectCount),
              })}
            </div>
          </div>
          <div className="space-y-1">
            <div className="text-[var(--sdk-color-text-muted)]">{t('Gateway coverage')}</div>
            <div className="font-medium text-[var(--sdk-color-text-primary)]">
              {t('{active} active / {total} total', {
                active: formatNumber(selectedTenant.activeApiKeyCount),
                total: formatNumber(selectedTenant.apiKeyCount),
              })}
            </div>
            <div className="text-[var(--sdk-color-text-secondary)]">
              {selectedTenant.environmentSummary}
            </div>
          </div>
          <div className="space-y-1">
            <div className="text-[var(--sdk-color-text-muted)]">{t('Traffic footprint')}</div>
            <div className="font-medium text-[var(--sdk-color-text-primary)]">
              {t('{count} requests', { count: formatNumber(selectedTenant.requestCount) })}
            </div>
            <div className="text-[var(--sdk-color-text-secondary)]">
              {t('{count} tokens', { count: formatNumber(selectedTenant.tokenCount) })}
            </div>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('Linked projects')}</CardTitle>
          <CardDescription>
            {t('The busiest workspace environments stay visible here so operators can judge ownership, traffic, and access key coverage before opening another dialog.')}
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-3">
          {selectedTenant.projectRecords.length ? (
            selectedTenant.projectRecords.slice(0, 3).map((project) => (
              <div
                className="rounded-2xl border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-4"
                key={project.id}
              >
                <div className="flex flex-wrap items-center justify-between gap-3">
                  <div>
                    <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">
                      {project.name}
                    </div>
                    <div className="mt-1 text-xs text-[var(--sdk-color-text-secondary)]">
                      {project.id}
                    </div>
                  </div>
                  <div className="text-xs uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
                    {t('{count} active / {total} total', {
                      count: formatNumber(project.activeApiKeyCount),
                      total: formatNumber(project.apiKeyCount),
                    })}
                  </div>
                </div>
                <div className="mt-3 grid gap-3 text-sm text-[var(--sdk-color-text-secondary)] sm:grid-cols-3">
                  <div>{t('{count} requests', { count: formatNumber(project.requestCount) })}</div>
                  <div>{t('{count} usage units', { count: formatNumber(project.usageUnits) })}</div>
                  <div>{t('{count} tokens', { count: formatNumber(project.tokenCount) })}</div>
                </div>
              </div>
            ))
          ) : (
            <div className="rounded-2xl border border-dashed border-[var(--sdk-color-border-default)] p-4 text-sm text-[var(--sdk-color-text-secondary)]">
              {t('No workspace environments are linked to this tenant yet.')}
            </div>
          )}
        </CardContent>
      </Card>

      <InlineAlert
        description={
          selectedTenant.canIssueApiKey
            ? t('At least one workspace environment exists, so live access key issuance can proceed immediately.')
            : t('Issue live access keys only after at least one workspace environment exists for the selected tenant.')
        }
        showIcon
        title={selectedTenant.canIssueApiKey ? t('Key issuance ready') : t('Key issuance guardrail')}
        tone="info"
      />
    </div>
  );
}
