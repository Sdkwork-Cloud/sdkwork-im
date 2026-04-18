import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Button } from '../../ui/button';
import { Input } from '../../ui/input';
import { SectionHeader } from './SectionHeader';
import { WorkspaceScaffold } from './WorkspaceScaffold';

describe('WorkspaceScaffold', () => {
  it('renders a reusable desktop workspace shell with header, sidebar, main canvas, inspector, bottom panel, and status bar', () => {
    const { container } = render(
      <WorkspaceScaffold
        header={(
          <SectionHeader
            actions={<Button variant="secondary">Publish</Button>}
            description="Desktop editor shell with reusable slots."
            title="Media workspace"
          />
        )}
        inspector={{
          children: <div>Inspector details</div>,
          footer: <Button>Apply changes</Button>,
          summary: 'Selected assets can be edited from the shared inspector rail.',
          title: 'Inspector',
        }}
        main={{
          children: <div>Primary canvas</div>,
          description: 'Main editing and preview surface.',
          slotProps: {
            body: {
              className: 'p-0',
            },
          },
          title: 'Canvas',
        }}
        sidebar={{
          children: <div>Library tree</div>,
          title: 'Library',
        }}
        bottomPanel={{
          children: <div>Activity and logs</div>,
          title: 'Timeline',
        }}
        statusBar={<div>Autosave enabled</div>}
        toolbar={<Input aria-label="Search workspace" placeholder="Search workspace" />}
      />,
    );

    expect(screen.getByText('Media workspace')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search workspace')).toBeInTheDocument();
    expect(screen.getByText('Library')).toBeInTheDocument();
    expect(screen.getByText('Primary canvas')).toBeInTheDocument();
    expect(screen.getByText('Inspector')).toBeInTheDocument();
    expect(screen.getByText('Activity and logs')).toBeInTheDocument();
    expect(screen.getByText('Autosave enabled')).toBeInTheDocument();
    expect(container.querySelectorAll('[data-sdk-panel-resize-handle]').length).toBe(3);
  });

  it('supports a main-only scaffold without resize handles', () => {
    const { container } = render(
      <WorkspaceScaffold
        main={{
          children: <div>Focused workspace</div>,
          title: 'Editor',
        }}
      />,
    );

    expect(screen.getByText('Focused workspace')).toBeInTheDocument();
    expect(container.querySelectorAll('[data-sdk-panel-resize-handle]').length).toBe(0);
  });
});
