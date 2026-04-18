import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';

type RuntimeMetadataKind = 'pattern' | 'ui';

const exactUiComponentNames = [
  ...uiComponentCatalog.actions,
  ...uiComponentCatalog.dataEntry,
  ...uiComponentCatalog.dataDisplay,
  ...uiComponentCatalog.feedback,
  ...uiComponentCatalog.form,
  ...uiComponentCatalog.layout,
  ...uiComponentCatalog.navigation,
  ...uiComponentCatalog.overlays,
].filter((exportName) => /^[A-Z]/.test(exportName));

const exactPatternComponentNames = uiComponentCatalog.patterns;

const abstractRuntimeMetadataComponents = {
  ContextMenu: 'Radix root primitive is context-only and does not render a stable DOM surface.',
  ContextMenuPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  ContextMenuSub: 'Submenu composition primitive is context-only and does not render a stable DOM surface.',
  Dialog: 'Dialog root is state/context only and does not render a stable DOM surface.',
  DialogPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  Drawer: 'Drawer root is state/context only and does not render a stable DOM surface.',
  DrawerPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  DropdownMenu: 'Dropdown menu root is state/context only and does not render a stable DOM surface.',
  DropdownMenuPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  DropdownMenuSub: 'Submenu composition primitive is context-only and does not render a stable DOM surface.',
  Form: 'React Hook Form provider is context-only and does not render a stable DOM surface.',
  FormField: 'React Hook Form controller wrapper is context-only and does not render a stable DOM surface.',
  HoverCard: 'Hover card root is state/context only and does not render a stable DOM surface.',
  MenubarMenu: 'Menubar menu composition primitive is context-only and does not render a stable DOM surface.',
  MenubarPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  MenubarSub: 'Submenu composition primitive is context-only and does not render a stable DOM surface.',
  Modal: 'Modal root is state/context only and does not render a stable DOM surface.',
  ModalPortal: 'Portal primitives relocate children but do not own a stable DOM surface.',
  Popover: 'Popover root is state/context only and does not render a stable DOM surface.',
  Select: 'Select root is state/context only and does not render a stable DOM surface.',
  Tooltip: 'Tooltip root is state/context only and does not render a stable DOM surface.',
  TooltipProvider: 'Tooltip provider is context-only and does not render a stable DOM surface.',
} as const satisfies Record<string, string>;

function collectFiles(root: string): string[] {
  const entries = readdirSync(root, { withFileTypes: true });

  return entries.flatMap((entry) => {
    const filePath = join(root, entry.name);

    if (entry.isDirectory()) {
      return collectFiles(filePath);
    }

    return filePath.endsWith('.ts') || filePath.endsWith('.tsx') ? [filePath] : [];
  });
}

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function toKebabCase(value: string) {
  return value
    .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1-$2')
    .toLowerCase();
}

function hasExactRuntimeMarker(source: string, kind: RuntimeMetadataKind, marker: string) {
  const attribute = kind === 'pattern' ? 'data-sdk-pattern' : 'data-sdk-ui';
  const pattern = new RegExp(
    `${escapeRegExp(attribute)}(?:=|['"]\\s*:)[\\s\\S]{0,200}?['"]${escapeRegExp(marker)}['"]`,
  );

  return pattern.test(source);
}

describe('runtime metadata identity contract', () => {
  it('gives every public framework surface an exact runtime metadata identity or an explicit abstract exemption', () => {
    const publicComponentNames = [...exactUiComponentNames, ...exactPatternComponentNames];
    const publicComponentNameSet = new Set<string>(publicComponentNames);
    const sourceFiles = collectFiles('src/components');
    const sources = sourceFiles.map((filePath) => readFileSync(filePath, 'utf8'));
    const offenders: string[] = [];

    for (const componentName of exactUiComponentNames) {
      if (componentName in abstractRuntimeMetadataComponents) {
        continue;
      }

      const expectedMarker = toKebabCase(componentName);
      if (!sources.some((source) => hasExactRuntimeMarker(source, 'ui', expectedMarker))) {
        offenders.push(`${componentName} -> data-sdk-ui="${expectedMarker}"`);
      }
    }

    for (const componentName of exactPatternComponentNames) {
      if (componentName in abstractRuntimeMetadataComponents) {
        continue;
      }

      const expectedMarker = toKebabCase(componentName);
      if (!sources.some((source) => hasExactRuntimeMarker(source, 'pattern', expectedMarker))) {
        offenders.push(`${componentName} -> data-sdk-pattern="${expectedMarker}"`);
      }
    }

    const undocumentedAbstractComponents = Object.entries(abstractRuntimeMetadataComponents)
      .filter(([componentName, reason]) => !publicComponentNameSet.has(componentName) || !reason.trim())
      .map(([componentName]) => componentName);

    expect(undocumentedAbstractComponents).toEqual([]);
    expect(offenders).toEqual([]);
  });
});
