export const portalMockData = {
  home: {
    hero: {
      eyebrow: '租户即时通中枢',
      title: 'Craw Chat 租户门户',
      description:
        '面向租户的即时通信管理后台，把会话、实时链路、媒体、自动化与治理统一到一个运营视角。',
    },
    pillars: [
      {
        title: '会话统筹',
        description: '统一看板覆盖收件箱、人工交接、系统频道与服务时效风险。',
      },
      {
        title: '实时稳态',
        description: '把会话恢复、在线态势、订阅窗口和设备同步压缩到同一操作面。',
      },
      {
        title: '媒体与治理',
        description: '从素材、RTC 到审计、供应商健康，一套门户闭环。',
      },
    ],
  },
  auth: {
    eyebrow: '演示租户入口',
    title: '进入 Nebula Commerce IM',
    description:
      '当前已预置演示租户工作台，你可以直接进入控制台体验完整运营链路。',
    details: [
      { label: '工作区', value: 'Nebula Commerce IM' },
      { label: '角色', value: '租户运营负责人' },
      { label: '覆盖范围', value: '会话 / 实时链路 / 治理' },
    ],
    primaryActionLabel: '使用演示租户登录',
    secondaryActionLabel: '返回首页',
  },
  session: {
    token: 'tenant-demo-session',
    user: {
      name: '林涛',
      role: '租户运营负责人',
      email: 'lin.tao@nebula-commerce.example',
    },
  },
  workspace: {
    name: 'Nebula Commerce IM',
    slug: 'nebula-commerce-im',
    tier: 'Enterprise',
    region: 'CN-East / Multi-AZ',
    supportPlan: 'Platinum',
    seats: 84,
    activeBrands: 12,
    uptime: '99.983%',
  },
  dashboard: {
    hero: {
      title: '租户运营总览',
      description: '把当班队列、实时链路、媒体与治理异常压成一个接管面。',
      kpis: [
        { label: '今日消息量', value: '284 万', delta: '+12%', tone: 'positive', caption: '峰值 4.1 千条/秒' },
        { label: '待接待会话', value: '34', delta: '-18%', tone: 'positive', caption: 'VIP 6 / 普通 28' },
        { label: '恢复成功率', value: '99.42%', delta: '+0.7 个百分点', tone: 'positive', caption: '过去 6 小时' },
        { label: '治理告警', value: '3', delta: '2 个严重', tone: 'warning', caption: '需在 20 分钟内收敛' },
      ],
    },
    pressure: [
      { label: 'VIP 队列', caption: '高价值订单售后', value: '72%', percent: 72, tone: 'warning' },
      { label: '机器人辅助', caption: '需要人工兜底', value: '41%', percent: 41, tone: 'neutral' },
      { label: '夜班就绪度', caption: '跨时区值守准备', value: '88%', percent: 88, tone: 'positive' },
    ],
    posture: [
      { label: '实时确认延迟', value: '0.9 秒', status: '健康', tone: 'positive', description: '窗口确认仍在护栏范围内。' },
      { label: 'RTC 桥接', value: '2 个房间待处理', status: '观察', tone: 'warning', description: '两个会话正在等待区域回切。' },
      { label: '自动化重试', value: '7 个任务', status: '升级', tone: 'critical', description: '优惠券恢复预案需要人工复核。' },
    ],
    priorities: [
      { title: '稳定 VIP 退款队列', description: '将两条会话从机器人辅助转交给高级电商席位。' },
      { title: '确认 RTC 回切库存', description: '晚高峰前回放北京区域的回调样本。' },
      { title: '收口供应商绑定漂移', description: '媒体回调路由与已提交的运行时绑定不一致。' },
    ],
    timeline: [
      { title: '09:20 交接高峰', description: '活动群发后人工兜底需求上升 17%。' },
      { title: '10:05 在线态重同步', description: '恢复路径在 47 秒内修复了陈旧设备群。' },
      { title: '10:40 审计导出就绪', description: '晨间治理包已可用于合规复核。' },
    ],
  },
  conversations: {
    hero: {
      title: '会话运营面',
      description: '看住收件箱、人工交接、消息编辑与已读状态流转，保证租户服务面不失真。',
    },
    pipeline: [
      { label: '新进收件箱', value: '34', percent: 34, caption: '等待首次响应', tone: 'warning' },
      { label: '机器人辅助', value: '18', percent: 18, caption: '需要人工兜底', tone: 'neutral' },
      { label: 'VIP 升级', value: '6', percent: 6, caption: '紧急与高优先会话', tone: 'critical' },
    ],
    handoffs: [
      { conversation: '退款 / #IM-4821', owner: '账单机器人', next: '电商席位三组', wait: '2 分钟', priority: '紧急' },
      { conversation: '发货 / #IM-4788', owner: '履约机器人', next: '调度席位一组', wait: '5 分钟', priority: '高优先' },
      { conversation: '跨境 / #IM-4670', owner: '坐席协同', next: '高级队列五组', wait: '7 分钟', priority: '紧急' },
    ],
    watchlist: [
      { topic: 'VIP 退款聚集', customer: '星舟零售', unread: '5', sentiment: '脆弱', sla: '04:12' },
      { topic: '支付失败循环', customer: '南城优选', unread: '3', sentiment: '升级中', sla: '06:48' },
      { topic: '发货改期', customer: '北港百货', unread: '2', sentiment: '恢复中', sla: '09:05' },
    ],
    systemChannels: [
      { title: '运营广播', description: '活动群发与路由变更的服务播报。' },
      { title: '风控冻结频道', description: '支付复核介入的共享系统通知。' },
      { title: '门店运营战情室', description: '旺季升级流的回放协同频道。' },
    ],
  },
  realtime: {
    hero: {
      title: '实时链路态势',
      description: '从会话恢复到订阅窗口，把实时链路稳定性拉到租户操作视角。',
    },
    posture: [
      { label: '会话恢复成功率', value: '99.42%', status: '稳定', tone: 'positive', description: '过去 6 小时完成 1.84 万次恢复。' },
      { label: '在线心跳延迟', value: '1.4 秒', status: '观察', tone: 'warning', description: '东区 Android 设备群存在抖动。' },
      { label: '实时积压', value: '182 事件', status: '稳定', tone: 'positive', description: '低于 400 事件护栏。' },
    ],
    subscriptions: [
      { label: '订单提醒', value: '92%', percent: 92, caption: '租户级路由主干', tone: 'positive' },
      { label: '门店服务收件箱', value: '61%', percent: 61, caption: '高峰积压压力', tone: 'warning' },
      { label: '活动战情室', value: '47%', percent: 47, caption: '群发后确认漂移监控', tone: 'neutral' },
    ],
    devices: [
      { owner: '门店运营 01', device: 'iPhone 15 Pro', sync: '7 秒前', lag: '0.8 秒', state: '健康' },
      { owner: '调度 12', device: 'Android 平板', sync: '13 秒前', lag: '1.6 秒', state: '关注' },
      { owner: '支持负责人', device: 'MacBook Air', sync: '5 秒前', lag: '0.3 秒', state: '健康' },
    ],
    events: [
      { title: '在线群组重建完成', description: '112 台陈旧设备在网络抖动后恢复。' },
      { title: '确认窗口已收窄', description: '活动战情室订阅者的回放游标已重置。' },
      { title: '断链隔离已生效', description: '一枚过期令牌已被引导到重新登录。' },
    ],
  },
  media: {
    hero: {
      title: '媒体与 RTC 工作面',
      description: '把上传链路、资产落盘、流式会话和 RTC 房间状态统一到一个工作面。',
    },
    assets: [
      { asset: '退款凭证-240409.zip', type: '压缩包', state: '就绪', queue: '0 分钟', owner: '电商席位' },
      { asset: '活动主视觉剪辑.mp4', type: '视频', state: '转码中', queue: '3 分钟', owner: '增长运营' },
      { asset: '门店语音备注.m4a', type: '音频', state: '已绑定', queue: '0 分钟', owner: '门店运营' },
    ],
    rtcSessions: [
      { room: 'VIP 售后关怀 17 号房', region: '杭州', participants: '4 人', state: '通话中', note: '已开启录制' },
      { room: '大促值守协同室', region: '北京', participants: '12 人', state: '回切中', note: '主供应商性能下降' },
      { room: '商家入驻培训 3 号房', region: '上海', participants: '3 人', state: '已排程', note: '18 分钟后开始' },
    ],
    providers: [
      { label: '媒体供应商', value: '健康', status: '健康', tone: 'positive', description: '下载地址签名延迟仍在基线内。' },
      { label: 'RTC 供应商', value: '关注', status: '回切中', tone: 'warning', description: '一个区域仍在使用备用资源池。' },
      { label: '录制归档', value: '漂移', status: '严重', tone: 'critical', description: '归档回调目标与已提交绑定不一致。' },
    ],
    streams: [
      { title: '活动现场记录', description: '当前流仍在写入，已追加 124 帧，检查点 11 已提交。' },
      { title: '商家入驻转写', description: '帧序列稳定，等待结束信号。' },
      { title: '门店战情室便笺', description: '陈旧帧突增后已挂起中止阈值。' },
    ],
  },
  automation: {
    hero: {
      title: '自动化与通知',
      description: '把工作流执行、通知投递和运营预案放到一个可接管的执行面。',
    },
    summary: [
      { label: '工作流成功率', value: '97.8%', status: '稳定', tone: 'positive', description: '过去 24 小时共 1.2 千次运行。' },
      { label: '待处理重试', value: '7', status: '需复核', tone: 'warning', description: '主要集中在优惠券和退款提醒。' },
      { label: '通知时效', value: '99.1%', status: '稳定', tone: 'positive', description: '推送 + 短信混合投递。' },
    ],
    executions: [
      { flow: '退款恢复', owner: '电商运营', state: '重试中', age: '8 分钟', impact: '92 位客户' },
      { flow: '夜班准备', owner: '运营控制台', state: '已完成', age: '14 分钟', impact: '3 个小队' },
      { flow: '门店故障广播', owner: '现场支持', state: '排队中', age: '2 分钟', impact: '18 家门店' },
    ],
    notifications: [
      { task: 'VIP 回切短信', channel: '短信', state: '已投递 98%', drift: '0.3 个百分点' },
      { task: '商家提醒', channel: '推送', state: '已投递 95%', drift: '1.4 个百分点' },
      { task: '运营广播', channel: '系统', state: '已投递 100%', drift: '0 个百分点' },
    ],
    playbooks: [
      { title: '退款救援', description: '第二次机器人拒绝后立即转人工。' },
      { title: '高峰排班', description: 'VIP 等待超过 4 分钟时开启次级队列。' },
      { title: 'RTC 回切演练', description: '晚高峰前核验供应商故障切换能力。' },
    ],
  },
  governance: {
    hero: {
      title: '治理与合规',
      description: '把审计、供应商健康、运行诊断和租户合规视图集中起来，支持持续值守。',
    },
    auditRecords: [
      { action: '供应商绑定预览', actor: '运营负责人', scope: '媒体归档', status: '已复核' },
      { action: 'VIP 队列覆盖', actor: '服务总监', scope: '会话路由', status: '已应用' },
      { action: 'RTC 回切演练', actor: '实时值守', scope: '北京 RTC 区域', status: '已留痕' },
    ],
    providerHealth: [
      { label: '媒体签名', value: '健康', status: '健康', tone: 'positive', description: '近 95% 的签名请求在 118 毫秒内完成' },
      { label: 'RTC 区域回调', value: '关注', status: '回切中', tone: 'warning', description: '等待回放校验' },
      { label: '用户模块绑定', value: '一致', status: '一致', tone: 'positive', description: '未发现漂移' },
    ],
    diagnostics: [
      { title: '运行目录证据已齐备', description: '晨检包已写入治理账本。' },
      { title: '回放状态正常', description: '投影延迟仍在租户承诺时延范围内。' },
      { title: '供应商漂移未收口', description: '归档回调路径与现行策略快照不一致。' },
    ],
    checklist: [
      { title: '18:00 前收口归档漂移', description: '影响客服升级场景的录制合规。' },
      { title: '导出午后审计包', description: '用于租户信任复核。' },
      { title: '复核北京 RTC 回切', description: '保证活动高峰晚班稳定。' },
    ],
  },
};
