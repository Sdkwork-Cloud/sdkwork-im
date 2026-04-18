import { describe, expect, it } from 'vitest';
import ts from 'typescript';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for UI collection surface API contract.');
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
    throw new Error(`Unable to resolve module for UI collection surface API contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for UI collection surface API contract: ${modulePath}`);
  }

  return moduleSymbol;
}

function getExportPropertyNames(modulePath: string, exportName: string) {
  const moduleSymbol = getModuleSymbol(modulePath);
  const exportSymbol = checker.getExportsOfModule(moduleSymbol).find((symbol) => symbol.getName() === exportName);

  if (!exportSymbol) {
    throw new Error(`Unable to resolve export ${exportName} from ${modulePath}`);
  }

  return checker
    .getPropertiesOfType(checker.getDeclaredTypeOfSymbol(exportSymbol))
    .map((property) => property.getName());
}

describe('UI collection surface API contract', () => {
  it('publishes structured collection surface props on data-driven collection components', () => {
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'RichTreeProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'RichTreeProps')).toContain('getItemProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'RichTreeProps')).toContain('getItemSlotProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'TimelineProps')).toContain('getItemProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'TimelineProps')).toContain('getItemSlotProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedProps')).toContain('getItemProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedProps')).toContain('getItemSlotProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterProps')).toContain('getItemProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterProps')).toContain(
      'getItemSlotProps',
    );
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabsProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabsProps')).toContain(
      'getItemProps',
    );
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabsProps')).toContain(
      'getItemSlotProps',
    );
  });

  it('publishes item-level slot props on public collection item components', () => {
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedItemProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterItemProps')).toContain(
      'slotProps',
    );
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperProps')).toContain('getItemProps');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperProps')).toContain('getItemSlotProps');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperItemProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabProps')).toContain(
      'slotProps',
    );
  });

  it('forbids ad hoc repeated-surface class hooks on governed collection contracts', () => {
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'RichTreeProps')).not.toContain('itemClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'RichTreeProps')).not.toContain('contentClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'TimelineProps')).not.toContain('itemClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'TimelineProps')).not.toContain('indicatorClassName');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedProps')).not.toContain('itemClassName');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'ActivityFeedProps')).not.toContain('headerClassName');
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterProps')).not.toContain(
      'itemClassName',
    );
    expect(getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterProps')).not.toContain(
      'headerClassName',
    );
    expect(
      getExportPropertyNames('src/components/ui/feedback/index.ts', 'NotificationCenterItemProps'),
    ).not.toContain('actionClassName');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperProps')).not.toContain('itemClassName');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperItemProps')).not.toContain('indicatorClassName');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperItemProps')).not.toContain('connectorClassName');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'StepperItemProps')).not.toContain('contentClassName');
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabsProps')).not.toContain(
      'itemClassName',
    );
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabsProps')).not.toContain(
      'tabListClassName',
    );
    expect(getExportPropertyNames('src/components/ui/navigation/index.ts', 'WorkspaceTabProps')).not.toContain(
      'closeButtonClassName',
    );
  });
});
