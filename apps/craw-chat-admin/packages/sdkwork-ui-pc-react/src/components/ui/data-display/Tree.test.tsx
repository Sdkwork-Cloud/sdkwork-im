import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Tree, type TreeNodeData } from './index';

afterEach(() => {
  cleanup();
});

describe('Tree', () => {
  it('toggles folders and notifies when a leaf node is selected', () => {
    const handleSelectedIdChange = vi.fn();
    const treeData: TreeNodeData[] = [
      {
        id: 'workspace',
        label: 'Workspace',
        children: [
          {
            id: 'api-keys',
            label: 'API Keys',
          },
        ],
      },
    ];

    render(<Tree data={treeData} onSelectedIdChange={handleSelectedIdChange} />);

    expect(screen.getByRole('tree')).toBeInTheDocument();
    expect(screen.queryByText('API Keys')).not.toBeInTheDocument();

    fireEvent.click(screen.getByText('Workspace'));
    expect(screen.getByText('API Keys')).toBeInTheDocument();

    fireEvent.click(screen.getByText('API Keys'));
    expect(handleSelectedIdChange).toHaveBeenCalledWith('api-keys');
  });

  it('supports multi-select trees for desktop explorer patterns', () => {
    const handleSelectedIdsChange = vi.fn();
    const treeData: TreeNodeData[] = [
      {
        id: 'workspace',
        label: 'Workspace',
        children: [
          {
            id: 'readme',
            label: 'README.md',
          },
          {
            id: 'settings',
            label: 'settings.json',
          },
        ],
      },
    ];

    render(
      <Tree
        data={treeData}
        defaultExpandedIds={['workspace']}
        onSelectedIdsChange={handleSelectedIdsChange}
        selectionMode="multiple"
      />,
    );

    fireEvent.click(screen.getByText('README.md'));
    fireEvent.click(screen.getByText('settings.json'));

    expect(handleSelectedIdsChange).toHaveBeenLastCalledWith(['readme', 'settings']);
  });

  it('renders checkboxes and cascades parent checked state', () => {
    const handleCheckedIdsChange = vi.fn();
    const treeData: TreeNodeData[] = [
      {
        id: 'workspace',
        label: 'Workspace',
        children: [
          {
            id: 'readme',
            label: 'README.md',
          },
          {
            id: 'settings',
            label: 'settings.json',
          },
        ],
      },
    ];

    render(
      <Tree
        checkable
        data={treeData}
        defaultExpandedIds={['workspace']}
        onCheckedIdsChange={handleCheckedIdsChange}
      />,
    );

    fireEvent.click(screen.getByLabelText('Check item README.md'));

    expect(handleCheckedIdsChange).toHaveBeenCalledWith(['readme']);
    expect(screen.getByLabelText('Check item Workspace')).toHaveAttribute('aria-checked', 'mixed');
  });

  it('supports keyboard expansion and lazy branch loading for desktop workspaces', () => {
    const handleLoadChildren = vi.fn();
    const treeData: TreeNodeData[] = [
      {
        id: 'remote',
        label: 'Remote workspace',
        hasChildren: true,
      },
    ];

    render(<Tree data={treeData} loadChildren={handleLoadChildren} />);

    const remoteItem = screen.getByRole('treeitem', { name: 'Remote workspace' });
    remoteItem.focus();
    fireEvent.keyDown(remoteItem, { key: 'ArrowRight' });

    expect(handleLoadChildren).toHaveBeenCalledTimes(1);
  });

  it('supports Home and End keyboard navigation across the visible tree collection', () => {
    const treeData: TreeNodeData[] = [
      {
        id: 'workspace',
        label: 'Workspace',
        children: [
          {
            id: 'readme',
            label: 'README.md',
          },
          {
            id: 'settings',
            label: 'settings.json',
          },
        ],
      },
    ];

    render(<Tree data={treeData} defaultExpandedIds={['workspace']} />);

    const workspaceItem = screen.getByRole('treeitem', { name: 'Workspace' });
    const settingsItem = screen.getByRole('treeitem', { name: 'settings.json' });

    workspaceItem.focus();
    fireEvent.keyDown(workspaceItem, { key: 'End' });

    expect(settingsItem).toHaveFocus();

    fireEvent.keyDown(settingsItem, { key: 'Home' });

    expect(workspaceItem).toHaveFocus();
  });
});
