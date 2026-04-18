import { after, afterEach, beforeEach } from 'node:test';

import {
  resetActivePortalDataSource,
  setActivePortalDataSource,
} from '../../packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js';
import { mockPortalDataSource } from '../../packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js';

function installMockPortalDefaultDataSource() {
  globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__ = mockPortalDataSource;
  setActivePortalDataSource(mockPortalDataSource);
}

beforeEach(() => {
  installMockPortalDefaultDataSource();
});

afterEach(() => {
  installMockPortalDefaultDataSource();
});

after(() => {
  delete globalThis.__CRAW_CHAT_PORTAL_DEFAULT_DATA_SOURCE__;
  resetActivePortalDataSource();
});
