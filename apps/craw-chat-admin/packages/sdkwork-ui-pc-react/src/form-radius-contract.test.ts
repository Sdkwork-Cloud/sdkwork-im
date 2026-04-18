import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

function readSource(filePath: string) {
  return readFileSync(filePath, 'utf8');
}

describe('form radius contract', () => {
  it('publishes a compact radius ladder for field, control, and panel surfaces', () => {
    const themeSource = readSource('src/theme/sdkwork-theme.ts');
    const stylesheetSource = readSource('src/styles/sdkwork-ui.css');

    expect(themeSource).toContain('field: string;');
    expect(themeSource).toContain("control: '0.5rem'");
    expect(themeSource).toContain("field: '0.375rem'");
    expect(themeSource).toContain("panel: '1rem'");
    expect(themeSource).toContain("'--sdk-radius-control': theme.radius.control");
    expect(themeSource).toContain("'--sdk-radius-field': theme.radius.field");
    expect(themeSource).toContain("'--sdk-radius-panel': theme.radius.panel");
    expect(stylesheetSource).toContain('--sdk-radius-control: 0.5rem;');
    expect(stylesheetSource).toContain('--sdk-radius-field: 0.375rem;');
    expect(stylesheetSource).toContain('--sdk-radius-panel: 1rem;');
  });

  it('uses field radius on input-family controls while preserving control radius for buttons', () => {
    expect(readSource('src/components/ui/input.tsx')).toContain('rounded-[var(--sdk-radius-field)]');
    expect(readSource('src/components/ui/textarea.tsx')).toContain('rounded-[var(--sdk-radius-field)]');
    expect(readSource('src/components/ui/select.tsx')).toContain('rounded-[var(--sdk-radius-field)]');
    expect(readSource('src/components/ui/data-entry/date-input.tsx')).toContain('var(--sdk-radius-field)');
    expect(readSource('src/components/ui/data-entry/number-input.tsx')).toContain('var(--sdk-radius-field)');
    expect(readSource('src/components/ui/data-entry/segmented-control.tsx')).toContain('var(--sdk-radius-field)');
    expect(readSource('src/components/ui/button.tsx')).toContain('rounded-[var(--sdk-radius-control)]');
    expect(readSource('src/components/ui/button.tsx')).not.toContain('sdk-radius-field');
  });

  it('uses control radius instead of panel radius on form and upload shells', () => {
    expect(readSource('src/components/ui/form/form-layout.tsx')).toContain('rounded-[var(--sdk-radius-control)]');
    expect(readSource('src/components/ui/form/form-layout.tsx')).not.toContain('rounded-[var(--sdk-radius-panel)]');
    expect(readSource('src/components/ui/form/filter-bar.tsx')).toContain('rounded-[var(--sdk-radius-control)]');
    expect(readSource('src/components/ui/form/filter-bar.tsx')).not.toContain('rounded-[var(--sdk-radius-panel)]');
    expect(readSource('src/components/ui/data-entry/upload/upload-dropzone.tsx')).toContain(
      'rounded-[var(--sdk-radius-control)]',
    );
    expect(readSource('src/components/ui/data-entry/upload/upload-dropzone.tsx')).not.toContain(
      'rounded-[var(--sdk-radius-panel)]',
    );
    expect(readSource('src/components/ui/data-entry/upload/upload-item.tsx')).toContain(
      'rounded-[var(--sdk-radius-control)]',
    );
    expect(readSource('src/components/ui/data-entry/upload/upload-item.tsx')).not.toContain(
      'rounded-[var(--sdk-radius-panel)]',
    );
  });

  it('avoids ad hoc radius arithmetic on dense interactive surfaces', () => {
    const files = [
      'src/components/ui/actions/command.tsx',
      'src/components/ui/actions/toolbar-button.tsx',
      'src/components/ui/data-entry/date-input.tsx',
      'src/components/ui/data-entry/number-input.tsx',
      'src/components/ui/data-entry/segmented-control.tsx',
      'src/components/ui/dropdown-menu.tsx',
      'src/components/ui/feedback/toast.tsx',
      'src/components/ui/layout/sidebar-section.tsx',
      'src/components/ui/navigation/menubar.tsx',
      'src/components/ui/navigation/workspace-tabs.tsx',
      'src/components/ui/overlays/context-menu.tsx',
      'src/components/ui/select.tsx',
      'src/components/ui/tabs.tsx',
      'src/components/ui/tooltip.tsx',
    ];

    files.forEach((filePath) => {
      expect(readSource(filePath)).not.toContain('calc(var(--sdk-radius');
    });
  });

  it('uses field radius on dense selection rows and compact collection shells', () => {
    expect(readSource('src/components/ui/actions/command.tsx')).toContain(
      'rounded-[var(--sdk-radius-field)]',
    );
    expect(readSource('src/components/ui/dropdown-menu.tsx')).toContain(
      'rounded-[var(--sdk-radius-field)]',
    );
    expect(readSource('src/components/ui/navigation/menubar.tsx')).toContain(
      'rounded-[var(--sdk-radius-field)]',
    );
    expect(readSource('src/components/ui/data-display/tree.tsx')).toContain(
      'rounded-[var(--sdk-radius-field)]',
    );
    expect(readSource('src/components/ui/data-display/data-table/styles.ts')).toContain(
      'rounded-[var(--sdk-radius-field)]',
    );
  });
});
