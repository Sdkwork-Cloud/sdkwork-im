import ts from 'typescript';
import { describe, expect, it } from 'vitest';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public semantic callback types contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const semanticCallbackPropNames = new Set([
  'cell',
  'confirmLabel',
  'getItemBadge',
  'getItemDescription',
  'getItemId',
  'getItemKey',
  'getItemLabel',
  'getRowId',
  'getRowSelectionLabel',
  'getSectionDescription',
  'getSectionId',
  'getSectionLabel',
  'loadChildren',
  'renderActions',
  'renderItem',
  'renderLabel',
  'renderSection',
  'rowActions',
  'selectionSummary',
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
    throw new Error(`Unable to resolve source module for public semantic callback types contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public semantic callback types contract: ${modulePath}`);
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

function isFunctionLikeTypeDeclaration(node: ts.Node | undefined): boolean {
  if (!node) {
    return false;
  }

  if (ts.isFunctionTypeNode(node) || ts.isCallSignatureDeclaration(node) || ts.isMethodSignature(node)) {
    return true;
  }

  if (ts.isTypeAliasDeclaration(node)) {
    return isFunctionLikeTypeDeclaration(node.type);
  }

  if (ts.isParenthesizedTypeNode(node)) {
    return isFunctionLikeTypeDeclaration(node.type);
  }

  if (ts.isUnionTypeNode(node) || ts.isIntersectionTypeNode(node)) {
    return node.types.some((memberTypeNode) => isFunctionLikeTypeDeclaration(memberTypeNode));
  }

  if (ts.isTypeLiteralNode(node)) {
    return node.members.some((member) => isFunctionLikeTypeDeclaration(member));
  }

  return false;
}

type FunctionBranch =
  | { kind: 'anonymous' }
  | { kind: 'named'; typeName: string };

function getFunctionBranches(typeNode: ts.TypeNode): FunctionBranch[] {
  if (ts.isParenthesizedTypeNode(typeNode)) {
    return getFunctionBranches(typeNode.type);
  }

  if (ts.isUnionTypeNode(typeNode) || ts.isIntersectionTypeNode(typeNode)) {
    return typeNode.types.flatMap((memberTypeNode) => getFunctionBranches(memberTypeNode));
  }

  if (ts.isFunctionTypeNode(typeNode)) {
    return [{ kind: 'anonymous' }];
  }

  if (!ts.isTypeReferenceNode(typeNode)) {
    return [];
  }

  const typeNameText = typeNode.typeName.getText();
  const referencedType = checker.getTypeFromTypeNode(typeNode);
  const referencedSymbol = referencedType.aliasSymbol ?? referencedType.getSymbol();
  const resolvedSymbol = referencedSymbol ? resolveExportSymbol(referencedSymbol) : undefined;
  const declaration = resolvedSymbol?.declarations?.[0];

  if (!isFunctionLikeTypeDeclaration(declaration)) {
    return [];
  }

  return [{ kind: 'named', typeName: typeNameText }];
}

function getSemanticCallbackMembers(symbol: ts.Symbol) {
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

    if (!semanticCallbackPropNames.has(member.name.text)) {
      return [];
    }

    return [{
      propertyName: member.name.text,
      functionBranches: getFunctionBranches(member.type),
    }];
  });
}

describe('public semantic callback types contract', () => {
  it('publishes semantic collection and resolver callbacks through named exported Renderer, Resolver, or Handler contracts', () => {
    const offenders: string[] = [];
    const rootExportNames = getModuleExportNames('src/index.ts');

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      const barrelExportNames = getModuleExportNames(modulePath);

      for (const exportSymbol of checker.getExportsOfModule(getModuleSymbol(modulePath))) {
        const resolvedSymbol = resolveExportSymbol(exportSymbol);
        const semanticCallbackMembers = getSemanticCallbackMembers(resolvedSymbol);

        for (const member of semanticCallbackMembers) {
          for (const branch of member.functionBranches) {
            if (branch.kind === 'anonymous') {
              offenders.push(`${resolvedSymbol.getName()}.${member.propertyName} uses an anonymous public callback type`);
              continue;
            }

            if (!/(Renderer|Resolver|Handler)$/.test(branch.typeName)) {
              offenders.push(
                `${resolvedSymbol.getName()}.${member.propertyName} references ${branch.typeName} instead of a *Renderer, *Resolver, or *Handler contract`,
              );
            }

            if (!barrelExportNames.has(branch.typeName)) {
              offenders.push(`${modulePath} missing ${branch.typeName}`);
            }

            if (!rootExportNames.has(branch.typeName)) {
              offenders.push(`src/index.ts missing ${branch.typeName}`);
            }
          }
        }
      }
    }

    expect(offenders).toEqual([]);
  });
});
