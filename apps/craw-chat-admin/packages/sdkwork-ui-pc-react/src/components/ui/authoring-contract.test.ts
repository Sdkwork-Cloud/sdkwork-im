import { readFileSync, readdirSync } from 'node:fs';
import { join, relative } from 'node:path';
import ts from 'typescript';
import { describe, expect, it } from 'vitest';

function collectComponentFiles(root: string): string[] {
  const entries = readdirSync(root, { withFileTypes: true });

  return entries.flatMap((entry) => {
    const filePath = join(root, entry.name);

    if (entry.isDirectory()) {
      return collectComponentFiles(filePath);
    }

    return filePath.endsWith('.tsx') && !filePath.endsWith('.test.tsx') ? [filePath] : [];
  });
}

function collectExportedPropsTypes(sourceFile: ts.SourceFile): Set<string> {
  const exportedTypes = new Set<string>();

  sourceFile.forEachChild((node) => {
    if (!(ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node))) {
      return;
    }

    const isExported = node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword);

    if (!isExported) {
      return;
    }

    exportedTypes.add(node.name.text);
  });

  return exportedTypes;
}

function describeForwardRefPropsType(
  sourceFile: ts.SourceFile,
  typeNode: ts.TypeNode | undefined,
): string {
  if (!typeNode) {
    return '<missing>';
  }

  return typeNode.getText(sourceFile).replace(/\s+/g, ' ').trim();
}

describe('authoring contract', () => {
  it('uses explicit exported ComponentNameProps contracts for forwardRef components', () => {
    const componentFiles = collectComponentFiles('src/components');

    const offenders = componentFiles.flatMap((filePath) => {
      const source = readFileSync(filePath, 'utf8');
      const sourceFile = ts.createSourceFile(filePath, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
      const exportedTypes = collectExportedPropsTypes(sourceFile);
      const fileOffenders: string[] = [];

      function visit(node: ts.Node) {
        if (
          ts.isVariableDeclaration(node)
          && ts.isIdentifier(node.name)
          && node.initializer
          && ts.isCallExpression(node.initializer)
          && ts.isPropertyAccessExpression(node.initializer.expression)
          && node.initializer.expression.expression.getText(sourceFile) === 'React'
          && node.initializer.expression.name.text === 'forwardRef'
        ) {
          const componentName = node.name.text;
          const expectedPropsName = `${componentName}Props`;
          const propsTypeNode = node.initializer.typeArguments?.[1];
          const usesNamedPropsType = propsTypeNode
            && ts.isTypeReferenceNode(propsTypeNode)
            && ts.isIdentifier(propsTypeNode.typeName)
            && propsTypeNode.typeName.text === expectedPropsName;
          const exportsExpectedPropsType = exportedTypes.has(expectedPropsName);

          if (!usesNamedPropsType || !exportsExpectedPropsType) {
            fileOffenders.push(
              `${relative(process.cwd(), filePath)} -> ${componentName} :: expected exported ${expectedPropsName}, found ${describeForwardRefPropsType(sourceFile, propsTypeNode)}`,
            );
          }
        }

        ts.forEachChild(node, visit);
      }

      visit(sourceFile);

      return fileOffenders;
    });

    expect(offenders).toEqual([]);
  });
});
