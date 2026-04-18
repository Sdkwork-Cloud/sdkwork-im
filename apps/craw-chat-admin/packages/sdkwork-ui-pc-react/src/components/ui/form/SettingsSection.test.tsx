import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { SettingsSection } from './index';

describe('SettingsSection', () => {
  it('renders a lightweight settings section with title, description, actions, and content', () => {
    render(
      <SettingsSection
        actions={<button type="button">Reset</button>}
        description="Tune workspace defaults and provider behavior."
        title="Workspace"
      >
        <div>Settings content</div>
      </SettingsSection>,
    );

    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByText('Tune workspace defaults and provider behavior.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
    expect(screen.getByText('Settings content')).toBeInTheDocument();
  });
});
