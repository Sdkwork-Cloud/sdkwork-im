import { applyPortalTheme } from '../../../../craw-chat-portal-commons/src/index.js';

export function initializeAppProviders({ shellStore }) {
  applyPortalTheme(shellStore.getState().theme);

  return shellStore.subscribe((state) => {
    applyPortalTheme(state.theme);
  });
}
