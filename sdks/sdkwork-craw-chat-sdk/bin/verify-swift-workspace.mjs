#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'swift',
  workspace: 'sdkwork-craw-chat-sdk-swift',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-b',
  readmeRequiredTerms: [
    'CrawChatBackendSdk',
    'CrawChatSdkClient',
    'Tier B',
    'transport package',
  ],
});
