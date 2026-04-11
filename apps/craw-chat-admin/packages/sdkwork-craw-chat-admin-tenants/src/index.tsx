import { useDeferredValue, useEffect, useState } from 'react';
import type { ChangeEvent, FormEvent } from 'react';
import {
  Button,
  Card,
  CardContent,
  Input,
  Label,
  StatusBadge,
  type DataTableColumn,
} from '@sdkwork/ui-pc-react';
import {
  AdminActionChip,
  AdminGuidanceList,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
  useAdminWorkbench,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

import { ApiKeyDialog } from './page/ApiKeyDialog';
import { PlaintextApiKeyDialog } from './page/PlaintextApiKeyDialog';
import { ProjectDialog } from './page/ProjectDialog';
import { TenantDialog } from './page/TenantDialog';
import { TenantsDetailDrawer } from './page/TenantsDetailDrawer';
import { TenantsRegistrySection } from './page/TenantsRegistrySection';
import {
  ConfirmActionDialog,
  buildTenantDirectoryRows,
  createApiKeyDraft,
  defaultProjectId,
  defaultTenantId,
  parseApiKeyExpiryInputValue,
  resolveTenantSelectionLabel,
  type ApiKeyDraft,
  type RevealedApiKey,
  type TenantDirectoryRow,
} from './page/shared';

export function TenantsPage({ snapshot }: AdminPageProps) {
  const {
    handleCreateApiKey,
    handleDeleteTenant,
    handleSaveProject,
    handleSaveTenant,
    status,
  } = useAdminWorkbench();
  const { formatNumber, t } = useAdminI18n();
  const [tenantDraft, setTenantDraft] = useState({ id: '', name: '' });
  const [projectDraft, setProjectDraft] = useState({
    tenant_id: defaultTenantId(snapshot),
    id: '',
    name: '',
  });
  const [apiKeyDraft, setApiKeyDraft] = useState<ApiKeyDraft>(() =>
    createApiKeyDraft(snapshot),
  );
  const [search, setSearch] = useState('');
  const [selectedTenantId, setSelectedTenantId] = useState<string | null>(null);
  const [isDetailDrawerOpen, setIsDetailDrawerOpen] = useState(false);
  const [isTenantDialogOpen, setIsTenantDialogOpen] = useState(false);
  const [isProjectDialogOpen, setIsProjectDialogOpen] = useState(false);
  const [isApiKeyDialogOpen, setIsApiKeyDialogOpen] = useState(false);
  const [revealedApiKey, setRevealedApiKey] = useState<RevealedApiKey>(null);
  const [pendingDelete, setPendingDelete] = useState<{
    id: string;
    label: string;
  } | null>(null);
  const deferredQuery = useDeferredValue(search.trim().toLowerCase());

  const tenantRows = buildTenantDirectoryRows(snapshot, deferredQuery);
  const selectedTenant = tenantRows.find((tenant) => tenant.id === selectedTenantId) ?? null;
  const projectPreviewId =
    projectDraft.id || defaultProjectId(snapshot, projectDraft.tenant_id);
  const selectedProjectUsage = snapshot.usageSummary.projects.find(
    (project) => project.project_id === projectPreviewId,
  );
  const selectedProjectBilling = snapshot.billingSummary.projects.find(
    (project) => project.project_id === projectPreviewId,
  );
  const selectedProjectTokens = snapshot.usageRecords
    .filter((record) => record.project_id === projectPreviewId)
    .reduce((sum, record) => sum + record.total_tokens, 0);
  const totalProjects = tenantRows.reduce((sum, tenant) => sum + tenant.projectCount, 0);
  const totalPortalUsers = tenantRows.reduce(
    (sum, tenant) => sum + tenant.portalUserCount,
    0,
  );
  const activeApiKeyCount = tenantRows.reduce(
    (sum, tenant) => sum + tenant.activeApiKeyCount,
    0,
  );
  const totalRequests = tenantRows.reduce(
    (sum, tenant) => sum + tenant.requestCount,
    0,
  );
  const availableApiKeyProjects = snapshot.projects.filter(
    (project) => project.tenant_id === apiKeyDraft.tenant_id,
  );
  const columns: DataTableColumn<TenantDirectoryRow>[] = [
    {
      id: 'tenant',
      header: t('Tenant'),
      cell: (tenant: TenantDirectoryRow) => (
        <div className="space-y-1">
          <div className="font-medium text-[var(--sdk-color-text-primary)]">
            {tenant.name}
          </div>
          <div className="text-sm text-[var(--sdk-color-text-secondary)]">
            {t('Tenant ID: {id}', { id: tenant.id })}
          </div>
        </div>
      ),
    },
    {
      id: 'projects',
      header: t('Workspaces'),
      cell: (tenant: TenantDirectoryRow) => (
        <div className="space-y-1 text-sm text-[var(--sdk-color-text-secondary)]">
          <div>{t('{count} attached', { count: formatNumber(tenant.projectCount) })}</div>
          <div>{tenant.projectSummary}</div>
        </div>
      ),
    },
    {
      id: 'portal-users',
      align: 'right',
      header: t('Portal users'),
      cell: (tenant: TenantDirectoryRow) => formatNumber(tenant.portalUserCount),
      width: 120,
    },
    {
      id: 'gateway',
      header: t('Access posture'),
      cell: (tenant: TenantDirectoryRow) => (
        <div className="space-y-1 text-sm text-[var(--sdk-color-text-secondary)]">
          <div>
            {t('{active} active / {total} total', {
              active: formatNumber(tenant.activeApiKeyCount),
              total: formatNumber(tenant.apiKeyCount),
            })}
          </div>
          <div>{tenant.environmentSummary}</div>
        </div>
      ),
    },
    {
      id: 'traffic',
      header: t('Traffic'),
      cell: (tenant: TenantDirectoryRow) => (
        <div className="space-y-1 text-sm text-[var(--sdk-color-text-secondary)]">
          <div>{t('{count} requests', { count: formatNumber(tenant.requestCount) })}</div>
          <div>{t('{count} tokens', { count: formatNumber(tenant.tokenCount) })}</div>
        </div>
      ),
    },
    {
      id: 'readiness',
      header: t('Readiness'),
      cell: (tenant: TenantDirectoryRow) => (
        <StatusBadge
          showIcon
          status={tenant.canIssueApiKey ? t('Ready') : t('Incomplete')}
          variant={tenant.canIssueApiKey ? 'success' : 'warning'}
        />
      ),
      width: 140,
    },
  ];

  useEffect(() => {
    if (selectedTenantId && !tenantRows.some((tenant) => tenant.id === selectedTenantId)) {
      setSelectedTenantId(null);
      setIsDetailDrawerOpen(false);
    }
  }, [selectedTenantId, tenantRows]);

  function resetTenantDialog() {
    setTenantDraft({ id: '', name: '' });
    setIsTenantDialogOpen(false);
  }

  function resetProjectDialog() {
    const tenantId = defaultTenantId(snapshot);

    setProjectDraft({
      tenant_id: tenantId,
      id: '',
      name: '',
    });
    setIsProjectDialogOpen(false);
  }

  function resetApiKeyDialog() {
    setApiKeyDraft(createApiKeyDraft(snapshot));
    setIsApiKeyDialogOpen(false);
  }

  function handleTenantDialogOpenChange(open: boolean) {
    if (!open) {
      resetTenantDialog();
      return;
    }

    setIsTenantDialogOpen(true);
  }

  function handleProjectDialogOpenChange(open: boolean) {
    if (!open) {
      resetProjectDialog();
      return;
    }

    setIsProjectDialogOpen(true);
  }

  function handleApiKeyDialogOpenChange(open: boolean) {
    if (!open) {
      resetApiKeyDialog();
      return;
    }

    setIsApiKeyDialogOpen(true);
  }

  function openTenantDialog(tenant?: TenantDirectoryRow) {
    setTenantDraft(
      tenant
        ? {
            id: tenant.id,
            name: tenant.name,
          }
        : { id: '', name: '' },
    );
    setIsTenantDialogOpen(true);
  }

  function openProjectDialog(tenant?: TenantDirectoryRow) {
    const tenantId = tenant?.id ?? defaultTenantId(snapshot);

    setProjectDraft({
      tenant_id: tenantId,
      id: '',
      name: '',
    });
    setIsProjectDialogOpen(true);
  }

  function openApiKeyDialog(tenant?: TenantDirectoryRow) {
    const tenantId = tenant?.id ?? defaultTenantId(snapshot);

    setApiKeyDraft(
      createApiKeyDraft(snapshot, {
        tenant_id: tenantId,
        project_id: defaultProjectId(snapshot, tenantId),
      }),
    );
    setIsApiKeyDialogOpen(true);
  }

  function openDetailDrawer(tenant: TenantDirectoryRow) {
    setSelectedTenantId(tenant.id);
    setIsDetailDrawerOpen(true);
  }

  function handleDetailDrawerOpenChange(open: boolean) {
    setIsDetailDrawerOpen(open);

    if (!open) {
      setSelectedTenantId(null);
    }
  }

  async function handleTenantSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await handleSaveTenant(tenantDraft);
    resetTenantDialog();
  }

  async function handleProjectSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    await handleSaveProject(projectDraft);
    resetProjectDialog();
  }

  async function handleApiKeySubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const normalizedLabel = apiKeyDraft.label.trim();
    const normalizedNotes = apiKeyDraft.notes.trim();
    const parsedExpiresAt = parseApiKeyExpiryInputValue(apiKeyDraft.expires_at_local);

    const created = await handleCreateApiKey({
      tenant_id: apiKeyDraft.tenant_id,
      project_id: apiKeyDraft.project_id,
      environment: apiKeyDraft.environment,
      label: normalizedLabel || undefined,
      notes: normalizedNotes || undefined,
      expires_at_ms:
        parsedExpiresAt !== undefined
        && Number.isFinite(parsedExpiresAt)
        && Number.isInteger(parsedExpiresAt)
          ? parsedExpiresAt
          : undefined,
    });
    setRevealedApiKey(created);
    resetApiKeyDialog();
  }

  async function confirmDelete() {
    if (!pendingDelete) {
      return;
    }

    await handleDeleteTenant(pendingDelete.id);
    setPendingDelete(null);
    setSelectedTenantId(null);
    setIsDetailDrawerOpen(false);
  }

  return (
    <>
      <AdminPageFrame
        actions={
          <>
            <AdminActionChip label={t('Workspace governance')} />
            <AdminActionChip label={t('Organizations')} tone="success" />
            <AdminActionChip label={t('Issue key')} tone="warning" />
          </>
        }
        description={t(
          'Tenant posture, workspace ownership, and operating coverage are kept together so regional operators can scale safely.',
        )}
        eyebrow={t('Tenant governance')}
        rail={
          <div className="space-y-6">
            <AdminSectionCard
              description={t(
                'A quick ledger of coverage helps decide where onboarding, compliance, or staffing is needed.',
              )}
              title={t('Workspace posture')}
            >
              <AdminGuidanceList>
                <div>
                  {t('Organizations with dedicated moderation coverage: {count}', {
                    count: formatNumber(snapshot.tenants.length),
                  })}
                </div>
                <div>
                  {t('{count} active tenant workspaces are contributing to the current operator load.', {
                    count: formatNumber(totalProjects),
                  })}
                </div>
                <div>{t('Workspace handoff templates aligned for every active tenant cluster.')}</div>
                <div>{t('Regional escalation paths can be reviewed before new launches are approved.')}</div>
              </AdminGuidanceList>
            </AdminSectionCard>
            <AdminSectionCard
              description={t(
                'Registry workflow now keeps table selection, detail review, and focused dialogs aligned for tenant governance.',
              )}
              title={t('Operator handoff')}
            >
              <AdminGuidanceList>
                <div>{t('Live workspace status: {status}', { status: t(status) })}</div>
                <div>{t('Detail drawers keep the selected tenant in view while edits stay isolated in dialogs.')}</div>
                <div>{t('Key issuance remains guarded until a workspace exists and coverage can be reviewed.')}</div>
              </AdminGuidanceList>
            </AdminSectionCard>
          </div>
        }
        title={t('Tenants')}
      >
        <div className="grid gap-4 md:grid-cols-3">
          <AdminMetricCard
            detail={t('Tenant posture stays green when each organization has workspace ownership and live operator coverage.')}
            label={t('Tenant posture')}
            value={formatNumber(snapshot.tenants.length)}
          />
          <AdminMetricCard
            detail={t('Organizations currently represented in the IM estate.')}
            label={t('Organizations')}
            value={formatNumber(snapshot.tenants.length)}
          />
          <AdminMetricCard
            detail={t('Workspaces connected to the operator shell.')}
            label={t('Workspace')}
            value={formatNumber(snapshot.projects.length)}
          />
        </div>

        <AdminSectionCard
          description={t(
            'Tenant onboarding is table-first so operations teams can verify ownership, scope, live access coverage, and workspace density at a glance.',
          )}
          title={t('Tenant directory')}
        >
          <div className="space-y-4">
            <Card className="border-dashed">
              <CardContent className="p-4">
                <form
                  className="flex flex-wrap items-end gap-3"
                  onSubmit={(event) => event.preventDefault()}
                >
                  <div className="min-w-[18rem] flex-[1.4] space-y-2">
                    <Label htmlFor="tenant-search">{t('Search tenants')}</Label>
                    <Input
                      id="tenant-search"
                      onChange={(event: ChangeEvent<HTMLInputElement>) =>
                        setSearch(event.target.value)
                      }
                      placeholder={t('Search tenants, workspaces, environments, or access key labels')}
                      value={search}
                    />
                  </div>
                  <div className="flex flex-wrap items-center gap-2">
                    <Button onClick={() => openTenantDialog()} type="button" variant="primary">
                      {t('New tenant')}
                    </Button>
                    <Button onClick={() => openProjectDialog()} type="button" variant="outline">
                      {t('New workspace')}
                    </Button>
                    <Button onClick={() => openApiKeyDialog()} type="button" variant="outline">
                      {t('Issue key')}
                    </Button>
                  </div>
                </form>
                <div className="mt-3 flex flex-wrap items-center gap-3 text-sm text-[var(--admin-text-secondary)]">
                  <span>{t('{count} visible', { count: formatNumber(tenantRows.length) })}</span>
                  <span>{t('{count} workspaces', { count: formatNumber(totalProjects) })}</span>
                  <span>{t('{count} portal users', { count: formatNumber(totalPortalUsers) })}</span>
                  <span>{t('{count} active keys', { count: formatNumber(activeApiKeyCount) })}</span>
                  <span>{t('{count} requests in the current operating window.', { count: formatNumber(totalRequests) })}</span>
                </div>
              </CardContent>
            </Card>

            <TenantsRegistrySection
              activeApiKeyCount={activeApiKeyCount}
              columns={columns}
              filteredTenants={tenantRows}
              onOpenApiKeyDialog={openApiKeyDialog}
              onOpenProjectDialog={openProjectDialog}
              onOpenTenantDialog={openTenantDialog}
              onRequestDelete={(tenant) =>
                setPendingDelete({
                  id: tenant.id,
                  label: resolveTenantSelectionLabel(tenant, t),
                })
              }
              onSelectTenant={openDetailDrawer}
              selectedTenantId={selectedTenantId}
              totalPortalUsers={totalPortalUsers}
              totalProjects={totalProjects}
            />
          </div>
        </AdminSectionCard>
      </AdminPageFrame>

      <TenantsDetailDrawer
        canDelete={selectedTenant ? selectedTenant.canDelete : false}
        onDelete={() => {
          if (!selectedTenant) {
            return;
          }

          setPendingDelete({
            id: selectedTenant.id,
            label: resolveTenantSelectionLabel(selectedTenant, t),
          });
        }}
        onEdit={() => {
          if (!selectedTenant) {
            return;
          }

          setIsDetailDrawerOpen(false);
          openTenantDialog(selectedTenant);
        }}
        onIssueKey={() => {
          if (!selectedTenant) {
            return;
          }

          setIsDetailDrawerOpen(false);
          openApiKeyDialog(selectedTenant);
        }}
        onNewProject={() => {
          if (!selectedTenant) {
            return;
          }

          setIsDetailDrawerOpen(false);
          openProjectDialog(selectedTenant);
        }}
        onOpenChange={handleDetailDrawerOpenChange}
        open={isDetailDrawerOpen}
        selectedTenant={selectedTenant}
      />

      <TenantDialog
        draft={tenantDraft}
        onOpenChange={handleTenantDialogOpenChange}
        onSubmit={(event) => void handleTenantSubmit(event)}
        open={isTenantDialogOpen}
        setDraft={setTenantDraft}
      />

      <ProjectDialog
        draft={projectDraft}
        onOpenChange={handleProjectDialogOpenChange}
        onSubmit={(event) => void handleProjectSubmit(event)}
        open={isProjectDialogOpen}
        selectedProjectBilling={selectedProjectBilling}
        selectedProjectTokens={selectedProjectTokens}
        selectedProjectUsage={selectedProjectUsage}
        setDraft={setProjectDraft}
        snapshot={snapshot}
      />

      <ApiKeyDialog
        availableProjects={availableApiKeyProjects}
        draft={apiKeyDraft}
        onOpenChange={handleApiKeyDialogOpenChange}
        onSubmit={(event) => void handleApiKeySubmit(event)}
        open={isApiKeyDialogOpen}
        setDraft={setApiKeyDraft}
        snapshot={snapshot}
      />

      <PlaintextApiKeyDialog
        onClose={() => setRevealedApiKey(null)}
        projects={snapshot.projects}
        revealedApiKey={revealedApiKey}
      />

      <ConfirmActionDialog
        confirmLabel={t('Delete now')}
        description={
          pendingDelete
            ? t(
                'Delete {label}. This permanently removes the selected resource from the workspace registry.',
                { label: pendingDelete.label },
              )
            : ''
        }
        onConfirm={() => void confirmDelete()}
        onOpenChange={(open) => {
          if (!open) {
            setPendingDelete(null);
          }
        }}
        open={Boolean(pendingDelete)}
        title={t('Delete workspace resource')}
      />
    </>
  );
}
