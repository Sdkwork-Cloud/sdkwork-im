import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const starExportOnlyPattern = /^\s*(export\s+\*\s+from\s+['"][^'"]+['"];\s*)+$/;

const publicBarrelFiles = [
  'src/index.ts',
  'src/components/patterns/index.ts',
  'src/components/patterns/app-shell/index.ts',
  'src/components/patterns/command/index.ts',
  'src/components/patterns/desktop-shell/index.ts',
  'src/components/patterns/detail/index.ts',
  'src/components/patterns/picker/index.ts',
  'src/components/patterns/settings/index.ts',
  'src/components/patterns/system/index.ts',
  'src/components/patterns/workbench/index.ts',
  'src/components/patterns/workspace/index.ts',
  'src/components/ui/index.ts',
  'src/components/ui/actions/index.ts',
  'src/components/ui/data-display/index.ts',
  'src/components/ui/data-entry/index.ts',
  'src/components/ui/data-entry/upload/index.ts',
  'src/components/ui/feedback/index.ts',
  'src/components/ui/form/index.ts',
  'src/components/ui/layout/index.ts',
  'src/components/ui/navigation/index.ts',
  'src/components/ui/overlays/index.ts',
  'src/theme/index.ts',
] as const;

describe('barrel contract', () => {
  it('keeps public barrel entrypoints as exact star re-exports', () => {
    const offenders = publicBarrelFiles.filter((filePath) => {
      const content = readFileSync(filePath, 'utf8').trim();

      return !starExportOnlyPattern.test(content);
    });

    expect(offenders).toEqual([]);
  });
});
