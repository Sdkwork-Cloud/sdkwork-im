import { PORTAL_THEME_OPTIONS } from '../../../craw-chat-portal-types/src/index.js';
import { shouldPersistConsolePath } from '../application/router/navigation.js';
import { createStore } from '../lib/createStore.js';
import { persistShellPreferences, readShellPreferences } from '../lib/portalPreferences.js';

const validThemeIds = new Set(PORTAL_THEME_OPTIONS.map((theme) => theme.id));
const validConsoleEntryModes = new Set(['resume', 'pinned']);

const defaultState = {
  settingsOpen: false,
  sidebarCollapsed: false,
  consoleEntryMode: 'resume',
  hiddenConsolePaths: [],
  pinnedConsolePath: '/console/dashboard',
  lastConsolePath: '/console/dashboard',
  theme: PORTAL_THEME_OPTIONS[0].id,
};

function sanitizeShellState(persisted) {
  return {
    settingsOpen: false,
    sidebarCollapsed:
      typeof persisted?.sidebarCollapsed === 'boolean'
        ? persisted.sidebarCollapsed
        : defaultState.sidebarCollapsed,
    consoleEntryMode:
      typeof persisted?.consoleEntryMode === 'string' && validConsoleEntryModes.has(persisted.consoleEntryMode)
        ? persisted.consoleEntryMode
        : defaultState.consoleEntryMode,
    hiddenConsolePaths: Array.isArray(persisted?.hiddenConsolePaths)
      ? [...new Set(persisted.hiddenConsolePaths.filter((path) => shouldPersistConsolePath(path)))]
      : defaultState.hiddenConsolePaths,
    pinnedConsolePath:
      typeof persisted?.pinnedConsolePath === 'string' && shouldPersistConsolePath(persisted.pinnedConsolePath)
        ? persisted.pinnedConsolePath
        : defaultState.pinnedConsolePath,
    lastConsolePath:
      typeof persisted?.lastConsolePath === 'string' && shouldPersistConsolePath(persisted.lastConsolePath)
        ? persisted.lastConsolePath
        : defaultState.lastConsolePath,
    theme:
      typeof persisted?.theme === 'string' && validThemeIds.has(persisted.theme)
        ? persisted.theme
        : defaultState.theme,
  };
}

export function createPortalShellStore() {
  const persisted = readShellPreferences();
  const store = createStore(sanitizeShellState(persisted));

  store.subscribe((state) => {
    persistShellPreferences({
      lastConsolePath: state.lastConsolePath,
      sidebarCollapsed: state.sidebarCollapsed,
      consoleEntryMode: state.consoleEntryMode,
      hiddenConsolePaths: state.hiddenConsolePaths,
      pinnedConsolePath: state.pinnedConsolePath,
      theme: state.theme,
    });
  });

  return {
    ...store,
    toggleSettings() {
      store.setState((state) => ({ settingsOpen: !state.settingsOpen }));
    },
    closeSettings() {
      store.setState({ settingsOpen: false });
    },
    toggleSidebar() {
      store.setState((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
    },
    setConsoleEntryMode(consoleEntryMode) {
      if (!validConsoleEntryModes.has(consoleEntryMode)) {
        return;
      }
      store.setState({ consoleEntryMode });
    },
    setPinnedConsolePath(pinnedConsolePath) {
      if (!shouldPersistConsolePath(pinnedConsolePath)) {
        return;
      }
      store.setState({ pinnedConsolePath });
    },
    toggleHiddenConsolePath(hiddenConsolePath) {
      if (!shouldPersistConsolePath(hiddenConsolePath)) {
        return;
      }

      store.setState((state) => ({
        hiddenConsolePaths: state.hiddenConsolePaths.includes(hiddenConsolePath)
          ? state.hiddenConsolePaths.filter((path) => path !== hiddenConsolePath)
          : [...state.hiddenConsolePaths, hiddenConsolePath],
      }));
    },
    setTheme(theme) {
      if (!validThemeIds.has(theme)) {
        return;
      }
      store.setState({ theme });
    },
    setLastConsolePath(lastConsolePath) {
      if (!shouldPersistConsolePath(lastConsolePath)) {
        return;
      }
      store.setState({ lastConsolePath });
    },
    resetShellPreferences(options = {}) {
      store.setState((state) => ({
        settingsOpen: options.keepSettingsOpen === true ? state.settingsOpen : false,
        sidebarCollapsed: defaultState.sidebarCollapsed,
        consoleEntryMode: defaultState.consoleEntryMode,
        hiddenConsolePaths: defaultState.hiddenConsolePaths,
        pinnedConsolePath: defaultState.pinnedConsolePath,
        lastConsolePath: state.lastConsolePath,
        theme: defaultState.theme,
      }));
    },
  };
}
