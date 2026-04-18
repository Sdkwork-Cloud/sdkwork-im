import { existsSync, readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import packageJson from '../package.json';
import {
  frameworkAssetContracts,
  frameworkModuleContracts,
  frameworkPackageFiles,
  frameworkPackageSideEffects,
} from '../build/package-contract';

const expectedPackageExports = Object.fromEntries([
  ...frameworkModuleContracts.map((contract) => [
    contract.subpath,
    {
      types: contract.distTypesPath,
      import: contract.distImportPath,
    },
  ]),
  ...frameworkAssetContracts.map((contract) => [contract.subpath, contract.distPath]),
]);

describe('package publish contract', () => {
  it('keeps published files, side effects, and export map aligned with the framework contract', () => {
    expect(packageJson.files).toEqual([...frameworkPackageFiles]);
    expect(packageJson.sideEffects).toEqual([...frameworkPackageSideEffects]);
    expect(packageJson.exports).toEqual(expectedPackageExports);
  });

  it('keeps legacy root entry fields aligned with the root subpath export', () => {
    const rootContract = frameworkModuleContracts.find((contract) => contract.subpath === '.');

    expect(rootContract).toBeDefined();
    expect(packageJson.main).toBe(rootContract?.distImportPath);
    expect(packageJson.module).toBe(rootContract?.distImportPath);
    expect(packageJson.types).toBe(rootContract?.distTypesPath);
  });

  it('backs every published subpath with a real source entry', () => {
    const offenders = [...frameworkModuleContracts, ...frameworkAssetContracts]
      .filter((contract) => !existsSync(contract.sourcePath))
      .map((contract) => `${contract.subpath} -> ${contract.sourcePath}`);

    expect(offenders).toEqual([]);
  });

  it('keeps JavaScript entrypoints side-effect-free and isolates CSS to the stylesheet entry', () => {
    const jsEntrypointOffenders = frameworkModuleContracts
      .filter((contract) => /\.css['"]/.test(readFileSync(contract.sourcePath, 'utf8')))
      .map((contract) => contract.sourcePath);
    const stylesheetEntrySource = readFileSync('src/styles/index.ts', 'utf8').trim();

    expect(jsEntrypointOffenders).toEqual([]);
    expect(stylesheetEntrySource).toBe("import './sdkwork-ui.css';");
  });
});
