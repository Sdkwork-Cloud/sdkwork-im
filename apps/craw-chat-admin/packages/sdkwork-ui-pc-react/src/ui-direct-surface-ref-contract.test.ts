import ts from 'typescript';
import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for UI direct surface ref contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const abstractUiRuntimeComponents = new Set([
  'ContextMenu',
  'ContextMenuPortal',
  'ContextMenuSub',
  'Dialog',
  'DialogPortal',
  'Drawer',
  'DrawerPortal',
  'DropdownMenu',
  'DropdownMenuPortal',
  'DropdownMenuSub',
  'Form',
  'FormField',
  'HoverCard',
  'MenubarMenu',
  'MenubarPortal',
  'MenubarSub',
  'Modal',
  'ModalPortal',
  'Popover',
  'Select',
  'Tooltip',
  'TooltipProvider',
]);

function getModuleSymbol(modulePath: string) {
  const normalizedModulePath = modulePath.replace(/\\/g, '/');
  const sourceFile = program
    .getSourceFiles()
    .find((candidate) => candidate.fileName.replace(/\\/g, '/').endsWith(normalizedModulePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve module for UI direct surface ref contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for UI direct surface ref contract: ${modulePath}`);
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

function hasExactUiMarker(attributes: ts.JsxAttributes, marker: string) {
  return attributes.properties.some((attribute) => {
    if (
      !ts.isJsxAttribute(attribute)
      || !ts.isIdentifier(attribute.name)
      || attribute.name.text !== 'data-sdk-ui'
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

function declarationOwnsIntrinsicMarkedSurface(declaration: ts.Declaration, marker: string) {
  let ownsSurface = false;

  function visit(node: ts.Node): void {
    if (ownsSurface) {
      return;
    }

    if ((ts.isJsxOpeningElement(node) || ts.isJsxSelfClosingElement(node))
      && isIntrinsicJsxTag(node.tagName)
      && hasExactUiMarker(node.attributes, marker)) {
      ownsSurface = true;
      return;
    }

    ts.forEachChild(node, visit);
  }

  ts.forEachChild(declaration, visit);

  return ownsSurface;
}

describe('UI direct surface ref contract', () => {
  it('uses forwardRef for every public UI component that owns an intrinsic marked runtime surface', () => {
    const publicUiRuntimeExports = [
      ...uiComponentCatalog.actions,
      ...uiComponentCatalog.dataEntry,
      ...uiComponentCatalog.dataDisplay,
      ...uiComponentCatalog.feedback,
      ...uiComponentCatalog.form,
      ...uiComponentCatalog.layout,
      ...uiComponentCatalog.navigation,
      ...uiComponentCatalog.overlays,
    ].filter((exportName) => /^[A-Z]/.test(exportName) && !abstractUiRuntimeComponents.has(exportName));

    const moduleExports = checker.getExportsOfModule(getModuleSymbol('src/components/ui/index.ts'));
    const offenders: string[] = [];

    for (const exportName of publicUiRuntimeExports) {
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
