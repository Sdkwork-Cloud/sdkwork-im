import { cleanup, render } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  AppShell,
  DesktopAppHeader,
  DesktopShellFrame,
  DesktopTitleBar,
  DesktopWindowControls,
  InspectorPanel,
  InspectorRail,
  ListDetailWorkspace,
  ManagementWorkbench,
  NavigationRail,
  PageHeader,
  PickerSelectionFooter,
  RestartRequiredNotice,
  SettingsCenter,
  WorkspacePanel,
  WorkspaceScaffold,
} from './components/patterns';
import type { DesktopWindowController } from './components/patterns/desktop-shell/DesktopWindowControls';

afterEach(() => {
  cleanup();
});

const surfaceProbeClassName = 'sdk-pattern-surface-probe';
const surfaceProbeDataAttribute = 'data-contract-root';
const surfaceProbeStyle = { outline: '1px solid transparent' } as const;

const desktopWindowController: DesktopWindowController = {
  closeWindow: async () => undefined,
  getPlatform: () => 'desktop',
  isWindowMaximized: async () => false,
  maximizeWindow: async () => undefined,
  minimizeWindow: async () => undefined,
  restoreWindow: async () => undefined,
  subscribeWindowMaximized: () => () => undefined,
};

type ContractCase = {
  pattern: string;
  render: () => ReturnType<typeof render>;
};

const contractCases: ContractCase[] = [
  {
    pattern: 'app-shell',
    render: () =>
      render(
        <AppShell
          className={surfaceProbeClassName}
          content={<div>App shell content</div>}
          data-contract-root="true"
          id="contract-app-shell"
          style={surfaceProbeStyle}
        />,
      ),
  },
  {
    pattern: 'page-header',
    render: () =>
      render(
        <PageHeader
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-page-header"
          style={surfaceProbeStyle}
          title="Page header"
        />,
      ),
  },
  {
    pattern: 'navigation-rail',
    render: () =>
      render(
        <NavigationRail
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-navigation-rail"
          style={surfaceProbeStyle}
        >
          <span>Navigation</span>
        </NavigationRail>,
      ),
  },
  {
    pattern: 'inspector-panel',
    render: () =>
      render(
        <InspectorPanel
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-inspector-panel"
          style={surfaceProbeStyle}
        >
          <div>Inspector content</div>
        </InspectorPanel>,
      ),
  },
  {
    pattern: 'inspector-rail',
    render: () =>
      render(
        <InspectorRail
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-inspector-rail"
          style={surfaceProbeStyle}
        >
          <div>Inspector rail content</div>
        </InspectorRail>,
      ),
  },
  {
    pattern: 'workspace-panel',
    render: () =>
      render(
        <WorkspacePanel
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-workspace-panel"
          style={surfaceProbeStyle}
        >
          <div>Workspace content</div>
        </WorkspacePanel>,
      ),
  },
  {
    pattern: 'settings-center',
    render: () =>
      render(
        <SettingsCenter
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-settings-center"
          sections={[
            {
              items: [{ id: 'appearance', label: 'Appearance' }],
            },
          ]}
          style={surfaceProbeStyle}
          title="Settings"
        >
          <div>Settings content</div>
        </SettingsCenter>,
      ),
  },
  {
    pattern: 'management-workbench',
    render: () =>
      render(
        <ManagementWorkbench
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-management-workbench"
          main={{ children: <div>Main content</div> }}
          style={surfaceProbeStyle}
          title="Management"
        />,
      ),
  },
  {
    pattern: 'list-detail-workspace',
    render: () =>
      render(
        <ListDetailWorkspace
          className={surfaceProbeClassName}
          content={{ children: <div>List detail content</div> }}
          data-contract-root="true"
          id="contract-list-detail-workspace"
          style={surfaceProbeStyle}
        />,
      ),
  },
  {
    pattern: 'workspace-scaffold',
    render: () =>
      render(
        <WorkspaceScaffold
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-workspace-scaffold"
          main={{ children: <div>Scaffold main</div> }}
          style={surfaceProbeStyle}
        />,
      ),
  },
  {
    pattern: 'picker-selection-footer',
    render: () =>
      render(
        <PickerSelectionFooter
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-picker-selection-footer"
          style={surfaceProbeStyle}
          summary="2 selected"
        />,
      ),
  },
  {
    pattern: 'restart-required-notice',
    render: () =>
      render(
        <RestartRequiredNotice
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-restart-required-notice"
          style={surfaceProbeStyle}
        />,
      ),
  },
  {
    pattern: 'desktop-app-header',
    render: () =>
      render(
        <DesktopAppHeader
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-desktop-app-header"
          style={surfaceProbeStyle}
          title="Desktop app header"
        />,
      ),
  },
  {
    pattern: 'desktop-shell-frame',
    render: () =>
      render(
        <DesktopShellFrame
          className={surfaceProbeClassName}
          content={<div>Desktop shell frame content</div>}
          data-contract-root="true"
          id="contract-desktop-shell-frame"
          style={surfaceProbeStyle}
          title="Desktop shell frame"
        />,
      ),
  },
  {
    pattern: 'desktop-title-bar',
    render: () =>
      render(
        <DesktopTitleBar
          className={surfaceProbeClassName}
          data-contract-root="true"
          id="contract-desktop-title-bar"
          style={surfaceProbeStyle}
        />,
      ),
  },
  {
    pattern: 'desktop-window-controls',
    render: () =>
      render(
        <DesktopWindowControls
          className={surfaceProbeClassName}
          controller={desktopWindowController}
          data-contract-root="true"
          id="contract-desktop-window-controls"
          style={surfaceProbeStyle}
        />,
      ),
  },
];

describe('pattern direct surface contract', () => {
  it('keeps direct-surface patterns open to standard DOM root props', () => {
    for (const contractCase of contractCases) {
      const view = contractCase.render();
      const surface = view.container.querySelector(`[data-sdk-pattern="${contractCase.pattern}"]`);

      expect(surface, `Missing pattern surface for ${contractCase.pattern}`).not.toBeNull();
      expect(surface).toHaveAttribute(
        'id',
        `contract-${contractCase.pattern}`,
      );
      expect(surface).toHaveAttribute(surfaceProbeDataAttribute, 'true');
      expect(surface).toHaveClass(surfaceProbeClassName);
      expect(surface).toHaveStyle(surfaceProbeStyle);

      view.unmount();
    }
  });
});
