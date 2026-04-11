import {
  Badge,
  FormGrid,
  FormSection,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  SettingsField,
} from '@sdkwork/ui-pc-react';

import {
  ADMIN_LOCALE_OPTIONS,
  useAdminAppStore,
  useAdminI18n,
  useAdminWorkbench,
} from 'sdkwork-craw-chat-admin-core';

import { SettingsBadge, SettingsSummaryCard } from './Shared';

export function GeneralSettings() {
  const {
    hiddenSidebarItems,
    isSidebarCollapsed,
    sidebarWidth,
    themeColor,
    themeMode,
  } = useAdminAppStore();
  const { sessionUser, status } = useAdminWorkbench();
  const { locale, setLocale, t } = useAdminI18n();

  return (
    <div className="space-y-8">
      <FormSection
        description={t(
          'Choose the operator workspace language. Dates, numbers, and shared shell copy follow this setting immediately.',
        )}
        title={t('Language and locale')}
      >
        <FormGrid columns={1}>
          <SettingsField
            controlId="admin-settings-language"
            description={t('Language updates every route label, shell notice, and workspace detail immediately.')}
            label={t('Language')}
            layout="vertical"
          >
            <Select
              onValueChange={(value: string) => setLocale(value as typeof locale)}
              value={locale}
            >
              <SelectTrigger id="admin-settings-language">
                <SelectValue placeholder={t('Language')} />
              </SelectTrigger>
              <SelectContent>
                {ADMIN_LOCALE_OPTIONS.map((option) => (
                  <SelectItem key={option.id} value={option.id}>
                    {t(option.label)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </SettingsField>
        </FormGrid>
      </FormSection>

      <FormSection
        actions={<SettingsBadge variant="outline">{t('Quick actions')}</SettingsBadge>}
        description={t('Open the Command center to launch routes, refresh workspace posture, open the Settings center, or sign out without leaving the current shell.')}
        title={t('Command center')}
      >
        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-5">
          <SettingsSummaryCard
            badge={t('Search shortcuts')}
            detail={t('Route launch, workspace refresh, settings access, and sign-out actions without leaving the current shell.')}
            label={t('Command center')}
            value={t('Ctrl K')}
          />
          <SettingsSummaryCard
            detail={t('Refresh workspace posture when operators need a new snapshot of queues, incidents, and runtime state.')}
            label={t('Workspace refresh')}
            value={t('Refresh')}
          />
          <SettingsSummaryCard
            detail={t('Use the Settings center as the control directory for navigation, shell posture, and operator continuity.')}
            label={t('Settings center')}
            value={t('Operations directory')}
          />
          <SettingsSummaryCard
            detail={t('Review the live incident stack, shift handoff risk, and escalation routes without leaving the current shell.')}
            label={t('Operations pulse')}
            value={t('Pulse')}
          />
          <SettingsSummaryCard
            detail={t('Continuity cue, Capability tags, and Required permissions stay visible while operators move between governance modules.')}
            label={t('Route context strip')}
            value={t('Continuity cue')}
          />
        </div>
      </FormSection>

      <FormSection
        actions={
          <SettingsBadge variant={sessionUser?.active ? 'success' : 'warning'}>
            {sessionUser?.active ? t('live operator summary') : t('Workspace')}
          </SettingsBadge>
        }
        description={t('Current shell posture for the IM operator workspace.')}
        title={t('Workspace posture')}
      >
        <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
          <SettingsSummaryCard
            badge={sessionUser?.active ? t('active') : t('Settings center')}
            detail={sessionUser?.email ?? t(status)}
            label={t('Operator')}
            value={sessionUser?.display_name ?? t('IM operator')}
          />
          <SettingsSummaryCard label={t('Theme mode')} value={t(themeMode)} />
          <SettingsSummaryCard label={t('Theme color')} value={t(themeColor)} />
          <SettingsSummaryCard
            label={t('Sidebar mode')}
            value={isSidebarCollapsed ? t('collapsed') : t('expanded')}
          />
          <SettingsSummaryCard label={t('Sidebar width')} value={`${sidebarWidth}px`} />
          <SettingsSummaryCard
            label={t('Hidden nav items')}
            value={hiddenSidebarItems.length}
          />
        </div>
      </FormSection>
    </div>
  );
}
