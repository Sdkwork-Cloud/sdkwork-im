import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import zhCN from './locales/zh-CN.json';
import enUS from './locales/en-US.json';

const i18n = i18next.createInstance();

i18n
  .use(initReactI18next)
  .init({
    resources: {
      'zh-CN': { enterprise: zhCN },
      'en-US': { enterprise: enUS },
    },
    lng: 'zh-CN',
    fallbackLng: 'zh-CN',
    ns: ['enterprise'],
    defaultNS: 'enterprise',
    interpolation: {
      escapeValue: false,
    },
  });

export default i18n;
