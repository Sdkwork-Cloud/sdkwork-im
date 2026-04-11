import { escapeHtml } from '../../../craw-chat-portal-commons/src/index.js';
import { deriveTopbarStatusItems } from './PortalShellPosture.js';

const WORKSPACE_META_LABELS = {
  Enterprise: '企业版',
  Platinum: '白金护航',
  Gold: '黄金护航',
  Silver: '白银护航',
  Premium: '高级版',
  Pro: '专业版',
  Standard: '标准版',
  Basic: '基础版',
};

function normalizeWorkspaceMetaLabel(value) {
  const normalizedValue = String(value ?? '').trim();

  return WORKSPACE_META_LABELS[normalizedValue] ?? normalizedValue;
}

const WORKSPACE_REGION_LABELS = {
  'CN-East': '华东',
  'CN-North': '华北',
  'CN-South': '华南',
  'CN-West': '西部',
  'Multi-AZ': '多可用区',
  'Single-AZ': '单可用区',
};

function normalizeWorkspaceRegionLabel(value) {
  const normalizedValue = String(value ?? '').trim();

  return Object.entries(WORKSPACE_REGION_LABELS).reduce(
    (label, [source, target]) => label.replaceAll(source, target),
    normalizedValue,
  );
}

function resolveOperatorRole(user) {
  const role = String(user?.role ?? user?.title ?? '').trim();

  return role || null;
}

export function renderPortalTopNavigation({
  currentRouteLabel,
  routeGroupLabel = '',
  routeCapabilities = [],
  routeSummary,
  shellState,
  settingsOpen = false,
  workspace,
  user,
}) {
  const shellStatusItems = deriveTopbarStatusItems(shellState);
  const operatorRole = resolveOperatorRole(user);
  const visibleRouteGroupLabel = String(routeGroupLabel ?? '').trim();
  const visibleRouteCapabilities = Array.isArray(routeCapabilities)
    ? routeCapabilities.filter((tag) => String(tag ?? '').trim().length > 0)
    : [];

  return `
    <header class="portal-topbar">
      <div class="portal-topbar__intro">
        <p class="portal-topbar__eyebrow">在线工作区</p>
        <h1 class="portal-topbar__title">${escapeHtml(currentRouteLabel)}</h1>
        <p class="portal-topbar__description">${escapeHtml(workspace.name)} · ${escapeHtml(normalizeWorkspaceRegionLabel(workspace.region))}</p>
        <p class="portal-topbar__support">${escapeHtml(routeSummary)}</p>
        ${
          visibleRouteGroupLabel
            ? `
              <div aria-label="当前值守域" class="portal-topbar__domain">
                <span>当前值守域</span>
                <strong>${escapeHtml(visibleRouteGroupLabel)}</strong>
              </div>
            `
            : ''
        }
        ${
          visibleRouteCapabilities.length > 0
            ? `
              <div aria-label="当前工作面能力焦点" class="portal-topbar__capabilities">
                ${visibleRouteCapabilities
                  .map(
                    (tag) => `<span class="portal-badge portal-topbar__capability">${escapeHtml(tag)}</span>`,
                  )
                  .join('')}
              </div>
            `
            : ''
        }
        <div aria-label="工作台状态摘要" class="portal-topbar__status-strip">
          ${shellStatusItems
            .map(
              (item) => `
                <article class="portal-topbar__status-pill">
                  <span>${escapeHtml(item.label)}</span>
                  <strong>${escapeHtml(item.value)}</strong>
                </article>
              `,
            )
            .join('')}
        </div>
      </div>
      <div class="portal-topbar__meta">
        <span class="portal-chip">${escapeHtml(normalizeWorkspaceMetaLabel(workspace.tier))}</span>
        <span class="portal-chip">${escapeHtml(normalizeWorkspaceMetaLabel(workspace.supportPlan))}</span>
        ${operatorRole ? `<span class="portal-chip">${escapeHtml(operatorRole)}</span>` : ''}
        <span class="portal-chip">${escapeHtml(user.name)}</span>
        <button
          aria-controls="portal-settings-panel"
          aria-expanded="${settingsOpen ? 'true' : 'false'}"
          aria-haspopup="dialog"
          class="portal-button portal-button--ghost"
          data-command="toggle-settings"
          type="button"
        >
          工作台设置
        </button>
        <button class="portal-button portal-button--primary" data-command="sign-out" type="button">退出登录</button>
      </div>
    </header>
  `;
}
