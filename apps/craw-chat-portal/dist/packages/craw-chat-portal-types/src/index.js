export const PORTAL_TOP_LEVEL_ROUTE_KEYS = ['home', 'login', 'console'];

export const PORTAL_ROUTE_KEYS = [
  'dashboard',
  'conversations',
  'realtime',
  'media',
  'automation',
  'governance',
];

export const PORTAL_NAVIGATION_GROUP_LABELS = {
  operations: '运营',
  experience: '体验',
  enablement: '支撑',
  governance: '治理',
};

export const PORTAL_THEME_OPTIONS = [
  {
    id: 'signal',
    label: '信号台',
    description: '钢青与琥珀配色，突出指挥中枢的清晰层次。',
  },
  {
    id: 'atlas',
    label: '纵横网格',
    description: '中性石墨基底，配合冷青强调。',
  },
  {
    id: 'ember',
    label: '余烬班次',
    description: '暖铜高亮，强化值守与告警氛围。',
  },
];

export const PORTAL_CONSOLE_ENTRY_OPTIONS = [
  {
    id: 'resume',
    label: '继续上次模块',
    description: '适合连续值守，重新进入控制台时恢复最近一次工作模块。',
  },
  {
    id: 'pinned',
    label: '固定进入模块',
    description: '适合标准班次，始终从指定模块开始当班处理。',
  },
];
