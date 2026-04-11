import { renderPortalBrandMark } from './PortalBrandMark.js';
import { renderPortalCommandDeck } from './PortalCommandDeck.js';
import { renderPortalNavigationRail } from './PortalNavigationRail.js';
import { renderPortalSettingsPanel } from './PortalSettingsPanel.js';
import { renderPortalTopNavigation } from './PortalTopNavigation.js';

export function renderPortalDesktopShell({
  currentPath,
  currentRouteEntry,
  currentRouteLabel,
  pageHtml,
  routeManifest,
  shellState,
  user,
  workspace,
}) {
  return `
    <div class="portal-shell ${shellState.sidebarCollapsed ? 'is-rail-collapsed' : ''}">
      ${renderPortalNavigationRail({ routeManifest, currentPath, shellState, workspace })}
      <div class="portal-shell__main">
        <div class="portal-shell__masthead">
          ${renderPortalBrandMark()}
          ${renderPortalTopNavigation({
            currentRouteLabel,
            routeGroupLabel: currentRouteEntry.productModule.navigation.groupLabel,
            routeCapabilities: currentRouteEntry.productModule.capabilityTags,
            routeSummary: currentRouteEntry.productModule.summary,
            shellState,
            settingsOpen: shellState.settingsOpen,
            workspace,
            user,
          })}
        </div>
        ${renderPortalCommandDeck({ currentRouteEntry, routeManifest, workspace, shellState })}
        <main class="portal-shell__content">${pageHtml}</main>
      </div>
      ${renderPortalSettingsPanel({ routeManifest, shellState, currentPath })}
    </div>
  `;
}
