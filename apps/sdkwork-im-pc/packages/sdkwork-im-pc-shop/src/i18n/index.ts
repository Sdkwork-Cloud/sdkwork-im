import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

import zhCNCommon from './locales/zh-CN/common.json';
import zhCNShop from './locales/zh-CN/shop.json';
import zhCNProduct from './locales/zh-CN/product.json';
import zhCNCart from './locales/zh-CN/cart.json';
import zhCNCheckout from './locales/zh-CN/checkout.json';

import enUSCommon from './locales/en-US/common.json';
import enUSShop from './locales/en-US/shop.json';
import enUSProduct from './locales/en-US/product.json';
import enUSCart from './locales/en-US/cart.json';
import enUSCheckout from './locales/en-US/checkout.json';

const resources = {
  'zh-CN': {
    common: zhCNCommon,
    shop: zhCNShop,
    product: zhCNProduct,
    cart: zhCNCart,
    checkout: zhCNCheckout,
  },
  'en-US': {
    common: enUSCommon,
    shop: enUSShop,
    product: enUSProduct,
    cart: enUSCart,
    checkout: enUSCheckout,
  },
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: 'zh-CN', // Default language
    fallbackLng: 'zh-CN',
    ns: ['common', 'shop', 'product', 'cart', 'checkout'],
    defaultNS: 'common',
    interpolation: {
      escapeValue: false, // React already safeguards from XSS
    },
  });

export default i18n;
