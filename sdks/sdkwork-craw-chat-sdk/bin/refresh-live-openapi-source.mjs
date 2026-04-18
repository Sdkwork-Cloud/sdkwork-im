#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { resolveSdkworkGeneratorRoot } from './sdk-paths.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    schemaUrl: '',
    output: '',
    timeoutMs: 30000,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];

    if (current === '--schema-url') {
      parsed.schemaUrl = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--output') {
      parsed.output = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--timeout-ms') {
      const value = Number.parseInt(argv[index + 1] || '', 10);
      if (!Number.isFinite(value) || value <= 0) {
        fail(`Invalid --timeout-ms value: ${argv[index + 1] || ''}`);
      }
      parsed.timeoutMs = value;
      index += 1;
      continue;
    }

    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.schemaUrl) {
    fail('Missing required argument: --schema-url');
  }
  if (!parsed.output) {
    fail('Missing required argument: --output');
  }

  return parsed;
}

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  const generatorRoot = resolveSdkworkGeneratorRoot(workspaceRoot);
  const yamlPath = path.join(generatorRoot, 'node_modules', 'js-yaml', 'dist', 'js-yaml.mjs');

  if (!existsSync(yamlPath)) {
    fail(`Unable to locate js-yaml from generator workspace: ${yamlPath}`);
  }

  const yamlModule = await import(pathToFileURL(yamlPath).href);
  return yamlModule.default;
}

function parseOpenApiDocument(raw, yaml, sourceDescription) {
  const trimmed = raw.trim();
  if (!trimmed) {
    fail(`${sourceDescription} returned an empty schema document.`);
  }

  let document;
  try {
    document = trimmed.startsWith('{') || trimmed.startsWith('[')
      ? JSON.parse(trimmed)
      : yaml.load(raw);
  } catch (error) {
    fail(`${sourceDescription} did not return valid JSON or YAML: ${error.message}`);
  }

  if (!document || typeof document !== 'object') {
    fail(`${sourceDescription} did not decode to an OpenAPI object.`);
  }
  if (typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    fail(`${sourceDescription} must return an OpenAPI 3.x document.`);
  }

  return document;
}

function normalizeOutput(raw, response, document, yaml) {
  const contentType = response.headers.get('content-type') || '';
  const trimmed = raw.trim();

  if (contentType.includes('json') || trimmed.startsWith('{') || trimmed.startsWith('[')) {
    return yaml.dump(document, {
      noRefs: true,
      sortKeys: false,
      lineWidth: 120,
    });
  }

  return raw.endsWith('\n') ? raw : `${raw}\n`;
}

const args = parseArgs(process.argv.slice(2));
const yaml = await loadYaml();
const outputPath = path.resolve(args.output);
const response = await fetch(args.schemaUrl, {
  headers: {
    accept: 'application/yaml, text/yaml, application/json, text/plain;q=0.9, */*;q=0.8',
  },
  signal: AbortSignal.timeout(args.timeoutMs),
}).catch((error) => {
  fail(`Failed to fetch live OpenAPI schema from ${args.schemaUrl}: ${error.message}`);
});

if (!response.ok) {
  fail(`Live OpenAPI schema request failed (${response.status} ${response.statusText}) for ${args.schemaUrl}`);
}

const raw = await response.text();
const document = parseOpenApiDocument(raw, yaml, args.schemaUrl);
const nextContents = normalizeOutput(raw, response, document, yaml);

if (existsSync(outputPath)) {
  const currentContents = readFileSync(outputPath, 'utf8');
  if (currentContents === nextContents) {
    process.stdout.write(outputPath);
    process.exit(0);
  }
}

mkdirSync(path.dirname(outputPath), { recursive: true });
writeFileSync(outputPath, nextContents, 'utf8');
process.stdout.write(outputPath);
