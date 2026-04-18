import { renderMainLayout } from '../layouts/MainLayout.js';
import { renderPortalSiteLayout } from '../layouts/PortalSiteLayout.js';
import { initializeAppProviders } from '../providers/AppProviders.js';
import {
  resolveConsoleEntryPath,
  resolveLoginRedirectTarget,
  resolveUnknownPathRedirect,
  shouldPersistConsolePath,
} from '../router/navigation.js';
import { PORTAL_ROUTE_PATHS, normalizePortalPath } from '../router/routePaths.js';
import { portalRouteManifest } from '../router/routeManifest.js';
import { createPortalAuthStore } from '../../store/usePortalAuthStore.js';
import { createPortalShellStore } from '../../store/usePortalShellStore.js';
import { renderSurface } from '../../../../craw-chat-portal-commons/src/index.js';
import { renderPortalHomePage } from '../../../../craw-chat-portal-home/src/index.js';
import { renderPortalAuthPage } from '../../../../craw-chat-portal-auth/src/index.js';
import { renderPortalDashboardPage } from '../../../../craw-chat-portal-dashboard/src/index.js';
import { renderPortalConversationsPage } from '../../../../craw-chat-portal-conversations/src/index.js';
import { renderPortalRealtimePage } from '../../../../craw-chat-portal-realtime/src/index.js';
import { renderPortalMediaPage } from '../../../../craw-chat-portal-media/src/index.js';
import { renderPortalAutomationPage } from '../../../../craw-chat-portal-automation/src/index.js';
import { renderPortalGovernancePage } from '../../../../craw-chat-portal-governance/src/index.js';

function currentPath() {
  return normalizePortalPath(window.location.pathname);
}

function resolveInteractiveTarget(node) {
  if (!node) {
    return null;
  }

  const legacyInteractiveSelector = '[data-route],[data-command],[data-theme]';
  const interactiveSelector =
    `${legacyInteractiveSelector},[data-console-entry-mode],[data-default-console-path],[data-sidebar-console-path]`;

  if (typeof node.closest === 'function') {
    return node.closest(interactiveSelector);
  }

  let current = node.parentElement ?? null;
  while (current) {
    if (
      typeof current.matches === 'function' &&
      (current.matches(interactiveSelector) || current.matches(legacyInteractiveSelector))
    ) {
      return current;
    }
    current = current.parentElement ?? null;
  }

  return null;
}

function isDisabledInteractiveTarget(target) {
  if (!target) {
    return false;
  }

  if (target.disabled === true) {
    return true;
  }

  if (typeof target.getAttribute === 'function' && target.getAttribute('aria-disabled') === 'true') {
    return true;
  }

  if (typeof target.hasAttribute === 'function' && target.hasAttribute('disabled')) {
    return true;
  }

  return false;
}

function queryWithinApp(root, selector) {
  if (typeof root?.querySelector === 'function') {
    return root.querySelector(selector);
  }

  if (typeof document?.querySelector === 'function') {
    return document.querySelector(selector);
  }

  return null;
}

function queryAllWithinApp(root, selector) {
  if (typeof root?.querySelectorAll === 'function') {
    return Array.from(root.querySelectorAll(selector));
  }

  if (typeof document?.querySelectorAll === 'function') {
    return Array.from(document.querySelectorAll(selector));
  }

  return [];
}

function readInputValue(root, selector, { trim = true } = {}) {
  const input = queryWithinApp(root, selector);
  if (!input || typeof input.value !== 'string') {
    return null;
  }

  return trim ? input.value.trim() : input.value;
}

function readPortalSignInCredentials(root) {
  const tenantId = readInputValue(root, '[name="tenantId"]');
  const login = readInputValue(root, '[name="login"]');
  const password = readInputValue(root, '[name="password"]', { trim: false });

  if (tenantId === null && login === null && password === null) {
    return null;
  }

  return {
    tenantId: tenantId ?? '',
    login: login ?? '',
    password: password ?? '',
  };
}

function renderSiteState({ eyebrow, title, description, actions = '' }) {
  return renderPortalSiteLayout({
    body: renderSurface({
      eyebrow,
      title,
      description,
      body: '<div class="portal-state-panel__body"></div>',
      actions,
      className: 'portal-state-panel',
    }),
  });
}

export function createPortalProductApp(root) {
  const authStore = createPortalAuthStore();
  const shellStore = createPortalShellStore();
  const teardownThemeSync = initializeAppProviders({ shellStore });
  let lastObservedPath = currentPath();
  let pendingFocusSelector = null;
  let activeRenderId = 0;
  let destroyed = false;
  let lastFailureStage = null;
  let lastSignInCredentials = null;

  const renderers = {
    [PORTAL_ROUTE_PATHS.dashboard]: renderPortalDashboardPage,
    [PORTAL_ROUTE_PATHS.conversations]: renderPortalConversationsPage,
    [PORTAL_ROUTE_PATHS.realtime]: renderPortalRealtimePage,
    [PORTAL_ROUTE_PATHS.media]: renderPortalMediaPage,
    [PORTAL_ROUTE_PATHS.automation]: renderPortalAutomationPage,
    [PORTAL_ROUTE_PATHS.governance]: renderPortalGovernancePage,
  };

  function queueFocus(selector) {
    pendingFocusSelector = selector;
  }

  function queueSettingsFocus(selector) {
    queueFocus(`#portal-settings-panel ${selector}`);
  }

  function queueSettingsControlFocus(target) {
    if (!shellStore.getState().settingsOpen) {
      return;
    }

    if (target.dataset.theme) {
      queueSettingsFocus(`[data-theme="${target.dataset.theme}"]`);
      return;
    }

    if (target.dataset.consoleEntryMode) {
      queueSettingsFocus(`[data-console-entry-mode="${target.dataset.consoleEntryMode}"]`);
      return;
    }

    if (target.dataset.defaultConsolePath) {
      queueSettingsFocus(`[data-default-console-path="${target.dataset.defaultConsolePath}"]`);
      return;
    }

    if (target.dataset.sidebarConsolePath) {
      queueSettingsFocus(`[data-sidebar-console-path="${target.dataset.sidebarConsolePath}"]`);
      return;
    }

    if (target.dataset.command === 'toggle-sidebar' && typeof target.closest === 'function' && target.closest('#portal-settings-panel')) {
      queueSettingsFocus('[data-command="toggle-sidebar"]');
    }
  }

  function createRenderId() {
    activeRenderId += 1;
    return activeRenderId;
  }

  function canCommitRender(renderId) {
    return !destroyed && renderId === activeRenderId;
  }

  function flushPendingFocus() {
    if (!pendingFocusSelector) {
      return;
    }

    const target = queryWithinApp(root, pendingFocusSelector);
    pendingFocusSelector = null;

    if (typeof target?.focus === 'function') {
      target.focus();
    }
  }

  function trapSettingsFocus(event) {
    if (event.key !== 'Tab' || !shellStore.getState().settingsOpen) {
      return false;
    }

    const focusTargets = queryAllWithinApp(root, '#portal-settings-panel button').filter(
      (target) => typeof target?.focus === 'function' && !isDisabledInteractiveTarget(target),
    );

    if (focusTargets.length === 0) {
      return false;
    }

    const firstTarget = focusTargets[0];
    const lastTarget = focusTargets[focusTargets.length - 1];
    const activeElement = document?.activeElement ?? null;

    if (event.shiftKey) {
      if (activeElement === firstTarget || !focusTargets.includes(activeElement)) {
        event.preventDefault();
        lastTarget.focus();
        return true;
      }

      return false;
    }

    if (activeElement === lastTarget || !focusTargets.includes(activeElement)) {
      event.preventDefault();
      firstTarget.focus();
      return true;
    }

    return false;
  }

  function renderRoot(html, renderId = activeRenderId) {
    if (!canCommitRender(renderId)) {
      return false;
    }

    root.innerHTML = html;
    flushPendingFocus();
    return true;
  }

  function renderConsoleState({ routeEntry, shellState, user, workspace, eyebrow, title, description, actions = '' }) {
    return renderMainLayout({
      currentPath: routeEntry.path,
      currentRouteEntry: routeEntry,
      currentRouteLabel: routeEntry.productModule.displayName,
      pageHtml: `
        <div class="portal-page">
          ${renderSurface({
            eyebrow,
            title,
            description,
            body: '<div class="portal-state-panel__body"></div>',
            actions,
            className: 'portal-state-panel',
          })}
        </div>
      `,
      routeManifest: portalRouteManifest,
      shellState,
      user,
      workspace,
    });
  }

  function renderLoadingState({ path, authState, shellState, mode = 'bootstrap' }) {
    const routeEntry = portalRouteManifest.find((entry) => entry.path === path);
    const isPublicHomePath = path === PORTAL_ROUTE_PATHS.home;
    const isPublicLoginPath = path === PORTAL_ROUTE_PATHS.login;

    if (mode === 'sign-in') {
      return renderSiteState({
        eyebrow: '租户入口',
        title: '正在接入演示租户',
        description: '正在建立演示租户会话并准备控制台工作区。',
      });
    }

    if (
      mode === 'retry' &&
      authState.isAuthenticated &&
      routeEntry &&
      authState.user &&
      authState.workspace
    ) {
      return renderConsoleState({
        routeEntry,
        shellState,
        user: authState.user,
        workspace: authState.workspace,
        eyebrow: '模块刷新',
        title: '正在刷新工作区模块',
        description: '正在拉取当前路由的最新快照，不会中断当前控制台壳层。',
      });
    }

    if (path.startsWith(PORTAL_ROUTE_PATHS.console)) {
      return renderSiteState({
        eyebrow: '控制台恢复',
        title: '正在恢复租户控制台',
        description: '正在校验当前工作区会话并重新接入操作台壳层。',
      });
    }

    if (isPublicLoginPath) {
      return renderSiteState({
        eyebrow: '租户入口',
        title: '正在准备租户入口',
        description: '正在加载最新的租户登录说明，稍后展示登录控件。',
      });
    }

    if (isPublicHomePath) {
      return renderSiteState({
        eyebrow: '门户引导',
        title: '正在准备租户门户',
        description: '正在加载最新的租户概览，稍后展示门户入口。',
      });
    }

    return renderSiteState({
      eyebrow: '门户引导',
      title: '正在准备租户工作区',
      description: '正在加载最新的门户快照，稍后展示租户入口。',
    });
  }

  function renderFailureState({ path, authState, shellState, stage = 'snapshot' }) {
    const routeEntry = portalRouteManifest.find((entry) => entry.path === path);
    const isPublicHomeFailure =
      stage === 'snapshot' && path === PORTAL_ROUTE_PATHS.home;
    const isPublicLoginFailure =
      stage === 'snapshot' && path === PORTAL_ROUTE_PATHS.login;
    const title =
      stage === 'hydrate'
        ? '租户控制台暂时不可用'
        : stage === 'sign-in'
          ? '租户登录暂时不可用'
          : isPublicHomeFailure
            ? '租户门户暂时不可用'
            : isPublicLoginFailure
              ? '租户入口暂时不可用'
              : '模块数据暂时不可用';
    const description =
      stage === 'hydrate'
        ? '当前工作区会话校验失败。请在会话服务恢复后重试控制台恢复。'
        : stage === 'sign-in'
          ? '演示租户会话建立失败。请在认证服务恢复后重试登录。'
          : isPublicHomeFailure
            ? '最新租户概览加载失败。请在门户控制面恢复后重试。'
            : isPublicLoginFailure
              ? '租户登录说明加载失败。请在门户控制面恢复后重试。'
              : '最新工作区快照加载失败。请在上游数据恢复后重试模块同步。';
    const primaryActionLabel =
      stage === 'hydrate'
        ? '重试控制台恢复'
        : stage === 'sign-in'
          ? '重试登录'
          : isPublicHomeFailure
            ? '重试门户概览'
            : isPublicLoginFailure
              ? '重试租户入口'
              : '重试模块同步';
    const actions = `
      <button class="portal-button portal-button--primary" data-command="retry-render" type="button">
        ${primaryActionLabel}
      </button>
    `;

    if (authState.isAuthenticated && routeEntry && authState.user && authState.workspace) {
      return renderConsoleState({
        routeEntry,
        shellState,
        user: authState.user,
        workspace: authState.workspace,
        eyebrow: stage === 'hydrate' ? '控制台恢复' : '模块恢复',
        title,
        description,
        actions,
      });
    }

    return renderSiteState({
      eyebrow:
        stage === 'hydrate'
          ? '控制台恢复'
          : stage === 'sign-in'
            ? '租户入口恢复'
          : isPublicLoginFailure
            ? '租户入口恢复'
            : '门户恢复',
      title,
      description,
      actions,
    });
  }

  function retryRender() {
    if (lastFailureStage === 'hydrate') {
      retryHydration();
      return;
    }

    if (lastFailureStage === 'sign-in') {
      retrySignIn();
      return;
    }

    const renderId = createRenderId();
    lastFailureStage = null;
    renderRoot(
      renderLoadingState({
        path: currentPath(),
        authState: authStore.getState(),
        shellState: shellStore.getState(),
        mode: 'retry',
      }),
      renderId,
    );
    void renderApp();
  }

  function retryHydration() {
    const renderId = createRenderId();
    lastFailureStage = null;

    renderRoot(
      renderLoadingState({
        path: currentPath(),
        authState: authStore.getState(),
        shellState: shellStore.getState(),
        mode: 'retry',
      }),
      renderId,
    );

    void authStore.hydrate().catch(() => {
      if (!canCommitRender(renderId)) {
        return;
      }

      lastFailureStage = 'hydrate';
      const failureRenderId = createRenderId();
      renderRoot(
        renderFailureState({
          path: currentPath(),
          authState: authStore.getState(),
          shellState: shellStore.getState(),
          stage: 'hydrate',
        }),
        failureRenderId,
      );
    });
  }

  function retrySignIn(credentials = lastSignInCredentials) {
    const requestedCredentials =
      credentials && typeof credentials === 'object'
        ? {
            tenantId: credentials.tenantId ?? '',
            login: credentials.login ?? '',
            password: credentials.password ?? '',
          }
        : undefined;

    lastSignInCredentials = requestedCredentials ?? null;
    const renderId = createRenderId();
    lastFailureStage = null;

    renderRoot(
      renderLoadingState({
        path: currentPath(),
        authState: authStore.getState(),
        shellState: shellStore.getState(),
        mode: 'sign-in',
      }),
      renderId,
    );

    void authStore
      .signIn(requestedCredentials)
      .then(() => {
        if (!canCommitRender(renderId)) {
          return;
        }

        lastSignInCredentials = null;
        const activeShellState = shellStore.getState();
        navigate(
          resolveLoginRedirectTarget(window.location.search, {
            lastConsolePath: activeShellState.lastConsolePath,
            consoleEntryMode: activeShellState.consoleEntryMode,
            pinnedConsolePath: activeShellState.pinnedConsolePath,
          }),
          { replace: true },
        );
      })
      .catch(() => {
        if (!canCommitRender(renderId)) {
          return;
        }

        lastFailureStage = 'sign-in';
        const failureRenderId = createRenderId();
        renderRoot(
          renderFailureState({
            path: currentPath(),
            authState: authStore.getState(),
            shellState: shellStore.getState(),
            stage: 'sign-in',
          }),
          failureRenderId,
        );
      });
  }

  function closeSettings({ restoreFocus = false } = {}) {
    if (!shellStore.getState().settingsOpen) {
      return;
    }

    if (restoreFocus) {
      queueFocus('[data-command="toggle-settings"]');
    }

    shellStore.closeSettings();
  }

  function isSidebarVisibilityLocked(sidebarConsolePath) {
    if (!shouldPersistConsolePath(sidebarConsolePath)) {
      return false;
    }

    if (currentPath() === sidebarConsolePath) {
      return true;
    }

    const shellState = shellStore.getState();
    return shellState.consoleEntryMode === 'pinned' && shellState.pinnedConsolePath === sidebarConsolePath;
  }

  function navigate(path, { replace = false } = {}) {
    const nextPath = normalizePortalPath(path);
    if (nextPath === currentPath()) {
      return;
    }

    const hadSettingsOpen = shellStore.getState().settingsOpen;

    window.history[replace ? 'replaceState' : 'pushState']({}, '', nextPath);

    if (hadSettingsOpen) {
      closeSettings();
    }

    if (shouldPersistConsolePath(nextPath) && shellStore.getState().lastConsolePath !== nextPath) {
      shellStore.setLastConsolePath(nextPath);
    }

    window.dispatchEvent(new Event('popstate'));
  }

  async function renderApp() {
    const renderId = createRenderId();
    const path = currentPath();

    try {
      const authState = authStore.getState();
      const shellState = shellStore.getState();
      const routeEntry = portalRouteManifest.find((entry) => entry.path === path);

      if (path === PORTAL_ROUTE_PATHS.home) {
        lastFailureStage = null;
        const html = renderPortalSiteLayout({ body: await renderPortalHomePage() });
        renderRoot(html, renderId);
        return;
      }

      if (path === PORTAL_ROUTE_PATHS.login) {
        if (authState.isAuthenticated) {
          lastFailureStage = null;
          navigate(
            resolveConsoleEntryPath({
              isAuthenticated: true,
              lastConsolePath: shellState.lastConsolePath,
              consoleEntryMode: shellState.consoleEntryMode,
              pinnedConsolePath: shellState.pinnedConsolePath,
            }),
            { replace: true },
          );
          return;
        }

        lastFailureStage = null;
        renderRoot(renderPortalSiteLayout({ body: await renderPortalAuthPage() }), renderId);
        return;
      }

      if (path === PORTAL_ROUTE_PATHS.console) {
        lastFailureStage = null;
        navigate(
          resolveConsoleEntryPath({
            isAuthenticated: authState.isAuthenticated,
            lastConsolePath: shellState.lastConsolePath,
            consoleEntryMode: shellState.consoleEntryMode,
            pinnedConsolePath: shellState.pinnedConsolePath,
          }),
          { replace: true },
        );
        return;
      }

      if (!routeEntry) {
        lastFailureStage = null;
        navigate(
          resolveUnknownPathRedirect({
            isAuthenticated: authState.isAuthenticated,
            lastConsolePath: shellState.lastConsolePath,
            consoleEntryMode: shellState.consoleEntryMode,
            pinnedConsolePath: shellState.pinnedConsolePath,
          }),
          { replace: true },
        );
        return;
      }

      if (!authState.isAuthenticated) {
        lastFailureStage = null;
        navigate(
          resolveConsoleEntryPath({
            isAuthenticated: false,
            lastConsolePath: path,
            consoleEntryMode: shellState.consoleEntryMode,
            pinnedConsolePath: shellState.pinnedConsolePath,
          }),
          { replace: true },
        );
        return;
      }

      if (shellState.lastConsolePath !== path) {
        shellStore.setLastConsolePath(path);
      }

      const pageHtml = await renderers[path]();
      if (!canCommitRender(renderId)) {
        return;
      }

      lastFailureStage = null;
      const activeShellState = shellStore.getState();
      renderRoot(renderMainLayout({
        currentPath: path,
        currentRouteEntry: routeEntry,
        currentRouteLabel: routeEntry.productModule.displayName,
        pageHtml,
        routeManifest: portalRouteManifest,
        shellState: activeShellState,
        user: authState.user,
        workspace: authState.workspace,
      }), renderId);
    } catch {
      lastFailureStage = 'snapshot';
      renderRoot(
        renderFailureState({
          path,
          authState: authStore.getState(),
          shellState: shellStore.getState(),
        }),
        renderId,
      );
    }
  }

  async function handleClick(event) {
    const target = resolveInteractiveTarget(event.target);
    if (!target) {
      return;
    }

    if (isDisabledInteractiveTarget(target)) {
      return;
    }

    if (target.dataset.theme) {
      queueSettingsControlFocus(target);
      shellStore.setTheme(target.dataset.theme);
      return;
    }

    if (target.dataset.consoleEntryMode) {
      queueSettingsControlFocus(target);
      shellStore.setConsoleEntryMode(target.dataset.consoleEntryMode);
      return;
    }

    if (target.dataset.defaultConsolePath) {
      queueSettingsControlFocus(target);
      shellStore.setPinnedConsolePath(target.dataset.defaultConsolePath);
      return;
    }

    if (target.dataset.sidebarConsolePath) {
      if (isSidebarVisibilityLocked(target.dataset.sidebarConsolePath)) {
        return;
      }
      queueSettingsControlFocus(target);
      shellStore.toggleHiddenConsolePath(target.dataset.sidebarConsolePath);
      return;
    }

    if (target.dataset.route) {
      navigate(target.dataset.route);
      return;
    }

    switch (target.dataset.command) {
      case 'toggle-settings':
        if (shellStore.getState().settingsOpen) {
          closeSettings({ restoreFocus: true });
          return;
        }

        queueFocus('[data-command="close-settings"]');
        shellStore.toggleSettings();
        return;
      case 'close-settings':
        closeSettings({ restoreFocus: true });
        return;
      case 'dismiss-settings':
        if (event.target === target) {
          closeSettings({ restoreFocus: true });
        }
        return;
      case 'toggle-sidebar':
        queueSettingsControlFocus(target);
        shellStore.toggleSidebar();
        return;
      case 'reset-shell-preferences': {
        const hadSettingsOpen = shellStore.getState().settingsOpen;
        if (hadSettingsOpen) {
          queueFocus('[data-command="close-settings"]');
          shellStore.resetShellPreferences({ keepSettingsOpen: true });
          return;
        }

        shellStore.resetShellPreferences();
        return;
      }
      case 'retry-render':
        retryRender();
        return;
      case 'portal-sign-in':
        event.preventDefault?.();
        retrySignIn(readPortalSignInCredentials(root));
        return;
      case 'demo-sign-in':
        event.preventDefault?.();
        retrySignIn(lastSignInCredentials ?? readPortalSignInCredentials(root));
        return;
      case 'sign-out':
        closeSettings();
        lastSignInCredentials = null;
        authStore.signOut();
        navigate(PORTAL_ROUTE_PATHS.home, { replace: true });
        return;
      default:
        return;
    }
  }

  function handleKeydown(event) {
    if (trapSettingsFocus(event)) {
      return;
    }

    if (event.key !== 'Escape' || !shellStore.getState().settingsOpen) {
      return;
    }

    event.preventDefault();
    closeSettings({ restoreFocus: true });
  }

  function handleRouteChange() {
    const nextPath = currentPath();
    const pathChanged = nextPath !== lastObservedPath;
    lastObservedPath = nextPath;

    if (pathChanged && shellStore.getState().settingsOpen) {
      closeSettings();
      return;
    }

    void renderApp();
  }

  root.addEventListener('click', handleClick);
  window.addEventListener('keydown', handleKeydown);
  window.addEventListener('popstate', handleRouteChange);

  const unsubscribers = [
    authStore.subscribe(() => {
      void renderApp();
    }),
    shellStore.subscribe(() => {
      void renderApp();
    }),
  ];

  const bootstrapRenderId = createRenderId();
  lastFailureStage = null;
  renderRoot(
    renderLoadingState({
      path: currentPath(),
      authState: authStore.getState(),
      shellState: shellStore.getState(),
    }),
    bootstrapRenderId,
  );

  void authStore.hydrate().catch(() => {
    if (!canCommitRender(bootstrapRenderId)) {
      return;
    }

    if (
      currentPath() === PORTAL_ROUTE_PATHS.home ||
      currentPath() === PORTAL_ROUTE_PATHS.login
    ) {
      lastFailureStage = null;
      void renderApp();
      return;
    }

    lastFailureStage = 'hydrate';
    const failureRenderId = createRenderId();
    renderRoot(
      renderFailureState({
        path: currentPath(),
        authState: authStore.getState(),
        shellState: shellStore.getState(),
        stage: 'hydrate',
      }),
      failureRenderId,
    );
  });

  return () => {
    destroyed = true;
    root.removeEventListener('click', handleClick);
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('popstate', handleRouteChange);
    teardownThemeSync();
    for (const unsubscribe of unsubscribers) {
      unsubscribe();
    }
  };
}
