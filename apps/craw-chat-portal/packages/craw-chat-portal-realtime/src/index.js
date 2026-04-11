import {
  escapeHtml,
  renderBulletList,
  renderClusterCards,
  renderDataTable,
  renderProgressList,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalRealtimeViewModel } from './services/index.js';

export async function renderPortalRealtimePage() {
  const model = await buildPortalRealtimeViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">实时链路</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
      </section>
      ${renderSurface({
        eyebrow: '态势',
        title: '会话与在线态势',
        description: '判断是否需要把实时问题上升到值守操作面。',
        body: renderClusterCards(model.posture),
      })}
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '订阅',
          title: '实时订阅负载',
          description: '每条租户主链路的窗口压力与积压。',
          body: renderProgressList(model.subscriptions),
        })}
        ${renderSurface({
          eyebrow: '设备',
          title: '设备同步看板',
          description: '关注同步延迟和异构终端分布。',
          body: renderDataTable({
            columns: ['负责人', '设备', '最近同步', '延迟', '状态'],
            rows: model.devices.map((item) => [item.owner, item.device, item.sync, item.lag, item.state]),
          }),
        })}
      </div>
      ${renderSurface({
        eyebrow: '事件',
        title: '值守事件轨迹',
        description: '用事件语言讲清这条实时链路正在发生什么。',
        body: renderBulletList(model.events),
      })}
    </div>
  `;
}
