import { render, screen } from '@testing-library/react';
import { Plus, Search } from 'lucide-react';
import { describe, expect, it } from 'vitest';
import { Button } from '../button';
import { IconButton } from '../actions';
import { Toolbar, ToolbarGroup, ToolbarSeparator, ToolbarSpacer } from './index';

describe('Toolbar', () => {
  it('renders a desktop toolbar with grouped actions and spacer-driven layout', () => {
    render(
      <Toolbar aria-label="Workspace actions">
        <ToolbarGroup>
          <IconButton aria-label="Search workspace" variant="ghost">
            <Search />
          </IconButton>
          <ToolbarSeparator />
          <Button size="sm" variant="secondary">
            Filters
          </Button>
        </ToolbarGroup>
        <ToolbarSpacer />
        <ToolbarGroup>
          <Button size="sm">
            <Plus className="h-4 w-4" />
            Create
          </Button>
        </ToolbarGroup>
      </Toolbar>,
    );

    const toolbar = screen.getByRole('toolbar', { name: 'Workspace actions' });

    expect(toolbar).toBeInTheDocument();
    expect(toolbar).toHaveClass('flex', 'flex-wrap');
    expect(screen.getByRole('separator')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Search workspace' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Create' })).toBeInTheDocument();
  });
});
