import type { ReactNode } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@sdkwork/ui-pc-react';
import {
  formatAdminNumber,
  translateAdminText,
} from 'sdkwork-craw-chat-admin-core';
import type {
  AdminPageProps,
  CreatedGatewayApiKey,
} from 'sdkwork-craw-chat-admin-types';

type TenantsSnapshot = AdminPageProps['snapshot'];

export type ApiKeyEnvironment = 'production' | 'staging' | 'development';

export type ApiKeyDraft = {
  tenant_id: string;
  project_id: string;
  environment: ApiKeyEnvironment;
  label: string;
  notes: string;
  expires_at_ms: string;
};

export type TenantProjectSummary = {
  id: string;
  name: string;
  requestCount: number;
  tokenCount: number;
  usageUnits: number;
  apiKeyCount: number;
  activeApiKeyCount: number;
};

export type TenantDirectoryRow = {
  id: string;
  name: string;
  projectCount: number;
  projectSummary: string;
  portalUserCount: number;
  apiKeyCount: number;
  activeApiKeyCount: number;
  environmentSummary: string;
  requestCount: number;
  tokenCount: number;
  usageUnits: number;
  canDelete: boolean;
  canIssueApiKey: boolean;
  projectRecords: TenantProjectSummary[];
  searchHaystack: string;
};

export function defaultTenantId(snapshot: TenantsSnapshot): string {
  return snapshot.tenants[0]?.id ?? '';
}

export function defaultProjectId(
  snapshot: TenantsSnapshot,
  tenantId: string,
): string {
  return snapshot.projects.find((project) => project.tenant_id === tenantId)?.id ?? '';
}

export function createApiKeyDraft(
  snapshot: TenantsSnapshot,
  overrides: Partial<ApiKeyDraft> = {},
): ApiKeyDraft {
  const tenantId = overrides.tenant_id ?? defaultTenantId(snapshot);

  return {
    tenant_id: tenantId,
    project_id: overrides.project_id ?? defaultProjectId(snapshot, tenantId),
    environment: 'production',
    label: '',
    notes: '',
    expires_at_ms: '',
    ...overrides,
  };
}

export function formatNumber(value: number) {
  return formatAdminNumber(value);
}

export function buildTenantDirectoryRows(
  snapshot: TenantsSnapshot,
  normalizedSearch: string,
): TenantDirectoryRow[] {
  const usageByProject = new Map(
    snapshot.usageSummary.projects.map((project) => [
      project.project_id,
      project.request_count,
    ]),
  );
  const billingByProject = new Map(
    snapshot.billingSummary.projects.map((project) => [
      project.project_id,
      project.used_units,
    ]),
  );
  const tokensByProject = new Map<string, number>();

  for (const record of snapshot.usageRecords) {
    tokensByProject.set(
      record.project_id,
      (tokensByProject.get(record.project_id) ?? 0) + record.total_tokens,
    );
  }

  return snapshot.tenants
    .map((tenant) => {
      const projects = snapshot.projects.filter((project) => project.tenant_id === tenant.id);
      const projectIds = new Set(projects.map((project) => project.id));
      const portalUsers = snapshot.portalUsers.filter(
        (user) => user.workspace_tenant_id === tenant.id,
      );
      const tenantApiKeys = snapshot.apiKeys.filter(
        (key) => key.tenant_id === tenant.id || projectIds.has(key.project_id),
      );
      const activeApiKeyCount = tenantApiKeys.filter((key) => key.active).length;
      const environmentSummary = Array.from(
        new Set(tenantApiKeys.map((key) => key.environment)),
      )
        .sort()
        .join(', ');
      const projectRecords = projects
        .map((project) => ({
          id: project.id,
          name: project.name,
          requestCount: usageByProject.get(project.id) ?? 0,
          tokenCount: tokensByProject.get(project.id) ?? 0,
          usageUnits: billingByProject.get(project.id) ?? 0,
          apiKeyCount: tenantApiKeys.filter((key) => key.project_id === project.id).length,
          activeApiKeyCount: tenantApiKeys.filter(
            (key) => key.project_id === project.id && key.active,
          ).length,
        }))
        .sort(
          (left, right) =>
            right.requestCount - left.requestCount
            || right.activeApiKeyCount - left.activeApiKeyCount
            || left.name.localeCompare(right.name),
        );
      const requestCount = projectRecords.reduce(
        (sum, project) => sum + project.requestCount,
        0,
      );
      const tokenCount = projectRecords.reduce(
        (sum, project) => sum + project.tokenCount,
        0,
      );
      const usageUnits = projectRecords.reduce(
        (sum, project) => sum + project.usageUnits,
        0,
      );
      const projectSummary = projectRecords.length
        ? projectRecords
            .slice(0, 2)
            .map((project) => project.name)
            .join(', ')
        : translateAdminText('No workspace environments');

      return {
        id: tenant.id,
        name: tenant.name,
        projectCount: projects.length,
        projectSummary,
        portalUserCount: portalUsers.length,
        apiKeyCount: tenantApiKeys.length,
        activeApiKeyCount,
        environmentSummary: environmentSummary || translateAdminText('No keys'),
        requestCount,
        tokenCount,
        usageUnits,
        canDelete: projects.length === 0 && portalUsers.length === 0,
        canIssueApiKey: projects.length > 0,
        projectRecords,
        searchHaystack: [
          tenant.id,
          tenant.name,
          ...projects.flatMap((project) => [project.id, project.name]),
          ...portalUsers.flatMap((user) => [user.display_name, user.email]),
          ...tenantApiKeys.flatMap((key) => [
            key.project_id,
            key.environment,
            key.label,
            key.notes ?? '',
          ]),
        ]
          .join(' ')
          .toLowerCase(),
      };
    })
    .filter((tenant) => !normalizedSearch || tenant.searchHaystack.includes(normalizedSearch))
    .sort(
      (left, right) =>
        right.requestCount - left.requestCount
        || right.portalUserCount - left.portalUserCount
        || left.name.localeCompare(right.name)
        || left.id.localeCompare(right.id),
    );
}

export async function copyToClipboard(value: string): Promise<void> {
  if (navigator.clipboard) {
    await navigator.clipboard.writeText(value);
  }
}

export function DialogField({
  children,
  description,
  htmlFor,
  label,
}: {
  children: ReactNode;
  description?: ReactNode;
  htmlFor?: string;
  label: ReactNode;
}) {
  return (
    <div className="space-y-2">
      <Label htmlFor={htmlFor}>{label}</Label>
      {children}
      {description ? (
        <div className="text-xs text-[var(--sdk-color-text-secondary)]">
          {description}
        </div>
      ) : null}
    </div>
  );
}

export function SelectField<T extends string>({
  description,
  disabled,
  label,
  onValueChange,
  options,
  placeholder,
  value,
}: {
  description?: ReactNode;
  disabled?: boolean;
  label: ReactNode;
  onValueChange: (value: T) => void;
  options: Array<{ label: ReactNode; value: T }>;
  placeholder?: string;
  value: T;
}) {
  return (
    <div className="space-y-2">
      <Label>{label}</Label>
      <Select
        disabled={disabled}
        onValueChange={(nextValue: string) => onValueChange(nextValue as T)}
        value={value}
      >
        <SelectTrigger>
          <SelectValue placeholder={placeholder ?? String(label)} />
        </SelectTrigger>
        <SelectContent>
          {options.map((option) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      {description ? (
        <div className="text-xs text-[var(--sdk-color-text-secondary)]">
          {description}
        </div>
      ) : null}
    </div>
  );
}

export function ConfirmActionDialog({
  confirmLabel = translateAdminText('Confirm'),
  description,
  onConfirm,
  onOpenChange,
  open,
  title,
}: {
  confirmLabel?: string;
  description: ReactNode;
  onConfirm: () => void | Promise<void>;
  onOpenChange: (open: boolean) => void;
  open: boolean;
  title: ReactNode;
}) {
  return (
    <Dialog onOpenChange={onOpenChange} open={open}>
      <DialogContent className="w-[min(92vw,28rem)]">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button onClick={() => onOpenChange(false)} type="button" variant="outline">
            {translateAdminText('Cancel')}
          </Button>
          <Button onClick={() => void onConfirm()} type="button" variant="danger">
            {confirmLabel}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

export type RevealedApiKey = CreatedGatewayApiKey | null;
