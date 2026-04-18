import { describe, expect, it } from 'vitest';
import ts from 'typescript';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for pattern slot props API contract.');
}

const tsConfigFile = ts.readConfigFile(tsConfigPath, ts.sys.readFile);
const tsConfig = ts.parseJsonConfigFileContent(tsConfigFile.config, ts.sys, process.cwd());
const program = ts.createProgram({
  rootNames: tsConfig.fileNames,
  options: tsConfig.options,
});
const checker = program.getTypeChecker();

const disallowedSlotClassNamePattern = /^(?!className$).+ClassName$/;

const requiredSlotPropsContracts = [
  'AnchoredPickerSurfaceProps',
  'CrudWorkbenchEditorProps',
  'CrudWorkbenchProps',
  'DesktopAppHeaderProps',
  'DesktopShellFrameProps',
  'DesktopTitleBarProps',
  'DetailDrawerProps',
  'EntityPickerDialogProps',
  'InspectorRailProps',
  'ListDetailWorkspaceContentProps',
  'ListDetailWorkspaceDetailProps',
  'ListDetailWorkspaceProps',
  'ListDetailWorkspaceSidebarProps',
  'ManagementWorkbenchDetailProps',
  'ManagementWorkbenchMainProps',
  'ManagementWorkbenchProps',
  'OperationDrawerProps',
  'PickerDialogProps',
  'TwoPaneSelectorPopoverProps',
  'WorkspacePanelProps',
  'WorkspaceScaffoldBottomProps',
  'WorkspaceScaffoldInspectorProps',
  'WorkspaceScaffoldMainProps',
  'WorkspaceScaffoldProps',
  'WorkspaceScaffoldSidebarProps',
] as const;

function getModuleExportSymbol(modulePath: string, exportName: string) {
  const normalizedModulePath = modulePath.replace(/\\/g, '/');
  const sourceFile = program
    .getSourceFiles()
    .find((candidate) => candidate.fileName.replace(/\\/g, '/').endsWith(normalizedModulePath));

  if (!sourceFile) {
    throw new Error(`Unable to resolve module for pattern slot props API contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for pattern slot props API contract: ${modulePath}`);
  }

  const exportSymbol = checker.getExportsOfModule(moduleSymbol).find((symbol) => symbol.getName() === exportName);

  if (!exportSymbol) {
    throw new Error(`Unable to resolve export ${exportName} from ${modulePath}`);
  }

  return exportSymbol;
}

function getExportPropertyNames(modulePath: string, exportName: string) {
  const exportSymbol = getModuleExportSymbol(modulePath, exportName);
  const declaredType = checker.getDeclaredTypeOfSymbol(exportSymbol);
  return checker.getPropertiesOfType(declaredType).map((property) => property.getName());
}

describe('pattern slot props API contract', () => {
  it('removes ad hoc internal ClassName props from public pattern contracts', () => {
    const patternModulePath = 'src/components/patterns/index.ts';
    const patternModuleSymbol = getModuleExportSymbol(patternModulePath, 'AppShell');
    const moduleSymbol = checker.getSymbolAtLocation(patternModuleSymbol.declarations?.[0] ?? patternModuleSymbol.valueDeclaration!);
    void moduleSymbol;

    const sourceFile = program
      .getSourceFiles()
      .find((candidate) => candidate.fileName.replace(/\\/g, '/').endsWith(patternModulePath));

    if (!sourceFile) {
      throw new Error(`Unable to resolve module for pattern slot props API contract: ${patternModulePath}`);
    }

    const sourceModuleSymbol = checker.getSymbolAtLocation(sourceFile)
      ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

    if (!sourceModuleSymbol) {
      throw new Error(`Unable to resolve module symbol for pattern slot props API contract: ${patternModulePath}`);
    }

    const offenders: string[] = [];

    for (const exportSymbol of checker.getExportsOfModule(sourceModuleSymbol)) {
      const exportName = exportSymbol.getName();

      if (!exportName.endsWith('Props')) {
        continue;
      }

      const propertyNames = checker
        .getPropertiesOfType(checker.getDeclaredTypeOfSymbol(exportSymbol))
        .map((property) => property.getName());

      for (const propertyName of propertyNames) {
        if (disallowedSlotClassNamePattern.test(propertyName)) {
          offenders.push(`${exportName}.${propertyName}`);
        }
      }
    }

    expect(offenders).toEqual([]);
  });

  it('publishes slotProps on composite public pattern contracts with internal regions', () => {
    const offenders = requiredSlotPropsContracts.filter(
      (exportName) => !getExportPropertyNames('src/components/patterns/index.ts', exportName).includes('slotProps'),
    );

    expect(offenders).toEqual([]);
  });
});
