import { cleanup, fireEvent, render, screen, within } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Button } from '../button';
import { DataTable } from './data-table';

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

const pagedRows: AssetRow[] = Array.from({ length: 25 }, (_, index) => ({
  id: `asset-${index + 1}`,
  name: `Asset ${index + 1}`,
  owner: `Owner ${index + 1}`,
  status: index % 2 === 0 ? 'Ready' : 'Review',
}));

const sortableRows = [
  { id: 'asset-z', name: 'Zulu', owner: 'Owner Z', status: 'Ready', rank: 30 },
  { id: 'asset-a', name: 'Alpha', owner: 'Owner A', status: 'Review', rank: 10 },
  { id: 'asset-e', name: 'Echo', owner: 'Owner E', status: 'Ready', rank: 20 },
];

afterEach(() => {
  cleanup();
});

describe('DataTable', () => {
  it('renders a desktop data table with toolbar, row actions, and bulk selection state', () => {
    const onSelectedRowIdsChange = vi.fn();

    render(
      <DataTable
        columns={[
          { id: 'name', header: 'Name', cell: (row) => row.name },
          { id: 'owner', header: 'Owner', cell: (row) => row.owner },
          { id: 'status', header: 'Status', cell: (row) => row.status },
        ]}
        footer={<div>Page 1 of 1</div>}
        getRowId={(row) => row.id}
        getRowSelectionLabel={(row) => row.name}
        rowActions={(row) => (
          <Button size="sm" variant="ghost">
            Open {row.name}
          </Button>
        )}
        rows={rows}
        selectable
        selectedRowIds={['asset-1']}
        selectionBar={{
          actions: <Button size="sm">Archive selection</Button>,
          description: 'Apply actions to the currently selected assets.',
          title: 'Selected assets',
        }}
        title="Assets"
        toolbar={<Button variant="secondary">New asset</Button>}
        onSelectedRowIdsChange={onSelectedRowIdsChange}
      />,
    );

    expect(screen.getByText('Assets')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'New asset' })).toBeInTheDocument();
    expect(screen.getByText('Launch Brief')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Open Launch Brief' })).toBeInTheDocument();
    expect(screen.getByText('Selected assets')).toBeInTheDocument();
    expect(screen.getByText('Archive selection')).toBeInTheDocument();
    expect(screen.getByText('Page 1 of 1')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('checkbox', { name: 'Select row Moodboard' }));

    expect(onSelectedRowIdsChange).toHaveBeenCalledWith(['asset-1', 'asset-2']);
  });

  it('supports hidden columns and select-all interactions', () => {
    const onSelectedRowIdsChange = vi.fn();

    render(
      <DataTable
        columnVisibility={{ status: false }}
        columns={[
          { id: 'name', header: 'Name', cell: (row) => row.name },
          { id: 'owner', header: 'Owner', cell: (row) => row.owner },
          { id: 'status', header: 'Status', cell: (row) => row.status },
        ]}
        getRowId={(row) => row.id}
        rows={rows}
        selectable
        selectedRowIds={[]}
        title="Assets"
        onSelectedRowIdsChange={onSelectedRowIdsChange}
      />,
    );

    expect(screen.getByText('Name')).toBeInTheDocument();
    expect(screen.getByText('Owner')).toBeInTheDocument();
    expect(screen.queryByText('Status')).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole('checkbox', { name: 'Select all rows' }));

    expect(onSelectedRowIdsChange).toHaveBeenCalledWith(['asset-1', 'asset-2']);
  });

  it('renders loading and empty states', () => {
    const { rerender } = render(
      <DataTable
        columns={[{ id: 'name', header: 'Name', cell: (row: AssetRow) => row.name }]}
        loading
        loadingLabel="Loading assets"
        rows={[]}
        title="Assets"
      />,
    );

    expect(screen.getByText('Loading assets')).toBeInTheDocument();

    rerender(
      <DataTable
        columns={[{ id: 'name', header: 'Name', cell: (row: AssetRow) => row.name }]}
        emptyDescription="Create a new asset or adjust your filters."
        emptyTitle="No assets"
        rows={[]}
        title="Assets"
      />,
    );

    expect(screen.getByText('No assets')).toBeInTheDocument();
    expect(screen.getByText('Create a new asset or adjust your filters.')).toBeInTheDocument();
  });

  it('supports built-in client pagination with page summary and previous/next controls', () => {
    const onPageChange = vi.fn();
    const DataTableAny = DataTable as unknown as (props: Record<string, unknown>) => React.JSX.Element;

    render(
      <DataTableAny
        columns={[
          { id: 'name', header: 'Name', cell: (row: AssetRow) => row.name },
          { id: 'owner', header: 'Owner', cell: (row: AssetRow) => row.owner },
        ]}
        pagination={{
          defaultPage: 2,
          defaultPageSize: 10,
          onPageChange,
        }}
        rows={pagedRows}
        title="Assets"
      />,
    );

    expect(screen.getByText('Showing 11-20 of 25')).toBeInTheDocument();
    expect(screen.getByText('Asset 11')).toBeInTheDocument();
    expect(screen.queryByText('Asset 1')).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Next page' }));

    expect(onPageChange).toHaveBeenCalledWith(3);
    expect(screen.getByText('Showing 21-25 of 25')).toBeInTheDocument();
    expect(screen.getByText('Asset 25')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Previous page' }));

    expect(onPageChange).toHaveBeenCalledWith(2);
    expect(screen.getByText('Showing 11-20 of 25')).toBeInTheDocument();
  });

  it('supports built-in page size selection and resets to the first page when page size changes', () => {
    const onPageChange = vi.fn();
    const onPageSizeChange = vi.fn();
    const DataTableAny = DataTable as unknown as (props: Record<string, unknown>) => React.JSX.Element;

    render(
      <DataTableAny
        columns={[
          { id: 'name', header: 'Name', cell: (row: AssetRow) => row.name },
          { id: 'owner', header: 'Owner', cell: (row: AssetRow) => row.owner },
        ]}
        pagination={{
          defaultPage: 2,
          defaultPageSize: 10,
          onPageChange,
          onPageSizeChange,
          pageSizeOptions: [10, 20, 50],
        }}
        rows={pagedRows}
        title="Assets"
      />,
    );

    expect(screen.getByRole('combobox', { name: 'Rows per page' })).toBeInTheDocument();
    expect(screen.getByText('Showing 11-20 of 25')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('combobox', { name: 'Rows per page' }));
    fireEvent.click(screen.getByRole('option', { name: '20' }));

    expect(onPageSizeChange).toHaveBeenCalledWith(20);
    expect(onPageChange).toHaveBeenCalledWith(1);
    expect(screen.getByText('Showing 1-20 of 25')).toBeInTheDocument();
    expect(screen.getByText('Asset 1')).toBeInTheDocument();
    expect(screen.queryByText('Asset 21')).not.toBeInTheDocument();
  });

  it('supports controlled server pagination with explicit total rows', () => {
    const onPageChange = vi.fn();
    const DataTableAny = DataTable as unknown as (props: Record<string, unknown>) => React.JSX.Element;
    const serverRows = Array.from({ length: 10 }, (_, index) => ({
      id: `asset-${index + 21}`,
      name: `Asset ${index + 21}`,
      owner: `Owner ${index + 21}`,
      status: index % 2 === 0 ? 'Ready' : 'Review',
    }));

    render(
      <DataTableAny
        columns={[
          { id: 'name', header: 'Name', cell: (row: AssetRow) => row.name },
          { id: 'owner', header: 'Owner', cell: (row: AssetRow) => row.owner },
        ]}
        pagination={{
          mode: 'server',
          onPageChange,
          page: 3,
          pageSize: 10,
          rowCount: 52,
        }}
        rows={serverRows}
        title="Assets"
      />,
    );

    expect(screen.getByText('Showing 21-30 of 52')).toBeInTheDocument();
    expect(screen.getByText('Asset 21')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Next page' }));
    fireEvent.click(screen.getByRole('button', { name: 'Page 6' }));

    expect(onPageChange).toHaveBeenNthCalledWith(1, 4);
    expect(onPageChange).toHaveBeenNthCalledWith(2, 6);
  });

  it('supports accessible client-side sorting and resets pagination when sort order changes', () => {
    const onSortingChange = vi.fn();
    const DataTableAny = DataTable as unknown as (props: Record<string, unknown>) => React.JSX.Element;

    render(
      <DataTableAny
        columns={[
          {
            accessorKey: 'name',
            cell: (row: (typeof sortableRows)[number]) => row.name,
            header: 'Name',
            id: 'name',
            sortable: true,
          },
          {
            accessorKey: 'rank',
            align: 'right',
            cell: (row: (typeof sortableRows)[number]) => row.rank,
            header: 'Rank',
            id: 'rank',
          },
        ]}
        defaultSorting={[{ desc: true, id: 'name' }]}
        onSortingChange={onSortingChange}
        pagination={{
          defaultPage: 2,
          defaultPageSize: 1,
        }}
        rows={sortableRows}
        title="Sortable assets"
      />,
    );

    const table = screen.getByRole('table');
    const getFirstRow = () => within(table).getAllByRole('row')[1];
    const nameHeader = screen.getByRole('columnheader', { name: 'Name' });
    const sortButton = screen.getByRole('button', { name: 'Sort by Name' });

    expect(nameHeader).toHaveAttribute('aria-sort', 'descending');
    expect(screen.getByText('Showing 2-2 of 3')).toBeInTheDocument();
    expect(within(getFirstRow()).getByText('Echo')).toBeInTheDocument();

    fireEvent.click(sortButton);

    expect(onSortingChange).toHaveBeenLastCalledWith([{ desc: false, id: 'name' }]);
    expect(nameHeader).toHaveAttribute('aria-sort', 'ascending');
    expect(screen.getByText('Showing 1-1 of 3')).toBeInTheDocument();
    expect(within(getFirstRow()).getByText('Alpha')).toBeInTheDocument();

    fireEvent.click(sortButton);

    expect(onSortingChange).toHaveBeenLastCalledWith([{ desc: true, id: 'name' }]);
    expect(nameHeader).toHaveAttribute('aria-sort', 'descending');
    expect(screen.getByText('Showing 1-1 of 3')).toBeInTheDocument();
    expect(within(getFirstRow()).getByText('Zulu')).toBeInTheDocument();
  });
});
