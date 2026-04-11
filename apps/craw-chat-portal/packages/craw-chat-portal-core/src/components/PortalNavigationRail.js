import { escapeHtml } from '../../../craw-chat-portal-commons/src/index.js';

function deriveCompactMark(value, fallback = 'CP') {
  const normalizedValue = String(value ?? '').trim();
  const cjkMatch = normalizedValue.match(/[\p{Script=Han}]/u);

  if (cjkMatch) {
    return cjkMatch[0];
  }

  const mark = normalizedValue
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean)
    .map((segment) => segment[0]?.toUpperCase() ?? '')
    .join('')
    .slice(0, 2);

  return mark || fallback;
}

function deriveWorkspaceLabel(workspace) {
  const explicitLabel = String(workspace.label ?? '').trim();

  if (explicitLabel) {
    return explicitLabel;
  }

  return '租户工作区';
}

export function renderPortalNavigationRail({ routeManifest, currentPath, shellState, workspace }) {
  const hiddenConsolePaths = Array.isArray(shellState.hiddenConsolePaths) ? shellState.hiddenConsolePaths : [];
  const pinnedConsolePath =
    shellState.consoleEntryMode === 'pinned' ? shellState.pinnedConsolePath : null;
  const visibleEntries = routeManifest.filter((entry) => {
    if (entry.path === currentPath) {
      return true;
    }

    if (pinnedConsolePath && entry.path === pinnedConsolePath) {
      return true;
    }

    return !hiddenConsolePaths.includes(entry.path);
  });

  const groupedEntries = visibleEntries.reduce((accumulator, entry) => {
    const group = entry.productModule.navigation.group;
    accumulator[group] ??= {
      label: entry.productModule.navigation.groupLabel,
      entries: [],
    };
    accumulator[group].entries.push(entry);
    return accumulator;
  }, {});

  const groupHtml = Object.values(groupedEntries)
    .map(
      (group) => `
        <section class="portal-rail__group">
          <p class="portal-rail__group-label">${escapeHtml(group.label)}</p>
          ${group.entries
            .sort((left, right) => left.productModule.navigation.order - right.productModule.navigation.order)
            .map(
              (entry) => `
                <button
                  aria-current="${entry.path === currentPath ? 'page' : 'false'}"
                  aria-label="${escapeHtml(entry.productModule.displayName)}"
                  class="portal-rail__link ${entry.path === currentPath ? 'is-active' : ''}"
                  data-route="${escapeHtml(entry.path)}"
                  title="${escapeHtml(entry.productModule.displayName)}"
                  type="button"
                >
                  <span class="portal-rail__glyph">${deriveCompactMark(entry.productModule.displayName, entry.productModule.displayName[0])}</span>
                  <span class="portal-rail__link-copy">
                    <span class="portal-rail__link-label">${escapeHtml(entry.productModule.displayName)}</span>
                    <small>${escapeHtml(entry.productModule.capabilityTags[0])}</small>
                  </span>
                </button>
              `,
            )
            .join('')}
        </section>
      `,
    )
    .join('');

  return `
    <aside class="portal-rail ${shellState.sidebarCollapsed ? 'is-collapsed' : ''}">
      <div class="portal-rail__workspace" title="${escapeHtml(workspace.name)}">
        <span class="portal-rail__workspace-mark">${deriveCompactMark(workspace.name, 'TW')}</span>
        <div class="portal-rail__workspace-copy">
          <p class="portal-rail__workspace-label">${escapeHtml(deriveWorkspaceLabel(workspace))}</p>
          <strong>${escapeHtml(workspace.name)}</strong>
          <span>${escapeHtml(workspace.activeBrands)} 个品牌 · ${escapeHtml(workspace.seats)} 个操作席位</span>
        </div>
      </div>
      ${groupHtml}
      <div class="portal-rail__footer">
        <button class="portal-button portal-button--ghost" data-command="toggle-sidebar" type="button">
          ${shellState.sidebarCollapsed ? '展开侧栏' : '收起侧栏'}
        </button>
      </div>
    </aside>
  `;
}
