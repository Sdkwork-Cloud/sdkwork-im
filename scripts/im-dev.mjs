#!/usr/bin/env node

import { spawn } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { runSdkworkChatPcDev } from './lib/im-pc-dev.mjs';
import {
  DEFAULT_DEV_PROFILE_ID,
  loadEnvFile,
  loadProfile,
  mergeRuntimeEnv,
  REPO_ROOT,
  resolveDevProfileId,
} from './lib/im-topology.mjs';

function parseArgs(argv) {
  const settings = {
    target: 'browser',
    database: undefined,
    hosting: 'self-hosted',
    serviceLayout: 'split-services',
    help: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    if (arg === '--target') {
      settings.target = argv[index + 1] ?? settings.target;
      index += 1;
      continue;
    }
    if (arg === '--database') {
      settings.database = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--hosting') {
      settings.hosting = argv[index + 1] ?? settings.hosting;
      index += 1;
      continue;
    }
    if (arg === '--service-layout') {
      settings.serviceLayout = argv[index + 1] ?? settings.serviceLayout;
      index += 1;
    }
  }

  return settings;
}

function printHelp() {
  console.log(`Usage: node scripts/im-dev.mjs [options]

Topology-aware IM dev entry. Loads configs/topology profile env via @sdkwork/app-topology.

Options:
  --hosting <self-hosted|cloud-hosted>              Default: self-hosted
  --service-layout <split-services|unified-process> Default: split-services
  --target <browser|desktop>                        Default: browser
  --database <postgres|sqlite>
  --help, -h
`);
}

async function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    printHelp();
    process.exit(0);
  }

  const profileId = resolveDevProfileId(settings.hosting, settings.serviceLayout)
    || DEFAULT_DEV_PROFILE_ID;
  const profileEnv = loadProfile(profileId);
  const envFile = settings.database === 'postgres'
    ? '.env.postgres'
    : settings.database === 'sqlite'
      ? '.env.sqlite'
      : undefined;
  const fileEnv = envFile ? loadEnvFile(envFile) : {};
  const childEnv = mergeRuntimeEnv(process.env, profileEnv, fileEnv, {
    SDKWORK_IM_PROFILE_ID: profileId,
  });

  const runnerArgv = ['--target', settings.target];
  if (settings.database) {
    runnerArgv.push('--database', settings.database);
  }

  await runSdkworkChatPcDev({
    argv: runnerArgv,
    env: childEnv,
    repoRoot: REPO_ROOT,
  });
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
});
