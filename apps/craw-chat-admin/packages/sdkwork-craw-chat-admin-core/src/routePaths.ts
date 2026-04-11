import type { AdminRouteKey } from 'sdkwork-craw-chat-admin-types';

export const ADMIN_ROUTE_PATHS = {
  ROOT: '/',
  AUTH: '/auth',
  LOGIN: '/login',
  REGISTER: '/register',
  FORGOT_PASSWORD: '/forgot-password',
  OVERVIEW: '/overview',
  TENANTS: '/tenants',
  USERS: '/users',
  CONVERSATIONS: '/conversations',
  MESSAGES: '/messages',
  GROUPS: '/groups',
  MODERATION: '/moderation',
  AUTOMATION: '/automation',
  ANNOUNCEMENTS: '/announcements',
  REALTIME: '/realtime',
  SYSTEM: '/system',
  SETTINGS: '/settings',
} as const;

const AUTH_ROUTE_PATHS = [
  ADMIN_ROUTE_PATHS.AUTH,
  ADMIN_ROUTE_PATHS.LOGIN,
  ADMIN_ROUTE_PATHS.REGISTER,
  ADMIN_ROUTE_PATHS.FORGOT_PASSWORD,
] as const;

function normalizeAdminPathname(pathname: string): string {
  return pathname.endsWith('/') && pathname !== '/' ? pathname.slice(0, -1) : pathname;
}

export const adminRoutePathByKey: Record<AdminRouteKey, string> = {
  overview: ADMIN_ROUTE_PATHS.OVERVIEW,
  tenants: ADMIN_ROUTE_PATHS.TENANTS,
  users: ADMIN_ROUTE_PATHS.USERS,
  conversations: ADMIN_ROUTE_PATHS.CONVERSATIONS,
  messages: ADMIN_ROUTE_PATHS.MESSAGES,
  groups: ADMIN_ROUTE_PATHS.GROUPS,
  moderation: ADMIN_ROUTE_PATHS.MODERATION,
  automation: ADMIN_ROUTE_PATHS.AUTOMATION,
  announcements: ADMIN_ROUTE_PATHS.ANNOUNCEMENTS,
  realtime: ADMIN_ROUTE_PATHS.REALTIME,
  system: ADMIN_ROUTE_PATHS.SYSTEM,
  settings: ADMIN_ROUTE_PATHS.SETTINGS,
};

export function adminRouteKeyFromPathname(pathname: string): AdminRouteKey | null {
  const normalized = normalizeAdminPathname(pathname);
  const match = Object.entries(adminRoutePathByKey).find(([, path]) => path === normalized);
  return (match?.[0] as AdminRouteKey | undefined) ?? null;
}

export function isAdminAuthPath(pathname: string): boolean {
  return AUTH_ROUTE_PATHS.includes(
    normalizeAdminPathname(pathname) as (typeof AUTH_ROUTE_PATHS)[number],
  );
}
