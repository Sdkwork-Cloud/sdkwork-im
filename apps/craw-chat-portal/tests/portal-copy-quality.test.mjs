import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

const appRoot = path.resolve('apps/craw-chat-portal');

async function importModule(relativePath) {
  return import(pathToFileURL(path.join(appRoot, relativePath)).href);
}

test('portal entry pages render clean tenant-facing Chinese copy', async () => {
  const layoutModule = await importModule('packages/craw-chat-portal-core/src/application/layouts/PortalSiteLayout.js');
  const brandModule = await importModule('packages/craw-chat-portal-core/src/components/PortalBrandMark.js');
  const homeModule = await importModule('packages/craw-chat-portal-home/src/index.js');
  const authModule = await importModule('packages/craw-chat-portal-auth/src/index.js');

  const siteLayoutHtml = layoutModule.renderPortalSiteLayout({ body: '<section>入口内容</section>' });
  const brandHtml = brandModule.renderPortalBrandMark();
  const homeHtml = await homeModule.renderPortalHomePage();
  const authHtml = await authModule.renderPortalAuthPage();

  assert.match(siteLayoutHtml, /租户门户/);
  assert.match(siteLayoutHtml, />首页</);
  assert.match(siteLayoutHtml, /进入控制台/);
  assert.doesNotMatch(siteLayoutHtml, /Tenant Portal|Home|Open Console/);

  assert.match(brandHtml, /租户控制台/);
  assert.doesNotMatch(brandHtml, /Tenant Portal/);

  assert.match(homeHtml, /Craw Chat 租户门户/);
  assert.match(homeHtml, /面向租户的即时通信管理后台/);
  assert.match(homeHtml, /进入租户控制台/);
  assert.match(homeHtml, /直达总览台/);
  assert.match(homeHtml, /门户能力面/);
  assert.match(homeHtml, /围绕租户 IM 的六块核心工作面/);
  assert.match(homeHtml, /会话统筹/);
  assert.match(homeHtml, /服务时效风险/);
  assert.match(homeHtml, /泛用数据面板/);
  assert.match(homeHtml, /实时稳态/);
  assert.match(homeHtml, /媒体与治理/);
  assert.doesNotMatch(
    homeHtml,
    /Tenant Command Center|Craw Chat Portal|Portal Scope|Conversation Control|Realtime Reliability|Media to Governance|直达 Dashboard|SLA 风险| BI /,
  );
  assert.doesNotMatch(homeHtml, /杩涘叆绉熸埛鎺у埗鍙/);

  assert.match(authHtml, /演示租户入口/);
  assert.match(authHtml, /进入 Nebula Commerce IM/);
  assert.match(authHtml, /工作区/);
  assert.match(authHtml, /角色/);
  assert.match(authHtml, /覆盖范围/);
  assert.match(authHtml, /租户运营负责人/);
  assert.match(authHtml, /会话 \/ 实时链路 \/ 治理/);
  assert.match(authHtml, /当前已预置演示租户工作台/);
  assert.match(authHtml, /直接进入控制台体验完整运营链路/);
  assert.match(authHtml, /使用演示租户登录/);
  assert.doesNotMatch(authHtml, /Demo Tenant Access|Workspace|Role|Scope|Tenant Operations Director|Conversations \/ Realtime \/ Governance|SDK|portal-api|演示会话边界/);
  assert.doesNotMatch(authHtml, /浣跨敤婕旂ず绉熸埛鐧诲綍/);
});

test('portal console modules render professional operator copy without mojibake', async () => {
  const dashboardModule = await importModule('packages/craw-chat-portal-dashboard/src/index.js');
  const conversationsModule = await importModule('packages/craw-chat-portal-conversations/src/index.js');
  const realtimeModule = await importModule('packages/craw-chat-portal-realtime/src/index.js');
  const mediaModule = await importModule('packages/craw-chat-portal-media/src/index.js');
  const automationModule = await importModule('packages/craw-chat-portal-automation/src/index.js');
  const governanceModule = await importModule('packages/craw-chat-portal-governance/src/index.js');

  const dashboardHtml = await dashboardModule.renderPortalDashboardPage();
  const conversationsHtml = await conversationsModule.renderPortalConversationsPage();
  const realtimeHtml = await realtimeModule.renderPortalRealtimePage();
  const mediaHtml = await mediaModule.renderPortalMediaPage();
  const automationHtml = await automationModule.renderPortalAutomationPage();
  const governanceHtml = await governanceModule.renderPortalGovernancePage();

  assert.match(dashboardHtml, /总览台/);
  assert.match(dashboardHtml, /查看治理/);
  assert.match(dashboardHtml, /当班队列压力/);
  assert.match(dashboardHtml, /租户运营总览/);
  assert.match(dashboardHtml, /峰值 4\.1 千条\/秒/);
  assert.match(conversationsHtml, /会话/);
  assert.match(conversationsHtml, /人工交接台/);
  assert.match(conversationsHtml, /高风险会话/);
  assert.match(conversationsHtml, /会话运营面/);
  assert.match(conversationsHtml, /电商席位三组/);
  assert.match(conversationsHtml, /调度席位一组/);
  assert.match(conversationsHtml, /高级队列五组/);
  assert.match(conversationsHtml, /星舟零售/);
  assert.match(conversationsHtml, /南城优选/);
  assert.match(conversationsHtml, /北港百货/);
  assert.match(conversationsHtml, /响应时限/);
  assert.match(conversationsHtml, /紧急与高优先会话/);
  assert.match(conversationsHtml, /紧急/);
  assert.match(conversationsHtml, /高优先/);
  assert.match(realtimeHtml, /实时链路/);
  assert.match(realtimeHtml, /会话与在线态势/);
  assert.match(realtimeHtml, /设备同步看板/);
  assert.match(realtimeHtml, /实时链路态势/);
  assert.match(mediaHtml, /媒体与 RTC/);
  assert.match(mediaHtml, /素材生命周期台/);
  assert.match(mediaHtml, /供应商就绪度/);
  assert.match(mediaHtml, /媒体与 RTC 工作面/);
  assert.match(mediaHtml, /退款凭证-240409\.zip/);
  assert.match(mediaHtml, /活动主视觉剪辑\.mp4/);
  assert.match(mediaHtml, /门店语音备注\.m4a/);
  assert.match(mediaHtml, /VIP 售后关怀 17 号房/);
  assert.match(mediaHtml, /大促值守协同室/);
  assert.match(mediaHtml, /商家入驻培训 3 号房/);
  assert.match(automationHtml, /自动化/);
  assert.match(automationHtml, /工作流执行台/);
  assert.match(automationHtml, /操作预案/);
  assert.match(automationHtml, /自动化与通知/);
  assert.match(automationHtml, /通知时效/);
  assert.match(automationHtml, /过去 24 小时共 1\.2 千次运行。/);
  assert.match(governanceHtml, /治理/);
  assert.match(governanceHtml, /租户审计账本/);
  assert.match(governanceHtml, /合规事项/);
  assert.match(governanceHtml, /治理与合规/);
  assert.match(governanceHtml, /运营负责人/);
  assert.match(governanceHtml, /媒体归档/);
  assert.match(governanceHtml, /服务总监/);
  assert.match(governanceHtml, /会话路由/);
  assert.match(governanceHtml, /实时值守/);
  assert.match(governanceHtml, /北京 RTC 区域/);
  assert.match(governanceHtml, /近 95% 的签名请求在 118 毫秒内完成/);
  assert.match(governanceHtml, /投影延迟仍在租户承诺时延范围内/);

  assert.match(dashboardHtml, /把当班队列、实时链路、媒体与治理异常压成一个接管面/);
  assert.match(conversationsHtml, /优先处理高价值和高情绪波动会话/);
  assert.match(realtimeHtml, /判断是否需要把实时问题上升到值守操作面/);
  assert.match(mediaHtml, /素材上传、转码和绑定状态统一跟踪/);
  assert.match(automationHtml, /工作流与通知的稳定性不应该被埋在平台日志里/);
  assert.match(governanceHtml, /所有关键操作都应该带着范围和结果被记录/);

  assert.doesNotMatch(dashboardHtml, /Dashboard|Inspect Queue|Review Governance|Shift queue pressure|Tenant operations overview|VIP queue|Bot assist|Night shift readiness|Realtime ack lag|Automation retries/);
  assert.doesNotMatch(dashboardHtml, /4\.1k/);
  assert.doesNotMatch(conversationsHtml, /Conversations|Agent handoff desk|High-risk conversations|System Channels|Conversation operations|New inbox|Bot assist|VIP escalated|Operations broadcast|A1 调度席位|A3 电商席位|A5 高级队列|SLA|P1\/P2|>P1<|>P2<|Arc One|Kite South|North Dock/);
  assert.doesNotMatch(realtimeHtml, /Realtime|Session and presence posture|Realtime subscription load|Device sync board|Operator event trail|Realtime posture|Session resume success|Presence heartbeat lag|Order alerts|Store service inbox|operator/);
  assert.doesNotMatch(mediaHtml, /Media & RTC|Media lifecycle desk|Provider readiness|Stream session ledger|Media and RTC workspace|Archive|Transcoding|Media provider|fallback|checkpoint|vip-aftercare-17|ops-warroom-peak|merchant-onboarding-3|refund-proof-240409\.zip|campaign-hero-cut\.mp4|store-audio-note\.m4a/);
  assert.doesNotMatch(automationHtml, /Automation|Execution posture|Workflow execution desk|Delivery surface|Operator playbooks|Automation and notifications|Workflow success|Retries waiting|Refund recovery|VIP fallback SMS|通知 SLA|1\.2k/);
  assert.doesNotMatch(governanceHtml, /Governance|Tenant audit ledger|Health and binding posture|Runtime diagnostics|Compliance actions|Governance and compliance|Provider binding preview|Reviewed|Media signing|provider health|Runtime dir evidence complete|ops\.lead|media\.archive|service\.director|conversation\.routing|realtime\.sre|rtc\.region\.beijing|P95|SLO/);

  assert.doesNotMatch(dashboardHtml, /鎶婂綋鐝槦鍒椼€/);
  assert.doesNotMatch(conversationsHtml, /浼樺厛澶勭悊楂樹环鍊/);
  assert.doesNotMatch(realtimeHtml, /鍒ゆ柇鏄惁闇€瑕/);
  assert.doesNotMatch(mediaHtml, /绱犳潗涓婁紶/);
  assert.doesNotMatch(automationHtml, /宸ヤ綔娴佷笌閫氱煡/);
  assert.doesNotMatch(governanceHtml, /鎵€鏈夊叧閿搷浣/);
});
