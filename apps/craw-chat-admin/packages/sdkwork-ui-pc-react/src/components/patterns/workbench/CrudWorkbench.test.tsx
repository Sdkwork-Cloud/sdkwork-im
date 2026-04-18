import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { Button } from '../../ui/button';
import { FilterBar, FilterBarActions, FilterBarSection } from '../../ui/form';
import { Input } from '../../ui/input';
import { CrudWorkbench } from './CrudWorkbench';

interface AssetRow {
  id: string;
  name: string;
  owner: string;
  status: string;
}

const rows: AssetRow[] = [
  { id: 'asset-1', name: 'Launch Brief', owner: 'SDKWORK Ops', status: 'Ready' },
  { id: 'asset-2', name: 'Moodboard', owner: 'Design Team', status: 'Review' },
];

afterEach(() => {
  cleanup();
});

describe('CrudWorkbench', () => {
  it('renders a reusable CRUD desktop scaffold with filters, data table, detail rail, selection state, and editor panel', () => {
    render(
      <CrudWorkbench
        actions={<Button variant="secondary">Create asset</Button>}
        description="Shared CRUD shell for desktop resource management."
        detail={{
          children: <div>Selected asset metadata</div>,
          footer: <Button size="sm">Open detail drawer</Button>,
          title: 'Inspector',
        }}
        editor={{
          actions: <Button size="sm">Save draft</Button>,
          children: <div>Asset editor form</div>,
          description: 'Create or edit the selected desktop resource.',
          title: 'Editor',
        }}
        filters={
          <FilterBar title="Filters">
            <FilterBarSection>
              <Input aria-label="Search assets" placeholder="Search assets" />
            </FilterBarSection>
            <FilterBarActions>
              <Button variant="outline">Reset</Button>
            </FilterBarActions>
          </FilterBar>
        }
        footer={<div>Showing 2 assets</div>}
        table={{
          columns: [
            { id: 'name', header: 'Name', cell: (row) => row.name },
            { id: 'owner', header: 'Owner', cell: (row) => row.owner },
            { id: 'status', header: 'Status', cell: (row) => row.status },
          ],
          getRowId: (row) => row.id,
          rowActions: (row) => (
            <Button size="sm" variant="ghost">
              Open {row.name}
            </Button>
          ),
          rows,
          selectable: true,
          selectedRowIds: ['asset-1'],
          selectionBar: {
            actions: <Button size="sm">Archive selection</Button>,
            description: 'Apply actions to the selected assets.',
            title: 'Selected assets',
          },
          title: 'Assets',
          toolbar: <Button variant="outline">Export</Button>,
        }}
        title="Asset management"
      />,
    );

    expect(screen.getByText('Asset management')).toBeInTheDocument();
    expect(screen.getByText('Shared CRUD shell for desktop resource management.')).toBeInTheDocument();
    expect(screen.getByText('Filters')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search assets')).toBeInTheDocument();
    expect(screen.getByText('Launch Brief')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Open Launch Brief' })).toBeInTheDocument();
    expect(screen.getByText('Selected assets')).toBeInTheDocument();
    expect(screen.getByText('Inspector')).toBeInTheDocument();
    expect(screen.getByText('Selected asset metadata')).toBeInTheDocument();
    expect(screen.getByText('Editor')).toBeInTheDocument();
    expect(screen.getByText('Asset editor form')).toBeInTheDocument();
    expect(screen.getByText('Showing 2 assets')).toBeInTheDocument();
  });

  it('supports a lean CRUD workbench without detail or editor regions', () => {
    render(
      <CrudWorkbench
        table={{
          columns: [{ id: 'name', header: 'Name', cell: (row: AssetRow) => row.name }],
          rows,
          title: 'Assets',
        }}
        title="Asset management"
      />,
    );

    expect(screen.getByText('Asset management')).toBeInTheDocument();
    expect(screen.getByText('Launch Brief')).toBeInTheDocument();
    expect(screen.queryByText('Inspector')).not.toBeInTheDocument();
    expect(screen.queryByText('Editor')).not.toBeInTheDocument();
  });
});
