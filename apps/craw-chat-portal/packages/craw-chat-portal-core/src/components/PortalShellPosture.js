import { PORTAL_THEME_OPTIONS } from '../../../craw-chat-portal-types/src/index.js';

const DEFAULT_SHELL_PREFERENCES = {
  sidebarCollapsed: false,
  consoleEntryMode: 'resume',
  pinnedConsolePath: '/console/dashboard',
  theme: 'signal',
};

function normalizeConsoleEntryMode(consoleEntryMode) {
  return consoleEntryMode === 'pinned' ? 'pinned' : 'resume';
}

function normalizeHiddenConsolePaths(hiddenConsolePaths) {
  return Array.isArray(hiddenConsolePaths) ? hiddenConsolePaths : [];
}

function resolveRouteEntry(routeManifest = [], path) {
  return routeManifest.find((entry) => entry.path === path) ?? null;
}

function resolvePinnedConsolePath(routeManifest = [], shellState = {}) {
  if (routeManifest.some((entry) => entry.path === shellState.pinnedConsolePath)) {
    return shellState.pinnedConsolePath;
  }

  return routeManifest[0]?.path ?? DEFAULT_SHELL_PREFERENCES.pinnedConsolePath;
}

function resolveActiveConsolePath({ routeManifest = [], currentPath = '', shellState = {} }) {
  if (resolveRouteEntry(routeManifest, currentPath)) {
    return currentPath;
  }

  if (resolveRouteEntry(routeManifest, shellState.lastConsolePath)) {
    return shellState.lastConsolePath;
  }

  return resolvePinnedConsolePath(routeManifest, shellState);
}

function deriveEffectiveHiddenConsoleCount({
  currentPath = '',
  hiddenConsolePaths = [],
  consoleEntryMode = 'resume',
  pinnedConsolePath = '',
}) {
  const lockedPinnedPath = consoleEntryMode === 'pinned' ? pinnedConsolePath : null;

  return hiddenConsolePaths.filter((path) => path !== currentPath && path !== lockedPinnedPath).length;
}

export function resolveRouteLabel(routeManifest = [], path, fallback) {
  return resolveRouteEntry(routeManifest, path)?.productModule.displayName ?? fallback;
}

export function resolveThemeLabel(themeId) {
  return PORTAL_THEME_OPTIONS.find((theme) => theme.id === themeId)?.label ?? '信号台';
}

export function deriveShellCustomizationCount(shellState = {}) {
  const hiddenConsolePaths = normalizeHiddenConsolePaths(shellState.hiddenConsolePaths);

  return [
    shellState.sidebarCollapsed !== DEFAULT_SHELL_PREFERENCES.sidebarCollapsed,
    normalizeConsoleEntryMode(shellState.consoleEntryMode) !== DEFAULT_SHELL_PREFERENCES.consoleEntryMode,
    shellState.pinnedConsolePath !== DEFAULT_SHELL_PREFERENCES.pinnedConsolePath,
    shellState.theme !== DEFAULT_SHELL_PREFERENCES.theme,
    hiddenConsolePaths.length > 0,
  ].filter(Boolean).length;
}

export function deriveShellResetLabel(shellState = {}) {
  const customizationCount = deriveShellCustomizationCount(shellState);

  return customizationCount > 0 ? `恢复 ${customizationCount} 项偏好` : '恢复默认偏好';
}

export function deriveShellResetAction(shellState = {}) {
  const customizationCount = deriveShellCustomizationCount(shellState);

  if (customizationCount > 0) {
    return {
      customizationCount,
      disabled: false,
      label: `恢复 ${customizationCount} 项偏好`,
    };
  }

  return {
    customizationCount,
    disabled: true,
    label: '当前已是标准布局',
  };
}

export function isShellCustomized(shellState = {}) {
  return deriveShellCustomizationCount(shellState) > 0;
}

export function deriveTopbarStatusItems(shellState = {}) {
  const consoleEntryMode = normalizeConsoleEntryMode(shellState.consoleEntryMode);
  const customizationCount = deriveShellCustomizationCount(shellState);

  return [
    {
      label: '接管方式',
      value: consoleEntryMode === 'pinned' ? '固定入口值守' : '继续上次模块',
    },
    {
      label: '布局状态',
      value: customizationCount > 0 ? `已个性化布局 · ${customizationCount} 项偏好` : '标准布局',
    },
    {
      label: '侧栏状态',
      value: shellState.sidebarCollapsed ? '收起' : '展开',
    },
  ];
}

export function deriveCommandDeckPostureCards({ currentRouteEntry, routeManifest, shellState = {} }) {
  const consoleEntryMode = normalizeConsoleEntryMode(shellState.consoleEntryMode);
  const pinnedConsolePath = resolvePinnedConsolePath(routeManifest, shellState);
  const pinnedConsoleLabel = resolveRouteLabel(routeManifest, pinnedConsolePath, '总览台');
  const hiddenConsolePaths = normalizeHiddenConsolePaths(shellState.hiddenConsolePaths);
  const effectiveHiddenCount = deriveEffectiveHiddenConsoleCount({
    currentPath: currentRouteEntry.path,
    hiddenConsolePaths,
    consoleEntryMode,
    pinnedConsolePath,
  });
  const activeThemeLabel = resolveThemeLabel(shellState.theme);

  return [
    {
      label: '当前值守姿态',
      value: consoleEntryMode === 'pinned' ? '固定进入模块' : '继续上次模块',
      detail:
        consoleEntryMode === 'pinned'
          ? `重新进入控制台时，从 ${pinnedConsoleLabel} 开始当班。`
          : '重新进入控制台时，跟随最近一次工作模块继续接管。',
    },
    {
      label: '默认入口',
      value: consoleEntryMode === 'pinned' ? pinnedConsoleLabel : '跟随最近工作面',
      detail:
        consoleEntryMode === 'pinned'
          ? '固定入口会在侧栏中保持优先可见。'
          : '系统会根据最近一次工作路由恢复操作面。',
    },
    {
      label: '侧栏聚焦',
      value: effectiveHiddenCount > 0 ? `已收起 ${effectiveHiddenCount} 个模块` : '全部模块可见',
      detail: shellState.sidebarCollapsed ? '侧栏已收起' : '侧栏展开',
    },
    {
      label: '工作台主题',
      value: activeThemeLabel,
      detail:
        shellState.theme === DEFAULT_SHELL_PREFERENCES.theme
          ? '标准主题'
          : '已启用个性化主题',
    },
  ];
}

export function deriveSettingsPanelSummary({ routeManifest = [], currentPath = '', shellState = {} }) {
  const activeConsolePath = resolveActiveConsolePath({ routeManifest, currentPath, shellState });
  const currentRouteEntry = resolveRouteEntry(routeManifest, activeConsolePath);
  const currentRouteLabel =
    currentRouteEntry?.productModule?.displayName ?? resolveRouteLabel(routeManifest, activeConsolePath, '当前工作面');
  const currentGroupLabel = currentRouteEntry?.productModule?.navigation?.groupLabel ?? '值守域';
  const currentCapability = currentRouteEntry?.productModule?.capabilityTags?.[0] ?? '工作面';
  const consoleEntryMode = normalizeConsoleEntryMode(shellState.consoleEntryMode);
  const pinnedConsolePath = resolvePinnedConsolePath(routeManifest, shellState);
  const pinnedConsoleLabel = resolveRouteLabel(routeManifest, pinnedConsolePath, '总览台');
  const hiddenConsolePaths = normalizeHiddenConsolePaths(shellState.hiddenConsolePaths);
  const hiddenConsoleCount = deriveEffectiveHiddenConsoleCount({
    currentPath: activeConsolePath,
    hiddenConsolePaths,
    consoleEntryMode,
    pinnedConsolePath,
  });
  const activeThemeLabel = resolveThemeLabel(shellState.theme);
  const customizationCount = deriveShellCustomizationCount(shellState);

  return {
    activeConsolePath,
    label: customizationCount > 0 ? `已同步 ${customizationCount} 项值守偏好` : '标准值守布局',
    description:
      customizationCount > 0
        ? `当前布局已同步 ${customizationCount} 项值守偏好，进入策略、主题与侧栏焦点都会按当前值守方式恢复。`
        : '当前布局仍保持标准值守方式，可按班次调整进入策略、主题和侧栏聚焦。',
    items: [
      {
        label: '当前值守模块',
        value: currentRouteLabel,
        detail: `${currentGroupLabel} · ${currentCapability}`,
      },
      {
        label: '接管方式',
        value: consoleEntryMode === 'pinned' ? '固定进入模块' : '继续上次模块',
        detail: `${consoleEntryMode === 'pinned' ? '固定入口' : '待命入口'}：${pinnedConsoleLabel}`,
      },
      {
        label: '侧栏焦点',
        value: hiddenConsoleCount > 0 ? `已收起 ${hiddenConsoleCount} 个模块` : '全部模块可见',
        detail: shellState.sidebarCollapsed ? '侧栏已收起' : '侧栏展开',
      },
      {
        label: '工作台主题',
        value: activeThemeLabel,
        detail:
          shellState.theme === DEFAULT_SHELL_PREFERENCES.theme
            ? '标准主题'
            : '已启用个性化主题',
      },
    ],
  };
}
