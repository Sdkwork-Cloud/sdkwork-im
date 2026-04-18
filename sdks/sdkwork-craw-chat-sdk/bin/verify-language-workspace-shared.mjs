#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(language, message) {
  console.error(`[sdkwork-craw-chat-sdk] ${language} workspace verification failed: ${message}`);
  process.exit(1);
}

function assertConsumerPackage(language, entry, expected) {
  if (!expected) {
    return;
  }
  if (!entry.consumerPackage) {
    fail(language, 'Assembly entry must record consumerPackage metadata.');
  }
  for (const [field, expectedValue] of Object.entries(expected)) {
    if (entry.consumerPackage?.[field] !== expectedValue) {
      fail(
        language,
        `Assembly consumerPackage ${field} mismatch: expected ${expectedValue}, received ${entry.consumerPackage?.[field]}.`,
      );
    }
  }
}

function assertRequiredPackageLayers(language, entry, requiredPackageLayers = []) {
  for (const requiredLayer of requiredPackageLayers) {
    if (!entry.packages?.some((candidate) => candidate.layer === requiredLayer)) {
      fail(language, `Assembly entry must record the ${requiredLayer} package layer.`);
    }
  }
}

function assertReadmeRequiredTerms(language, readmeSource, readmeRequiredTerms = []) {
  for (const requiredTerm of readmeRequiredTerms) {
    if (!readmeSource.includes(requiredTerm)) {
      fail(language, `README must include the language-specific marker ${requiredTerm}.`);
    }
  }
}

function assertReadmeForbiddenTerms(language, readmeSource, readmeForbiddenTerms = []) {
  for (const forbiddenTerm of readmeForbiddenTerms) {
    if (readmeSource.includes(forbiddenTerm)) {
      fail(language, `README must not include the language-specific marker ${forbiddenTerm}.`);
    }
  }
}

function assertScopedReadmeTerms(
  language,
  readmePath,
  readmeLabel,
  requiredTerms = [],
  forbiddenTerms = [],
) {
  if (!requiredTerms.length && !forbiddenTerms.length) {
    return;
  }
  if (!existsSync(readmePath)) {
    fail(language, `Missing ${readmeLabel}: ${readmePath}`);
  }

  const readmeSource = readFileSync(readmePath, 'utf8');
  for (const requiredTerm of requiredTerms) {
    if (!readmeSource.includes(requiredTerm)) {
      fail(language, `${readmeLabel} must include the language-specific marker ${requiredTerm}.`);
    }
  }
  for (const forbiddenTerm of forbiddenTerms) {
    if (readmeSource.includes(forbiddenTerm)) {
      fail(language, `${readmeLabel} must not include the language-specific marker ${forbiddenTerm}.`);
    }
  }
}

export function verifyLanguageWorkspace(config) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  const workspacePath = path.join(workspaceRoot, config.workspace);
  const readmePath = path.join(workspacePath, 'README.md');
  const generatedPath = path.join(workspacePath, 'generated', 'server-openapi');
  const generatedReadmePath = path.join(generatedPath, 'README.md');
  const composedPath = path.join(workspacePath, 'composed');
  const composedReadmePath = path.join(composedPath, 'README.md');
  const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');

  for (const [label, targetPath] of [
    ['workspace', workspacePath],
    ['README', readmePath],
    ['generated boundary', generatedPath],
    ['composed boundary', composedPath],
    ['assembly metadata', assemblyPath],
  ]) {
    if (!existsSync(targetPath)) {
      fail(config.language, `Missing ${label}: ${targetPath}`);
    }
  }

  const readmeSource = readFileSync(readmePath, 'utf8');
  if (!/generated\/server-openapi/.test(readmeSource)) {
    fail(config.language, 'README must document generated/server-openapi as the generator-owned boundary.');
  }
  if (!/\bcomposed\b/.test(readmeSource)) {
    fail(config.language, 'README must document composed as the manual-owned boundary.');
  }
  if (!/sdk-gen/.test(readmeSource) || !/sdk-verify/.test(readmeSource)) {
    fail(config.language, 'README must document sdk-gen and sdk-verify entrypoints.');
  }
  assertReadmeRequiredTerms(config.language, readmeSource, config.readmeRequiredTerms);
  assertReadmeForbiddenTerms(config.language, readmeSource, config.readmeForbiddenTerms);
  assertScopedReadmeTerms(
    config.language,
    generatedReadmePath,
    'Generated README',
    config.generatedReadmeRequiredTerms,
    config.generatedReadmeForbiddenTerms,
  );
  assertScopedReadmeTerms(
    config.language,
    composedReadmePath,
    'Composed README',
    config.composedReadmeRequiredTerms,
    config.composedReadmeForbiddenTerms,
  );

  const assembly = JSON.parse(readFileSync(assemblyPath, 'utf8'));
  const entry = Array.isArray(assembly.languages)
    ? assembly.languages.find((candidate) => candidate.language === config.language)
    : null;
  if (!entry) {
    fail(config.language, '.sdkwork-assembly.json is missing the language entry.');
  }
  if (entry.workspace !== config.workspace) {
    fail(config.language, `Assembly workspace mismatch: expected ${config.workspace}, received ${entry.workspace}.`);
  }
  if (entry.primaryClient !== config.primaryClient) {
    fail(config.language, `Assembly primary client mismatch: expected ${config.primaryClient}, received ${entry.primaryClient}.`);
  }
  if (entry.maturityTier !== config.maturityTier) {
    fail(config.language, `Assembly maturity tier mismatch: expected ${config.maturityTier}, received ${entry.maturityTier}.`);
  }
  if (!Array.isArray(entry.packages) || entry.packages.length < 2) {
    fail(config.language, 'Assembly entry must record generated and composed package layers.');
  }
  assertRequiredPackageLayers(config.language, entry, config.requiredPackageLayers);
  assertConsumerPackage(config.language, entry, config.consumerPackage);

  console.log(`[sdkwork-craw-chat-sdk] ${config.language} workspace verification passed.`);
}
