import { describe, expect, it } from 'vitest';
import ts from 'typescript';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for UI slot props API contract.');
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
    throw new Error(`Unable to resolve module for UI slot props API contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for UI slot props API contract: ${modulePath}`);
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

describe('UI slot props API contract', () => {
  it('publishes slotProps on public composite UI contracts with delegated or internal regions', () => {
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'ActionMenuButtonProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'SplitButtonProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'CommandDialogProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'CollectionGridProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableProps')).toContain('slotProps');
  });

  it('removes ad hoc internal ClassName props from governed composite UI contracts', () => {
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'ActionMenuButtonProps')).not.toContain('menuClassName');
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'SplitButtonProps')).not.toContain('menuClassName');
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'CommandDialogProps')).not.toContain('contentClassName');
    expect(getExportPropertyNames('src/components/ui/actions/index.ts', 'CommandDialogProps')).not.toContain('contentProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'CollectionGridProps')).not.toContain('gridClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableProps')).not.toContain('rowClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableColumn')).not.toContain('headerClassName');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableColumn')).not.toContain('className');
  });

  it('uses structured props instead of isolated class hooks for DataTable row and column surfaces', () => {
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableProps')).toContain('getRowProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableColumn')).toContain('cellProps');
    expect(getExportPropertyNames('src/components/ui/data-display/index.ts', 'DataTableColumn')).toContain('headerProps');
  });
});
