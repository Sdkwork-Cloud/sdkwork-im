import ts from 'typescript';
import { describe, expect, it } from 'vitest';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public helper types contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const publicContractNamePattern = /(Props|SlotProps|Context|Item|Section|Entry|Api|Controller|Result|Options)$/;
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
    throw new Error(`Unable to resolve source module for public helper types contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public helper types contract: ${modulePath}`);
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

function isExplicitlyExportedSymbol(symbol: ts.Symbol) {
  return (symbol.getDeclarations() ?? []).some((declaration) => {
    if (!('modifiers' in declaration) || !Array.isArray(declaration.modifiers)) {
      return false;
    }

    return declaration.modifiers.some((modifier: ts.ModifierLike) => modifier.kind === ts.SyntaxKind.ExportKeyword);
  });
}

function collectTypeReferenceNodes(node: ts.Node) {
  const typeReferenceNodes: ts.TypeReferenceNode[] = [];

  function visit(candidate: ts.Node) {
    if (ts.isTypeReferenceNode(candidate)) {
      typeReferenceNodes.push(candidate);
    }

    ts.forEachChild(candidate, visit);
  }

  visit(node);
  return typeReferenceNodes;
}

describe('public helper types contract', () => {
  it('publishes every local named helper type referenced by a public contract', () => {
    const offenders = new Set<string>();
    const rootExportNames = getModuleExportNames('src/index.ts');

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      const barrelExportNames = getModuleExportNames(modulePath);

      for (const exportSymbol of checker.getExportsOfModule(getModuleSymbol(modulePath))) {
        const resolvedSymbol = resolveExportSymbol(exportSymbol);
        const publicContractDeclarations = (resolvedSymbol.declarations ?? []).filter(
          (declaration): declaration is ts.InterfaceDeclaration | ts.TypeAliasDeclaration =>
            (ts.isInterfaceDeclaration(declaration) || ts.isTypeAliasDeclaration(declaration))
            && publicContractNamePattern.test(resolvedSymbol.getName()),
        );

        for (const declaration of publicContractDeclarations) {
          const declarationPath = normalizePath(declaration.getSourceFile().fileName);
          const genericTypeParameterNames = new Set((declaration.typeParameters ?? []).map((typeParameter) => typeParameter.name.text));

          for (const typeReferenceNode of collectTypeReferenceNodes(declaration)) {
            const referencedSymbol = checker.getSymbolAtLocation(typeReferenceNode.typeName);
            const resolvedReferenceSymbol = referencedSymbol ? resolveExportSymbol(referencedSymbol) : undefined;

            if (!resolvedReferenceSymbol) {
              continue;
            }

            const referencedName = resolvedReferenceSymbol.getName();

            if (!/^[A-Z]/.test(referencedName) || genericTypeParameterNames.has(referencedName)) {
              continue;
            }

            const referencedDeclaration = resolvedReferenceSymbol.declarations?.[0];

            if (!referencedDeclaration || normalizePath(referencedDeclaration.getSourceFile().fileName) !== declarationPath) {
              continue;
            }

            if (referencedName === resolvedSymbol.getName()) {
              continue;
            }

            if (!isExplicitlyExportedSymbol(resolvedReferenceSymbol)) {
              offenders.add(`${resolvedSymbol.getName()} references private local helper type ${referencedName}`);
              continue;
            }

            if (!barrelExportNames.has(referencedName)) {
              offenders.add(`${modulePath} missing ${referencedName}`);
            }

            if (!rootExportNames.has(referencedName)) {
              offenders.add(`src/index.ts missing ${referencedName}`);
            }
          }
        }
      }
    }

    expect([...offenders]).toEqual([]);
  });
});
