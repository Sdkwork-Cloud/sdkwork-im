import { AdminWorkbenchProvider } from 'sdkwork-control-plane-core';

import { AppProviders } from '../providers/AppProviders';
import { AppRoutes } from '../router/AppRoutes';

export function AppRoot() {
  return (
    <AppProviders>
      <AdminWorkbenchProvider>
        <AppRoutes />
      </AdminWorkbenchProvider>
    </AppProviders>
  );
}
