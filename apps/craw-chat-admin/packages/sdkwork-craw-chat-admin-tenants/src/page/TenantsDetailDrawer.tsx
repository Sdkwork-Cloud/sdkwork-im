import {
  Button,
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  StatusBadge,
} from '@sdkwork/ui-pc-react';
import { useAdminI18n } from 'sdkwork-craw-chat-admin-core';

import { TenantsDetailPanel } from './TenantsDetailPanel';
import type { TenantDirectoryRow } from './shared';

type TenantsDetailDrawerProps = {
  canDelete: boolean;
  onDelete: () => void;
  onEdit: () => void;
  onIssueKey: () => void;
  onNewProject: () => void;
  onOpenChange: (open: boolean) => void;
  open: boolean;
  selectedTenant: TenantDirectoryRow | null;
};

export function TenantsDetailDrawer({
  canDelete,
  onDelete,
  onEdit,
  onIssueKey,
  onNewProject,
  onOpenChange,
  open,
  selectedTenant,
}: TenantsDetailDrawerProps) {
  const { t } = useAdminI18n();

  return (
    <Drawer open={open} onOpenChange={onOpenChange}>
      <DrawerContent side="right" size="xl">
        {selectedTenant ? (
          <>
            <DrawerHeader>
              <div className="space-y-3">
                <div className="flex flex-wrap items-start justify-between gap-3">
                  <div className="space-y-1">
                    <DrawerTitle>{selectedTenant.name}</DrawerTitle>
                    <DrawerDescription>{selectedTenant.id}</DrawerDescription>
                  </div>
                  <div className="flex flex-wrap gap-2">
                    <StatusBadge
                      showIcon
                      status={
                        selectedTenant.canIssueApiKey
                          ? t('Key issuance ready')
                          : t('Key issuance guardrail')
                      }
                      variant={selectedTenant.canIssueApiKey ? 'success' : 'warning'}
                    />
                    <StatusBadge
                      showIcon
                      status={
                        selectedTenant.activeApiKeyCount > 0
                          ? t('Gateway covered')
                          : t('No active keys')
                      }
                      variant={
                        selectedTenant.activeApiKeyCount > 0 ? 'success' : 'secondary'
                      }
                    />
                  </div>
                </div>
              </div>
            </DrawerHeader>

            <DrawerBody className="space-y-4">
              <TenantsDetailPanel selectedTenant={selectedTenant} />
            </DrawerBody>

            <DrawerFooter className="flex flex-wrap items-center justify-between gap-3">
              <div className="text-xs text-[var(--sdk-color-text-secondary)]">
                {t(
                  'Tenant operations stay scoped here while project creation and key issuance remain in focused dialogs.',
                )}
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <Button onClick={onEdit} size="sm" type="button" variant="outline">
                  {t('Edit')}
                </Button>
                <Button onClick={onNewProject} size="sm" type="button" variant="outline">
                  {t('New project')}
                </Button>
                <Button
                  disabled={!selectedTenant.canIssueApiKey}
                  onClick={onIssueKey}
                  size="sm"
                  type="button"
                  variant={selectedTenant.canIssueApiKey ? 'primary' : 'outline'}
                >
                  {t('Issue key')}
                </Button>
                <Button
                  disabled={!canDelete}
                  onClick={onDelete}
                  size="sm"
                  type="button"
                  variant="danger"
                >
                  {t('Delete')}
                </Button>
              </div>
            </DrawerFooter>
          </>
        ) : null}
      </DrawerContent>
    </Drawer>
  );
}
