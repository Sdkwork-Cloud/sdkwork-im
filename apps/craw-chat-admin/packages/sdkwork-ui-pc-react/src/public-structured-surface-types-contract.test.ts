import ts from 'typescript';
import { describe, expect, it } from 'vitest';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public structured surface types contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const structuredSurfacePropNames = new Set([
  'cellProps',
  'getItemProps',
  'getItemSlotProps',
  'getRowProps',
  'headerProps',
]);

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
    throw new Error(`Unable to resolve source module for public structured surface types contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public structured surface types contract: ${modulePath}`);
  }

  return moduleSymbol;
}

function getModuleExportNames(modulePath: string) {
  const normalizedModulePath = normalizePath(modulePath);
  const cached = exportCache.get(normalizedModulePath);

  if (cached) {
    return cached;
  }

  const exportNames = new Set(checker.getExportsOfModule(getModuleSymbol(modulePath)).map((symbol) => symbol.getName()));
  exportCache.set(normalizedModulePath, exportNames);
  return exportNames;
}

function resolveExportSymbol(symbol: ts.Symbol) {
  if (symbol.flags & ts.SymbolFlags.Alias) {
    return checker.getAliasedSymbol(symbol);
  }

  return symbol;
}

function getStructuredSurfaceTypeNames(typeNode: ts.TypeNode): string[] {
  if (ts.isParenthesizedTypeNode(typeNode)) {
    return getStructuredSurfaceTypeNames(typeNode.type);
  }

  if (ts.isUnionTypeNode(typeNode)) {
    return typeNode.types.flatMap((memberTypeNode) => getStructuredSurfaceTypeNames(memberTypeNode));
  }

  if (ts.isTypeReferenceNode(typeNode)) {
    if (ts.isIdentifier(typeNode.typeName)) {
      return [typeNode.typeName.text];
    }

    return ['<anonymous>'];
  }

  return ['<anonymous>'];
}

function getStructuredSurfaceMembers(symbol: ts.Symbol) {
  const resolvedSymbol = resolveExportSymbol(symbol);
  const declaration = resolvedSymbol.declarations?.find(ts.isInterfaceDeclaration)
    ?? resolvedSymbol.declarations?.find(
      (candidate): candidate is ts.TypeAliasDeclaration =>
        ts.isTypeAliasDeclaration(candidate) && ts.isTypeLiteralNode(candidate.type),
    );

  if (!declaration || !('members' in declaration)) {
    return [];
  }

  return declaration.members.flatMap((member) => {
    if (!ts.isPropertySignature(member) || !member.type || !ts.isIdentifier(member.name)) {
      return [];
    }

    if (!structuredSurfacePropNames.has(member.name.text)) {
      return [];
    }

    return [
      {
        propertyName: member.name.text,
        typeNames: getStructuredSurfaceTypeNames(member.type),
      },
    ];
  });
}

describe('public structured surface types contract', () => {
  it('publishes public structured surface props through named exported Props or PropsResolver contracts', () => {
    const offenders: string[] = [];
    const rootExportNames = getModuleExportNames('src/index.ts');

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      const barrelExportNames = getModuleExportNames(modulePath);

      for (const exportSymbol of checker.getExportsOfModule(getModuleSymbol(modulePath))) {
        const resolvedSymbol = resolveExportSymbol(exportSymbol);
        const structuredSurfaceMembers = getStructuredSurfaceMembers(resolvedSymbol);

        for (const member of structuredSurfaceMembers) {
          for (const typeName of member.typeNames) {
            if (typeName === '<anonymous>') {
              offenders.push(`${resolvedSymbol.getName()}.${member.propertyName} uses an anonymous public type`);
              continue;
            }

            if (!/(Props|PropsResolver)$/.test(typeName)) {
              offenders.push(
                `${resolvedSymbol.getName()}.${member.propertyName} references ${typeName} instead of a *Props or *PropsResolver contract`,
              );
            }

            if (!barrelExportNames.has(typeName)) {
              offenders.push(`${modulePath} missing ${typeName}`);
            }

            if (!rootExportNames.has(typeName)) {
              offenders.push(`src/index.ts missing ${typeName}`);
            }
          }
        }
      }
    }

    expect(offenders).toEqual([]);
  });
});
