import type { ComponentType } from 'react';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import * as patterns from '../index';

const { DirtyStateBar, SettingsCenter } = patterns as Record<string, ComponentType<any>>;

afterEach(() => {
  cleanup();
});

describe('SettingsCenter', () => {
  it('renders desktop settings navigation, search, actions, and active content', () => {
    const handleSearchChange = vi.fn();
    const handleActiveItemChange = vi.fn();

    render(
      <SettingsCenter
        actions={<button type="button">Export</button>}
        activeItem="appearance"
        description="Configure shared desktop behavior."
        onActiveItemChange={handleActiveItemChange}
        onSearchChange={handleSearchChange}
        searchValue=""
        sections={[
          {
            items: [
              {
                description: 'Colors, density, theme, and appearance mode.',
                id: 'appearance',
                keywords: ['theme'],
                label: 'Appearance',
              },
              { description: 'Identity and security settings.', id: 'account', label: 'Account' },
            ],
            title: 'Workspace',
          },
        ]}
        title="Settings Center"
      >
        <div>Appearance form</div>
      </SettingsCenter>,
    );

    expect(screen.getByText('Settings Center')).toBeInTheDocument();
    expect(screen.getByText('Configure shared desktop behavior.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Export' })).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search settings')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Appearance' })).toHaveAttribute('data-state', 'active');
    expect(screen.getByRole('button', { name: 'Appearance' })).toHaveAttribute(
      'data-slot',
      'settings-center-nav-item',
    );
    expect(screen.getByRole('button', { name: 'Appearance' })).toHaveClass(
      'rounded-[var(--sdk-radius-field)]',
    );
    fireEvent.click(screen.getByRole('button', { name: 'Account' }));
    expect(handleActiveItemChange).toHaveBeenCalledWith('account');
    expect(screen.getByText('Appearance form')).toBeInTheDocument();

    fireEvent.change(screen.getByPlaceholderText('Search settings'), {
      target: { value: 'account' },
    });
    expect(handleSearchChange).toHaveBeenCalledWith('account');
  });

  it('filters settings navigation and shows an empty search result state when nothing matches', () => {
    render(
      <SettingsCenter
        searchValue="privacy"
        sections={[
          {
            items: [{ id: 'appearance', label: 'Appearance', keywords: ['theme', 'color'] }],
            title: 'Workspace',
          },
        ]}
        title="Settings Center"
      >
        <div>Appearance form</div>
      </SettingsCenter>,
    );

    expect(screen.queryByRole('button', { name: 'Appearance' })).not.toBeInTheDocument();
    expect(screen.getByText('No settings match your search')).toBeInTheDocument();
  });
});

describe('DirtyStateBar', () => {
  it('renders save and reset actions for unsaved desktop settings', () => {
    render(
      <DirtyStateBar
        actions={
          <>
            <button type="button">Reset</button>
            <button type="button">Save changes</button>
          </>
        }
        description="Unsaved updates will apply to the current workspace."
        title="You have unsaved changes"
      />,
    );

    expect(screen.getByText('You have unsaved changes')).toBeInTheDocument();
    expect(screen.getByText('Unsaved updates will apply to the current workspace.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save changes' })).toBeInTheDocument();
  });
});
