import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { SidebarSection } from './index';

describe('SidebarSection', () => {
  it('toggles collapsible sidebar content from its header button', () => {
    render(
      <SidebarSection defaultExpanded={false} title="Favorites">
        <div>Sidebar content</div>
      </SidebarSection>,
    );

    const trigger = screen.getByRole('button', { name: 'Favorites' });

    expect(trigger).toHaveAttribute('aria-expanded', 'false');
    expect(screen.queryByText('Sidebar content')).not.toBeInTheDocument();

    fireEvent.click(trigger);

    expect(trigger).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByText('Sidebar content')).toBeInTheDocument();
  });
});
