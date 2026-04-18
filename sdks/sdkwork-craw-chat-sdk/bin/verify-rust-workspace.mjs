#!/usr/bin/env node
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

verifyLanguageWorkspace({
  language: 'rust',
  workspace: 'sdkwork-craw-chat-sdk-rust',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-a',
  readmeRequiredTerms: [
    'sdkwork-craw-chat-backend-sdk',
    'craw_chat_sdk',
    'CrawChatSdkClient',
    'Tier A',
    'transport crate',
  ],
});
