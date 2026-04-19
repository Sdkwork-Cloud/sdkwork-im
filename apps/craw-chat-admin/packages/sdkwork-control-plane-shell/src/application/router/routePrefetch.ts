import { adminRouteManifest } from 'sdkwork-control-plane-core';
import type { AdminRouteModuleId } from 'sdkwork-control-plane-types';

const adminRouteModuleLoaders: Record<AdminRouteModuleId, () => Promise<unknown>> = {
  'sdkwork-control-plane-overview': () => import('sdkwork-control-plane-overview'),
  'sdkwork-control-plane-tenants': () => import('sdkwork-control-plane-tenants'),
  'sdkwork-control-plane-users': () => import('sdkwork-control-plane-users'),
  'sdkwork-control-plane-conversations': () => import('sdkwork-control-plane-conversations'),
  'sdkwork-control-plane-messages': () => import('sdkwork-control-plane-messages'),
  'sdkwork-control-plane-groups': () => import('sdkwork-control-plane-groups'),
  'sdkwork-control-plane-moderation': () => import('sdkwork-control-plane-moderation'),
  'sdkwork-control-plane-automation': () => import('sdkwork-control-plane-automation'),
  'sdkwork-control-plane-announcements': () => import('sdkwork-control-plane-announcements'),
  'sdkwork-control-plane-realtime': () => import('sdkwork-control-plane-realtime'),
  'sdkwork-control-plane-system': () => import('sdkwork-control-plane-system'),
  'sdkwork-control-plane-storage': () => import('sdkwork-control-plane-storage'),
  'sdkwork-control-plane-settings': () => import('sdkwork-control-plane-settings'),
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
