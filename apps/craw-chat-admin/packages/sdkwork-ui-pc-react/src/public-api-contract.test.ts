import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';
import * as packageExports from './index';
import * as patternExports from './components/patterns';
import * as uiExports from './components/ui';
import * as actionExports from './components/ui/actions';
import * as dataDisplayExports from './components/ui/data-display';
import * as dataEntryExports from './components/ui/data-entry';
import * as feedbackExports from './components/ui/feedback';
import * as formExports from './components/ui/form';
import * as layoutExports from './components/ui/layout';
import * as navigationExports from './components/ui/navigation';
import * as overlaysExports from './components/ui/overlays';
import * as themeExports from './theme';

const uiNamespaceExports = {
  actions: actionExports,
  dataDisplay: dataDisplayExports,
  dataEntry: dataEntryExports,
  feedback: feedbackExports,
  form: formExports,
  layout: layoutExports,
  navigation: navigationExports,
  overlays: overlaysExports,
} as const;

describe('public API contract', () => {
  it('keeps the catalog aligned with public package entrypoints', () => {
    const offenders: string[] = [];

    for (const [category, exportsObject] of Object.entries(uiNamespaceExports) as Array<
      [keyof typeof uiNamespaceExports, Record<string, unknown>]
    >) {
      for (const exportName of uiComponentCatalog[category]) {
        if (!(exportName in exportsObject)) {
          offenders.push(`src/components/ui/${category} missing ${exportName}`);
        }

        if (!(exportName in uiExports)) {
          offenders.push(`src/components/ui missing ${exportName}`);
        }

        if (!(exportName in packageExports)) {
          offenders.push(`src/index missing ${exportName}`);
        }
      }
    }

    for (const exportName of uiComponentCatalog.patterns) {
      if (!(exportName in patternExports)) {
        offenders.push(`src/components/patterns missing ${exportName}`);
      }

      if (!(exportName in packageExports)) {
        offenders.push(`src/index missing ${exportName}`);
      }
    }

    for (const exportName of uiComponentCatalog.theme) {
      if (!(exportName in themeExports)) {
        offenders.push(`src/theme missing ${exportName}`);
      }

      if (!(exportName in packageExports)) {
        offenders.push(`src/index missing ${exportName}`);
      }
    }

    for (const exportName of uiComponentCatalog.utils) {
      if (!(exportName in packageExports)) {
        offenders.push(`src/index missing ${exportName}`);
      }
    }

    expect(offenders).toEqual([]);
  });

  it('assigns each catalog component to exactly one UI domain barrel', () => {
    const offenders: string[] = [];

    for (const [category] of Object.entries(uiNamespaceExports) as Array<
      [keyof typeof uiNamespaceExports, Record<string, unknown>]
    >) {
      for (const exportName of uiComponentCatalog[category]) {
        const owners = (Object.entries(uiNamespaceExports) as Array<
          [keyof typeof uiNamespaceExports, Record<string, unknown>]
        >)
          .filter(([, exportsObject]) => exportName in exportsObject)
          .map(([owner]) => owner);

        if (owners.length !== 1 || owners[0] !== category) {
          offenders.push(`${exportName} owners: ${owners.join(', ') || '<none>'}, expected: ${category}`);
        }
      }
    }

    expect(offenders).toEqual([]);
  });
});
