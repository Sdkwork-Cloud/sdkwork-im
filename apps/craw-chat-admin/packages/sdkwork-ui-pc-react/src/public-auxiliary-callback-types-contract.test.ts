import ts from 'typescript';
import { describe, expect, it } from 'vitest';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for public auxiliary callback types contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const auxiliaryCallbackTypeNamePattern = /(Factory|Handler|Renderer|Resolver|Subscriber|Unsubscribe)$/;
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
    throw new Error(`Unable to resolve source module for public auxiliary callback types contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for public auxiliary callback types contract: ${modulePath}`);
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

  if (ts.isTypeLiteralNode(typeNode)) {
    if (typeNode.members.some((member) => ts.isCallSignatureDeclaration(member) || ts.isMethodSignature(member))) {
      return [{ kind: 'anonymous' }];
    }

    return [];
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

function getTypeLiteralMembers(typeNode: ts.TypeNode): ts.TypeElement[] {
  if (ts.isParenthesizedTypeNode(typeNode)) {
    return getTypeLiteralMembers(typeNode.type);
  }

  if (ts.isUnionTypeNode(typeNode) || ts.isIntersectionTypeNode(typeNode)) {
    return typeNode.types.flatMap((memberTypeNode) => getTypeLiteralMembers(memberTypeNode));
  }

  if (ts.isTypeLiteralNode(typeNode)) {
    return [...typeNode.members];
  }

  return [];
}

type GovernedMember = {
  functionBranches: FunctionBranch[];
  memberName: string;
};

function getAuxiliaryCallbackMembers(symbol: ts.Symbol): GovernedMember[] {
  const resolvedSymbol = resolveExportSymbol(symbol);
  const symbolName = resolvedSymbol.getName();

  if (symbolName.endsWith('Props') || auxiliaryCallbackTypeNamePattern.test(symbolName)) {
    return [];
  }

  const typeAliasDeclaration = resolvedSymbol.declarations?.find(ts.isTypeAliasDeclaration);
  const interfaceDeclaration = resolvedSymbol.declarations?.find(ts.isInterfaceDeclaration);
  const members: GovernedMember[] = [];

  if (typeAliasDeclaration) {
    const callableBranches = getFunctionBranches(typeAliasDeclaration.type);

    if (callableBranches.length > 0) {
      members.push({
        functionBranches: callableBranches,
        memberName: '<callable>',
      });
    }

    for (const member of getTypeLiteralMembers(typeAliasDeclaration.type)) {
      if (ts.isPropertySignature(member) && member.type && ts.isIdentifier(member.name)) {
        const functionBranches = getFunctionBranches(member.type);

        if (functionBranches.length > 0) {
          members.push({
            functionBranches,
            memberName: member.name.text,
          });
        }

        continue;
      }

      if (ts.isMethodSignature(member) && ts.isIdentifier(member.name)) {
        members.push({
          functionBranches: [{ kind: 'anonymous' }],
          memberName: member.name.text,
        });
        continue;
      }

      if (ts.isCallSignatureDeclaration(member)) {
        members.push({
          functionBranches: [{ kind: 'anonymous' }],
          memberName: '<callable>',
        });
      }
    }

    return members;
  }

  if (!interfaceDeclaration) {
    return [];
  }

  for (const member of interfaceDeclaration.members) {
    if (ts.isPropertySignature(member) && member.type && ts.isIdentifier(member.name)) {
      const functionBranches = getFunctionBranches(member.type);

      if (functionBranches.length > 0) {
        members.push({
          functionBranches,
          memberName: member.name.text,
        });
      }

      continue;
    }

    if (ts.isMethodSignature(member) && ts.isIdentifier(member.name)) {
      members.push({
        functionBranches: [{ kind: 'anonymous' }],
        memberName: member.name.text,
      });
      continue;
    }

    if (ts.isCallSignatureDeclaration(member)) {
      members.push({
        functionBranches: [{ kind: 'anonymous' }],
        memberName: '<callable>',
      });
    }
  }

  return members;
}

describe('public auxiliary callback types contract', () => {
  it('publishes public non-props callback surfaces through named exported contracts', () => {
    const offenders: string[] = [];
    const rootExportNames = getModuleExportNames('src/index.ts');

    for (const modulePath of ['src/components/ui/index.ts', 'src/components/patterns/index.ts']) {
      const barrelExportNames = getModuleExportNames(modulePath);

      for (const exportSymbol of checker.getExportsOfModule(getModuleSymbol(modulePath))) {
        const resolvedSymbol = resolveExportSymbol(exportSymbol);
        const auxiliaryCallbackMembers = getAuxiliaryCallbackMembers(resolvedSymbol);

        for (const member of auxiliaryCallbackMembers) {
          for (const branch of member.functionBranches) {
            if (branch.kind === 'anonymous') {
              offenders.push(`${resolvedSymbol.getName()}.${member.memberName} uses an anonymous public callback type`);
              continue;
            }

            if (!auxiliaryCallbackTypeNamePattern.test(branch.typeName)) {
              offenders.push(
                `${resolvedSymbol.getName()}.${member.memberName} references ${branch.typeName} instead of a governed callback contract`,
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
