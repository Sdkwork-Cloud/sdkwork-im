#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'python',
  workspace: 'sdkwork-craw-chat-sdk-python',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-b',
  readmeRequiredTerms: [
    'sdkwork-craw-chat-backend-sdk',
    'CrawChatSdkClient',
    'Tier B',
    'transport package',
  ],
});
