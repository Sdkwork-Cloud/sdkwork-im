import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Button } from '../button';
import { FormActions, FormGrid, FormSection } from './index';

describe('Form layout helpers', () => {
  it('renders desktop-oriented form sections, grids, and action rows', () => {
    render(
      <FormSection
        title="Workspace Defaults"
        description="Configure the default desktop behavior for newly created workspaces."
        actions={<Button variant="ghost">Reset</Button>}
      >
        <FormGrid columns={2}>
          <div>Field A</div>
          <div>Field B</div>
        </FormGrid>
        <FormActions>
          <Button variant="secondary">Cancel</Button>
          <Button>Save</Button>
        </FormActions>
      </FormSection>,
    );

    expect(screen.getByRole('region', { name: 'Workspace Defaults' })).toBeInTheDocument();
    expect(screen.getByText('Configure the default desktop behavior for newly created workspaces.')).toBeInTheDocument();
    expect(screen.getByText('Field A')).toBeInTheDocument();
    expect(screen.getByText('Field B')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save' })).toBeInTheDocument();
  });
});
