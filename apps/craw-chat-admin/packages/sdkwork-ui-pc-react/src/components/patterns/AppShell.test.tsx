import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { AppShell } from './AppShell';
import { SDKWORK_DARK_THEME } from '../../theme';

describe('AppShell', () => {
  it('renders standard PC application regions', () => {
    render(
      <AppShell
        header={<div>Header</div>}
        sidebar={<div>Sidebar</div>}
        content={<div>Workspace</div>}
        footer={<div>Footer</div>}
      />,
    );

    expect(screen.getByText('Header')).toBeInTheDocument();
    expect(screen.getByText('Sidebar')).toBeInTheDocument();
    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByText('Footer')).toBeInTheDocument();
  });

  it('applies theme variables and shell data markers to the root element', () => {
    const { container } = render(<AppShell content={<div>Workspace</div>} />);
    const shell = container.firstElementChild;

    expect(shell).toHaveAttribute('data-sdk-shell', 'app');
    expect(shell).toHaveStyle(`--sdk-color-surface-canvas: ${SDKWORK_DARK_THEME.surface.canvas}`);
  });
});
