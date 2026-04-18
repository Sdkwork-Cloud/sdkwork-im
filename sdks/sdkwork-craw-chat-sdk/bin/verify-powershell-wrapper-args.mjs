#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

const generatePs1 = readFileSync(path.join(scriptDir, 'generate-sdk.ps1'), 'utf8');
const verifyPs1 = readFileSync(path.join(scriptDir, 'verify-sdk.ps1'), 'utf8');
const readme = readFileSync(path.join(workspaceRoot, 'README.md'), 'utf8');

const failures = [];
const legacyDefaultLanguagesPattern = /\@\("typescript",\s*"flutter"\s*\)/;
const fullDefaultLanguagesPattern =
  /\@\("typescript",\s*"flutter",\s*"rust",\s*"java",\s*"csharp",\s*"swift",\s*"kotlin",\s*"go",\s*"python"\s*\)/;

for (const [label, source] of [
  ['generate-sdk.ps1', generatePs1],
  ['verify-sdk.ps1', verifyPs1],
]) {
  if (!/function Normalize-LanguageList/.test(source)) {
    failures.push(`${label} must declare Normalize-LanguageList.`);
  }
  if (!/\$Languages = Normalize-LanguageList \$Languages/.test(source)) {
    failures.push(`${label} must normalize the Languages parameter before use.`);
  }
}

if (!/function Invoke-PowerShellFileStep/.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must declare Invoke-PowerShellFileStep to isolate PowerShell-script invocation from stale LASTEXITCODE state.');
}

if (!/Invoke-PowerShellFileStep[\s\S]*?-Path \$StartLocalScript[\s\S]*?-Step "start-local"/.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must invoke start-local.ps1 through Invoke-PowerShellFileStep.');
}

if (!/Invoke-PowerShellFileStep[\s\S]*?-Path \$StopLocalScript[\s\S]*?-Step "stop-local"/.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must invoke stop-local.ps1 through Invoke-PowerShellFileStep.');
}

if (/Assert-LastExitCode "stop-local"/.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must not assert stop-local.ps1 through raw LASTEXITCODE checks because cleanup can inherit stale native exit codes.');
}

if (!/-Languages typescript,flutter/.test(readme)) {
  failures.push('Workspace README must keep documenting the comma-separated PowerShell example that the wrappers support.');
}
if (!/-Languages typescript,flutter,rust,java,csharp,swift,kotlin,go,python/.test(readme)) {
  failures.push('Workspace README must document the full comma-separated PowerShell example for all official SDK languages.');
}
if (!fullDefaultLanguagesPattern.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must default to the full official language set.');
}
if (legacyDefaultLanguagesPattern.test(generatePs1)) {
  failures.push('generate-sdk.ps1 must not keep the legacy typescript/flutter-only default.');
}
if (!fullDefaultLanguagesPattern.test(verifyPs1)) {
  failures.push('verify-sdk.ps1 must default to the full official language set.');
}
if (legacyDefaultLanguagesPattern.test(verifyPs1)) {
  failures.push('verify-sdk.ps1 must not keep the legacy typescript/flutter-only default.');
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] PowerShell wrapper argument verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] PowerShell wrapper argument verification passed.');
