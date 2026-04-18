import { render, screen } from '@testing-library/react';
import { Settings2 } from 'lucide-react';
import { describe, expect, it } from 'vitest';
import { IconButton } from './index';

describe('IconButton', () => {
  it('renders an icon-first action button with the shared icon sizing contract', () => {
    render(
      <IconButton aria-label="Open settings" variant="ghost">
        <Settings2 />
      </IconButton>,
    );

    const button = screen.getByRole('button', { name: 'Open settings' });

    expect(button).toBeInTheDocument();
    expect(button).toHaveClass('h-10', 'w-10');
    expect(button.querySelector('svg')).not.toBeNull();
  });
});
