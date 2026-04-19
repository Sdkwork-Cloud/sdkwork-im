import {
  SearchCommandPalette,
  type SearchCommandPaletteItem,
} from '@sdkwork/ui-pc-react';
import {
  Blocks,
  Building2,
  CircleUserRound,
  Gauge,
  LogOut,
  Monitor,
  PanelsTopLeft,
  RefreshCw,
  ServerCog,
  ShieldCheck,
  TimerReset,
  type LucideIcon,
  Settings2,
  Users,
  Waypoints,
} from 'lucide-react';
import { startTransition, useDeferredValue, useEffect } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import {
  adminRouteManifest,
  useAdminAppStore,
  useAdminI18n,
  useAdminWorkbench,
} from 'sdkwork-control-plane-core';
import type { AdminRouteKey } from 'sdkwork-control-plane-types';

import { prefetchSidebarRoute } from '../application/router/routePrefetch';
import { ROUTE_PATHS } from '../application/router/routePaths';

type CommandPaletteAction = SearchCommandPaletteItem & {
  path?: string;
  run: () => void;
  searchText: string;
};

const routeIconByKey: Record<AdminRouteKey, LucideIcon> = {
  announcements: PanelsTopLeft,
  automation: Blocks,
  conversations: Waypoints,
  groups: Users,
  messages: TimerReset,
  moderation: ShieldCheck,
  overview: Gauge,
  realtime: Monitor,
  settings: Settings2,
  system: ServerCog,
  storage: ServerCog,
  tenants: Building2,
  users: CircleUserRound,
};

function createActionId(prefix: string, value: string) {
  return `${prefix}:${value}`;
}

function normalizeSearchText(parts: Array<string | null | undefined>) {
  return parts
    .filter((part): part is string => Boolean(part))
    .join(' ')
    .toLowerCase();
}

export function CommandPalette() {
  const navigate = useNavigate();
  const location = useLocation();
  const { t } = useAdminI18n();
  const { handleLogout, refreshWorkspace, sessionUser } = useAdminWorkbench();
  const closeCommandPalette = useAdminAppStore((state) => state.closeCommandPalette);
  const commandSearchValue = useAdminAppStore((state) => state.commandSearchValue);
  const isCommandPaletteOpen = useAdminAppStore((state) => state.isCommandPaletteOpen);
  const setCommandPaletteOpen = useAdminAppStore((state) => state.setCommandPaletteOpen);
  const setCommandSearchValue = useAdminAppStore((state) => state.setCommandSearchValue);
  const deferredCommandSearchValue = useDeferredValue(commandSearchValue.trim().toLowerCase());

  const routeActions = adminRouteManifest.map<CommandPaletteAction>((route) => {
    const Icon = routeIconByKey[route.key];
    const routeGroup = route.group ?? 'Operations';

    return {
      description: [t(routeGroup), t(route.detail)].join(' / '),
      icon: <Icon className="h-4 w-4" />,
      id: createActionId('route', route.key),
      keywords: [
        route.key,
        route.path,
        routeGroup,
        route.detail,
        route.eyebrow,
        ...route.productModule.capabilityTags,
      ],
      label: t(route.label),
      path: route.path,
      run: () => {
        prefetchSidebarRoute(route.path);
        closeCommandPalette();
        startTransition(() => {
          navigate(route.path);
        });
      },
      searchText: normalizeSearchText([
        route.label,
        routeGroup,
        route.detail,
        route.eyebrow,
        route.path,
        ...route.productModule.capabilityTags,
      ]),
      value: `${route.label} ${routeGroup}`,
    };
  });

  const quickActions: CommandPaletteAction[] = [
    {
      description: t(
        'Jump directly to overview, governance routes, refresh posture, or sign out without leaving the current shell.',
      ),
      icon: <RefreshCw className="h-4 w-4" />,
      id: createActionId('action', 'refresh-workspace'),
      keywords: ['refresh', 'workspace', 'snapshot', 'queues', 'incident', 'posture'],
      label: t('Workspace refresh'),
      run: () => {
        closeCommandPalette();
        void refreshWorkspace();
      },
      searchText: normalizeSearchText(['refresh workspace queues incidents posture snapshot']),
      value: 'refresh workspace',
    },
    {
      description: t(
        'Use the Settings center as the control directory for navigation, shell posture, and operator continuity.',
      ),
      icon: <Settings2 className="h-4 w-4" />,
      id: createActionId('action', 'open-settings-center'),
      keywords: ['settings', 'control directory', 'operator continuity', 'preferences'],
      label: t('Settings center'),
      run: () => {
        closeCommandPalette();
        startTransition(() => {
          navigate(ROUTE_PATHS.SETTINGS);
        });
      },
      searchText: normalizeSearchText(['settings center control directory operator continuity']),
      value: 'settings center',
    },
    {
      description: t(
        'Review every route group as an operator-facing directory so the shell, search entrypoint, and left rail share the same module map.',
      ),
      icon: <PanelsTopLeft className="h-4 w-4" />,
      id: createActionId('action', 'open-navigation-directory'),
      keywords: ['navigation', 'directory', 'routes', 'operations', 'settings', 'sidebar'],
      label: t('Operations directory'),
      run: () => {
        const settingsNavigationPath = `${ROUTE_PATHS.SETTINGS}?tab=navigation`;
        closeCommandPalette();
        startTransition(() => {
          navigate(settingsNavigationPath);
        });
      },
      searchText: normalizeSearchText([
        'operations directory navigation routes settings sidebar',
      ]),
      value: 'operations directory',
    },
    {
      description: t('Sign out'),
      icon: <LogOut className="h-4 w-4" />,
      id: createActionId('action', 'sign-out'),
      keywords: ['logout', 'session', 'security', 'operator'],
      label: t('Sign out'),
      run: () => {
        closeCommandPalette();
        handleLogout();
        startTransition(() => {
          navigate(ROUTE_PATHS.LOGIN, { replace: true });
        });
      },
      searchText: normalizeSearchText(['sign out logout security session operator']),
      value: 'sign out',
    },
  ];

  const routeGroups = new Map<string, CommandPaletteAction[]>();

  for (const routeAction of routeActions) {
    const groupLabel =
      adminRouteManifest.find((route) => createActionId('route', route.key) === routeAction.id)
        ?.group ?? 'Operations';
    const items = routeGroups.get(groupLabel) ?? [];
    items.push(routeAction);
    routeGroups.set(groupLabel, items);
  }

  const matchedRouteActions = routeActions.filter(
    (routeAction) =>
      !deferredCommandSearchValue || routeAction.searchText.includes(deferredCommandSearchValue),
  );

  useEffect(() => {
    closeCommandPalette();
  }, [closeCommandPalette, location.pathname, location.search]);

  useEffect(() => {
    if (!isCommandPaletteOpen) {
      return;
    }

    for (const routeAction of matchedRouteActions.slice(0, 3)) {
      if (routeAction.path) {
        prefetchSidebarRoute(routeAction.path);
      }
    }
  }, [isCommandPaletteOpen, matchedRouteActions]);

  const actionById = new Map([...quickActions, ...routeActions].map((action) => [action.id, action]));

  return (
    <SearchCommandPalette
      closeOnSelect={false}
      emptyState={
        <div className="px-4 py-8 text-sm text-[var(--sdk-color-text-secondary)]">
          <p className="font-medium text-[var(--sdk-color-text-primary)]">
            {t('No command matches your search.')}
          </p>
          <p className="mt-2">
            {t(
              'Jump directly to overview, governance routes, refresh posture, or sign out without leaving the current shell.',
            )}
          </p>
        </div>
      }
      footer={
        <div className="flex flex-wrap items-center gap-2 border-t border-[var(--admin-border-color)] px-4 py-3 text-xs text-[var(--admin-text-secondary)]">
          <span className="rounded-full border border-[var(--admin-border-color)] px-2 py-1 font-semibold text-[var(--admin-text-primary)]">
            {t('Route launch')} {matchedRouteActions.length}
          </span>
          <span className="rounded-full border border-[var(--admin-border-color)] px-2 py-1 font-semibold text-[var(--admin-text-primary)]">
            {t('Quick actions')} {quickActions.length}
          </span>
          <span className="min-w-0 truncate">
            {sessionUser?.display_name
              ? `${sessionUser.display_name} / ${t('Command center')}`
              : t('Command center')}
          </span>
        </div>
      }
      groups={[
        {
          heading: t('Quick actions'),
          id: 'quick-actions',
          items: quickActions,
        },
        ...Array.from(routeGroups.entries()).map(([group, items]) => ({
          heading: t(group),
          id: createActionId('group', group.toLowerCase().replaceAll(/\s+/g, '-')),
          items,
        })),
      ]}
      onItemSelect={(item: SearchCommandPaletteItem) => {
        actionById.get(item.id)?.run();
      }}
      onOpenChange={(open: boolean) => {
        if (open) {
          setCommandPaletteOpen(true);
          return;
        }

        closeCommandPalette();
      }}
      onSearchValueChange={setCommandSearchValue}
      open={isCommandPaletteOpen}
      placeholder={t('Search routes, quick actions, and operator tools')}
      searchValue={commandSearchValue}
      slotProps={{
        content: {
          className:
            'max-w-3xl overflow-hidden border border-[var(--admin-border-color)] bg-[var(--admin-content-background)] shadow-[0_32px_96px_rgba(11,18,32,0.48)] backdrop-blur-xl',
        },
      }}
      title={t('Command center')}
    />
  );
}
