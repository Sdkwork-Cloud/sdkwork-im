export const PORTAL_SHELL_STORAGE_KEY = 'craw-chat-portal.shell.v1';

function normalizeShellPreferences(value) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    return null;
  }

  const normalized = {};

  if (typeof value.lastConsolePath === 'string') {
    normalized.lastConsolePath = value.lastConsolePath;
  }

  if (typeof value.sidebarCollapsed === 'boolean') {
    normalized.sidebarCollapsed = value.sidebarCollapsed;
  }

  if (typeof value.consoleEntryMode === 'string') {
    normalized.consoleEntryMode = value.consoleEntryMode;
  }

  if (Array.isArray(value.hiddenConsolePaths)) {
    normalized.hiddenConsolePaths = value.hiddenConsolePaths.filter((item) => typeof item === 'string');
  }

  if (typeof value.pinnedConsolePath === 'string') {
    normalized.pinnedConsolePath = value.pinnedConsolePath;
  }

  if (typeof value.theme === 'string') {
    normalized.theme = value.theme;
  }

  return Object.keys(normalized).length > 0 ? normalized : null;
}

export function readShellPreferences() {
  if (typeof window === 'undefined') {
    return null;
  }

  try {
    const raw = window.localStorage.getItem(PORTAL_SHELL_STORAGE_KEY);
    return raw ? normalizeShellPreferences(JSON.parse(raw)) : null;
  } catch {
    return null;
  }
}

export function persistShellPreferences(preferences) {
  if (typeof window === 'undefined') {
    return;
  }

  try {
    window.localStorage.setItem(PORTAL_SHELL_STORAGE_KEY, JSON.stringify(preferences));
  } catch {
    // Storage failures should not break portal navigation or shell state updates.
  }
}
