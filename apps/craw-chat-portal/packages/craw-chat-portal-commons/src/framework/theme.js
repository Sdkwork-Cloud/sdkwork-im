import { PORTAL_THEME_OPTIONS } from '../../../craw-chat-portal-types/src/index.js';

export function getPortalThemeById(themeId) {
  return PORTAL_THEME_OPTIONS.find((theme) => theme.id === themeId) ?? PORTAL_THEME_OPTIONS[0];
}

export function applyPortalTheme(themeId) {
  if (typeof document === 'undefined') {
    return;
  }

  document.documentElement.dataset.portalTheme = getPortalThemeById(themeId).id;
}
