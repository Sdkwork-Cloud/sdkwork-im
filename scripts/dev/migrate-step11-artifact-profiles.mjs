#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const root = path.join(repoRoot, 'artifacts', 'perf', 'step-11');

const replacements = [
  ['"profile": "local-default"', '"profile": "self-hosted.split-services.development"'],
  ['"sourceProfile": "local-minimal"', '"sourceProfile": "self-hosted.split-services.development"'],
  [
    'CI Smoke Tier / local-minimal output',
    'CI Smoke Tier / self-hosted.split-services.development output',
  ],
  [
    'CI Smoke Tier / local-minimal evidence',
    'CI Smoke Tier / self-hosted.split-services.development evidence',
  ],
  ['local-default / capacity-dedicated', 'self-hosted.split-services.development / capacity-dedicated'],
  ['local-minimal evidence backfill', 'self-hosted.split-services.development evidence backfill'],
  ['for local-minimal evidence', 'for self-hosted.split-services.development evidence'],
  [
    'profile = local-default / capacity-dedicated',
    'profile = self-hosted.split-services.development / capacity-dedicated',
  ],
];

function walk(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const absolute = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(absolute);
      continue;
    }
    if (!/\.(?:json|md)$/iu.test(entry.name)) {
      continue;
    }
    let content = fs.readFileSync(absolute, 'utf8');
    const original = content;
    for (const [from, to] of replacements) {
      content = content.replaceAll(from, to);
    }
    if (content !== original) {
      fs.writeFileSync(absolute, content);
      console.log(`updated ${path.relative(repoRoot, absolute)}`);
    }
  }
}

walk(root);
console.log('step-11 artifact profile migration complete');
