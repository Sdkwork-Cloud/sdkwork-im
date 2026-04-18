#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'csharp',
  workspace: 'sdkwork-craw-chat-sdk-csharp',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-b',
  readmeRequiredTerms: [
    'Sdkwork.CrawChat.BackendSdk',
    'CrawChatSdkClient',
    'Tier B',
    'transport package',
  ],
});
