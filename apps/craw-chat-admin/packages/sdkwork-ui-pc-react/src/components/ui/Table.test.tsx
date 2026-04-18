import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from './table';

describe('Table', () => {
  it('renders shared desktop table primitives with semantic header and caption output', () => {
    render(
      <Table>
        <TableCaption>Workspace agents</TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
            <TableHead>Status</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell>Planner</TableCell>
            <TableCell>Ready</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );

    expect(screen.getByRole('table')).toBeInTheDocument();
    expect(screen.getByRole('columnheader', { name: 'Name' })).toBeInTheDocument();
    expect(screen.getByRole('cell', { name: 'Planner' })).toBeInTheDocument();
    expect(screen.getByText('Workspace agents')).toBeInTheDocument();
  });

  it('keeps the primitive scroll container structurally thin instead of baking in panel chrome', () => {
    render(
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell>Planner</TableCell>
          </TableRow>
        </TableBody>
      </Table>,
    );

    const table = screen.getAllByRole('table').at(-1)!;
    const scrollContainer = table.parentElement;

    expect(scrollContainer).not.toBeNull();
    expect(scrollContainer).toHaveClass('relative', 'w-full', 'overflow-auto');
    expect(scrollContainer).not.toHaveClass('rounded-[var(--sdk-radius-panel)]');
    expect(scrollContainer).not.toHaveClass('border');
  });
});
