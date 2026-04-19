#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  appendLanguagePowerShellForwarderFailures,
  finishPowerShellWrapperVerification,
  readPowerShellWrapperVerificationInputs,
  readWorkspacePowerShellWrapperSource,
} from '../../workspace-powershell-wrapper-verify-shared.mjs';

const prefix = 'sdkwork-control-plane-sdk';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { assembleSource, generateSource, verifySource } =
  readPowerShellWrapperVerificationInputs({ workspaceRoot, scriptDir });

const failures = [];

for (const source of [assembleSource, generateSource, verifySource]) {
  if (!source.includes('Normalize-LanguageList')) {
    failures.push('PowerShell wrappers must normalize comma-separated -Languages arguments.');
    break;
  }
}

appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-typescript/bin/sdk-gen.ps1',
  }),
  failures,
  label: 'TypeScript sdk-gen.ps1',
  rootScript: 'generate-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-flutter/bin/sdk-gen.ps1',
  }),
  failures,
  label: 'Flutter sdk-gen.ps1',
  rootScript: 'generate-sdk.ps1',
  language: 'flutter',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-typescript/bin/sdk-verify.ps1',
  }),
  failures,
  label: 'TypeScript sdk-verify.ps1',
  rootScript: 'verify-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-flutter/bin/sdk-verify.ps1',
  }),
  failures,
  label: 'Flutter sdk-verify.ps1',
  rootScript: 'verify-sdk.ps1',
  language: 'flutter',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-typescript/bin/sdk-assemble.ps1',
  }),
  failures,
  label: 'TypeScript sdk-assemble.ps1',
  rootScript: 'assemble-sdk.ps1',
  language: 'typescript',
});
appendLanguagePowerShellForwarderFailures({
  source: readWorkspacePowerShellWrapperSource({
    workspaceRoot,
    relativePath: 'sdkwork-control-plane-sdk-flutter/bin/sdk-assemble.ps1',
  }),
  failures,
  label: 'Flutter sdk-assemble.ps1',
  rootScript: 'assemble-sdk.ps1',
  language: 'flutter',
});

finishPowerShellWrapperVerification({ prefix, failures });
