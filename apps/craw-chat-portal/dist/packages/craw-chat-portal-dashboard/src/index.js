import {
  escapeHtml,
  renderBulletList,
  renderClusterCards,
  renderProgressList,
  renderStatCards,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalDashboardViewModel } from './services/index.js';

export async function renderPortalDashboardPage() {
  const model = await buildPortalDashboardViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">总览台</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
        <div class="portal-page-hero__actions">
          <button class="portal-button portal-button--ghost" data-route="/console/conversations" type="button">查看会话</button>
          <button class="portal-button portal-button--primary" data-route="/console/governance" type="button">查看治理</button>
        </div>
      </section>
      ${renderStatCards(model.hero.kpis)}
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '压力',
          title: '当班队列压力',
          description: '当前班次需要马上盯住的队列热度。',
          body: renderProgressList(model.pressure),
        })}
        ${renderSurface({
          eyebrow: '态势',
          title: '跨域稳定账本',
          description: '实时、RTC 与自动化共同构成的值守视图。',
          body: renderClusterCards(model.posture),
        })}
      </div>
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '预案',
          title: '值守优先级',
          description: '建议按这个顺序介入，压缩风险扩散面。',
          body: renderBulletList(model.priorities),
        })}
        ${renderSurface({
          eyebrow: '时间线',
          title: '班次事件时间线',
          description: '过去一小时已经发生的关键事件。',
          body: renderBulletList(model.timeline),
        })}
      </div>
    </div>
  `;
}
