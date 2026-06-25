export const SDKWORK_IM_PC_SETTINGS_STORAGE_KEY = 'im-settings';

export const SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT = 'sdkwork-im-pc:language-changed';

const LEGACY_SDKWORK_IM_PC_SETTINGS_STORAGE_KEYS = ['clawchat-settings'] as const;

export function readPersistedSettingsRecord(): Record<string, unknown> | undefined {
  if (typeof localStorage === 'undefined') {
    return undefined;
  }

  for (const storageKey of [
    SDKWORK_IM_PC_SETTINGS_STORAGE_KEY,
    ...LEGACY_SDKWORK_IM_PC_SETTINGS_STORAGE_KEYS,
  ]) {
    const rawValue = localStorage.getItem(storageKey);
    if (!rawValue) {
      continue;
    }

    try {
      const parsed = JSON.parse(rawValue) as Record<string, unknown>;
      if (
        storageKey !== SDKWORK_IM_PC_SETTINGS_STORAGE_KEY
        && parsed
        && typeof parsed === 'object'
        && !Array.isArray(parsed)
      ) {
        localStorage.setItem(
          SDKWORK_IM_PC_SETTINGS_STORAGE_KEY,
          JSON.stringify(parsed),
        );
        localStorage.removeItem(storageKey);
      }
      return parsed;
    } catch {
      if (storageKey === SDKWORK_IM_PC_SETTINGS_STORAGE_KEY) {
        localStorage.removeItem(storageKey);
      }
    }
  }

  return undefined;
}

export function resolvePersistedLanguage<T extends string>(
  supportedLanguages: readonly T[],
  fallback: T,
): T {
  const language = readPersistedSettingsRecord()?.lang;
  return supportedLanguages.includes(language as T) ? language as T : fallback;
}
