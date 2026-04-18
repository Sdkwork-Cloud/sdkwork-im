import { cleanup, render, screen } from '@testing-library/react';
import { Bold, Italic } from 'lucide-react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import * as actions from './index';

afterEach(() => {
  cleanup();
});

describe('ToolbarButton', () => {
  it('is exported from the actions domain', () => {
    expect(actions).toHaveProperty('ToolbarButton');
  });

  it('renders a pressed toolbar action with default sizing and shortcut-aware title copy', () => {
    const ToolbarButton = actions.ToolbarButton as React.ComponentType<any>;

    render(
      <ToolbarButton aria-label="Bold" pressed shortcut="Ctrl+B">
        <Bold className="h-4 w-4" />
      </ToolbarButton>,
    );

    const button = screen.getByRole('button', { name: 'Bold' });

    expect(button).toHaveAttribute('aria-pressed', 'true');
    expect(button).toHaveAttribute('title', 'Bold (Ctrl+B)');
    expect(button).toHaveClass('h-9', 'min-w-9');
  });

  it('supports the active alias and preserves editor focus on mouse down when requested', () => {
    const ToolbarButton = actions.ToolbarButton as React.ComponentType<any>;
    const onMouseDown = vi.fn();

    render(
      <ToolbarButton
        active
        aria-label="Italic"
        onMouseDown={onMouseDown}
        preserveFocusOnMouseDown
      >
        <Italic className="h-4 w-4" />
      </ToolbarButton>,
    );

    const button = screen.getByRole('button', { name: 'Italic' });
    const event = new MouseEvent('mousedown', { bubbles: true, cancelable: true });

    button.dispatchEvent(event);

    expect(event.defaultPrevented).toBe(true);
    expect(onMouseDown).toHaveBeenCalledTimes(1);
    expect(button).toHaveAttribute('aria-pressed', 'true');
  });

  it('renders icon and label content in compact mode for editor and timeline toolbars', () => {
    const ToolbarButton = actions.ToolbarButton as React.ComponentType<any>;

    render(
      <ToolbarButton size="compact">
        <Bold className="h-4 w-4" />
        <span>Bold</span>
      </ToolbarButton>,
    );

    const button = screen.getByRole('button', { name: 'Bold' });

    expect(button).toHaveClass('h-8', 'px-2');
    expect(button.querySelector('svg')).not.toBeNull();
  });
});
