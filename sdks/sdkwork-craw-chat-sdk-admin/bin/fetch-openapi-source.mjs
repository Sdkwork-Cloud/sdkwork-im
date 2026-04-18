#!/usr/bin/env node
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './sdk-generator-root.mjs';
import {
  assertOpenApi3Document,
  cloneOpenApiJson,
  failOpenApiSource,
  fetchOpenApiDocument,
  parseFetchOpenApiSourceArgs,
  sortKeysDeep,
  startRuntimeProcess,
  stopRuntimeProcess,
  waitForRuntimeOpenApi,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';

function normalizeOpenApiDocument(document) {
  assertOpenApi3Document({
    prefix,
    document,
    sourceLabel: `Fetched OpenAPI payload (${document?.openapi ?? 'unknown'})`,
  });

  const normalized = cloneOpenApiJson(document);
  normalized.servers = [{ url: '/' }];

  if (normalized.info && typeof normalized.info === 'object') {
    delete normalized.info.contact;
    delete normalized.info.license;
  }

  return sortKeysDeep(normalized);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(workspaceRoot, '..', '..');
const args = parseFetchOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  defaultUrl: 'http://127.0.0.1:18081/openapi.json',
});
const authorityPath = path.resolve(
  args.authority || path.join(workspaceRoot, 'openapi', 'admin-control-plane.openapi.yaml'),
);
const yaml = await loadGeneratorYaml(workspaceRoot);

let startedRuntime = null;

try {
  let document;
  try {
    document = await fetchOpenApiDocument(args.url, 5_000);
  } catch (initialError) {
    if (!args.launchRuntime) {
      throw initialError;
    }

    startedRuntime = startRuntimeProcess({
      command: 'cargo',
      args: ['run', '-p', 'control-plane-api'],
      cwd: repoRoot,
      stdio: ['ignore', 'inherit', 'inherit'],
      windowsHide: true,
    });
    document = await waitForRuntimeOpenApi(args.url, args.timeoutMs);
  }

  const normalized = normalizeOpenApiDocument(document);
  writeOpenApiYamlDocument({ filePath: authorityPath, document: normalized, yaml });
  process.stdout.write(authorityPath);
} catch (error) {
  const failureDetails = startedRuntime?.describeFailure?.() || '';
  if (failureDetails) {
    failOpenApiSource({
      prefix,
      message: `${error instanceof Error ? error.message : String(error)}\n${failureDetails}`,
    });
  }
  failOpenApiSource({
    prefix,
    message: error instanceof Error ? error.message : String(error),
  });
} finally {
  await stopRuntimeProcess(startedRuntime?.child);
}
