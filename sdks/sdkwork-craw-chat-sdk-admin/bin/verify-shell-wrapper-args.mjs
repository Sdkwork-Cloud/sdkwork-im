#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  appendShellWrapperForwardingFailures,
  finishShellWrapperVerification,
  readWorkspaceSource,
} from '../../workspace-shell-wrapper-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const failures = [];

appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-verify.sh',
  }),
  failures,
  label: 'TypeScript sdk-verify.sh',
  language: 'typescript',
  rootScript: 'verify-sdk.sh',
});
appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-verify.sh',
  }),
  failures,
  label: 'Flutter sdk-verify.sh',
  language: 'flutter',
  rootScript: 'verify-sdk.sh',
});
appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-gen.sh',
  }),
  failures,
  label: 'TypeScript sdk-gen.sh',
  language: 'typescript',
  rootScript: 'generate-sdk.sh',
});
appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-gen.sh',
  }),
  failures,
  label: 'Flutter sdk-gen.sh',
  language: 'flutter',
  rootScript: 'generate-sdk.sh',
});
appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-typescript/bin/sdk-assemble.sh',
  }),
  failures,
  label: 'TypeScript sdk-assemble.sh',
  language: 'typescript',
  rootScript: 'assemble-sdk.sh',
});
appendShellWrapperForwardingFailures({
  source: readWorkspaceSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-admin-flutter/bin/sdk-assemble.sh',
  }),
  failures,
  label: 'Flutter sdk-assemble.sh',
  language: 'flutter',
  rootScript: 'assemble-sdk.sh',
});

finishShellWrapperVerification({ prefix, failures });
