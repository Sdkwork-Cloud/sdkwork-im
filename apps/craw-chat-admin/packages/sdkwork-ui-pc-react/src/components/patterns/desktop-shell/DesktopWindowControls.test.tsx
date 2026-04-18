import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import {
  DesktopWindowControls,
  type DesktopWindowController,
} from './DesktopWindowControls';

interface TestWindowController extends DesktopWindowController {
  emitMaximized: (value: boolean) => void;
}

function createWindowController(
  overrides: Partial<DesktopWindowController> = {},
): TestWindowController {
  let listener: ((value: boolean) => void) | null = null;

  const controller: DesktopWindowController = {
    closeWindow: vi.fn().mockResolvedValue(undefined),
    getPlatform: vi.fn().mockReturnValue('desktop'),
    isWindowMaximized: vi.fn().mockResolvedValue(false),
    maximizeWindow: vi.fn().mockResolvedValue(undefined),
    minimizeWindow: vi.fn().mockResolvedValue(undefined),
    restoreWindow: vi.fn().mockResolvedValue(undefined),
    subscribeWindowMaximized: vi.fn().mockImplementation(async (next) => {
      listener = next;
      return () => {
        listener = null;
      };
    }),
    ...overrides,
  };

  return {
    ...controller,
    emitMaximized: (value) => {
      listener?.(value);
    },
  };
}

describe('DesktopWindowControls', () => {
  it('renders nothing when the controller is unavailable on desktop', () => {
    const { container } = render(<DesktopWindowControls controller={null} />);

    expect(container.firstChild).toBeNull();
  });

  it('renders nothing on web controllers', () => {
    const controller = createWindowController({
      getPlatform: vi.fn().mockReturnValue('web'),
    });
    const { container } = render(<DesktopWindowControls controller={controller} />);

    expect(container.firstChild).toBeNull();
  });

  it('invokes minimize, maximize, restore, and close actions as window state changes', async () => {
    const controller = createWindowController();

    render(<DesktopWindowControls controller={controller} />);

    fireEvent.click(await screen.findByRole('button', { name: 'Minimize window' }));
    fireEvent.click(screen.getByRole('button', { name: 'Maximize window' }));
    fireEvent.click(screen.getByRole('button', { name: 'Close window' }));

    expect(controller.minimizeWindow).toHaveBeenCalledTimes(1);
    expect(controller.maximizeWindow).toHaveBeenCalledTimes(1);
    expect(controller.closeWindow).toHaveBeenCalledTimes(1);

    controller.emitMaximized(true);

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Restore window' })).toBeInTheDocument();
    });

    fireEvent.click(screen.getByRole('button', { name: 'Restore window' }));

    expect(controller.restoreWindow).toHaveBeenCalledTimes(1);
  });
});
