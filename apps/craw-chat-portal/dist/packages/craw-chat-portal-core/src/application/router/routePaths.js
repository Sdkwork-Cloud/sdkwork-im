export const PORTAL_ROUTE_PATHS = {
  home: '/',
  login: '/login',
  console: '/console',
  dashboard: '/console/dashboard',
  conversations: '/console/conversations',
  realtime: '/console/realtime',
  media: '/console/media',
  automation: '/console/automation',
  governance: '/console/governance',
};

export const PORTAL_CONSOLE_ROUTE_PATHS = Object.freeze([
  PORTAL_ROUTE_PATHS.dashboard,
  PORTAL_ROUTE_PATHS.conversations,
  PORTAL_ROUTE_PATHS.realtime,
  PORTAL_ROUTE_PATHS.media,
  PORTAL_ROUTE_PATHS.automation,
  PORTAL_ROUTE_PATHS.governance,
]);

export function resolvePortalPath(routeKey) {
  return PORTAL_ROUTE_PATHS[routeKey];
}

export function normalizePortalPath(pathname) {
  if (!pathname) {
    return PORTAL_ROUTE_PATHS.home;
  }

  if (pathname.length > 1 && pathname.endsWith('/')) {
    return pathname.slice(0, -1);
  }

  return pathname;
}
