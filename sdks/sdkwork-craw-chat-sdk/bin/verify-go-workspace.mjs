#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'go',
  workspace: 'sdkwork-craw-chat-sdk-go',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-b',
  readmeRequiredTerms: [
    'github.com/sdkwork/craw-chat-backend-sdk',
    'CrawChatSdkClient',
    'Tier B',
    'transport module',
  ],
});
