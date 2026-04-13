import { httpPortalDataSource } from './dataSources/httpPortalDataSource.js';
import { mockPortalDataSource } from './dataSources/mockPortalDataSource.js';

function resolveBasePortalDataSource() {
  if (typeof window !== 'undefined' && typeof window.fetch === 'function') {
    return httpPortalDataSource;
  }

  return mockPortalDataSource;
}

function isPlainObject(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return false;
  }

  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

export function createPortalDataSource(overrides = {}) {
  const baseDataSource = resolveBasePortalDataSource();

  if (!isPlainObject(overrides)) {
    throw new TypeError('Portal data source overrides must be a plain object.');
  }

  for (const [key, value] of Object.entries(overrides)) {
    if (!(key in baseDataSource)) {
      throw new TypeError(`Unknown portal data source override "${key}".`);
    }

    if (typeof baseDataSource[key] !== 'function') {
      continue;
    }

    if (typeof value !== 'function') {
      throw new TypeError(`Portal data source override "${key}" must be a function.`);
    }
  }

  return Object.freeze({
    ...baseDataSource,
    ...overrides,
  });
}
