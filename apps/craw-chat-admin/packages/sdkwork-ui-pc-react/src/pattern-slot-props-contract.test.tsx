import { cleanup, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import {
  AnchoredPickerSurface,
  DesktopAppHeader,
  DesktopTitleBar,
  DetailDrawer,
  EntityPickerDialog,
  InspectorRail,
  OperationDrawer,
  PickerDialog,
  TwoPaneSelectorPopover,
  WorkspacePanel,
} from './components/patterns';

const slotProbeClassName = 'sdk-slot-probe';
const slotProbeStyle = { outline: '1px solid transparent' } as const;

function queryRegion(region: string) {
  return document.body.querySelector(`[data-sdk-region="${region}"]`);
}

function querySlot(slot: string) {
  return document.body.querySelector(`[data-sdk-slot="${slot}"]`);
}

describe('pattern slot props contract', () => {
  it('applies structured slotProps to internal desktop and workspace pattern slots', () => {
    render(
      <>
        <DesktopTitleBar
          center="Search"
          leading="Brand"
          slotProps={{
            center: {
              className: slotProbeClassName,
              'data-slot-probe': 'desktop-title-bar-center',
              id: 'desktop-title-bar-center',
              style: slotProbeStyle,
            },
            centerShell: {
              className: 'desktop-title-bar-center-shell',
            },
            leading: {
              className: 'desktop-title-bar-leading',
            },
            trailing: {
              className: 'desktop-title-bar-trailing',
            },
          }}
          trailing="Actions"
        />
        <DesktopAppHeader
          slotProps={{
            bar: {
              className: 'desktop-app-header-bar',
            },
            brand: {
              className: slotProbeClassName,
              'data-slot-probe': 'desktop-app-header-brand',
              id: 'desktop-app-header-brand',
              style: slotProbeStyle,
            },
            subtitle: {
              className: 'desktop-app-header-subtitle',
            },
            title: {
              className: 'desktop-app-header-title',
            },
          }}
          subtitle="subtitle"
          title="title"
        />
        <WorkspacePanel
          slotProps={{
            body: {
              className: slotProbeClassName,
              'data-slot-probe': 'workspace-panel-body',
              id: 'workspace-panel-body',
              style: slotProbeStyle,
            },
            header: {
              className: 'workspace-panel-header',
            },
          }}
          title="Workspace"
        >
          Body
        </WorkspacePanel>
        <InspectorRail
          footer={<button type="button">Apply</button>}
          slotProps={{
            body: {
              className: slotProbeClassName,
              'data-slot-probe': 'inspector-rail-body',
              id: 'inspector-rail-body',
              style: slotProbeStyle,
            },
            footer: {
              className: 'inspector-rail-footer',
            },
            header: {
              className: 'inspector-rail-header',
            },
          }}
          title="Inspector"
        >
          Rail body
        </InspectorRail>
      </>,
    );

    expect(querySlot('center')).toHaveClass(slotProbeClassName);
    expect(querySlot('center')).toHaveAttribute(
      'data-slot-probe',
      'desktop-title-bar-center',
    );
    expect(querySlot('center')).toHaveAttribute(
      'id',
      'desktop-title-bar-center',
    );
    expect(querySlot('center')).toHaveStyle(slotProbeStyle);

    expect(queryRegion('desktop-app-header-brand')).toHaveClass(slotProbeClassName);
    expect(queryRegion('desktop-app-header-brand')).toHaveAttribute(
      'data-slot-probe',
      'desktop-app-header-brand',
    );
    expect(queryRegion('desktop-app-header-brand')).toHaveAttribute(
      'id',
      'desktop-app-header-brand',
    );
    expect(queryRegion('desktop-app-header-brand')).toHaveStyle(slotProbeStyle);

    expect(queryRegion('workspace-panel-body')).toHaveClass(slotProbeClassName);
    expect(queryRegion('workspace-panel-body')).toHaveAttribute(
      'data-slot-probe',
      'workspace-panel-body',
    );
    expect(queryRegion('workspace-panel-body')).toHaveAttribute(
      'id',
      'workspace-panel-body',
    );
    expect(queryRegion('workspace-panel-body')).toHaveStyle(slotProbeStyle);

    expect(queryRegion('inspector-rail-body')).toHaveClass(slotProbeClassName);
    expect(queryRegion('inspector-rail-body')).toHaveAttribute(
      'data-slot-probe',
      'inspector-rail-body',
    );
    expect(queryRegion('inspector-rail-body')).toHaveAttribute(
      'id',
      'inspector-rail-body',
    );
    expect(queryRegion('inspector-rail-body')).toHaveStyle(slotProbeStyle);
  });

  it('applies structured slotProps to delegated overlay pattern slots', () => {
    render(
      <AnchoredPickerSurface
        defaultOpen
        filters={<button type="button">Filter</button>}
        slotProps={{
          body: {
            className: slotProbeClassName,
            'data-slot-probe': 'anchored-picker-body',
            id: 'anchored-picker-body',
            style: slotProbeStyle,
          },
          content: {
            className: 'anchored-picker-content',
          },
          footer: {
            className: 'anchored-picker-footer',
          },
          header: {
            className: 'anchored-picker-header',
          },
        }}
        title="Anchored picker"
        trigger={<button type="button">Open anchored picker</button>}
      >
        Picker body
      </AnchoredPickerSurface>,
    );
    expect(queryRegion('anchored-picker-body')).toHaveClass(slotProbeClassName);
    cleanup();

    render(
      <PickerDialog
        filters={<button type="button">Filter</button>}
        open
        slotProps={{
          body: {
            className: slotProbeClassName,
            'data-slot-probe': 'picker-dialog-body',
            id: 'picker-dialog-body',
            style: slotProbeStyle,
          },
          content: {
            className: 'picker-dialog-content',
          },
          footer: {
            className: 'picker-dialog-footer',
          },
          header: {
            className: 'picker-dialog-header',
          },
          sidebar: {
            className: 'picker-dialog-sidebar',
          },
        }}
        title="Picker dialog"
      >
        Dialog body
      </PickerDialog>,
    );
    expect(queryRegion('picker-dialog-body')).toHaveClass(slotProbeClassName);
    cleanup();

    render(
      <DetailDrawer
        open
        slotProps={{
          body: {
            className: slotProbeClassName,
            'data-slot-probe': 'detail-drawer-body',
            id: 'detail-drawer-body',
            style: slotProbeStyle,
          },
          content: {
            className: 'detail-drawer-content',
          },
          header: {
            className: 'detail-drawer-header',
          },
        }}
        title="Detail drawer"
      >
        Detail body
      </DetailDrawer>,
    );
    expect(queryRegion('detail-drawer-body')).toHaveClass(slotProbeClassName);
    cleanup();

    render(
      <OperationDrawer
        footer={<button type="button">Submit</button>}
        open
        sidebar={<div>Sidebar</div>}
        slotProps={{
          body: {
            className: slotProbeClassName,
            'data-slot-probe': 'operation-drawer-body',
            id: 'operation-drawer-body',
            style: slotProbeStyle,
          },
          content: {
            className: 'operation-drawer-content',
          },
          footer: {
            className: 'operation-drawer-footer',
          },
          header: {
            className: 'operation-drawer-header',
          },
          sidebar: {
            className: 'operation-drawer-sidebar',
          },
        }}
        title="Operation drawer"
      >
        Operation body
      </OperationDrawer>,
    );
    expect(queryRegion('operation-drawer-body')).toHaveClass(slotProbeClassName);
  });

  it('applies structured slotProps to picker composition patterns', () => {
    render(
      <EntityPickerDialog
        items={[{ id: 'a', label: 'Alpha' }]}
        open
        renderItem={({ item }) => <button type="button">{item.label}</button>}
        slotProps={{
          body: {
            className: 'entity-picker-body',
          },
          items: {
            className: slotProbeClassName,
            'data-slot-probe': 'entity-picker-items',
            id: 'entity-picker-items',
            style: slotProbeStyle,
          },
        }}
        title="Entity picker"
      />,
    );
    expect(screen.getByText('Alpha')).toBeInTheDocument();
    expect(queryRegion('entity-picker-items')).toHaveClass(slotProbeClassName);
    expect(queryRegion('entity-picker-items')).toHaveAttribute(
      'data-slot-probe',
      'entity-picker-items',
    );
    expect(queryRegion('entity-picker-items')).toHaveAttribute(
      'id',
      'entity-picker-items',
    );
    expect(queryRegion('entity-picker-items')).toHaveStyle(slotProbeStyle);
    cleanup();

    render(
      <TwoPaneSelectorPopover
        defaultOpen
        sections={[
          {
            items: [{ id: 'one', label: 'One' }],
            section: { id: 'section-a', label: 'Section A' },
          },
        ]}
        slotProps={{
          body: {
            className: 'two-pane-body',
          },
          content: {
            className: 'two-pane-content',
          },
          items: {
            className: slotProbeClassName,
            'data-slot-probe': 'two-pane-items',
            id: 'two-pane-items',
            style: slotProbeStyle,
          },
          pickerSurface: {
            className: 'two-pane-picker-surface',
            'data-slot-probe': 'two-pane-picker-surface',
          },
          sections: {
            className: 'two-pane-sections',
          },
          surface: {
            className: 'two-pane-layout-surface',
            'data-slot-probe': 'two-pane-layout-surface',
          },
        }}
        trigger={<button type="button">Open two-pane</button>}
      />,
    );
    expect(queryRegion('anchored-picker-surface')).toHaveClass('two-pane-picker-surface');
    expect(queryRegion('anchored-picker-surface')).toHaveAttribute(
      'data-slot-probe',
      'two-pane-picker-surface',
    );
    expect(queryRegion('two-pane-selector-surface')).toHaveClass('two-pane-layout-surface');
    expect(queryRegion('two-pane-selector-surface')).toHaveAttribute(
      'data-slot-probe',
      'two-pane-layout-surface',
    );
    expect(queryRegion('two-pane-selector-items')).toHaveClass(slotProbeClassName);
    expect(queryRegion('two-pane-selector-items')).toHaveAttribute(
      'data-slot-probe',
      'two-pane-items',
    );
    expect(queryRegion('two-pane-selector-items')).toHaveAttribute(
      'id',
      'two-pane-items',
    );
    expect(queryRegion('two-pane-selector-items')).toHaveStyle(slotProbeStyle);
  });
});
