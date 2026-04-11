export function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

export function joinHtml(parts) {
  return parts.filter(Boolean).join('');
}

export function renderBadge({ label, tone = 'neutral' }) {
  return `<span class="portal-badge portal-badge--${escapeHtml(tone)}">${escapeHtml(label)}</span>`;
}

export function renderStatCards(items) {
  return `
    <div class="portal-stat-grid">
      ${items
        .map(
          (item) => `
            <article class="portal-stat-card">
              <p class="portal-stat-card__label">${escapeHtml(item.label)}</p>
              <div class="portal-stat-card__value-row">
                <strong class="portal-stat-card__value">${escapeHtml(item.value)}</strong>
                ${item.delta ? `<span class="portal-stat-card__delta portal-stat-card__delta--${escapeHtml(item.tone || 'neutral')}">${escapeHtml(item.delta)}</span>` : ''}
              </div>
              ${
                item.caption
                  ? `<p class="portal-stat-card__caption">${escapeHtml(item.caption)}</p>`
                  : ''
              }
            </article>
          `,
        )
        .join('')}
    </div>
  `;
}

export function renderSurface({
  eyebrow,
  title,
  description,
  body,
  actions = '',
  className = '',
}) {
  return `
    <section class="portal-surface ${className}">
      <header class="portal-surface__header">
        <div class="portal-surface__copy">
          ${eyebrow ? `<p class="portal-surface__eyebrow">${escapeHtml(eyebrow)}</p>` : ''}
          <h2 class="portal-surface__title">${escapeHtml(title)}</h2>
          ${
            description
              ? `<p class="portal-surface__description">${escapeHtml(description)}</p>`
              : ''
          }
        </div>
        ${actions ? `<div class="portal-surface__actions">${actions}</div>` : ''}
      </header>
      <div class="portal-surface__body">${body}</div>
    </section>
  `;
}

export function renderDataTable({ columns, rows }) {
  return `
    <div class="portal-table-wrap">
      <table class="portal-table">
        <thead>
          <tr>
            ${columns.map((column) => `<th>${escapeHtml(column)}</th>`).join('')}
          </tr>
        </thead>
        <tbody>
          ${rows
            .map(
              (row) => `
                <tr>
                  ${row.map((cell) => `<td>${escapeHtml(cell)}</td>`).join('')}
                </tr>
              `,
            )
            .join('')}
        </tbody>
      </table>
    </div>
  `;
}

export function renderProgressList(items) {
  return `
    <div class="portal-progress-list">
      ${items
        .map(
          (item) => `
            <article class="portal-progress-item">
              <div class="portal-progress-item__header">
                <div>
                  <strong>${escapeHtml(item.label)}</strong>
                  ${
                    item.caption
                      ? `<p class="portal-progress-item__caption">${escapeHtml(item.caption)}</p>`
                      : ''
                  }
                </div>
                <span>${escapeHtml(item.value)}</span>
              </div>
              <div class="portal-progress-item__track">
                <span class="portal-progress-item__bar portal-progress-item__bar--${escapeHtml(item.tone || 'neutral')}" style="width:${Math.max(8, Math.min(100, item.percent || 0))}%"></span>
              </div>
            </article>
          `,
        )
        .join('')}
    </div>
  `;
}

export function renderBulletList(items) {
  return `
    <ul class="portal-bullet-list">
      ${items
        .map(
          (item) => `
            <li class="portal-bullet-list__item">
              <span class="portal-bullet-list__dot"></span>
              <div>
                <strong>${escapeHtml(item.title)}</strong>
                ${item.description ? `<p>${escapeHtml(item.description)}</p>` : ''}
              </div>
            </li>
          `,
        )
        .join('')}
    </ul>
  `;
}

export function renderClusterCards(items) {
  return `
    <div class="portal-cluster-grid">
      ${items
        .map(
          (item) => `
            <article class="portal-cluster-card">
              <div class="portal-cluster-card__top">
                <p>${escapeHtml(item.label)}</p>
                ${renderBadge({ label: item.status, tone: item.tone || 'neutral' })}
              </div>
              <strong>${escapeHtml(item.value)}</strong>
              ${
                item.description
                  ? `<p class="portal-cluster-card__description">${escapeHtml(item.description)}</p>`
                  : ''
              }
            </article>
          `,
        )
        .join('')}
    </div>
  `;
}
