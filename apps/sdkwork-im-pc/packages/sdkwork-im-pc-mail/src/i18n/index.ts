import { createInstance } from 'i18next';
import { initReactI18next } from 'react-i18next';
import zhCN from './locales/zh-CN/mail.json';
import enUS from './locales/en-US/mail.json';

const SETTINGS_STORAGE_KEY = 'clawchat-settings';
const LANGUAGE_CHANGED_EVENT = 'sdkwork-chat-pc:language-changed';
const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;
type SupportedLanguage = typeof SUPPORTED_LANGUAGES[number];

function normalizeLanguage(value: unknown): SupportedLanguage {
  return SUPPORTED_LANGUAGES.includes(value as SupportedLanguage) ? value as SupportedLanguage : 'zh-CN';
}

function resolveInitialLanguage(): SupportedLanguage {
  if (typeof localStorage === 'undefined') return 'zh-CN';
  try {
    const stored = localStorage.getItem(SETTINGS_STORAGE_KEY);
    if (!stored) return 'zh-CN';
    return normalizeLanguage(JSON.parse(stored)?.lang);
  } catch { return 'zh-CN'; }
}

const i18n = createInstance();
i18n.use(initReactI18next).init({
  resources: { 'zh-CN': { mail: zhCN }, 'en-US': { mail: enUS } },
  lng: resolveInitialLanguage(),
  fallbackLng: 'zh-CN',
  ns: ['mail'],
  defaultNS: 'mail',
  interpolation: { escapeValue: false },
});

if (typeof window !== 'undefined') {
  window.addEventListener(LANGUAGE_CHANGED_EVENT, (event) => {
    const next = normalizeLanguage((event as CustomEvent<{ lang?: string }>).detail?.lang);
    if (i18n.language !== next) void i18n.changeLanguage(next);
  });
}

export default i18n;
