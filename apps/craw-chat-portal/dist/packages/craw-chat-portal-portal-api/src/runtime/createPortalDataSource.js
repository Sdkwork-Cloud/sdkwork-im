import { mockPortalDataSource } from './dataSources/mockPortalDataSource.js';

function isPlainObject(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) {
    return false;
  }

  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

export function createPortalDataSource(overrides = {}) {
  if (!isPlainObject(overrides)) {
    throw new TypeError('Portal data source overrides must be a plain object.');
  }

  for (const [key, value] of Object.entries(overrides)) {
    if (!(key in mockPortalDataSource)) {
      throw new TypeError(`Unknown portal data source override "${key}".`);
    }

    if (typeof mockPortalDataSource[key] !== 'function') {
      continue;
    }

    if (typeof value !== 'function') {
      throw new TypeError(`Portal data source override "${key}" must be a function.`);
    }
  }

  return Object.freeze({
    ...mockPortalDataSource,
    ...overrides,
  });
}
