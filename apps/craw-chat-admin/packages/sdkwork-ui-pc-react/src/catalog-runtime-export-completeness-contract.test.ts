import ts from 'typescript';
import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for catalog runtime export completeness contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const catalogRuntimeExports = new Set<string>([
  ...uiComponentCatalog.actions,
  ...uiComponentCatalog.dataEntry,
  ...uiComponentCatalog.dataDisplay,
  ...uiComponentCatalog.feedback,
  ...uiComponentCatalog.form,
  ...uiComponentCatalog.layout,
  ...uiComponentCatalog.navigation,
  ...uiComponentCatalog.overlays,
  ...uiComponentCatalog.patterns,
]);

function normalizePath(filePath: string) {
  return filePath.replace(/\\/g, '/');
}

function getModuleSymbol(modulePath: string) {
  const normalizedModulePath = normalizePath(modulePath);
  const sourceFile = program
    .getSourceFiles()
    .find((candidate) => normalizePath(candidate.fileName).endsWith(normalizedModulePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve source module for catalog runtime export completeness contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for catalog runtime export completeness contract: ${modulePath}`);
  }

  return moduleSymbol;
}

function resolveExportSymbol(symbol: ts.Symbol) {
  if (symbol.flags & ts.SymbolFlags.Alias) {
    return checker.getAliasedSymbol(symbol);
  }

  return symbol;
}

function isProbablyRuntimeComponent(symbol: ts.Symbol) {
  const resolvedSymbol = resolveExportSymbol(symbol);
  const symbolName = resolvedSymbol.getName();

  if (!/^[A-Z]/.test(symbolName)) {
    return false;
  }

  if (/(Api|Context|Controller|Entry|Handler|Options|Props|Renderer|Resolver|Result|SlotProps)$/.test(symbolName)) {
    return false;
  }

  const declaration = resolvedSymbol.valueDeclaration ?? resolvedSymbol.declarations?.[0];

  if (!declaration) {
    return false;
  }

  const symbolType = checker.getTypeOfSymbolAtLocation(resolvedSymbol, declaration);

  return symbolType.getCallSignatures().length > 0;
}

describe('catalog runtime export completeness contract', () => {
  it('keeps the catalog aligned with every public runtime component export', () => {
    const offenders: string[] = [];

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      for (const exportSymbol of checker.getExportsOfModule(getModuleSymbol(modulePath))) {
        const resolvedSymbol = resolveExportSymbol(exportSymbol);
        const exportName = resolvedSymbol.getName();

        if (!isProbablyRuntimeComponent(resolvedSymbol)) {
          continue;
        }

        if (!catalogRuntimeExports.has(exportName)) {
          offenders.push(`${modulePath} runtime export ${exportName} missing from uiComponentCatalog`);
        }
      }
    }

    expect(offenders).toEqual([]);
  });
});
