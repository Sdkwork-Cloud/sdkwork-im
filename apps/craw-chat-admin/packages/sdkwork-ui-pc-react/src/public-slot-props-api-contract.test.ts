import ts from 'typescript';
import { describe, expect, it } from 'vitest';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public slot props API contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const exportCache = new Map<string, Set<string>>();

function normalizePath(filePath: string) {
  return filePath.replace(/\\/g, '/');
}

function getModuleSymbol(modulePath: string) {
  const normalizedModulePath = normalizePath(modulePath);
  const sourceFile = program
    .getSourceFiles()
    .find((candidate) => normalizePath(candidate.fileName).endsWith(normalizedModulePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve source module for public slot props API contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public slot props API contract: ${modulePath}`);
  }

  return moduleSymbol;
}

function getModuleExportNames(modulePath: string) {
  const normalizedModulePath = normalizePath(modulePath);
  const cached = exportCache.get(normalizedModulePath);

  if (cached) {
    return cached;
  }

  const moduleSymbol = getModuleSymbol(modulePath);
  const exportNames = new Set(checker.getExportsOfModule(moduleSymbol).map((symbol) => symbol.getName()));
  exportCache.set(normalizedModulePath, exportNames);
  return exportNames;
}

function getPropsExportSymbols(modulePath: string) {
  return checker
    .getExportsOfModule(getModuleSymbol(modulePath))
    .filter((symbol) => symbol.getName().endsWith('Props'));
}

function getSlotPropsTypeName(symbol: ts.Symbol) {
  const declaration = symbol.declarations?.find(ts.isInterfaceDeclaration) ?? symbol.declarations?.find(ts.isTypeAliasDeclaration);

  if (!declaration || !('members' in declaration)) {
    return undefined;
  }

  for (const member of declaration.members) {
    if (!ts.isPropertySignature(member) || member.name.getText() !== 'slotProps' || !member.type) {
      continue;
    }

    if (!ts.isTypeReferenceNode(member.type) || !ts.isIdentifier(member.type.typeName)) {
      return '<anonymous>';
    }

    return member.type.typeName.text;
  }

  return undefined;
}

describe('public slot props API contract', () => {
  it('publishes slotProps through explicit named SlotProps contracts instead of anonymous shapes', () => {
    const offenders: string[] = [];
    const rootExportNames = getModuleExportNames('src/index.ts');

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      const barrelExportNames = getModuleExportNames(modulePath);

      for (const exportSymbol of getPropsExportSymbols(modulePath)) {
        const slotPropsTypeName = getSlotPropsTypeName(exportSymbol);

        if (!slotPropsTypeName) {
          continue;
        }

        if (slotPropsTypeName === '<anonymous>') {
          offenders.push(`${exportSymbol.getName()}.slotProps uses an anonymous type`);
          continue;
        }

        if (!slotPropsTypeName.endsWith('SlotProps')) {
          offenders.push(`${exportSymbol.getName()}.slotProps references ${slotPropsTypeName} instead of a *SlotProps contract`);
        }

        if (!barrelExportNames.has(slotPropsTypeName)) {
          offenders.push(`${modulePath} missing ${slotPropsTypeName}`);
        }

        if (!rootExportNames.has(slotPropsTypeName)) {
          offenders.push(`src/index.ts missing ${slotPropsTypeName}`);
        }
      }
    }

    expect(offenders).toEqual([]);
  });
});
