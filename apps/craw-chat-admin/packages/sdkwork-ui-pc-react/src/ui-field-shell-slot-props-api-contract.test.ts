import { describe, expect, it } from 'vitest';
import ts from 'typescript';

const tsConfigPath = ts.findConfigFile(process.cwd(), ts.sys.fileExists, 'tsconfig.json');

if (!tsConfigPath) {
  throw new Error('Unable to locate tsconfig.json for UI field shell slot props API contract.');
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
    throw new Error(`Unable to resolve module for UI field shell slot props API contract: ${modulePath}`);
  }

  const moduleSymbol = checker.getSymbolAtLocation(sourceFile)
    ?? (sourceFile as ts.SourceFile & { symbol?: ts.Symbol }).symbol;

  if (!moduleSymbol) {
    throw new Error(`Unable to resolve module symbol for UI field shell slot props API contract: ${modulePath}`);
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

describe('UI field shell slot props API contract', () => {
  it('publishes slotProps on composite field controls with owned shell surfaces', () => {
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'ComboboxProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'DateInputProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'NumberInputProps')).toContain('slotProps');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'TagInputProps')).toContain('slotProps');
  });

  it('forbids ad hoc shell class hooks on governed field controls', () => {
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'ComboboxProps')).not.toContain('clearButtonClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'ComboboxProps')).not.toContain('contentClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'ComboboxProps')).not.toContain('menuClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'DateInputProps')).not.toContain('calendarButtonClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'DateInputProps')).not.toContain('rootClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'NumberInputProps')).not.toContain('stepperClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'NumberInputProps')).not.toContain('rootClassName');
    expect(getExportPropertyNames('src/components/ui/data-entry/index.ts', 'TagInputProps')).not.toContain('rootClassName');
  });
});
