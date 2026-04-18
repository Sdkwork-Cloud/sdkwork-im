import type { ComponentType } from 'react';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import * as workspacePatterns from './index';

const { ListDetailWorkspace } = workspacePatterns as Record<string, ComponentType<any>>;

describe('ListDetailWorkspace', () => {
  it('renders sidebar, content, and detail panes for desktop list-detail workspaces', () => {
    render(
      <ListDetailWorkspace
        content={{
          children: <div>Main canvas</div>,
          description: 'Primary editing surface',
          title: 'Workspace',
        }}
        detail={{
          children: <div>Selected asset metadata</div>,
          footer: <button type="button">Save</button>,
          title: 'Inspector',
        }}
        sidebar={{
          children: <div>Asset list</div>,
          description: 'Shared asset navigation',
          title: 'Assets',
        }}
      />,
    );

    expect(screen.getByText('Assets')).toBeInTheDocument();
    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByText('Inspector')).toBeInTheDocument();
    expect(screen.getByText('Asset list')).toBeInTheDocument();
    expect(screen.getByText('Main canvas')).toBeInTheDocument();
    expect(screen.getByText('Selected asset metadata')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save' })).toBeInTheDocument();
    expect(screen.getAllByTestId('sdk-panel-resize-handle-grip')).toHaveLength(2);
  });

  it('can render a content-only workspace without split handles', () => {
    const { container } = render(
      <ListDetailWorkspace
        content={{
          children: <div>Single workspace view</div>,
          title: 'Editor',
        }}
      />,
    );

    expect(screen.getByText('Single workspace view')).toBeInTheDocument();
    expect(container.querySelectorAll('[data-sdk-panel-resize-handle]').length).toBe(0);
  });
});
