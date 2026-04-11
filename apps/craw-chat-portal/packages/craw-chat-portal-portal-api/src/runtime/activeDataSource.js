import { createPortalDataSource } from './createPortalDataSource.js';

let currentPortalDataSource = createPortalDataSource();

function createActivePortalDataSourceSurface() {
  const surface = {};

  for (const property of Reflect.ownKeys(currentPortalDataSource)) {
    Object.defineProperty(surface, property, {
      configurable: false,
      enumerable: true,
      get() {
        return Reflect.get(currentPortalDataSource, property);
      },
    });
  }

  return Object.preventExtensions(surface);
}

export const activePortalDataSource = createActivePortalDataSourceSurface();

export function getActivePortalDataSource() {
  return currentPortalDataSource;
}

export function setActivePortalDataSource(overrides = {}) {
  currentPortalDataSource = createPortalDataSource(overrides);
  return currentPortalDataSource;
}

export function resetActivePortalDataSource() {
  currentPortalDataSource = createPortalDataSource();
  return currentPortalDataSource;
}
