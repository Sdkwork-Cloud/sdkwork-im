import {
  escapeHtml,
  renderBulletList,
  renderDataTable,
  renderProgressList,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalConversationsViewModel } from './services/index.js';

export async function renderPortalConversationsPage() {
  const model = await buildPortalConversationsViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">会话</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
      </section>
      ${renderSurface({
        eyebrow: '队列',
        title: '收件箱与升级通道',
        description: '优先处理高价值和高情绪波动会话。',
        body: renderProgressList(model.pipeline),
      })}
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '交接',
          title: '人工交接台',
          description: '需要人工接管的会话按优先级聚合。',
          body: renderDataTable({
            columns: ['会话', '当前负责人', '下一队列', '等待时长', '优先级'],
            rows: model.handoffs.map((item) => [item.conversation, item.owner, item.next, item.wait, item.priority]),
          }),
        })}
        ${renderSurface({
          eyebrow: '关注',
          title: '高风险会话',
          description: '情绪、响应时限和未读压力交叉观察。',
          body: renderDataTable({
            columns: ['主题', '客户', '未读', '情绪', '响应时限'],
            rows: model.watchlist.map((item) => [item.topic, item.customer, item.unread, item.sentiment, item.sla]),
          }),
        })}
      </div>
      ${renderSurface({
        eyebrow: '系统频道',
        title: '运营广播流',
        description: '租户运营必须保留清晰的系统频道边界。',
        body: renderBulletList(model.systemChannels),
      })}
    </div>
  `;
}
