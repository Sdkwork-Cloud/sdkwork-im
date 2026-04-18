import { adminRouteManifest } from 'sdkwork-craw-chat-admin-core';
import type { AdminRouteModuleId } from 'sdkwork-craw-chat-admin-types';

const adminRouteModuleLoaders: Record<AdminRouteModuleId, () => Promise<unknown>> = {
  'sdkwork-craw-chat-admin-overview': () => import('sdkwork-craw-chat-admin-overview'),
  'sdkwork-craw-chat-admin-tenants': () => import('sdkwork-craw-chat-admin-tenants'),
  'sdkwork-craw-chat-admin-users': () => import('sdkwork-craw-chat-admin-users'),
  'sdkwork-craw-chat-admin-conversations': () => import('sdkwork-craw-chat-admin-conversations'),
  'sdkwork-craw-chat-admin-messages': () => import('sdkwork-craw-chat-admin-messages'),
  'sdkwork-craw-chat-admin-groups': () => import('sdkwork-craw-chat-admin-groups'),
  'sdkwork-craw-chat-admin-moderation': () => import('sdkwork-craw-chat-admin-moderation'),
  'sdkwork-craw-chat-admin-automation': () => import('sdkwork-craw-chat-admin-automation'),
  'sdkwork-craw-chat-admin-announcements': () => import('sdkwork-craw-chat-admin-announcements'),
  'sdkwork-craw-chat-admin-realtime': () => import('sdkwork-craw-chat-admin-realtime'),
  'sdkwork-craw-chat-admin-system': () => import('sdkwork-craw-chat-admin-system'),
  'sdkwork-craw-chat-admin-storage': () => import('sdkwork-craw-chat-admin-storage'),
  'sdkwork-craw-chat-admin-settings': () => import('sdkwork-craw-chat-admin-settings'),
};

export function loadAdminRouteModule(moduleId: AdminRouteModuleId) {
  return adminRouteModuleLoaders[moduleId]();
}

const sidebarRoutePrefetchers = adminRouteManifest
  .filter((route) => route.productModule.loading.prefetch === 'intent')
  .map((route) => [
    route.path,
    () => loadAdminRouteModule(route.moduleId),
  ]) as readonly SidebarRoutePrefetcher[];

type SidebarRoutePrefetcher = readonly [string, () => Promise<unknown>];
type ScheduledPrefetchHandle = unknown;

function normalizeRoutePath(pathname: string) {
  return pathname.split(/[?#]/, 1)[0] || pathname;
}

function resolveSidebarRoutePrefetcher(
  routePrefetchers: readonly SidebarRoutePrefetcher[],
  pathname: string,
) {
  const normalizedPath = normalizeRoutePath(pathname);
  return routePrefetchers.find(([prefix]) => (
    normalizedPath === prefix || normalizedPath.startsWith(`${prefix}/`)
  ));
}

export function createSidebarRoutePrefetchController(input: {
  routePrefetchers: readonly SidebarRoutePrefetcher[];
  scheduleDelayMs?: number;
  schedule?: (callback: () => void, delayMs: number) => ScheduledPrefetchHandle;
  clearScheduled?: (handle: ScheduledPrefetchHandle) => void;
}) {
  const {
    routePrefetchers,
    scheduleDelayMs = 120,
    schedule = (callback, delayMs) => window.setTimeout(callback, delayMs),
    clearScheduled = (handle) => window.clearTimeout(handle as number),
  } = input;

  const prefetchedSidebarRoutes = new Map<string, Promise<unknown>>();
  const scheduledSidebarRoutes = new Map<string, ScheduledPrefetchHandle>();

  const prefetch = (pathname: string) => {
    const match = resolveSidebarRoutePrefetcher(routePrefetchers, pathname);
    if (!match) {
      return;
    }

    const [routePrefix, loadRoute] = match;
    if (prefetchedSidebarRoutes.has(routePrefix)) {
      return;
    }

    const pending = loadRoute().catch((error) => {
      prefetchedSidebarRoutes.delete(routePrefix);
      throw error;
    });

    prefetchedSidebarRoutes.set(routePrefix, pending);
  };

  const cancel = (pathname: string) => {
    const match = resolveSidebarRoutePrefetcher(routePrefetchers, pathname);
    if (!match) {
      return;
    }

    const [routePrefix] = match;
    const scheduled = scheduledSidebarRoutes.get(routePrefix);
    if (!scheduled) {
      return;
    }

    clearScheduled(scheduled);
    scheduledSidebarRoutes.delete(routePrefix);
  };

  const queue = (pathname: string) => {
    const match = resolveSidebarRoutePrefetcher(routePrefetchers, pathname);
    if (!match) {
      return;
    }

    const [routePrefix] = match;
    if (prefetchedSidebarRoutes.has(routePrefix) || scheduledSidebarRoutes.has(routePrefix)) {
      return;
    }

    const handle = schedule(() => {
      scheduledSidebarRoutes.delete(routePrefix);
      prefetch(pathname);
    }, scheduleDelayMs);

    scheduledSidebarRoutes.set(routePrefix, handle);
  };

  return {
    prefetch,
    schedule: queue,
    cancel,
  };
}

const sidebarRoutePrefetchController = createSidebarRoutePrefetchController({
  routePrefetchers: sidebarRoutePrefetchers,
});

export function prefetchSidebarRoute(pathname: string) {
  sidebarRoutePrefetchController.prefetch(pathname);
}

export function scheduleSidebarRoutePrefetch(pathname: string) {
  sidebarRoutePrefetchController.schedule(pathname);
}

export function cancelSidebarRoutePrefetch(pathname: string) {
  sidebarRoutePrefetchController.cancel(pathname);
}
