import {
  Badge,
  Button,
  FormActions,
  FormGrid,
  FormSection,
  InlineAlert,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Textarea,
} from '@sdkwork/ui-pc-react';
import {
  AdminActionChip,
  AdminEmptyState,
  deleteTenantStorageConfig,
  getAdminErrorStatus,
  getGlobalStorageConfig,
  getTenantEffectiveStorageConfig,
  getTenantStorageConfig,
  AdminGuidanceList,
  AdminInsetCard,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  listStorageAuditTrail,
  listStorageProviders,
  saveGlobalStorageConfig,
  saveTenantStorageConfig,
  useAdminI18n,
  validateGlobalStorageConfig,
  validateTenantStorageConfig,
} from 'sdkwork-control-plane-core';
import type {
  AdminPageProps,
  StorageAuditRecord,
  StorageConfigSnapshotRecord,
  StorageCredentialMode,
  StorageFieldInputKind,
  StorageProviderSchemaRecord,
  StorageSchemaFieldRecord,
  StorageValidationRecord,
} from 'sdkwork-control-plane-types';
import { useEffect, useMemo, useState, type ChangeEvent } from 'react';

import {
  applyProviderSchema,
  buildStorageUpsertInput,
  credentialFieldsForMode,
  createStorageDraft,
  emptyStorageDraft,
  fieldValueAsBoolean,
  fieldValueAsString,
  findStorageProvider,
  resolveCredentialModeLabel,
  type ScopeMode,
  type StorageDraft,
  type StorageFieldDraftValue,
} from './storageDraft';

type PageNotice = {
  title: string;
  description: string;
  tone: 'info' | 'warning';
};

function inputTypeForField(inputKind: StorageFieldInputKind): 'text' | 'url' | 'number' | 'password' {
  switch (inputKind) {
    case 'url':
      return 'url';
    case 'number':
      return 'number';
    case 'secret':
      return 'password';
    default:
      return 'text';
  }
}

function tenantScopePlaceholder(tenantId: string): StorageConfigSnapshotRecord {
  return {
    scope: {
      kind: 'tenant',
      scopeId: tenantId,
    },
    binding: null,
    config: null,
    secret: null,
  };
}

function isNotFoundError(error: unknown): boolean {
  return getAdminErrorStatus(error) === 404;
}

function normalizeErrorMessage(error: unknown, fallbackMessage: string): string {
  if (error instanceof Error && error.message.trim()) {
    return error.message;
  }

  return fallbackMessage;
}

function resolveValidationTone(validation: StorageValidationRecord): 'success' | 'warning' {
  return validation.status === 'healthy' ? 'success' : 'warning';
}

function ScopeButton({
  active,
  children,
  disabled,
  onClick,
}: {
  active: boolean;
  children: string;
  disabled?: boolean;
  onClick: () => void;
}) {
  return (
    <button
      className={`rounded-full border px-3 py-2 text-sm font-medium transition-colors ${
        active
          ? 'border-[var(--admin-border-color)] bg-[var(--admin-sidebar-item-active)] text-[var(--admin-text-on-contrast)]'
          : 'border-[var(--admin-border-color)] bg-white/70 text-[var(--admin-text-primary)] hover:bg-[var(--admin-content-background)]/80'
      } ${disabled ? 'cursor-not-allowed opacity-50' : ''}`}
      disabled={disabled}
      onClick={onClick}
      type="button"
    >
      {children}
    </button>
  );
}

function StorageFieldEditor({
  field,
  value,
  onChange,
}: {
  field: StorageSchemaFieldRecord;
  value: StorageFieldDraftValue | undefined;
  onChange: (value: StorageFieldDraftValue) => void;
}) {
  if (field.inputKind === 'boolean') {
    return (
      <label className="flex min-h-[42px] items-center gap-3 rounded-2xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/50 px-4 py-3">
        <input
          checked={fieldValueAsBoolean(value)}
          className="h-4 w-4"
          onChange={(event: ChangeEvent<HTMLInputElement>) => onChange(event.target.checked)}
          type="checkbox"
        />
        <span className="text-sm text-[var(--admin-text-primary)]">{field.label}</span>
      </label>
    );
  }

  if (field.inputKind === 'json') {
    return (
      <Textarea
        onChange={(event: ChangeEvent<HTMLTextAreaElement>) => onChange(event.target.value)}
        placeholder={field.label}
        rows={5}
        value={fieldValueAsString(value)}
      />
    );
  }

  return (
    <Input
      onChange={(event: ChangeEvent<HTMLInputElement>) => onChange(event.target.value)}
      placeholder={field.label}
      type={inputTypeForField(field.inputKind)}
      value={fieldValueAsString(value)}
    />
  );
}

function PostureRail({
  activeProvider,
  auditTrail,
  effectiveSummary,
  formatDateTime,
  scopeMode,
  scopeSnapshot,
  selectedTenantId,
  t,
  validation,
}: {
  activeProvider: StorageProviderSchemaRecord | null;
  auditTrail: StorageAuditRecord[];
  effectiveSummary: {
    detail: string;
    provider: StorageProviderSchemaRecord | null;
    resolvedScopeLabel: string;
  } | null;
  formatDateTime: (value?: number | null) => string;
  scopeMode: ScopeMode;
  scopeSnapshot: StorageConfigSnapshotRecord | null;
  selectedTenantId: string;
  t: (text: string) => string;
  validation: StorageValidationRecord | null;
}) {
  return (
    <>
      <AdminSectionCard
        description={t(
          'Keep the selected scope, provider family, and validation posture visible while operators rotate credentials or change endpoint policy.',
        )}
        title={t('Current posture')}
      >
        <div className="space-y-3">
          {scopeSnapshot?.binding ? (
            <>
              <AdminInsetCard>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                  {t('Editing scope')}
                </div>
                <div className="mt-3 flex flex-wrap items-center gap-2">
                  <AdminActionChip
                    label={
                      scopeMode === 'tenant'
                        ? `${t('Tenant override')} / ${selectedTenantId || t('Unknown tenant')}`
                        : t('Global default')
                    }
                  />
                  <AdminActionChip
                    label={scopeSnapshot.binding.enabled ? t('Enabled') : t('Disabled')}
                    tone={scopeSnapshot.binding.enabled ? 'success' : 'warning'}
                  />
                </div>
              </AdminInsetCard>

              <AdminInsetCard>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                  {t('Provider')}
                </div>
                <div className="mt-3 text-sm font-semibold text-[var(--admin-text-primary)]">
                  {activeProvider?.displayName ?? scopeSnapshot.binding.providerPluginId}
                </div>
                <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                  {t('Bucket or container')}: {scopeSnapshot.config?.bucketOrContainer ?? t('Not configured')}
                </div>
                <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                  {t('Public base URL')}: {scopeSnapshot.config?.publicBaseUrl ?? t('Not configured')}
                </div>
              </AdminInsetCard>

              {scopeSnapshot.secret ? (
                <AdminInsetCard>
                  <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                    {t('Stored credentials')}
                  </div>
                  <div className="mt-3 text-sm text-[var(--admin-text-primary)]">
                    {t(resolveCredentialModeLabel(scopeSnapshot.secret.credentialMode))}
                  </div>
                  <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                    {t('Fingerprint')}: {scopeSnapshot.secret.secretFingerprint}
                  </div>
                </AdminInsetCard>
              ) : null}
            </>
          ) : (
            <AdminEmptyState
              detail={t(
                'This scope does not have a storage policy yet. Choose a provider and save the first configuration to establish upload posture.',
              )}
              title={t('No scope policy')}
            />
          )}

          {validation ? (
            <AdminInsetCard>
              <div className="flex flex-wrap items-center gap-2">
                <AdminActionChip label={t(validation.status)} tone={resolveValidationTone(validation)} />
                <Badge variant="outline">{t(validation.stage)}</Badge>
              </div>
              <div className="mt-3 text-sm text-[var(--admin-text-secondary)]">
                {validation.message}
              </div>
            </AdminInsetCard>
          ) : null}

          <AdminInsetCard>
            <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
              {t('Operator guidance')}
            </div>
            <AdminGuidanceList className="mt-3">
              <p>
                {t(
                  'Leave credential inputs blank when you only need to adjust bucket, endpoint, or public URL. The current provider secret stays in place for the same provider and credential mode.',
                )}
              </p>
              <p>
                {t(
                  'Switching provider or credential mode requires a fresh credential submission so runtime validation and upload presign behavior stay in sync.',
                )}
              </p>
              <p className="text-xs text-[var(--admin-text-muted)]">
                {t('Rendered at')}: {formatDateTime(Date.now())}
              </p>
            </AdminGuidanceList>
          </AdminInsetCard>
        </div>
      </AdminSectionCard>

      {scopeMode === 'tenant' ? (
        <AdminSectionCard
          description={t(
            'Tenant reads resolve from the override itself or from the inherited global default when no override exists.',
          )}
          title={t('Effective fallback')}
        >
          {effectiveSummary ? (
            <AdminInsetCard>
              <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                {t('Resolved scope')}
              </div>
              <div className="mt-3 text-sm font-semibold text-[var(--admin-text-primary)]">
                {effectiveSummary.resolvedScopeLabel}
              </div>
              <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                {effectiveSummary.provider?.displayName ?? t('Unknown provider')}
              </div>
              <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                {t('Bucket or container')}: {effectiveSummary.detail}
              </div>
            </AdminInsetCard>
          ) : (
            <AdminEmptyState
              detail={t('No tenant override or global fallback currently resolves for this tenant.')}
              title={t('No effective resolution')}
            />
          )}
        </AdminSectionCard>
      ) : null}

      <AdminSectionCard
        description={t(
          'Recent writes and deletes help operators trace whether global policy or tenant-specific overrides were changed most recently.',
        )}
        title={t('Audit trail')}
      >
        <div className="space-y-3">
          {auditTrail.length > 0 ? (
            auditTrail.slice(0, 8).map((record) => (
              <AdminInsetCard key={record.id}>
                <div className="flex flex-wrap items-center gap-2">
                  <AdminActionChip
                    label={t(record.action === 'delete' ? 'Deleted' : 'Updated')}
                    tone={record.action === 'delete' ? 'warning' : 'success'}
                  />
                  <Badge variant="outline">{record.providerPluginId}</Badge>
                </div>
                <div className="mt-3 text-sm font-medium text-[var(--admin-text-primary)]">
                  {record.scope.kind === 'tenant'
                    ? `${t('Tenant override')} / ${record.scope.scopeId ?? t('Unknown tenant')}`
                    : t('Global default')}
                </div>
                <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                  {formatDateTime(record.createdAtMs)}
                </div>
              </AdminInsetCard>
            ))
          ) : (
            <AdminEmptyState
              detail={t('The audit stream starts once operators save or delete a storage policy for the first time.')}
              title={t('No storage audit entries yet')}
            />
          )}
        </div>
      </AdminSectionCard>
    </>
  );
}

export function StoragePage({ snapshot }: AdminPageProps) {
  const { formatDateTime, formatNumber, t } = useAdminI18n();
  const [providers, setProviders] = useState<StorageProviderSchemaRecord[]>([]);
  const [auditTrail, setAuditTrail] = useState<StorageAuditRecord[]>([]);
  const [scopeMode, setScopeMode] = useState<ScopeMode>('global');
  const [selectedTenantId, setSelectedTenantId] = useState(snapshot.tenants[0]?.id ?? '');
  const [scopeSnapshot, setScopeSnapshot] = useState<StorageConfigSnapshotRecord | null>(null);
  const [validation, setValidation] = useState<StorageValidationRecord | null>(null);
  const [draft, setDraft] = useState<StorageDraft>(emptyStorageDraft);
  const [notice, setNotice] = useState<PageNotice | null>(null);
  const [catalogLoading, setCatalogLoading] = useState(true);
  const [scopeLoading, setScopeLoading] = useState(false);
  const [mutating, setMutating] = useState(false);
  const [effectiveSummary, setEffectiveSummary] = useState<{
    detail: string;
    provider: StorageProviderSchemaRecord | null;
    resolvedScopeLabel: string;
  } | null>(null);

  const activeProvider = useMemo(
    () => findStorageProvider(providers, draft.providerPluginId),
    [draft.providerPluginId, providers],
  );
  const activeCredentialFields = useMemo(
    () => credentialFieldsForMode(activeProvider, draft.credentialMode),
    [activeProvider, draft.credentialMode],
  );
  const tenantOptions = snapshot.tenants;

  useEffect(() => {
    if (scopeMode === 'tenant' && !tenantOptions.length) {
      setScopeMode('global');
    }
  }, [scopeMode, tenantOptions.length]);

  useEffect(() => {
    if (!selectedTenantId && tenantOptions.length > 0) {
      setSelectedTenantId(tenantOptions[0].id);
    }
  }, [selectedTenantId, tenantOptions]);

  useEffect(() => {
    let cancelled = false;

    async function loadCatalogState() {
      setCatalogLoading(true);
      try {
        const [providerRecords, auditRecords] = await Promise.all([
          listStorageProviders(),
          listStorageAuditTrail(),
        ]);
        if (cancelled) {
          return;
        }
        setProviders(providerRecords);
        setAuditTrail(auditRecords);
      } catch (error) {
        if (!cancelled) {
          setNotice({
            title: t('Unable to load storage catalog'),
            description: normalizeErrorMessage(error, t('Storage providers and audit trail could not be loaded.')),
            tone: 'warning',
          });
        }
      } finally {
        if (!cancelled) {
          setCatalogLoading(false);
        }
      }
    }

    void loadCatalogState();

    return () => {
      cancelled = true;
    };
  }, [t]);

  useEffect(() => {
    if (!providers.length) {
      return;
    }

    let cancelled = false;

    async function loadScopeState() {
      setScopeLoading(true);
      try {
        const nextSnapshot =
          scopeMode === 'tenant'
            ? selectedTenantId
              ? await getTenantStorageConfig(selectedTenantId)
              : tenantScopePlaceholder('')
            : await getGlobalStorageConfig();
        if (cancelled) {
          return;
        }

        const resolvedProvider = findStorageProvider(
          providers,
          nextSnapshot.binding?.providerPluginId ?? providers[0]?.providerPluginId ?? '',
        );

        setScopeSnapshot(nextSnapshot);
        setDraft(createStorageDraft(nextSnapshot, resolvedProvider ?? providers[0] ?? null));
        setValidation(null);
      } catch (error) {
        if (!cancelled) {
          setNotice({
            title: t('Unable to load storage scope'),
            description: normalizeErrorMessage(error, t('Storage scope data could not be refreshed.')),
            tone: 'warning',
          });
        }
      } finally {
        if (!cancelled) {
          setScopeLoading(false);
        }
      }
    }

    void loadScopeState();

    return () => {
      cancelled = true;
    };
  }, [providers, scopeMode, selectedTenantId, t]);

  useEffect(() => {
    let cancelled = false;

    async function loadEffectiveSummary() {
      if (scopeMode !== 'tenant' || !selectedTenantId) {
        setEffectiveSummary(null);
        return;
      }

      try {
        const effectiveRecord = await getTenantEffectiveStorageConfig(selectedTenantId).catch((error: unknown) => {
          if (isNotFoundError(error)) {
            return null;
          }
          throw error;
        });

        if (cancelled || !effectiveRecord) {
          if (!cancelled) {
            setEffectiveSummary(null);
          }
          return;
        }

        const provider = findStorageProvider(providers, effectiveRecord.binding.providerPluginId);
        setEffectiveSummary({
          detail: effectiveRecord.config.bucketOrContainer ?? t('Not configured'),
          provider,
          resolvedScopeLabel:
            effectiveRecord.resolvedScope.kind === 'tenant'
              ? `${t('Tenant override')} / ${effectiveRecord.resolvedScope.scopeId ?? t('Unknown tenant')}`
              : t('Global default'),
        });
      } catch (error) {
        if (!cancelled) {
          setNotice({
            title: t('Unable to load effective storage resolution'),
            description: normalizeErrorMessage(error, t('Tenant fallback resolution could not be loaded.')),
            tone: 'warning',
          });
        }
      }
    }

    void loadEffectiveSummary();

    return () => {
      cancelled = true;
    };
  }, [providers, scopeMode, selectedTenantId, t]);

  async function refreshAuditState() {
    const auditRecords = await listStorageAuditTrail();
    setAuditTrail(auditRecords);
  }

  async function refreshCatalogAndScope() {
    setMutating(true);
    try {
      const [providerRecords, auditRecords] = await Promise.all([
        listStorageProviders(),
        listStorageAuditTrail(),
      ]);
      setProviders(providerRecords);
      setAuditTrail(auditRecords);

      const nextSnapshot =
        scopeMode === 'tenant' && selectedTenantId
          ? await getTenantStorageConfig(selectedTenantId)
          : await getGlobalStorageConfig();
      const resolvedProvider = findStorageProvider(
        providerRecords,
        nextSnapshot.binding?.providerPluginId ?? providerRecords[0]?.providerPluginId ?? '',
      );
      setScopeSnapshot(nextSnapshot);
      setDraft(createStorageDraft(nextSnapshot, resolvedProvider ?? providerRecords[0] ?? null));
      setValidation(null);
      setNotice({
        title: t('Storage state refreshed'),
        description: t('Catalog, scope snapshot, and audit trail were refreshed from the live admin API.'),
        tone: 'info',
      });
    } catch (error) {
      setNotice({
        title: t('Refresh failed'),
        description: normalizeErrorMessage(error, t('Storage state could not be refreshed.')),
        tone: 'warning',
      });
    } finally {
      setMutating(false);
    }
  }

  async function handleValidate() {
    if (scopeMode === 'tenant' && !selectedTenantId) {
      setNotice({
        title: t('Tenant is required'),
        description: t('Choose a tenant before validating a tenant override.'),
        tone: 'warning',
      });
      return;
    }

    setMutating(true);
    try {
      const nextValidation =
        scopeMode === 'tenant' && selectedTenantId
          ? await validateTenantStorageConfig(selectedTenantId)
          : await validateGlobalStorageConfig();
      setValidation(nextValidation);
      setNotice({
        title: nextValidation.status === 'healthy' ? t('Storage validation passed') : t('Storage validation requires attention'),
        description: nextValidation.message,
        tone: nextValidation.status === 'healthy' ? 'info' : 'warning',
      });
    } catch (error) {
      setNotice({
        title: t('Validation request failed'),
        description: normalizeErrorMessage(error, t('Storage validation could not be completed.')),
        tone: 'warning',
      });
    } finally {
      setMutating(false);
    }
  }

  async function handleSave() {
    if (!activeProvider) {
      setNotice({
        title: t('Provider is required'),
        description: t('Choose a provider before saving storage configuration.'),
        tone: 'warning',
      });
      return;
    }

    if (scopeMode === 'tenant' && !selectedTenantId) {
      setNotice({
        title: t('Tenant is required'),
        description: t('Choose a tenant before saving a tenant override.'),
        tone: 'warning',
      });
      return;
    }

    setMutating(true);
    try {
      const payload = buildStorageUpsertInput({
        draft,
        providerSchema: activeProvider,
        currentSnapshot: scopeSnapshot,
        nowMs: Date.now(),
      });
      const nextSnapshot =
        scopeMode === 'tenant' && selectedTenantId
          ? await saveTenantStorageConfig(selectedTenantId, payload)
          : await saveGlobalStorageConfig(payload);
      setScopeSnapshot(nextSnapshot);
      setDraft(createStorageDraft(nextSnapshot, activeProvider));
      setValidation(null);
      await refreshAuditState();
      setNotice({
        title: t('Storage configuration saved'),
        description: t('The active storage scope was updated and the audit trail now reflects the latest write.'),
        tone: 'info',
      });
    } catch (error) {
      setNotice({
        title: t('Save failed'),
        description: normalizeErrorMessage(error, t('Storage configuration could not be saved.')),
        tone: 'warning',
      });
    } finally {
      setMutating(false);
    }
  }

  async function handleDeleteTenantOverride() {
    if (scopeMode !== 'tenant' || !selectedTenantId) {
      return;
    }

    setMutating(true);
    try {
      await deleteTenantStorageConfig(selectedTenantId);
      const [auditRecords, nextSnapshot] = await Promise.all([
        listStorageAuditTrail(),
        getTenantStorageConfig(selectedTenantId),
      ]);
      const fallbackProvider = findStorageProvider(
        providers,
        nextSnapshot.binding?.providerPluginId ?? providers[0]?.providerPluginId ?? '',
      );
      setAuditTrail(auditRecords);
      setScopeSnapshot(nextSnapshot);
      setDraft(createStorageDraft(nextSnapshot, fallbackProvider ?? providers[0] ?? null));
      setValidation(null);
      setNotice({
        title: t('Tenant override removed'),
        description: t('The tenant-specific storage policy was deleted. Effective reads now fall back to the global storage default.'),
        tone: 'info',
      });
    } catch (error) {
      setNotice({
        title: t('Delete failed'),
        description: normalizeErrorMessage(error, t('The tenant override could not be deleted.')),
        tone: 'warning',
      });
    } finally {
      setMutating(false);
    }
  }

  const validationSummary = validation ? `${t(validation.status)} / ${t(validation.stage)}` : t('Not run');

  return (
    <AdminPageFrame
      actions={
        <>
          <Badge variant="secondary">{t('object storage control plane')}</Badge>
          <Badge variant="outline">
            {scopeMode === 'tenant'
              ? `${t('Tenant override')} / ${selectedTenantId || t('Unknown tenant')}`
              : t('Global default')}
          </Badge>
        </>
      }
      description={t(
        'Manage global object storage defaults, tenant-specific overrides, credential rotation, and presigned upload readiness from one operator surface that matches the shared storage runtime contract.',
      )}
      eyebrow={t('Storage governance')}
      rail={
        <PostureRail
          activeProvider={activeProvider}
          auditTrail={auditTrail}
          effectiveSummary={effectiveSummary}
          formatDateTime={formatDateTime}
          scopeMode={scopeMode}
          scopeSnapshot={scopeSnapshot}
          selectedTenantId={selectedTenantId}
          t={t}
          validation={validation}
        />
      }
      title={t('Storage')}
    >
      {notice ? (
        <InlineAlert
          description={notice.description}
          showIcon
          title={notice.title}
          tone={notice.tone}
        />
      ) : null}

      <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
        <AdminMetricCard detail={t('Shared provider schemas available to the admin storage control plane.')} label={t('Providers')} value={formatNumber(providers.length)} />
        <AdminMetricCard detail={t('Tenants currently available for storage override selection.')} label={t('Tenants')} value={formatNumber(tenantOptions.length)} />
        <AdminMetricCard detail={t('Storage writes and deletes currently retained in the audit stream.')} label={t('Audit events')} value={formatNumber(auditTrail.length)} />
        <AdminMetricCard detail={t('Latest storage validation state for the active scope.')} label={t('Validation')} value={validationSummary} />
      </div>

      <AdminSectionCard
        description={t(
          'Switch between the global default and a tenant-specific override. Tenant scopes inherit the global policy when no dedicated override exists.',
        )}
        title={t('Scope selector')}
      >
        <div className="space-y-4">
          <div className="flex flex-wrap gap-2">
            <ScopeButton active={scopeMode === 'global'} onClick={() => setScopeMode('global')}>
              {t('Global default')}
            </ScopeButton>
            <ScopeButton active={scopeMode === 'tenant'} disabled={!tenantOptions.length} onClick={() => tenantOptions.length && setScopeMode('tenant')}>
              {t('Tenant override')}
            </ScopeButton>
          </div>

          {scopeMode === 'tenant' ? (
            tenantOptions.length > 0 ? (
              <div className="max-w-xl space-y-2">
                <Label>{t('Tenant')}</Label>
                <Select onValueChange={setSelectedTenantId} value={selectedTenantId}>
                  <SelectTrigger>
                    <SelectValue placeholder={t('Choose tenant')} />
                  </SelectTrigger>
                  <SelectContent>
                    {tenantOptions.map((tenant) => (
                      <SelectItem key={tenant.id} value={tenant.id}>
                        {tenant.name} / {tenant.id}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            ) : (
              <InlineAlert description={t('Create at least one tenant before configuring tenant-specific storage overrides.')} showIcon title={t('No tenant directory available')} tone="warning" />
            )
          ) : null}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={activeProvider
          ? t('Provider-specific common fields and credential fields come directly from the shared storage catalog contract so the admin experience stays aligned with backend capability metadata.')
          : t('Choose a storage provider to reveal common fields, credential modes, and capability tags.')}
        title={t('Provider profile')}
      >
        {catalogLoading && !providers.length ? (
          <AdminEmptyState detail={t('The storage provider catalog is still loading from the admin API.')} title={t('Loading storage catalog')} />
        ) : providers.length > 0 ? (
          <div className="space-y-5">
            <div className="grid gap-4 xl:grid-cols-[minmax(0,1fr)_280px]">
              <div className="space-y-2">
                <Label>{t('Provider')}</Label>
                <Select
                  onValueChange={(providerPluginId: string) => {
                    const nextProvider = findStorageProvider(providers, providerPluginId);
                    if (!nextProvider) {
                      return;
                    }
                    setDraft((currentDraft) => applyProviderSchema(currentDraft, nextProvider));
                    setValidation(null);
                  }}
                  value={draft.providerPluginId}
                >
                  <SelectTrigger>
                    <SelectValue placeholder={t('Choose provider')} />
                  </SelectTrigger>
                  <SelectContent>
                    {providers.map((provider) => (
                      <SelectItem key={provider.providerPluginId} value={provider.providerPluginId}>
                        {provider.displayName} / {provider.providerFamily}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <AdminInsetCard>
                <div className="text-xs font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                  {t('Activation')}
                </div>
                <label className="mt-3 flex items-center gap-3 text-sm text-[var(--admin-text-primary)]">
                  <input
                    checked={draft.enabled}
                    className="h-4 w-4"
                    onChange={(event: ChangeEvent<HTMLInputElement>) => setDraft((currentDraft) => ({ ...currentDraft, enabled: event.target.checked }))}
                    type="checkbox"
                  />
                  <span>{t('Enable this storage binding for the selected scope')}</span>
                </label>
              </AdminInsetCard>
            </div>

            {activeProvider ? (
              <div className="flex flex-wrap gap-2">
                <Badge variant="secondary">{activeProvider.providerFamily}</Badge>
                {activeProvider.capabilities.map((capability) => (
                  <Badge key={capability} variant="outline">
                    {capability}
                  </Badge>
                ))}
              </div>
            ) : null}
          </div>
        ) : (
          <AdminEmptyState detail={t('No storage providers were returned by the admin API.')} title={t('Storage catalog is empty')} />
        )}
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Common fields map to the provider schema contract. Core bucket, region, and URL fields are stored explicitly while provider-specific extras are captured in provider configuration JSON.',
        )}
        title={t('Connection and addressing')}
      >
        {activeProvider ? (
          <div className="space-y-6">
            <FormSection description={t('Fields below come from the provider schema so the admin console stays aligned with backend capability metadata.')} title={t('Common fields')}>
              <FormGrid columns={2}>
                {activeProvider.commonFields.map((field) => (
                  <div className={field.inputKind === 'json' ? 'space-y-2 md:col-span-2' : 'space-y-2'} key={field.name}>
                    {field.inputKind !== 'boolean' ? (
                      <Label>
                        {field.label}
                        {field.required ? ' *' : ''}
                      </Label>
                    ) : null}
                    <StorageFieldEditor
                      field={field}
                      onChange={(value) => setDraft((currentDraft) => ({
                        ...currentDraft,
                        commonFieldValues: {
                          ...currentDraft.commonFieldValues,
                          [field.name]: value,
                        },
                      }))}
                      value={draft.commonFieldValues[field.name]}
                    />
                  </div>
                ))}
              </FormGrid>
            </FormSection>

            <FormSection description={t('Advanced provider configuration is stored as JSON and merged with provider-specific common-field extras such as path-style routing flags.')} title={t('Advanced provider config')}>
              <div className="space-y-2">
                <Label>{t('Provider configuration JSON')}</Label>
                <Textarea
                  onChange={(event: ChangeEvent<HTMLTextAreaElement>) => setDraft((currentDraft) => ({ ...currentDraft, providerConfigText: event.target.value }))}
                  placeholder={t('{\"projectId\":\"tenant-project\"}')}
                  rows={6}
                  value={draft.providerConfigText}
                />
              </div>
            </FormSection>
          </div>
        ) : (
          <AdminEmptyState detail={t('Choose a provider to reveal the schema-backed connection fields.')} title={t('Provider fields are not available yet')} />
        )}
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Credential fields are packaged into an opaque provider payload. Submit new credentials only when creating a policy, rotating a secret, switching provider, or changing credential mode.',
        )}
        title={t('Credentials')}
      >
        {activeProvider ? (
          <div className="space-y-6">
            {scopeSnapshot?.secret ? (
              <InlineAlert
                description={t('Existing credentials are already configured for this scope. Leave credential fields blank to keep the current secret for the same provider and credential mode.')}
                showIcon
                title={`${t('Current secret')} / ${scopeSnapshot.secret.secretFingerprint}`}
                tone="info"
              />
            ) : (
              <InlineAlert
                description={t('No credentials are currently stored for this scope. Enter the provider credentials before saving or validation will fail.')}
                showIcon
                title={t('Credential submission required')}
                tone="warning"
              />
            )}

            <div className="max-w-xl space-y-2">
              <Label>{t('Credential mode')}</Label>
              <Select onValueChange={(value: string) => setDraft((currentDraft) => ({ ...currentDraft, credentialMode: value as StorageCredentialMode }))} value={draft.credentialMode}>
                <SelectTrigger>
                  <SelectValue placeholder={t('Choose credential mode')} />
                </SelectTrigger>
                <SelectContent>
                  {activeProvider.supportedCredentialModes.map((mode) => (
                    <SelectItem key={mode} value={mode}>
                      {t(resolveCredentialModeLabel(mode))}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <FormGrid columns={2}>
              {activeCredentialFields.map((field) => (
                <div className={field.inputKind === 'json' ? 'space-y-2 md:col-span-2' : 'space-y-2'} key={field.name}>
                  <Label>
                    {field.label}
                    {field.required ? ' *' : ''}
                  </Label>
                  {field.helpText ? (
                    <p className="text-xs text-[var(--admin-text-muted)]">{field.helpText}</p>
                  ) : null}
                  <StorageFieldEditor
                    field={field}
                    onChange={(value) => setDraft((currentDraft) => ({
                      ...currentDraft,
                      credentialFieldValues: {
                        ...currentDraft.credentialFieldValues,
                        [field.name]: value,
                      },
                    }))}
                    value={draft.credentialFieldValues[field.name]}
                  />
                </div>
              ))}
            </FormGrid>
          </div>
        ) : (
          <AdminEmptyState detail={t('Choose a provider before entering credentials.')} title={t('Credential fields are not available yet')} />
        )}
      </AdminSectionCard>

      {scopeLoading ? (
        <InlineAlert
          description={t('The selected storage scope is loading from the admin API.')}
          showIcon
          title={t('Loading storage scope')}
          tone="info"
        />
      ) : null}

      <FormActions>
        <Button disabled={mutating || catalogLoading} onClick={() => void refreshCatalogAndScope()} type="button" variant="outline">
          {t('Refresh')}
        </Button>
        <Button disabled={mutating || scopeLoading || !providers.length} onClick={() => void handleValidate()} type="button" variant="outline">
          {t('Validate')}
        </Button>
        {scopeMode === 'tenant' ? (
          <Button disabled={mutating || scopeLoading || !selectedTenantId} onClick={() => void handleDeleteTenantOverride()} type="button" variant="outline">
            {t('Delete tenant override')}
          </Button>
        ) : null}
        <Button disabled={mutating || scopeLoading || !activeProvider} onClick={() => void handleSave()} type="button" variant="primary">
          {t('Save storage policy')}
        </Button>
      </FormActions>
    </AdminPageFrame>
  );
}
