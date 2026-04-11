import {
  PORTAL_CONSOLE_ENTRY_OPTIONS,
  PORTAL_THEME_OPTIONS,
} from '../../../craw-chat-portal-types/src/index.js';
import { escapeHtml } from '../../../craw-chat-portal-commons/src/index.js';
import { deriveSettingsPanelSummary, deriveShellResetAction } from './PortalShellPosture.js';

function deriveConsoleModuleOptions(routeManifest = []) {
  return routeManifest
    .filter((entry) => entry?.productModule?.navigation?.sidebar)
    .sort((left, right) => left.productModule.navigation.order - right.productModule.navigation.order);
}

export function renderPortalSettingsPanel({ routeManifest = [], shellState, currentPath = '' }) {
  if (!shellState.settingsOpen) {
    return '';
  }

  const consoleEntryMode = shellState.consoleEntryMode === 'pinned' ? 'pinned' : 'resume';
  const defaultConsoleDescription =
    consoleEntryMode === 'pinned'
      ? '当前固定入口值守，重返控制台将从已选模块开始当班。'
      : '当前按最近一次工作面接管；这里保留待命入口，切换为固定进入模块后生效。';
  const activeDefaultConsoleLabel = consoleEntryMode === 'pinned' ? '当前固定入口' : '待命入口';
  const sidebarToggleLabel = shellState.sidebarCollapsed ? '展开侧栏' : '收起侧栏';
  const consoleModuleOptions = deriveConsoleModuleOptions(routeManifest);
  const hiddenConsolePaths = Array.isArray(shellState.hiddenConsolePaths) ? shellState.hiddenConsolePaths : [];
  const activeDefaultConsolePath = consoleModuleOptions.some(
    (entry) => entry.path === shellState.pinnedConsolePath,
  )
    ? shellState.pinnedConsolePath
    : consoleModuleOptions[0]?.path ?? '/console/dashboard';
  const shellSummary = deriveSettingsPanelSummary({ routeManifest, currentPath, shellState });
  const activeConsolePath = shellSummary.activeConsolePath;
  const resetAction = deriveShellResetAction(shellState);

  return `
    <div class="portal-settings-backdrop" data-command="dismiss-settings">
      <aside
        class="portal-settings-panel"
        id="portal-settings-panel"
        role="dialog"
        aria-modal="true"
        aria-labelledby="portal-settings-title"
      >
        <header class="portal-settings-panel__header">
          <div>
            <p class="portal-surface__eyebrow">工作区偏好</p>
            <h2 id="portal-settings-title">控制台外观</h2>
          </div>
          <button class="portal-button portal-button--ghost" data-command="close-settings" type="button">关闭</button>
        </header>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">当前壳层摘要</p>
          <p class="portal-settings-panel__description">
            <strong>${escapeHtml(shellSummary.label)}</strong>
            <span>${escapeHtml(shellSummary.description)}</span>
          </p>
          <div class="portal-settings-summary-grid">
            ${shellSummary.items
              .map(
                (item) => `
                  <article class="portal-settings-summary-card">
                    <span>${escapeHtml(item.label)}</span>
                    <strong>${escapeHtml(item.value)}</strong>
                    <p>${escapeHtml(item.detail)}</p>
                  </article>
                `,
              )
              .join('')}
          </div>
        </section>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">主题风格</p>
          <div class="portal-theme-options">
            ${PORTAL_THEME_OPTIONS.map(
              (theme) => `
                <button
                  aria-pressed="${theme.id === shellState.theme ? 'true' : 'false'}"
                  class="portal-theme-option ${theme.id === shellState.theme ? 'is-active' : ''}"
                  data-theme="${escapeHtml(theme.id)}"
                  type="button"
                >
                  <strong>${escapeHtml(theme.label)}</strong>
                  <span>${escapeHtml(theme.description)}</span>
                </button>
              `,
            ).join('')}
          </div>
        </section>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">控制台进入策略</p>
          <p class="portal-settings-panel__description">决定重新进入租户控制台时，优先恢复上次模块还是固定进入指定模块。</p>
          <div class="portal-settings-choice-grid">
            ${PORTAL_CONSOLE_ENTRY_OPTIONS.map(
              (option) => `
                <button
                  aria-pressed="${option.id === consoleEntryMode ? 'true' : 'false'}"
                  class="portal-settings-choice ${option.id === consoleEntryMode ? 'is-active' : ''}"
                  data-console-entry-mode="${escapeHtml(option.id)}"
                  type="button"
                >
                  <strong>${escapeHtml(option.label)}</strong>
                  <span>${escapeHtml(option.description)}</span>
                </button>
              `,
            ).join('')}
          </div>
        </section>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">默认进入模块</p>
          <p class="portal-settings-panel__description">${defaultConsoleDescription}</p>
          <div class="portal-settings-route-grid">
            ${consoleModuleOptions
              .map(
                (entry) => `
                  <button
                    aria-pressed="${entry.path === activeDefaultConsolePath ? 'true' : 'false'}"
                    class="portal-settings-route-option ${entry.path === activeDefaultConsolePath ? 'is-active' : ''}"
                    data-default-console-path="${escapeHtml(entry.path)}"
                    type="button"
                  >
                    <strong>${escapeHtml(entry.productModule.displayName)}</strong>
                    <span>${escapeHtml(entry.productModule.capabilityTags[0] ?? '工作面')}</span>
                    ${entry.path === activeDefaultConsolePath ? `<small>${escapeHtml(activeDefaultConsoleLabel)}</small>` : ''}
                  </button>
                `,
              )
              .join('')}
          </div>
        </section>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">侧栏显示模块</p>
          <p class="portal-settings-panel__description">收起不常用工作面，保持租户值守侧栏聚焦；当前模块与固定进入模块会始终保留可见。</p>
          <div class="portal-settings-route-grid">
            ${consoleModuleOptions
              .map((entry) => {
                const isCurrentModule = entry.path === activeConsolePath;
                const isPinnedModule = consoleEntryMode === 'pinned' && entry.path === activeDefaultConsolePath;
                const visibilityLockLabel = isCurrentModule
                  ? '当前模块始终可见'
                  : isPinnedModule
                    ? '固定入口始终可见'
                    : '';
                const isVisible = visibilityLockLabel ? true : !hiddenConsolePaths.includes(entry.path);

                return `
                  <button
                    aria-pressed="${isVisible ? 'true' : 'false'}"
                    class="portal-settings-route-option ${isVisible ? 'is-active' : ''}"
                    data-sidebar-console-path="${escapeHtml(entry.path)}"
                    ${visibilityLockLabel ? 'disabled' : ''}
                    type="button"
                  >
                    <strong>${escapeHtml(entry.productModule.displayName)}</strong>
                    <span>${escapeHtml(entry.productModule.capabilityTags[0] ?? '工作面')}</span>
                    ${visibilityLockLabel ? `<small>${escapeHtml(visibilityLockLabel)}</small>` : ''}
                  </button>
                `;
              })
              .join('')}
          </div>
        </section>
        <section class="portal-settings-panel__section">
          <p class="portal-settings-panel__label">侧栏行为</p>
          <p class="portal-settings-panel__description">快速切换侧栏展开状态，或恢复当前工作区的默认壳层偏好。</p>
          <div class="portal-settings-action-grid">
            <button
              aria-pressed="${shellState.sidebarCollapsed ? 'true' : 'false'}"
              class="portal-button portal-button--ghost portal-settings-action"
              data-command="toggle-sidebar"
              type="button"
            >
              ${sidebarToggleLabel}
            </button>
            <button
              class="portal-button portal-button--ghost portal-settings-action"
              data-command="reset-shell-preferences"
              ${resetAction.disabled ? 'disabled' : ''}
              type="button"
            >
              ${escapeHtml(resetAction.label)}
            </button>
          </div>
        </section>
      </aside>
    </div>
  `;
}
