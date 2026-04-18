import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  ActionMenuButton,
  CollectionGrid,
  CommandDialog,
  DataTable,
  DialogTitle,
  SplitButton,
} from './components/ui';

const slotProbeClassName = 'sdk-ui-slot-probe';
const slotProbeStyle = { outline: '1px solid transparent' } as const;

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

function queryRegion(region: string) {
  return document.body.querySelector(`[data-sdk-region="${region}"]`);
}

afterEach(() => {
  cleanup();
});

describe('UI slot props contract', () => {
  it('applies structured slotProps to delegated menu and dialog content surfaces', () => {
    render(
      <ActionMenuButton
        items={[{ key: 'open', label: 'Open item' }]}
        menuOpen
        slotProps={{
          content: {
            className: slotProbeClassName,
            'data-slot-probe': 'action-menu-content',
            id: 'action-menu-content',
            style: slotProbeStyle,
          },
        }}
      >
        Actions
      </ActionMenuButton>,
    );

    expect(queryUi('dropdown-menu-content')).toHaveClass(slotProbeClassName);
    expect(queryUi('dropdown-menu-content')).toHaveAttribute('data-slot-probe', 'action-menu-content');
    expect(queryUi('dropdown-menu-content')).toHaveAttribute('id', 'action-menu-content');
    expect(queryUi('dropdown-menu-content')).toHaveStyle(slotProbeStyle);
    cleanup();

    render(
      <SplitButton
        items={[{ key: 'duplicate', label: 'Duplicate item' }]}
        menuOpen
        onClick={() => undefined}
        slotProps={{
          content: {
            className: slotProbeClassName,
            'data-slot-probe': 'split-button-content',
            id: 'split-button-content',
            style: slotProbeStyle,
          },
        }}
      >
        Publish
      </SplitButton>,
    );

    expect(queryUi('dropdown-menu-content')).toHaveClass(slotProbeClassName);
    expect(queryUi('dropdown-menu-content')).toHaveAttribute('data-slot-probe', 'split-button-content');
    expect(queryUi('dropdown-menu-content')).toHaveAttribute('id', 'split-button-content');
    expect(queryUi('dropdown-menu-content')).toHaveStyle(slotProbeStyle);
    cleanup();

    render(
      <CommandDialog
        open
        slotProps={{
          command: {
            className: 'command-root-probe',
            'data-slot-probe': 'command-root',
          },
          content: {
            className: slotProbeClassName,
            'data-slot-probe': 'command-dialog-content',
            id: 'command-dialog-content',
            style: slotProbeStyle,
          },
        }}
      >
        <DialogTitle className="sr-only">Command dialog</DialogTitle>
        <div>Command body</div>
      </CommandDialog>,
    );

    expect(queryUi('command-dialog')).toHaveClass(slotProbeClassName);
    expect(queryUi('command-dialog')).toHaveAttribute('data-slot-probe', 'command-dialog-content');
    expect(queryUi('command-dialog')).toHaveAttribute('id', 'command-dialog-content');
    expect(queryUi('command-dialog')).toHaveStyle(slotProbeStyle);
    expect(queryUi('command')).toHaveClass('command-root-probe');
    expect(queryUi('command')).toHaveAttribute('data-slot-probe', 'command-root');
  });

  it('applies structured slotProps to CollectionGrid interior surfaces', () => {
    render(
      <CollectionGrid
        items={[{ id: 'a', name: 'Alpha' }]}
        pagination={<div>Page 1</div>}
        renderItem={(item) => <div>{item.name}</div>}
        slotProps={{
          grid: {
            className: slotProbeClassName,
            'data-slot-probe': 'collection-grid-grid',
            id: 'collection-grid-grid',
            style: slotProbeStyle,
          },
          pagination: {
            className: 'collection-grid-pagination-probe',
            'data-slot-probe': 'collection-grid-pagination',
          },
        }}
      />,
    );

    expect(queryRegion('collection-grid-grid')).toHaveClass(slotProbeClassName);
    expect(queryRegion('collection-grid-grid')).toHaveAttribute('data-slot-probe', 'collection-grid-grid');
    expect(queryRegion('collection-grid-grid')).toHaveAttribute('id', 'collection-grid-grid');
    expect(queryRegion('collection-grid-grid')).toHaveStyle(slotProbeStyle);
    expect(queryRegion('collection-grid-pagination')).toHaveClass('collection-grid-pagination-probe');
    expect(queryRegion('collection-grid-pagination')).toHaveAttribute('data-slot-probe', 'collection-grid-pagination');
  });

  it('applies structured slotProps and get*Props to DataTable owned surfaces', () => {
    render(
      <DataTable<{ id: string; name: string }>
        columns={[
          {
            cell: (row) => row.name,
            cellProps: {
              className: 'data-table-name-cell',
            },
            header: 'Name',
            headerProps: {
              className: slotProbeClassName,
              'data-slot-probe': 'data-table-name-header',
              id: 'data-table-name-header',
              style: slotProbeStyle,
            },
            id: 'name',
          },
        ]}
        footer={<div>Page 1 of 1</div>}
        getRowId={(row) => row.id}
        getRowProps={(row) => ({
          className: row.id === 'asset-1' ? slotProbeClassName : undefined,
          'data-slot-probe': row.id === 'asset-1' ? 'data-table-row' : undefined,
          id: row.id === 'asset-1' ? 'data-table-row' : undefined,
          style: row.id === 'asset-1' ? slotProbeStyle : undefined,
        })}
        rows={[{ id: 'asset-1', name: 'Launch Brief' }]}
        slotProps={{
          footer: {
            className: 'data-table-footer-probe',
            'data-slot-probe': 'data-table-footer',
          },
          header: {
            className: 'data-table-header-probe',
            'data-slot-probe': 'data-table-header',
          },
          table: {
            className: 'data-table-surface-probe',
            'data-slot-probe': 'data-table-surface',
          },
          toolbar: {
            className: 'data-table-toolbar-probe',
            'data-slot-probe': 'data-table-toolbar',
          },
        }}
        title="Assets"
        toolbar={<button type="button">New asset</button>}
      />,
    );

    expect(queryUi('table')).toHaveClass('data-table-surface-probe');
    expect(queryUi('table')).toHaveAttribute('data-slot-probe', 'data-table-surface');
    expect(queryRegion('data-table-header')).toHaveClass('data-table-header-probe');
    expect(queryRegion('data-table-header')).toHaveAttribute('data-slot-probe', 'data-table-header');
    expect(queryRegion('data-table-toolbar')).toHaveClass('data-table-toolbar-probe');
    expect(queryRegion('data-table-toolbar')).toHaveAttribute('data-slot-probe', 'data-table-toolbar');
    expect(screen.getByRole('columnheader', { name: 'Name' })).toHaveClass(slotProbeClassName);
    expect(screen.getByRole('columnheader', { name: 'Name' })).toHaveAttribute(
      'data-slot-probe',
      'data-table-name-header',
    );
    expect(screen.getByRole('columnheader', { name: 'Name' })).toHaveAttribute('id', 'data-table-name-header');
    expect(screen.getByRole('columnheader', { name: 'Name' })).toHaveStyle(slotProbeStyle);
    expect(screen.getByRole('row', { name: /launch brief/i })).toHaveClass(slotProbeClassName);
    expect(screen.getByRole('row', { name: /launch brief/i })).toHaveAttribute('data-slot-probe', 'data-table-row');
    expect(screen.getByRole('row', { name: /launch brief/i })).toHaveAttribute('id', 'data-table-row');
    expect(screen.getByRole('row', { name: /launch brief/i })).toHaveStyle(slotProbeStyle);
    expect(queryRegion('data-table-footer')).toHaveClass('data-table-footer-probe');
    expect(queryRegion('data-table-footer')).toHaveAttribute('data-slot-probe', 'data-table-footer');
  });
});
