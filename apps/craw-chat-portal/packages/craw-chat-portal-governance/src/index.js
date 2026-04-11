import {
  escapeHtml,
  renderBulletList,
  renderClusterCards,
  renderDataTable,
  renderSurface,
} from '../../craw-chat-portal-commons/src/index.js';
import { buildPortalGovernanceViewModel } from './services/index.js';

export async function renderPortalGovernancePage() {
  const model = await buildPortalGovernanceViewModel();

  return `
    <div class="portal-page">
      <section class="portal-page-hero">
        <div>
          <p class="portal-page-hero__eyebrow">治理</p>
          <h2 class="portal-page-hero__title">${escapeHtml(model.hero.title)}</h2>
          <p class="portal-page-hero__description">${escapeHtml(model.hero.description)}</p>
        </div>
      </section>
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '审计',
          title: '租户审计账本',
          description: '所有关键操作都应该带着范围和结果被记录。',
          body: renderDataTable({
            columns: ['操作', '执行人', '范围', '状态'],
            rows: model.auditRecords.map((item) => [item.action, item.actor, item.scope, item.status]),
          }),
        })}
        ${renderSurface({
          eyebrow: '健康度',
          title: '健康与绑定态势',
          description: '把供应商健康与运行时绑定一致性一起看。',
          body: renderClusterCards(model.providerHealth),
        })}
      </div>
      <div class="portal-grid portal-grid--two">
        ${renderSurface({
          eyebrow: '诊断',
          title: '运行诊断',
          description: '值守视角的运行时与回放状态。',
          body: renderBulletList(model.diagnostics),
        })}
        ${renderSurface({
          eyebrow: '清单',
          title: '合规事项',
          description: '今天必须收口的治理事项。',
          body: renderBulletList(model.checklist),
        })}
      </div>
    </div>
  `;
}
