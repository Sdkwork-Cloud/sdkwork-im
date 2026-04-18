import path from 'node:path';
import { fileURLToPath } from 'node:url';

export function resolvePortalAppRoot(importMetaUrl) {
  const testFilePath = fileURLToPath(importMetaUrl);
  return path.resolve(path.dirname(testFilePath), '..');
}

export function resolvePortalPackagesRoot(importMetaUrl) {
  return path.join(resolvePortalAppRoot(importMetaUrl), 'packages');
}

export function resolvePortalWorkspaceRoot(importMetaUrl) {
  return path.resolve(resolvePortalAppRoot(importMetaUrl), '..', '..');
}
