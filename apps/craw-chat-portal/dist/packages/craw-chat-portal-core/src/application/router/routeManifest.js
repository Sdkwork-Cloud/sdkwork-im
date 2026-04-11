import { PORTAL_NAVIGATION_GROUP_LABELS } from '../../../../craw-chat-portal-types/src/index.js';
import { PORTAL_ROUTE_PATHS } from './routePaths.js';

export const portalProductModules = [
  {
    moduleId: 'craw-chat-portal-dashboard',
    displayName: '总览台',
    routeKeys: ['dashboard'],
    capabilityTags: ['工作区总览', '队列压力', '响应时效'],
    requiredPermissions: ['portal.dashboard.read'],
    summary: '从队列压力、实时稳定性与治理态势总览租户当班运行。',
    commandDeck: {
      primaryActionLabel: '打开班次优先级',
      primaryActionRoute: '/console/dashboard',
      relatedRoutes: ['conversations', 'governance'],
    },
    navigation: { group: 'operations', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.operations, order: 10, sidebar: true },
  },
  {
    moduleId: 'craw-chat-portal-conversations',
    displayName: '会话',
    routeKeys: ['conversations'],
    capabilityTags: ['收件箱', '人工交接', '消息运营'],
    requiredPermissions: ['portal.conversations.read'],
    summary: '在同一工作台完成收件箱控制、流转交接、关注会话与系统频道处置。',
    commandDeck: {
      primaryActionLabel: '打开队列分诊',
      primaryActionRoute: '/console/conversations',
      relatedRoutes: ['dashboard', 'realtime'],
    },
    navigation: { group: 'operations', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.operations, order: 20, sidebar: true },
  },
  {
    moduleId: 'craw-chat-portal-realtime',
    displayName: '实时链路',
    routeKeys: ['realtime'],
    capabilityTags: ['会话恢复', '在线态势', '设备同步'],
    requiredPermissions: ['portal.realtime.read'],
    summary: '把会话恢复、在线状态延迟、事件积压与设备同步集中到一个操作面。',
    commandDeck: {
      primaryActionLabel: '打开实时演练手册',
      primaryActionRoute: '/console/realtime',
      relatedRoutes: ['conversations', 'governance'],
    },
    navigation: { group: 'operations', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.operations, order: 30, sidebar: true },
  },
  {
    moduleId: 'craw-chat-portal-media',
    displayName: '媒体与 RTC',
    routeKeys: ['media'],
    capabilityTags: ['素材生命周期', '流媒体会话', 'RTC 运维'],
    requiredPermissions: ['portal.media.read'],
    summary: '在同一工作区管理素材生命周期、流媒体会话、RTC 房间与供应商就绪度。',
    commandDeck: {
      primaryActionLabel: '检查媒体链路',
      primaryActionRoute: '/console/media',
      relatedRoutes: ['automation', 'governance'],
    },
    navigation: { group: 'experience', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.experience, order: 40, sidebar: true },
  },
  {
    moduleId: 'craw-chat-portal-automation',
    displayName: '自动化',
    routeKeys: ['automation'],
    capabilityTags: ['流程执行', '通知投递', '操作预案'],
    requiredPermissions: ['portal.automation.read'],
    summary: '通过监看重试队列、活动投放与预案就绪度，保持消息交付稳定。',
    commandDeck: {
      primaryActionLabel: '查看重试队列',
      primaryActionRoute: '/console/automation',
      relatedRoutes: ['dashboard', 'media'],
    },
    navigation: { group: 'enablement', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.enablement, order: 50, sidebar: true },
  },
  {
    moduleId: 'craw-chat-portal-governance',
    displayName: '治理',
    routeKeys: ['governance'],
    capabilityTags: ['审计', '供应商健康', '运行诊断'],
    requiredPermissions: ['portal.governance.read'],
    summary: '把审计证据、供应商健康与运行诊断串成闭环，收敛合规风险。',
    commandDeck: {
      primaryActionLabel: '查看治理账本',
      primaryActionRoute: '/console/governance',
      relatedRoutes: ['realtime', 'media'],
    },
    navigation: { group: 'governance', groupLabel: PORTAL_NAVIGATION_GROUP_LABELS.governance, order: 60, sidebar: true },
  },
];

export const portalRouteManifest = portalProductModules.flatMap((productModule) =>
  productModule.routeKeys.map((key) => ({
    key,
    path: PORTAL_ROUTE_PATHS[key],
    productModule,
  })),
);
