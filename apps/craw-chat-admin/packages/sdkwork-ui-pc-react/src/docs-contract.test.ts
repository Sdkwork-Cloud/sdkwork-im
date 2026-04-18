import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import ts from 'typescript';
import { describe, expect, it } from 'vitest';
import packageJson from '../package.json';
import {
  frameworkPackageName,
  resolveFrameworkSourcePathForSubpath,
  toFrameworkPackageSpecifier,
} from '../build/package-contract';

const documentedFiles = [
  'README.md',
  'docs/guide/getting-started.md',
  'docs/reference/package.md',
] as const;

const publishedSubpaths = new Set(Object.keys(packageJson.exports));
const packageExportNames = new Map<string, Set<string>>();
const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for docs contract test.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

type ImportReference = {
  readonly filePath: string;
  readonly packageSpecifier: string;
  readonly importedNames: readonly string[];
  readonly isSideEffectOnly: boolean;
};

function normalizeFilePath(filePath: string): string {
  return resolve(filePath).replace(/\\/g, '/');
}

function getModuleExportNames(sourcePath: string): Set<string> {
  const normalizedSourcePath = normalizeFilePath(sourcePath);
  const cached = packageExportNames.get(normalizedSourcePath);

  if (cached) {
    return cached;
  }

  const sourceFile = program.getSourceFiles().find((candidate) => candidate.fileName.replace(/\\/g, '/') === normalizedSourcePath);

  if (!sourceFile) {
    throw new Error(`Unable to resolve source module for docs contract: ${sourcePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for docs contract: ${sourcePath}`);
  }

  const exportNames = new Set(checker.getExportsOfModule(moduleSymbol).map((symbol) => symbol.getName()));
  packageExportNames.set(normalizedSourcePath, exportNames);

  return exportNames;
}

function toPackageSubpath(packageSpecifier: string): string {
  return packageSpecifier === frameworkPackageName
    ? '.'
    : `./${packageSpecifier.slice(frameworkPackageName.length + 1)}`;
}

function collectImportReferences(markdownPath: string): ImportReference[] {
  const markdown = readFileSync(markdownPath, 'utf8');
  const codeBlockPattern = /```(?:[a-z]+)?\r?\n([\s\S]*?)```/g;
  const references: ImportReference[] = [];
  let match: RegExpExecArray | null;
  let codeBlockIndex = 0;

  while ((match = codeBlockPattern.exec(markdown)) !== null) {
    const codeBlock = match[1];
    const sourceFile = ts.createSourceFile(
      `${markdownPath}#block-${codeBlockIndex}.tsx`,
      codeBlock,
      ts.ScriptTarget.Latest,
      true,
      ts.ScriptKind.TSX,
    );

    sourceFile.forEachChild((node) => {
      if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier)) {
        return;
      }

      const packageSpecifier = node.moduleSpecifier.text;

      if (!packageSpecifier.startsWith(frameworkPackageName)) {
        return;
      }

      const importedNames: string[] = [];

      if (node.importClause?.name) {
        importedNames.push('default');
      }

      if (node.importClause?.namedBindings) {
        if (ts.isNamespaceImport(node.importClause.namedBindings)) {
          importedNames.push('*');
        } else {
          importedNames.push(
            ...node.importClause.namedBindings.elements.map(
              (element) => element.propertyName?.text ?? element.name.text,
            ),
          );
        }
      }

      references.push({
        filePath: markdownPath,
        packageSpecifier,
        importedNames,
        isSideEffectOnly: !node.importClause,
      });
    });

    codeBlockIndex += 1;
  }

  return references;
}

function extractReadmeExportList(): string[] {
  const readme = readFileSync('README.md', 'utf8');
  const packageExportsSectionMatch = readme.match(/## Package Exports\r?\n([\s\S]*?)\r?\n## /);

  if (!packageExportsSectionMatch) {
    throw new Error('README.md is missing the "Package Exports" section.');
  }

  return packageExportsSectionMatch[1]
    .split(/\r?\n/)
    .map((line) => line.match(/^- `([^`]+)`$/)?.[1])
    .filter((line): line is string => Boolean(line));
}

describe('documentation contract', () => {
  it('keeps README package export inventory aligned with package.json', () => {
    const documentedExportList = extractReadmeExportList();
    const expectedExportList = Object.keys(packageJson.exports).map((subpath) => toFrameworkPackageSpecifier(subpath));

    expect(documentedExportList).toEqual(expectedExportList);
  });

  it('documents only published package subpaths', () => {
    const offenders = documentedFiles
      .flatMap(collectImportReferences)
      .filter((reference) => !publishedSubpaths.has(toPackageSubpath(reference.packageSpecifier)))
      .map((reference) => `${reference.filePath} -> ${reference.packageSpecifier}`);

    expect(offenders).toEqual([]);
  });

  it('keeps documented imports aligned with real source exports', () => {
    const offenders = documentedFiles.flatMap((filePath) => {
      const references = collectImportReferences(filePath);

      return references.flatMap((reference) => {
        const subpath = toPackageSubpath(reference.packageSpecifier);

        if (subpath === './styles.css') {
          return reference.isSideEffectOnly
            ? []
            : [`${reference.filePath} imports bindings from stylesheet export ${reference.packageSpecifier}`];
        }

        const sourcePath = resolveFrameworkSourcePathForSubpath(subpath);

        if (!sourcePath) {
          return [`${reference.filePath} uses undocumented framework subpath ${reference.packageSpecifier}`];
        }

        const exportNames = getModuleExportNames(sourcePath);

        return reference.importedNames
          .filter((importedName) => importedName !== '*')
          .filter((importedName) => !exportNames.has(importedName))
          .map((importedName) => `${reference.filePath} imports ${importedName} from ${reference.packageSpecifier}, but ${sourcePath} does not export it`);
      });
    });

    expect(offenders).toEqual([]);
  });
});
