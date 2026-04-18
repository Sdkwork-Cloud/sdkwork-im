import { LoadingBlock } from '@sdkwork/ui-pc-react/components/ui/feedback';
import { lazy, Suspense, type ReactNode } from 'react';
import { Navigate, Route, Routes, useLocation } from 'react-router-dom';

import { AdminLoginPage } from 'sdkwork-craw-chat-admin-auth';
import {
  ADMIN_ROUTE_PATHS,
  isAdminAuthPath,
  useAdminI18n,
  useAdminWorkbench,
} from 'sdkwork-craw-chat-admin-core';

import { MainLayout } from '../layouts/MainLayout';
import { ROUTE_PATHS } from './routePaths';

const OverviewPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-overview')).OverviewPage,
}));
const TenantsPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-tenants')).TenantsPage,
}));
const UsersPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-users')).UsersPage,
}));
const GroupsPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-groups')).GroupsPage,
}));
const AnnouncementsPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-announcements')).AnnouncementsPage,
}));
const ConversationsPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-conversations')).ConversationsPage,
}));
const MessagesPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-messages')).MessagesPage,
}));
const ModerationPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-moderation')).ModerationPage,
}));
const AutomationPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-automation')).AutomationPage,
}));
const RealtimePage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-realtime')).RealtimePage,
}));
const SystemPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-system')).SystemPage,
}));
const StoragePage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-storage')).StoragePage,
}));
const SettingsPage = lazy(async () => ({
  default: (await import('sdkwork-craw-chat-admin-settings')).SettingsPage,
}));

function RouteStage({
  children,
  routeKey,
}: {
  children: ReactNode;
  routeKey: string;
}) {
  return (
    <div className="admin-shell-route-stage" data-route-key={routeKey} key={routeKey}>
      <div className="admin-shell-route-scroll">{children}</div>
    </div>
  );
}

function LoadingScreen() {
  const { t } = useAdminI18n();

  return (
    <div className="admin-shell-route-stage admin-shell-route-stage-loading">
      <div className="admin-shell-route-scroll">
        <div className="flex min-h-full flex-col items-center justify-center gap-4 text-center">
          <LoadingBlock label={t('Synchronizing IM operator workspace...')} />
          <p className="max-w-md text-sm text-[var(--sdk-color-text-secondary)]">
            {t('Restoring theme, auth state, and live operator snapshots.')}
          </p>
        </div>
      </div>
    </div>
  );
}

function resolveRedirectTarget(rawTarget: string | null) {
  if (!rawTarget || !rawTarget.startsWith('/')) {
    return ROUTE_PATHS.OVERVIEW;
  }

  if (rawTarget === ROUTE_PATHS.ROOT || isAdminAuthPath(rawTarget)) {
    return ROUTE_PATHS.OVERVIEW;
  }

  return rawTarget;
}

function withRedirect(pathname: string, rawTarget: string | null) {
  const redirectTarget = resolveRedirectTarget(rawTarget);
  if (redirectTarget === ROUTE_PATHS.OVERVIEW) {
    return pathname;
  }

  const params = new URLSearchParams();
  params.set('redirect', redirectTarget);
  return `${pathname}?${params.toString()}`;
}

function ProtectedRoute({ children }: { children: ReactNode }) {
  const location = useLocation();
  const { authResolved, sessionUser } = useAdminWorkbench();

  if (!authResolved) {
    return <LoadingScreen />;
  }

  if (!sessionUser) {
    return (
      <Navigate
        replace
        to={withRedirect(ROUTE_PATHS.LOGIN, `${location.pathname}${location.search}`)}
      />
    );
  }

  return <>{children}</>;
}

function ProtectedPage({
  children,
  routeKey,
}: {
  children: ReactNode;
  routeKey: string;
}) {
  return (
    <ProtectedRoute>
      <MainLayout>
        <RouteStage routeKey={routeKey}>
          <Suspense fallback={<LoadingScreen />}>{children}</Suspense>
        </RouteStage>
      </MainLayout>
    </ProtectedRoute>
  );
}

export function AppRoutes() {
  const location = useLocation();
  const { authResolved, handleLogin, loading, sessionUser, snapshot, status } = useAdminWorkbench();

  const authRouteElement = !authResolved ? (
    <LoadingScreen />
  ) : (
    <AdminLoginPage
      isAuthenticated={Boolean(sessionUser)}
      loading={loading}
      onLogin={handleLogin}
      status={status}
    />
  );

  return (
    <Routes>
      <Route
        element={
          <Navigate
            replace
            to={withRedirect(ROUTE_PATHS.LOGIN, new URLSearchParams(location.search).get('redirect'))}
          />
        }
        path={ROUTE_PATHS.AUTH}
      />
      <Route element={authRouteElement} path={ROUTE_PATHS.LOGIN} />
      <Route element={authRouteElement} path={ROUTE_PATHS.REGISTER} />
      <Route element={authRouteElement} path={ROUTE_PATHS.FORGOT_PASSWORD} />
      <Route
        element={
          <ProtectedPage routeKey="overview">
            <OverviewPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.OVERVIEW}
      />
      <Route
        element={
          <ProtectedPage routeKey="tenants">
            <TenantsPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.TENANTS}
      />
      <Route
        element={
          <ProtectedPage routeKey="users">
            <UsersPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.USERS}
      />
      <Route
        element={
          <ProtectedPage routeKey="groups">
            <GroupsPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.GROUPS}
      />
      <Route
        element={
          <ProtectedPage routeKey="announcements">
            <AnnouncementsPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.ANNOUNCEMENTS}
      />
      <Route
        element={
          <ProtectedPage routeKey="conversations">
            <ConversationsPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.CONVERSATIONS}
      />
      <Route
        element={
          <ProtectedPage routeKey="messages">
            <MessagesPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.MESSAGES}
      />
      <Route
        element={
          <ProtectedPage routeKey="moderation">
            <ModerationPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.MODERATION}
      />
      <Route
        element={
          <ProtectedPage routeKey="automation">
            <AutomationPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.AUTOMATION}
      />
      <Route
        element={
          <ProtectedPage routeKey="realtime">
            <RealtimePage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.REALTIME}
      />
      <Route
        element={
          <ProtectedPage routeKey="system">
            <SystemPage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.SYSTEM}
      />
      <Route
        element={
          <ProtectedPage routeKey="storage">
            <StoragePage snapshot={snapshot} />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.STORAGE}
      />
      <Route
        element={
          <ProtectedPage routeKey="settings">
            <SettingsPage />
          </ProtectedPage>
        }
        path={ROUTE_PATHS.SETTINGS}
      />
      <Route element={<Navigate replace to={ROUTE_PATHS.OVERVIEW} />} path={ADMIN_ROUTE_PATHS.ROOT} />
      <Route
        element={<Navigate replace to={sessionUser ? ROUTE_PATHS.OVERVIEW : ROUTE_PATHS.LOGIN} />}
        path="*"
      />
    </Routes>
  );
}
