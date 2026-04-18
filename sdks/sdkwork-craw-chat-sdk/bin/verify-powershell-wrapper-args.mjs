#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  appendLanguagePowerShellForwarderFailures,
  finishPowerShellWrapperVerification,
  readPowerShellWrapperVerificationInputs,
  readWorkspacePowerShellWrapperSource,
} from '../../workspace-powershell-wrapper-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { assembleSource, generateSource, verifySource, readmeSource } =
  readPowerShellWrapperVerificationInputs({ workspaceRoot, scriptDir });

const failures = [];

for (const [label, source] of [
  ['assemble-sdk.ps1', assembleSource],
  ['generate-sdk.ps1', generateSource],
  ['verify-sdk.ps1', verifySource],
]) {
  if (!/function Normalize-LanguageList/.test(source)) {
    failures.push(`${label} must declare Normalize-LanguageList.`);
  }
  if (!/\$Languages = Normalize-LanguageList \$Languages/.test(source)) {
    failures.push(`${label} must normalize the Languages parameter before use.`);
  }
}

if (!/-Languages typescript,flutter/.test(readmeSource)) {
  failures.push('Workspace README must keep documenting the comma-separated PowerShell example that the wrappers support.');
}

appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-typescript/bin/sdk-gen.ps1',
  }),
  failures,
  label: 'TypeScript sdk-gen.ps1',
  rootScript: 'generate-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-flutter/bin/sdk-gen.ps1',
  }),
  failures,
  label: 'Flutter sdk-gen.ps1',
  rootScript: 'generate-sdk.ps1',
  language: 'flutter',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.ps1',
  }),
  failures,
  label: 'TypeScript sdk-verify.ps1',
  rootScript: 'verify-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.ps1',
  }),
  failures,
  label: 'Flutter sdk-verify.ps1',
  rootScript: 'verify-sdk.ps1',
  language: 'flutter',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-typescript/bin/sdk-assemble.ps1',
  }),
  failures,
  label: 'TypeScript sdk-assemble.ps1',
  rootScript: 'assemble-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-craw-chat-sdk-flutter/bin/sdk-assemble.ps1',
  }),
  failures,
  label: 'Flutter sdk-assemble.ps1',
  rootScript: 'assemble-sdk.ps1',
  language: 'flutter',
});

finishPowerShellWrapperVerification({ prefix, failures });
