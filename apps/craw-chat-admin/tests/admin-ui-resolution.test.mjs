import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');
const uiPackageRoot = path.resolve(
  appRoot,
  '..',
  '..',
  '..',
  '..',
  '..',
  'sdkwork-ui',
  'sdkwork-ui-pc-react',
);

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function resolveUiDeclaration(relativePath) {
  return path.join(uiPackageRoot, relativePath);
}

test('craw-chat admin tsconfig and shim resolve @sdkwork/ui-pc-react through current dist declaration entries', () => {
  const tsconfig = JSON.parse(read('tsconfig.json'));
  const shimSource = read('src/types/sdkwork-ui-pc-react-shim.d.ts');

  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react'], [
    'src/types/sdkwork-ui-pc-react-shim.d.ts',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/theme'], [
    '../../../../../sdkwork-ui/sdkwork-ui-pc-react/dist/theme/index.d.ts',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/*'], [
    '../../../../../sdkwork-ui/sdkwork-ui-pc-react/dist/*',
  ]);
  assert.deepEqual(tsconfig.compilerOptions.paths['@sdkwork/ui-pc-react/styles.css'], [
    '../../../../../sdkwork-ui/sdkwork-ui-pc-react/dist/sdkwork-ui.css',
  ]);

  assert.match(
    shimSource,
    /export \* from '\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/sdkwork-ui\/sdkwork-ui-pc-react\/dist\/index'/,
  );

  assert.equal(existsSync(resolveUiDeclaration('dist/index.d.ts')), true);
  assert.equal(existsSync(resolveUiDeclaration('dist/theme/index.d.ts')), true);
  assert.equal(existsSync(resolveUiDeclaration('dist/components/ui/feedback/index.d.ts')), true);
});
