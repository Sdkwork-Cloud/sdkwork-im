import { PORTAL_CONSOLE_ROUTE_PATHS, PORTAL_ROUTE_PATHS, normalizePortalPath } from './routePaths.js';

function isKnownConsoleRoutePath(pathname) {
  return PORTAL_CONSOLE_ROUTE_PATHS.includes(normalizePortalPath(pathname));
}

function buildLoginPath(redirectPath) {
  if (!redirectPath || redirectPath === PORTAL_ROUTE_PATHS.dashboard) {
    return PORTAL_ROUTE_PATHS.login;
  }

  return `${PORTAL_ROUTE_PATHS.login}?redirect=${encodeURIComponent(redirectPath)}`;
}

function resolvePreferredConsolePath({
  lastConsolePath,
  consoleEntryMode = 'resume',
  pinnedConsolePath = PORTAL_ROUTE_PATHS.dashboard,
} = {}) {
  const restoredPath = isKnownConsoleRoutePath(normalizePortalPath(lastConsolePath || ''))
    ? normalizePortalPath(lastConsolePath)
    : PORTAL_ROUTE_PATHS.dashboard;
  const preferredPinnedPath = isKnownConsoleRoutePath(normalizePortalPath(pinnedConsolePath || ''))
    ? normalizePortalPath(pinnedConsolePath)
    : restoredPath;

  return consoleEntryMode === 'pinned' ? preferredPinnedPath : restoredPath;
}

export function resolveLoginRedirectTarget(search, preferences = {}) {
  const params = new URLSearchParams(search.startsWith('?') ? search.slice(1) : search);
  const redirect = normalizePortalPath(params.get('redirect') || '');

  if (isKnownConsoleRoutePath(redirect)) {
    return redirect;
  }

  return resolvePreferredConsolePath(preferences);
}

export function resolveConsoleEntryPath({ isAuthenticated, ...preferences }) {
  const preferredPath = resolvePreferredConsolePath(preferences);

  if (isAuthenticated) {
    return preferredPath;
  }

  return buildLoginPath(preferredPath);
}

export function resolveUnknownPathRedirect({
  isAuthenticated,
  lastConsolePath = null,
  consoleEntryMode = 'resume',
  pinnedConsolePath = PORTAL_ROUTE_PATHS.dashboard,
}) {
  return isAuthenticated
    ? resolveConsoleEntryPath({
        isAuthenticated: true,
        lastConsolePath,
        consoleEntryMode,
        pinnedConsolePath,
      })
    : PORTAL_ROUTE_PATHS.home;
}

export function shouldPersistConsolePath(pathname) {
  return isKnownConsoleRoutePath(normalizePortalPath(pathname));
}
