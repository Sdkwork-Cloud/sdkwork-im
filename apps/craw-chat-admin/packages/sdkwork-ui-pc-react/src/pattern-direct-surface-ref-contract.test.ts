import ts from 'typescript';
import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for pattern direct surface ref contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

function getModuleSymbol(modulePath: string) {
  const normalizedModulePath = modulePath.replace(/\\/g, '/');
  const sourceFile = program
    .getSourceFiles()
    .find((candidate) => candidate.fileName.replace(/\\/g, '/').endsWith(normalizedModulePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve module for pattern direct surface ref contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for pattern direct surface ref contract: ${modulePath}`);
  }

  return moduleSymbol;
}

function resolveExportSymbol(symbol: ts.Symbol) {
  if (symbol.flags & ts.SymbolFlags.Alias) {
    return checker.getAliasedSymbol(symbol);
  }

  return symbol;
}

function toKebabCase(value: string) {
  return value
    .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1-$2')
    .toLowerCase();
}

function isForwardRefDeclaration(declaration: ts.Declaration | undefined, sourceFile: ts.SourceFile) {
  if (!declaration || !ts.isVariableDeclaration(declaration) || !declaration.initializer) {
    return false;
  }

  if (!ts.isCallExpression(declaration.initializer)) {
    return false;
  }

  const expression = declaration.initializer.expression;

  return ts.isPropertyAccessExpression(expression)
    && expression.expression.getText(sourceFile) === 'React'
    && expression.name.text === 'forwardRef';
}

function getJsxTagNameText(
  tagName: ts.JsxTagNameExpression | ts.Identifier | ts.PropertyAccessExpression | ts.ThisExpression | ts.JsxNamespacedName,
) {
  if (ts.isIdentifier(tagName)) {
    return tagName.text;
  }

  if (ts.isPropertyAccessExpression(tagName)) {
    return tagName.name.text;
  }

  if (ts.isJsxNamespacedName(tagName)) {
    return tagName.name.text;
  }

  return tagName.getText();
}

function isIntrinsicJsxTag(tagName: ts.JsxOpeningLikeElement['tagName']) {
  return /^[a-z]/.test(getJsxTagNameText(tagName));
}

function hasExactPatternMarker(attributes: ts.JsxAttributes, marker: string) {
  return attributes.properties.some((attribute) => {
    if (ts.isJsxSpreadAttribute(attribute)) {
      return spreadExpressionHasExactPatternMarker(attribute.expression, marker);
    }

    if (
      !ts.isJsxAttribute(attribute)
      || !ts.isIdentifier(attribute.name)
      || attribute.name.text !== 'data-sdk-pattern'
      || !attribute.initializer
    ) {
      return false;
    }

    if (ts.isStringLiteral(attribute.initializer)) {
      return attribute.initializer.text === marker;
    }

    return ts.isJsxExpression(attribute.initializer)
      && !!attribute.initializer.expression
      && ts.isStringLiteral(attribute.initializer.expression)
      && attribute.initializer.expression.text === marker;
  });
}

function spreadExpressionHasExactPatternMarker(expression: ts.Expression, marker: string): boolean {
  if (ts.isObjectLiteralExpression(expression)) {
    return objectLiteralHasExactPatternMarker(expression, marker);
  }

  if (ts.isCallExpression(expression)) {
    return expression.arguments.some((argument) => (
      ts.isObjectLiteralExpression(argument) && objectLiteralHasExactPatternMarker(argument, marker)
    ));
  }

  return false;
}

function objectLiteralHasExactPatternMarker(objectLiteral: ts.ObjectLiteralExpression, marker: string) {
  return objectLiteral.properties.some((property) => {
    if (!ts.isPropertyAssignment(property)) {
      return false;
    }

    const propertyName = getObjectLiteralPropertyName(property.name);
    if (propertyName !== 'data-sdk-pattern') {
      return false;
    }

    return isStringLiteralLikeExpression(property.initializer, marker);
  });
}

function getObjectLiteralPropertyName(name: ts.PropertyName) {
  if (ts.isIdentifier(name) || ts.isStringLiteral(name) || ts.isNumericLiteral(name)) {
    return name.text;
  }

  return name.getText().replace(/^['"]|['"]$/g, '');
}

function isStringLiteralLikeExpression(expression: ts.Expression, expectedValue: string) {
  if (ts.isStringLiteral(expression) || ts.isNoSubstitutionTemplateLiteral(expression)) {
    return expression.text === expectedValue;
  }

  return false;
}

function declarationOwnsIntrinsicMarkedSurface(declaration: ts.Declaration, marker: string) {
  let ownsSurface = false;

  function visit(node: ts.Node): void {
    if (ownsSurface) {
      return;
    }

    if ((ts.isJsxOpeningElement(node) || ts.isJsxSelfClosingElement(node))
      && isIntrinsicJsxTag(node.tagName)
      && hasExactPatternMarker(node.attributes, marker)) {
      ownsSurface = true;
      return;
    }

    ts.forEachChild(node, visit);
  }

  ts.forEachChild(declaration, visit);

  return ownsSurface;
}

describe('pattern direct surface ref contract', () => {
  it('uses forwardRef for every public pattern component that owns an intrinsic marked runtime surface', () => {
    const moduleExports = checker.getExportsOfModule(getModuleSymbol('src/components/patterns/index.ts'));
    const offenders: string[] = [];

    for (const exportName of uiComponentCatalog.patterns) {
      const exportSymbol = moduleExports.find((candidate) => resolveExportSymbol(candidate).getName() === exportName);

      if (!exportSymbol) {
        continue;
      }

      const resolvedSymbol = resolveExportSymbol(exportSymbol);
      const declaration = resolvedSymbol.valueDeclaration ?? resolvedSymbol.declarations?.[0];

      if (!declaration) {
        continue;
      }

      const sourceFile = declaration.getSourceFile();
      const expectedMarker = toKebabCase(exportName);

      if (!declarationOwnsIntrinsicMarkedSurface(declaration, expectedMarker)) {
        continue;
      }

      if (!isForwardRefDeclaration(declaration, sourceFile)) {
        offenders.push(`${exportName} -> ${sourceFile.fileName.replace(/\\\\/g, '/')}`);
      }
    }

    expect(offenders).toEqual([]);
  });
});
