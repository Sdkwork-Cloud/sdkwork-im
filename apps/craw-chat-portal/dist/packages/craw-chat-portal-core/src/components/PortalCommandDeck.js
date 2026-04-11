import { escapeHtml } from '../../../craw-chat-portal-commons/src/index.js';
import { deriveCommandDeckPostureCards, deriveShellResetLabel, isShellCustomized } from './PortalShellPosture.js';

export function renderPortalCommandDeck({ currentRouteEntry, routeManifest, workspace, shellState }) {
  const relatedEntries = currentRouteEntry.productModule.commandDeck.relatedRoutes
    .map((routeKey) => routeManifest.find((entry) => entry.key === routeKey))
    .filter(Boolean);
  const postureCards = deriveCommandDeckPostureCards({ currentRouteEntry, routeManifest, shellState });
  const customizedShell = isShellCustomized(shellState);
  const resetLabel = deriveShellResetLabel(shellState);

  return `
    <section class="portal-command-deck">
      <div class="portal-command-deck__summary">
        <p class="portal-command-deck__eyebrow">操作看板</p>
        <h2>${escapeHtml(currentRouteEntry.productModule.displayName)} 运行态势</h2>
        <p>${escapeHtml(currentRouteEntry.productModule.summary)}</p>
      </div>
      <div class="portal-command-deck__actions">
        <button class="portal-button portal-button--primary" data-route="${escapeHtml(currentRouteEntry.productModule.commandDeck.primaryActionRoute)}" type="button">
          ${escapeHtml(currentRouteEntry.productModule.commandDeck.primaryActionLabel)}
        </button>
        ${relatedEntries
          .map(
            (entry) => `
              <button class="portal-button portal-button--ghost" data-route="${escapeHtml(entry.path)}" type="button">
                ${escapeHtml(entry.productModule.displayName)}
              </button>
            `,
          )
          .join('')}
        ${
          customizedShell
            ? `<button class="portal-button portal-button--ghost" data-command="reset-shell-preferences" type="button">${escapeHtml(resetLabel)}</button>`
            : ''
        }
      </div>
      <div class="portal-command-deck__pulse">
        <article class="portal-command-deck__metric">
          <span>活跃品牌</span>
          <strong>${escapeHtml(workspace.activeBrands)}</strong>
        </article>
        <article class="portal-command-deck__metric">
          <span>操作席位</span>
          <strong>${escapeHtml(workspace.seats)}</strong>
        </article>
        <article class="portal-command-deck__metric">
          <span>工作区健康度</span>
          <strong>${escapeHtml(workspace.uptime)}</strong>
        </article>
      </div>
      <div class="portal-command-deck__posture">
        ${postureCards
          .map(
            (card) => `
              <article class="portal-command-deck__status">
                <span>${escapeHtml(card.label)}</span>
                <strong>${escapeHtml(card.value)}</strong>
                <p>${escapeHtml(card.detail)}</p>
              </article>
            `,
          )
          .join('')}
      </div>
    </section>
  `;
}
