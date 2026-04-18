import {
  resetActivePortalDataSource,
  setActivePortalDataSource,
} from '../../packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js';
import { mockPortalDataSource } from '../../packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js';

export function installPortalFixtureDataSource(overrides = {}) {
  return setActivePortalDataSource({
    ...mockPortalDataSource,
    ...overrides,
  });
}

export function restoreDefaultPortalDataSource() {
  return resetActivePortalDataSource();
}
