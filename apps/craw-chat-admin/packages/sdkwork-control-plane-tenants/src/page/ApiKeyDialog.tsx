import type {
  ChangeEvent,
  Dispatch,
  FormEvent,
  SetStateAction,
} from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  FormActions,
  FormGrid,
  FormSection,
  InlineAlert,
  Input,
  Textarea,
} from '@sdkwork/ui-pc-react';
import { useAdminI18n } from 'sdkwork-control-plane-core';
import type { AdminPageProps } from 'sdkwork-control-plane-types';

import {
  defaultProjectId,
  DialogField,
  formatApiKeyExpiryInputValue,
  resolveProjectSelectionLabel,
  resolveTenantSelectionLabel,
  SelectField,
  type ApiKeyDraft,
  type ApiKeyEnvironment,
} from './shared';

type ApiKeyDialogProps = {
  availableProjects: AdminPageProps['snapshot']['projects'];
  draft: ApiKeyDraft;
  onOpenChange: (open: boolean) => void;
  onSubmit: (event: FormEvent<HTMLFormElement>) => void;
  open: boolean;
  setDraft: Dispatch<SetStateAction<ApiKeyDraft>>;
  snapshot: AdminPageProps['snapshot'];
};

export function ApiKeyDialog({
  availableProjects,
  draft,
  onOpenChange,
  onSubmit,
  open,
  setDraft,
  snapshot,
}: ApiKeyDialogProps) {
  const { t } = useAdminI18n();

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="w-[min(92vw,52rem)]">
        <DialogHeader>
          <DialogTitle>{t('Issue key')}</DialogTitle>
          <DialogDescription>
            {t(
              'Issue a live access key in a focused dialog, then reveal the plaintext once for secure operator handoff.',
            )}
          </DialogDescription>
        </DialogHeader>

        <form className="space-y-6" onSubmit={onSubmit}>
          <FormSection
            description={t(
              'Scope determines which tenant and workspace own the new live access key.',
            )}
            title={t('Access key profile')}
          >
            <FormGrid columns={2}>
              <div className="space-y-2">
                {snapshot.tenants.length ? (
                  <SelectField
                    label={t('Tenant')}
                    onValueChange={(value) =>
                      setDraft((current) => ({
                        ...current,
                        tenant_id: value,
                        project_id: defaultProjectId(snapshot, value),
                      }))
                    }
                    options={snapshot.tenants.map((tenant) => ({
                      label: resolveTenantSelectionLabel(tenant, t),
                      value: tenant.id,
                    }))}
                    value={draft.tenant_id}
                  />
                ) : (
                  <DialogField label={t('Tenant')}>
                    <Input
                      onChange={(event: ChangeEvent<HTMLInputElement>) =>
                        setDraft((current) => ({
                          ...current,
                          tenant_id: event.target.value,
                        }))
                      }
                      required
                      value={draft.tenant_id}
                    />
                  </DialogField>
                )}
              </div>

              <div className="space-y-2">
                {availableProjects.length ? (
                  <SelectField
                    label={t('Workspace')}
                    onValueChange={(value) =>
                      setDraft((current) => ({
                        ...current,
                        project_id: value,
                      }))
                    }
                    options={availableProjects.map((project) => ({
                      label: resolveProjectSelectionLabel(project, t),
                      value: project.id,
                    }))}
                    value={draft.project_id}
                  />
                ) : (
                  <DialogField label={t('Workspace ID')}>
                    <Input
                      onChange={(event: ChangeEvent<HTMLInputElement>) =>
                        setDraft((current) => ({
                          ...current,
                          project_id: event.target.value,
                        }))
                      }
                      required
                      value={draft.project_id}
                    />
                  </DialogField>
                )}
              </div>

              <SelectField<ApiKeyEnvironment>
                label={t('Environment')}
                onValueChange={(value) =>
                  setDraft((current) => ({ ...current, environment: value }))
                }
                options={[
                  { label: t('Production'), value: 'production' },
                  { label: t('Staging'), value: 'staging' },
                  { label: t('Development'), value: 'development' },
                ]}
                value={draft.environment}
              />
            </FormGrid>

            {!availableProjects.length ? (
              <InlineAlert
                description={t(
                  'Create a workspace for this tenant before issuing a live access key.',
                )}
                showIcon
                title={t('Key issuance guardrail')}
                tone="warning"
              />
            ) : null}
          </FormSection>

          <FormSection
            description={t('Labels and notes keep the access key inventory readable for operators.')}
            title={t('Inventory metadata')}
          >
            <FormGrid columns={2}>
              <DialogField htmlFor="key-label" label={t('Key label')}>
                <Input
                  id="key-label"
                  onChange={(event: ChangeEvent<HTMLInputElement>) =>
                    setDraft((current) => ({ ...current, label: event.target.value }))
                  }
                  placeholder={t('Production App Key')}
                  value={draft.label}
                />
              </DialogField>

              <DialogField
                description={t('Leave blank to keep this key available until it is rotated or revoked.')}
                htmlFor="expires-at"
                label={t('Access expires')}
              >
                <Input
                  id="expires-at"
                  onChange={(event: ChangeEvent<HTMLInputElement>) =>
                    setDraft((current) => ({
                      ...current,
                      expires_at_local: event.target.value,
                    }))
                  }
                  step="60"
                  type="datetime-local"
                  value={formatApiKeyExpiryInputValue(draft.expires_at_local)}
                />
              </DialogField>

              <DialogField label={t('Notes')}>
                <Textarea
                  onChange={(event: ChangeEvent<HTMLTextAreaElement>) =>
                    setDraft((current) => ({ ...current, notes: event.target.value }))
                  }
                  placeholder={t('Retained for admin inventory')}
                  rows={3}
                  value={draft.notes}
                />
              </DialogField>
            </FormGrid>
          </FormSection>

          <FormActions>
            <Button onClick={() => onOpenChange(false)} type="button" variant="outline">
              {t('Cancel')}
            </Button>
            <Button disabled={!draft.project_id} type="submit" variant="primary">
              {t('Issue key')}
            </Button>
          </FormActions>
        </form>
      </DialogContent>
    </Dialog>
  );
}
