#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

const scanRoots = [
  'adapters',
  'artifacts',
  'docs/架构',
  'docs/step',
  'docs/review',
  'docs/部署',
  'README.md',
  'AGENTS.md',
];

const skipPathFragments = [
  '/target/',
  '/node_modules/',
  'migrate-topology-v2-baggage.mjs',
  'migrate-step11-artifact-profiles.mjs',
  'sdkwork-im-topology-baggage.test.mjs',
];

const replacements = [
  ['services/local-minimal-node', 'services/sdkwork-im-gateway'],
  ['cargo test -p local-minimal-node', 'cargo test -p sdkwork-im-gateway'],
  ['local-minimal-node', 'sdkwork-im-server'],
  ['"profile": "local-default"', '"profile": "self-hosted.split-services.development"'],
  ['"sourceProfile": "local-minimal"', '"sourceProfile": "self-hosted.split-services.development"'],
  ['-ProfileName local-default', '-ProfileName self-hosted.split-services.development'],
  ['profile = local-default', 'profile = self-hosted.split-services.development'],
  ['profile = local-minimal', 'profile = self-hosted.split-services.development'],
  ['local-default / capacity-dedicated', 'self-hosted.split-services.development / capacity-dedicated'],
  ['evidence/local-default', 'evidence/self-hosted.split-services.development'],
  [
    'local-default-post-release-evidence-index',
    'self-hosted.split-services.development-post-release-evidence-index',
  ],
  ['.runtime/local-minimal', '.runtime/self-hosted.split-services.development'],
  ['127.0.0.1:28090', '127.0.0.1:18079'],
  ['127.0.0.1:18090', '127.0.0.1:18079'],
  ['CI Smoke Tier / local-minimal output', 'CI Smoke Tier / self-hosted.split-services.development output'],
  ['CI Smoke Tier / local-minimal evidence', 'CI Smoke Tier / self-hosted.split-services.development evidence'],
  ['CI Smoke Tier / local-minimal`', 'CI Smoke Tier / self-hosted.split-services.development`'],
  ['for local-minimal evidence', 'for self-hosted.split-services.development evidence'],
  ['local-minimal evidence backfill', 'self-hosted.split-services.development evidence backfill'],
  ['local-minimal service contract', 'self-hosted.split-services.development service contract'],
  ['默认预发布 profile：`local-default`', '默认预发布 profile：`self-hosted.split-services.development`'],
  ['默认预发布 profile 为 `local-default`', '默认预发布 profile 为 `self-hosted.split-services.development`'],
  ['`local-default`', '`self-hosted.split-services.development`'],
  ['reuses the current `local-minimal`', 'uses topology v2 `self-hosted.split-services.development`'],
  ['powershell -NoProfile -ExecutionPolicy Bypass -File bin/deploy-local.ps1 -ProfileName self-hosted.split-services.development -SmokeBaseUrl http://127.0.0.1:18079', 'pnpm dev'],
  ['powershell -NoProfile -ExecutionPolicy Bypass -File bin/status-local.ps1 -ProfileName self-hosted.split-services.development', 'pnpm dev:server (topology v2)'],
  ['bin/deploy-local.ps1', 'pnpm dev'],
  ['bin/deploy-local.cmd', 'pnpm dev'],
  ['bin/deploy-local.sh', 'pnpm dev'],
  ['bin/start-local.ps1', 'pnpm dev:server'],
  ['bin/start-local.sh', 'pnpm dev:server'],
  ['bin/install-local.ps1', '(retired lifecycle script)'],
  ['bin/install-local.cmd', '(retired lifecycle script)'],
  ['bin/install-local.sh', '(retired lifecycle script)'],
  ['deployments/docker-compose/local-minimal.yml', '(removed compose file)'],
  ['docs/部署/local-default发布后验证样本.md', 'docs/部署/性能与灾备演练场景.md'],
  ['docs/部署/local-default发布后验证执行记录模板.md', 'docs/部署/性能与灾备演练场景.md'],
];

function applyRegexCleanup(content) {
  let next = content;
  next = next.replace(/(?<![\w-])local-minimal(?![\w-])/gu, 'self-hosted.split-services.development');
  next = next.replace(/(?<![\w-])local-default(?![\w-])/gu, 'self-hosted.split-services.development');
  next = next.replace(/\binstall-local\b/gu, 'retired-lifecycle-install');
  next = next.replace(/\bstart-local\b/gu, 'retired-lifecycle-start');
  next = next.replace(/\bdeploy-local\b/gu, 'retired-lifecycle-deploy');
  next = next.replace(/\bstatus-local\b/gu, 'retired-lifecycle-status');
  next = next.replace(/\bstop-local\b/gu, 'retired-lifecycle-stop');
  next = next.replace(/\brestart-local\b/gu, 'retired-lifecycle-restart');
  next = next.replace(/\.runtime\/local-[A-Za-z0-9.*-]+/gu, '.runtime/self-hosted.split-services.development');
  return next;
}

function slash(value) {
  return String(value).replaceAll('\\', '/');
}

function shouldSkip(relativePath) {
  const normalized = slash(relativePath);
  return skipPathFragments.some((fragment) => normalized.includes(fragment));
}

function listFiles(relativeRoot) {
  const absoluteRoot = path.join(repoRoot, relativeRoot);
  if (!fs.existsSync(absoluteRoot)) {
    return [];
  }
  const stat = fs.statSync(absoluteRoot);
  if (stat.isFile()) {
    return [relativeRoot];
  }
  const files = [];
  for (const entry of fs.readdirSync(absoluteRoot, { withFileTypes: true })) {
    const relativePath = path.join(relativeRoot, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFiles(relativePath));
      continue;
    }
    files.push(relativePath);
  }
  return files;
}

function isTextCandidate(relativePath) {
  return /\.(?:md|json|mjs|txt|rs|toml|yaml|yml|env\.example)$/u.test(relativePath);
}

let updatedCount = 0;

for (const root of scanRoots) {
  for (const relativePath of listFiles(root)) {
    if (!isTextCandidate(relativePath) || shouldSkip(relativePath)) {
      continue;
    }
    const absolutePath = path.join(repoRoot, relativePath);
    let content = fs.readFileSync(absolutePath, 'utf8');
    const original = content;
    for (const [from, to] of replacements) {
      content = content.replaceAll(from, to);
    }
    content = applyRegexCleanup(content);
    if (content !== original) {
      fs.writeFileSync(absolutePath, content);
      updatedCount += 1;
      console.log(`updated ${slash(relativePath)}`);
    }
  }
}

const legacyEvidenceDir = path.join(
  repoRoot,
  'artifacts',
  'releases',
  'wave-d-2026-04-08',
  'evidence',
  'local-default',
);
const migratedEvidenceDir = path.join(
  repoRoot,
  'artifacts',
  'releases',
  'wave-d-2026-04-08',
  'evidence',
  'self-hosted.split-services.development',
);
if (fs.existsSync(legacyEvidenceDir) && !fs.existsSync(migratedEvidenceDir)) {
  fs.renameSync(legacyEvidenceDir, migratedEvidenceDir);
  console.log('renamed evidence/local-default -> evidence/self-hosted.split-services.development');
}

console.log(`topology v2 baggage migration complete (${updatedCount} files updated)`);
