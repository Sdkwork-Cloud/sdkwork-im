import { escapeHtml, renderBulletList, renderSurface } from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalHomeViewModel } from './services/index.js';

export async function renderPortalHomePage() {
  const home = await buildPortalHomeViewModel();

  return `
    <section class="portal-hero">
      <p class="portal-hero__eyebrow">${escapeHtml(home.hero.eyebrow)}</p>
      <h2 class="portal-hero__title">${escapeHtml(home.hero.title)}</h2>
      <p class="portal-hero__description">${escapeHtml(home.hero.description)}</p>
      <div class="portal-hero__actions">
        <button class="portal-button portal-button--primary" data-route="/login" type="button">进入租户控制台</button>
        <button class="portal-button portal-button--ghost" data-route="/login?redirect=%2Fconsole%2Fdashboard" type="button">直达总览台</button>
      </div>
    </section>
    ${renderSurface({
      eyebrow: '门户能力面',
      title: '围绕租户 IM 的六块核心工作面',
      description: '不是泛用数据面板，而是服务于消息运营、实时链路和治理值守。',
      body: renderBulletList(home.pillars),
    })}
  `;
}
