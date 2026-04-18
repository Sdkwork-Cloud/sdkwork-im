import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { SectionHeader } from './SectionHeader';

describe('SectionHeader', () => {
  it('renders title, description, actions, and secondary content for desktop list pages', () => {
    const { container } = render(
      <SectionHeader
        actions={<button type="button">Import</button>}
        description="Shared search and action surface"
        sticky
        title="Asset Center"
      >
        <div>Filters</div>
      </SectionHeader>,
    );

    expect(screen.getByText('Asset Center')).toBeInTheDocument();
    expect(screen.getByText('Shared search and action surface')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Import' })).toBeInTheDocument();
    expect(screen.getByText('Filters')).toBeInTheDocument();
    expect(container.firstElementChild).toHaveClass('sticky');
  });
});
