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

if (!/-Languages typescript,flutter,rust/.test(readme)) {
  failures.push('Workspace README must keep documenting the comma-separated PowerShell example that the wrappers support for rust.');
}

if (!/powershell -ExecutionPolicy Bypass -File \.\\bin\\verify-sdk\.ps1 -Languages typescript,flutter,rust -WithDart/.test(readme)) {
  failures.push('Workspace README must document the PowerShell verification example with rust in the language list.');
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] PowerShell wrapper argument verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] PowerShell wrapper argument verification passed.');
