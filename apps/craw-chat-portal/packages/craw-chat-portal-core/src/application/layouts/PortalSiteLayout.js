export function renderPortalSiteLayout({ body }) {
  return `
    <div class="portal-site">
      <header class="portal-site__header">
        <div>
          <p class="portal-site__eyebrow">Craw Chat</p>
          <h1>租户门户</h1>
        </div>
        <nav class="portal-site__actions">
          <button class="portal-button portal-button--ghost" data-route="/" type="button">首页</button>
          <button class="portal-button portal-button--primary" data-route="/login" type="button">进入控制台</button>
        </nav>
      </header>
      <main class="portal-site__content">${body}</main>
    </div>
  `;
}
