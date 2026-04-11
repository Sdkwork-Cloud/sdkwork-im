import {
  escapeHtml,
  renderBulletList,
  renderClusterCards,
  renderDataTable,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalAutomationViewModel } from './services/index.js';

export async function renderPortalAutomationPage() {
  const model = await buildPortalAutomationViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">自动化</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
      </section>
      ${renderSurface({
        eyebrow: '总览',
        title: '执行态势',
        description: '工作流与通知的稳定性不应该被埋在平台日志里。',
        body: renderClusterCards(model.summary),
      })}
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '执行',
          title: '工作流执行台',
          description: '聚焦正在重试、排队和需要人工干预的流程。',
          body: renderDataTable({
            columns: ['流程', '负责人', '状态', '时长', '影响'],
            rows: model.executions.map((item) => [item.flow, item.owner, item.state, item.age, item.impact]),
          }),
        })}
        ${renderSurface({
          eyebrow: '通知',
          title: '投递面',
          description: '推送、短信与系统通知的投递表现。',
          body: renderDataTable({
            columns: ['任务', '渠道', '状态', '漂移'],
            rows: model.notifications.map((item) => [item.task, item.channel, item.state, item.drift]),
          }),
        })}
      </div>
      ${renderSurface({
        eyebrow: '预案',
        title: '操作预案',
        description: '把自动化失败后的人工接管路径明确出来。',
        body: renderBulletList(model.playbooks),
      })}
    </div>
  `;
}
