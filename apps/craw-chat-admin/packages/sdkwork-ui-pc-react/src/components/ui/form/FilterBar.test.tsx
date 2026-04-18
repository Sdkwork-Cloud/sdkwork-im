import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Button } from '../button';
import { Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../data-entry';
import { FilterBar, FilterBarActions, FilterBarSection } from './index';

describe('FilterBar', () => {
  it('renders a desktop filter surface with grouped controls and trailing actions', () => {
    const { container } = render(
      <FilterBar
        description="Search, filter, and act on the current desktop list."
        summary="Showing 24 matching records"
        title="Workspace Filters"
      >
        <FilterBarSection>
          <Input aria-label="Search keyword" placeholder="Search keyword" />
          <Select defaultValue="all">
            <SelectTrigger aria-label="Status">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All</SelectItem>
              <SelectItem value="active">Active</SelectItem>
            </SelectContent>
          </Select>
        </FilterBarSection>
        <FilterBarActions>
          <Button variant="secondary">Reset</Button>
          <Button>Apply filters</Button>
        </FilterBarActions>
      </FilterBar>,
    );

    expect(screen.getByRole('region', { name: 'Workspace Filters' })).toBeInTheDocument();
    expect(screen.getByText('Search, filter, and act on the current desktop list.')).toBeInTheDocument();
    expect(screen.getByText('Showing 24 matching records')).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: 'Search keyword' })).toBeInTheDocument();
    expect(screen.getByRole('combobox', { name: 'Status' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Apply filters' })).toBeInTheDocument();
    expect(container.querySelector('[data-sdk-ui="filter-bar"]')).toBeInTheDocument();
    expect(container.querySelector('[data-sdk-ui="filter-bar-actions"]')).toBeInTheDocument();
  });
});
