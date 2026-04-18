import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { SettingsField } from './index';

describe('SettingsField', () => {
  it('renders label, description, error, and reset affordance for desktop settings forms', () => {
    const onReset = vi.fn();

    render(
      <SettingsField
        controlId="settings-api-key"
        description="Used to authenticate requests for the active provider."
        error="API key is required."
        isModified
        label="API Key"
        onReset={onReset}
      >
        <input id="settings-api-key" />
      </SettingsField>,
    );

    expect(screen.getByText('API Key')).toBeInTheDocument();
    expect(
      screen.getByText('Used to authenticate requests for the active provider.'),
    ).toBeInTheDocument();
    expect(screen.getByText('API key is required.')).toBeInTheDocument();
    expect(screen.getByRole('textbox')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Reset setting' }));
    expect(onReset).toHaveBeenCalledTimes(1);
  });
});
