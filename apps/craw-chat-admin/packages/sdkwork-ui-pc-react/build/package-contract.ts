const frameworkPackageName = '@sdkwork/ui-pc-react';

type FrameworkModuleDefinition = {
  readonly subpath: string;
  readonly entryName: string;
  readonly sourcePath: string;
};

type FrameworkAssetDefinition = {
  readonly subpath: string;
  readonly entryName: string;
  readonly sourcePath: string;
  readonly distPath: string;
};

export type FrameworkModuleContract = FrameworkModuleDefinition & {
  readonly distImportPath: string;
  readonly distTypesPath: string;
};

export type FrameworkAssetContract = FrameworkAssetDefinition;

const frameworkModuleDefinitions: readonly FrameworkModuleDefinition[] = [
  { subpath: '.', entryName: 'index', sourcePath: 'src/index.ts' },
  { subpath: './theme', entryName: 'theme', sourcePath: 'src/theme/index.ts' },
  { subpath: './components/ui', entryName: 'components-ui', sourcePath: 'src/components/ui/index.ts' },
  { subpath: './components/ui/actions', entryName: 'ui-actions', sourcePath: 'src/components/ui/actions/index.ts' },
  { subpath: './components/ui/data-entry', entryName: 'ui-data-entry', sourcePath: 'src/components/ui/data-entry/index.ts' },
  { subpath: './components/ui/data-display', entryName: 'ui-data-display', sourcePath: 'src/components/ui/data-display/index.ts' },
  { subpath: './components/ui/form', entryName: 'ui-form', sourcePath: 'src/components/ui/form/index.ts' },
  { subpath: './components/ui/feedback', entryName: 'ui-feedback', sourcePath: 'src/components/ui/feedback/index.ts' },
  { subpath: './components/ui/layout', entryName: 'ui-layout', sourcePath: 'src/components/ui/layout/index.ts' },
  { subpath: './components/ui/navigation', entryName: 'ui-navigation', sourcePath: 'src/components/ui/navigation/index.ts' },
  { subpath: './components/ui/overlays', entryName: 'ui-overlays', sourcePath: 'src/components/ui/overlays/index.ts' },
  { subpath: './components/ui/catalog', entryName: 'ui-catalog', sourcePath: 'src/components/ui/catalog.ts' },
  { subpath: './components/patterns', entryName: 'components-patterns', sourcePath: 'src/components/patterns/index.ts' },
  { subpath: './components/patterns/app-shell', entryName: 'patterns-app-shell', sourcePath: 'src/components/patterns/app-shell/index.ts' },
  { subpath: './components/patterns/command', entryName: 'patterns-command', sourcePath: 'src/components/patterns/command/index.ts' },
  { subpath: './components/patterns/desktop-shell', entryName: 'patterns-desktop-shell', sourcePath: 'src/components/patterns/desktop-shell/index.ts' },
  { subpath: './components/patterns/detail', entryName: 'patterns-detail', sourcePath: 'src/components/patterns/detail/index.ts' },
  { subpath: './components/patterns/picker', entryName: 'patterns-picker', sourcePath: 'src/components/patterns/picker/index.ts' },
  { subpath: './components/patterns/settings', entryName: 'patterns-settings', sourcePath: 'src/components/patterns/settings/index.ts' },
  { subpath: './components/patterns/system', entryName: 'patterns-system', sourcePath: 'src/components/patterns/system/index.ts' },
  { subpath: './components/patterns/workbench', entryName: 'patterns-workbench', sourcePath: 'src/components/patterns/workbench/index.ts' },
  { subpath: './components/patterns/workspace', entryName: 'patterns-workspace', sourcePath: 'src/components/patterns/workspace/index.ts' },
] as const;

export const frameworkModuleContracts: readonly FrameworkModuleContract[] = frameworkModuleDefinitions.map(
  (definition) => ({
    ...definition,
    distImportPath: `./dist/${definition.entryName}.js`,
    distTypesPath: toDistTypesPath(definition.sourcePath),
  }),
);

export const frameworkAssetContracts: readonly FrameworkAssetContract[] = [
  {
    subpath: './styles.css',
    entryName: 'styles',
    sourcePath: 'src/styles/index.ts',
    distPath: './dist/sdkwork-ui.css',
  },
] as const;

export const frameworkPackageFiles = ['dist', 'README.md'] as const;

export const frameworkPackageSideEffects = ['**/*.css'] as const;

const frameworkSourcePathBySubpath = new Map<string, string>([
  ...frameworkModuleContracts.map((contract) => [contract.subpath, contract.sourcePath] as const),
  ...frameworkAssetContracts.map((contract) => [contract.subpath, contract.sourcePath] as const),
]);

export function createFrameworkLibEntrySourceMap(): Record<string, string> {
  return Object.fromEntries([
    ...frameworkModuleContracts.map((contract) => [contract.entryName, contract.sourcePath] as const),
    ...frameworkAssetContracts.map((contract) => [contract.entryName, contract.sourcePath] as const),
  ]);
}

export function resolveFrameworkSourcePathForSubpath(subpath: string): string | undefined {
  return frameworkSourcePathBySubpath.get(subpath);
}

export function toFrameworkPackageSpecifier(subpath: string): string {
  return subpath === '.'
    ? frameworkPackageName
    : `${frameworkPackageName}/${subpath.replace(/^\.\//, '')}`;
}

function toDistTypesPath(sourcePath: string): string {
  return `./dist/src/${sourcePath.replace(/^src\//, '').replace(/\.(ts|tsx)$/, '.d.ts')}`;
}

export { frameworkPackageName };
