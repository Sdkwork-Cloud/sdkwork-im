#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-control-plane-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    sourceFile: '',
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--source-file') {
      parsed.sourceFile = argv[index + 1] || '';
      index += 1;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.sourceFile) {
    fail('Missing required argument: --source-file');
  }

  return parsed;
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const outputPath = path.join(workspaceRoot, 'openapi', 'control-plane.openapi.json');
const sourcePath = path.resolve(args.sourceFile);

if (!existsSync(sourcePath)) {
  fail(`Source OpenAPI file not found: ${sourcePath}`);
}

let document;
try {
  document = JSON.parse(readFileSync(sourcePath, 'utf8'));
} catch (error) {
  fail(error instanceof Error ? `Source OpenAPI is not valid JSON: ${error.message}` : String(error));
}

const nextContents = `${JSON.stringify(document, null, 2)}\n`;
mkdirSync(path.dirname(outputPath), { recursive: true });
if (!existsSync(outputPath) || readFileSync(outputPath, 'utf8') !== nextContents) {
  writeFileSync(outputPath, nextContents, 'utf8');
}

process.stdout.write(outputPath);
