import path from 'node:path';
import { fileURLToPath } from 'node:url';

const supportRoot = path.dirname(fileURLToPath(import.meta.url));

export const testsRoot = path.resolve(supportRoot, '..');
export const appRoot = path.resolve(testsRoot, '..');
export const workspaceRoot = path.resolve(appRoot, '..', '..');

export function fromAppRoot(...segments) {
  return path.join(appRoot, ...segments);
}

export function fromWorkspaceRoot(...segments) {
  return path.join(workspaceRoot, ...segments);
}
