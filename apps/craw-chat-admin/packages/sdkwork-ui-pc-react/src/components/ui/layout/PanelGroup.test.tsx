import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import * as layout from './index';

afterEach(() => {
  cleanup();
});

describe('PanelGroup', () => {
  it('is exported from the layout domain', () => {
    expect(layout).toHaveProperty('PanelGroup');
    expect(layout).toHaveProperty('Panel');
    expect(layout).toHaveProperty('PanelResizeHandle');
  });

  it('renders a horizontal desktop split workspace with a visible resize handle grip', () => {
    const exports = layout as Record<string, unknown>;
    const PanelGroup = exports.PanelGroup as React.ComponentType<any>;
    const Panel = exports.Panel as React.ComponentType<any>;
    const PanelResizeHandle = exports.PanelResizeHandle as React.ComponentType<any>;

    render(
      <PanelGroup aria-label="Workspace split" direction="horizontal">
        <Panel defaultSize={28}>Navigation</Panel>
        <PanelResizeHandle withHandle />
        <Panel defaultSize={72}>Workspace</Panel>
      </PanelGroup>,
    );

    expect(screen.getByText('Navigation')).toBeInTheDocument();
    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByRole('separator')).toBeInTheDocument();
    expect(screen.getByTestId('sdk-panel-resize-handle-grip')).toBeInTheDocument();
  });

  it('supports vertical panel groups for timeline and inspector layouts', () => {
    const exports = layout as Record<string, unknown>;
    const PanelGroup = exports.PanelGroup as React.ComponentType<any>;
    const Panel = exports.Panel as React.ComponentType<any>;
    const PanelResizeHandle = exports.PanelResizeHandle as React.ComponentType<any>;

    render(
      <PanelGroup direction="vertical">
        <Panel defaultSize={60}>Editor</Panel>
        <PanelResizeHandle />
        <Panel defaultSize={40}>Timeline</Panel>
      </PanelGroup>,
    );

    expect(screen.getByText('Editor')).toBeInTheDocument();
    expect(screen.getByText('Timeline')).toBeInTheDocument();
    expect(screen.getByRole('separator')).toHaveAttribute('data-panel-group-direction', 'vertical');
  });
});
