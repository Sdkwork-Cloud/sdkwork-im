import {
  escapeHtml,
  renderBulletList,
  renderClusterCards,
  renderDataTable,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalMediaViewModel } from './services/index.js';

export async function renderPortalMediaPage() {
  const model = await buildPortalMediaViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">媒体与 RTC</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
      </section>
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '素材',
          title: '素材生命周期台',
          description: '素材上传、转码和绑定状态统一跟踪。',
          body: renderDataTable({
            columns: ['素材', '类型', '状态', '队列', '负责人'],
            rows: model.assets.map((item) => [item.asset, item.type, item.state, item.queue, item.owner]),
          }),
        })}
        ${renderSurface({
          eyebrow: 'RTC',
          title: '会话房间矩阵',
          description: '房间、区域、参与者和回切状态一屏可见。',
          body: renderDataTable({
            columns: ['房间', '区域', '参与者', '状态', '备注'],
            rows: model.rtcSessions.map((item) => [item.room, item.region, item.participants, item.state, item.note]),
          }),
        })}
      </div>
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '供应商',
          title: '供应商就绪度',
          description: '媒体、RTC 与归档链路的当前健康面。',
          body: renderClusterCards(model.providers),
        })}
        ${renderSurface({
          eyebrow: '流会话',
          title: '流式会话账本',
          description: '流式会话生命周期与检查点进度。',
          body: renderBulletList(model.streams),
        })}
      </div>
    </div>
  `;
}
