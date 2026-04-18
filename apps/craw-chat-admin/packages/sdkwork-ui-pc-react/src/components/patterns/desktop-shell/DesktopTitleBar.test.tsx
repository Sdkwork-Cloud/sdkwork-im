import { render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { DesktopTitleBar } from './DesktopTitleBar';
import type { DesktopWindowController } from './DesktopWindowControls';

function createWindowController(): DesktopWindowController {
  return {
    closeWindow: vi.fn().mockResolvedValue(undefined),
    getPlatform: vi.fn().mockReturnValue('desktop'),
    isWindowMaximized: vi.fn().mockResolvedValue(false),
    maximizeWindow: vi.fn().mockResolvedValue(undefined),
    minimizeWindow: vi.fn().mockResolvedValue(undefined),
    restoreWindow: vi.fn().mockResolvedValue(undefined),
    subscribeWindowMaximized: vi.fn().mockResolvedValue(() => undefined),
  };
}

describe('DesktopTitleBar', () => {
  it('renders leading, center, trailing, and shared window controls', async () => {
    render(
      <DesktopTitleBar
        center={<button type="button">Workspace</button>}
        leading={<div>Brand</div>}
        trailing={<button type="button">Settings</button>}
        windowController={createWindowController()}
      />,
    );

    expect(screen.getByText('Brand')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Workspace' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Settings' })).toBeInTheDocument();
    expect(await screen.findByRole('button', { name: 'Minimize window' })).toBeInTheDocument();
  });
});
