#!/usr/bin/env node

import { spawn } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import {
  loadEnvFile,
  REPO_ROOT,
  resolveIamDevEnv,
  resolveStandaloneGatewayConfigPath,
} from './lib/im-topology.mjs';

const repoRoot = REPO_ROOT;
const DEFAULT_ENVIRONMENT = 'development';

function cargoCommand() {
  return process.platform === 'win32' ? 'cargo.exe' : 'cargo';
}

function normalizeText(value) {
  const normalized = String(value ?? '').trim();
  return normalized || undefined;
}

function parseArgs(argv) {
  const settings = {
    environment: DEFAULT_ENVIRONMENT,
    config: undefined,
    devEnvFile: '.env.postgres',
    release: false,
    dryRun: false,
    help: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--help' || arg === '-h') {
      settings.help = true;
      continue;
    }
    if (arg === '--dry-run') {
      settings.dryRun = true;
      continue;
    }
    if (arg === '--release') {
      settings.release = true;
      continue;
    }
    if (arg === '--environment') {
      settings.environment = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--config') {
      settings.config = argv[index + 1];
      index += 1;
      continue;
    }
    if (arg === '--dev-env-file') {
      settings.devEnvFile = argv[index + 1];
      index += 1;
    }
  }

  return settings;
}

function resolveConfigPath(settings) {
  if (settings.config) {
    return path.isAbsolute(settings.config)
      ? settings.config
      : path.resolve(repoRoot, settings.config);
  }
  return resolveStandaloneGatewayConfigPath(
    { SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT: settings.environment },
    repoRoot,
  );
}

function printHelp() {
  console.log(`Usage: node scripts/gateway-standalone-run.mjs [options]

Sdkwork IM standalone gateway embeds appbase IAM and IM application ingress on one bind.
Use this for standalone deployment profiles. Cloud split profiles use sdkwork-api-gateway.

Options:
  --environment <development|production>  Config profile (default: development)
  --config <path>                         Explicit TOML config path
  --dev-env-file <path>                   Env file for IAM/database (default: .env.postgres)
  --release                               Build/run release profile
  --dry-run                               Print command without executing
  --help, -h                              Show this help

Environment overrides:
  SDKWORK_IM_STANDALONE_GATEWAY_CONFIG
  SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT
  SDKWORK_IM_STANDALONE_GATEWAY_BIND
  SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND
`);
}

function main() {
  const settings = parseArgs(process.argv.slice(2));
  if (settings.help) {
    printHelp();
    process.exit(0);
  }

  const configPath = resolveConfigPath(settings);
  if (!fs.existsSync(configPath)) {
    console.error(`[sdkwork-im] standalone gateway config not found: ${configPath}`);
    process.exit(1);
  }

  const fileEnv = loadEnvFile(settings.devEnvFile, repoRoot);
  const gatewayEnv = {
    ...resolveIamDevEnv({ ...process.env, ...fileEnv }, repoRoot),
    SDKWORK_IM_STANDALONE_GATEWAY_CONFIG: configPath,
    SDKWORK_IM_STANDALONE_GATEWAY_ENVIRONMENT: settings.environment,
  };

  const cargoArgs = [
    'run',
    '-p',
    'sdkwork-im-standalone-gateway',
    '--bin',
    'sdkwork-im-standalone-gateway',
    '--',
    '--config',
    configPath,
  ];
  if (settings.release) {
    cargoArgs.splice(1, 0, '--release');
  }

  if (settings.dryRun) {
    console.log(`[sdkwork-im-standalone-gateway] ${cargoCommand()} ${cargoArgs.join(' ')}`);
    process.exit(0);
  }

  const child = spawn(cargoCommand(), cargoArgs, {
    cwd: repoRoot,
    env: gatewayEnv,
    stdio: 'inherit',
    shell: false,
  });

  child.on('exit', (code) => {
    process.exitCode = code ?? 1;
  });
}

main();
