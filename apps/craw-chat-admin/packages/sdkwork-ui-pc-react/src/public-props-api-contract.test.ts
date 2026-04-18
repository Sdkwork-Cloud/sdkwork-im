import { describe, expect, it } from 'vitest';
import ts from 'typescript';
import { uiComponentCatalog } from './components/ui/catalog';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public props API contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const exportCache = new Map<string, Set<string>>();

function getModuleExportNames(sourcePath: string): Set<string> {
  const normalizedSourcePath = sourcePath.replace(/\\/g, '/');
  const cached = exportCache.get(normalizedSourcePath);

  if (cached) {
    return cached;
  }

  const sourceFile = program.getSourceFiles().find((candidate) => candidate.fileName.replace(/\\/g, '/').endsWith(normalizedSourcePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve source module for public props API contract: ${sourcePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public props API contract: ${sourcePath}`);
  }

  const exportNames = new Set(checker.getExportsOfModule(moduleSymbol).map((symbol) => symbol.getName()));
  exportCache.set(normalizedSourcePath, exportNames);

  return exportNames;
}

const uiNamespaceExports = {
  actions: getModuleExportNames('src/components/ui/actions/index.ts'),
  dataDisplay: getModuleExportNames('src/components/ui/data-display/index.ts'),
  dataEntry: getModuleExportNames('src/components/ui/data-entry/index.ts'),
  feedback: getModuleExportNames('src/components/ui/feedback/index.ts'),
  form: getModuleExportNames('src/components/ui/form/index.ts'),
  layout: getModuleExportNames('src/components/ui/layout/index.ts'),
  navigation: getModuleExportNames('src/components/ui/navigation/index.ts'),
  overlays: getModuleExportNames('src/components/ui/overlays/index.ts'),
} as const;

const uiExports = getModuleExportNames('src/components/ui/index.ts');
const patternExports = getModuleExportNames('src/components/patterns/index.ts');
const packageExports = getModuleExportNames('src/index.ts');

describe('public props API contract', () => {
  it('publishes every ComponentNameProps contract from the owning public entrypoints', () => {
    const offenders: string[] = [];

    for (const [category, exportsSet] of Object.entries(uiNamespaceExports) as Array<
      [keyof typeof uiNamespaceExports, Set<string>]
    >) {
      for (const componentName of uiComponentCatalog[category]) {
        if (!/^[A-Z]/.test(componentName)) {
          continue;
        }

        const propsName = `${componentName}Props`;

        if (!exportsSet.has(propsName)) {
          offenders.push(`src/components/ui/${category} missing ${propsName}`);
        }

        if (!uiExports.has(propsName)) {
          offenders.push(`src/components/ui missing ${propsName}`);
        }

        if (!packageExports.has(propsName)) {
          offenders.push(`src/index missing ${propsName}`);
        }
      }
    }

    for (const componentName of uiComponentCatalog.patterns) {
      if (!/^[A-Z]/.test(componentName)) {
        continue;
      }

      const propsName = `${componentName}Props`;

      if (!patternExports.has(propsName)) {
        offenders.push(`src/components/patterns missing ${propsName}`);
      }

      if (!packageExports.has(propsName)) {
        offenders.push(`src/index missing ${propsName}`);
      }
    }

    expect(offenders).toEqual([]);
  });
});
