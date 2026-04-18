#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'kotlin',
  workspace: 'sdkwork-craw-chat-sdk-kotlin',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-b',
  readmeRequiredTerms: [
    'com.sdkwork:craw-chat-backend-sdk',
    'CrawChatSdkClient',
    'Tier B',
    'transport artifact',
  ],
});
