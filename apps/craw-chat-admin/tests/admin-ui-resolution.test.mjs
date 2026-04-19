import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');
const uiPackageRoot = path.join(appRoot, 'packages', 'sdkwork-ui-pc-react');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function resolveUiDeclaration(relativePath) {
  return path.join(uiPackageRoot, relativePath);
}

test('craw-chat admin tsconfig and shim resolve @sdkwork/ui-pc-react through the local workspace dist contract', () => {
  const tsconfig = JSON.parse(read('tsconfig.json'));
  const shimSource = read('src/types/sdkwork-ui-pc-react-shim.d.ts');
  const rootPackageJson = JSON.parse(read('package.json'));
  const storagePackageJson = JSON.parse(
    read('packages/sdkwork-control-plane-storage/package.json'),
  );

  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react'], [
    'src/types/sdkwork-ui-pc-react-shim.d.ts',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/theme'], [
    'packages/sdkwork-ui-pc-react/dist/theme/index.d.ts',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/*'], [
    'packages/sdkwork-ui-pc-react/dist/*',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/styles.css'], [
    'packages/sdkwork-ui-pc-react/dist/sdkwork-ui.css',
  ]);

  assert.match(
    shimSource,
    /export \* from '\.\.\/\.\.\/packages\/sdkwork-ui-pc-react\/dist\/index'/,
  );
  assert.doesNotMatch(shimSource, /sdkwork-ui\/sdkwork-ui-pc-react/);
  assert.equal(rootPackageJson.dependencies['@sdkwork/ui-pc-react'], 'workspace:*');
  assert.equal(storagePackageJson.dependencies['@sdkwork/ui-pc-react'], 'workspace:*');

  assert.equal(existsSync(resolveUiDeclaration('dist/index.d.ts')), true);
  assert.equal(existsSync(resolveUiDeclaration('dist/theme/index.d.ts')), true);
  assert.equal(existsSync(resolveUiDeclaration('dist/components/ui/feedback/index.d.ts')), true);
});
