import { escapeHtml } from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalAuthViewModel } from './services/index.js';

export async function renderPortalAuthPage() {
  const model = await buildPortalAuthViewModel();

  return `
    <section class="portal-auth">
      <div class="portal-auth__card">
        <p class="portal-surface__eyebrow">${escapeHtml(model.eyebrow)}</p>
        <h2>${escapeHtml(model.title)}</h2>
        <p>${escapeHtml(model.description)}</p>
        <div class="portal-auth__details">
          ${model.details
            .map(
              (item) => `
                <div>
                  <span>${escapeHtml(item.label)}</span>
                  <strong>${escapeHtml(item.value)}</strong>
                </div>
              `,
            )
            .join('')}
        </div>
        <div class="portal-auth__actions">
          <button class="portal-button portal-button--primary" data-command="demo-sign-in" type="button">${escapeHtml(model.primaryActionLabel)}</button>
          <button class="portal-button portal-button--ghost" data-route="/" type="button">${escapeHtml(model.secondaryActionLabel)}</button>
        </div>
      </div>
    </section>
  `;
}
