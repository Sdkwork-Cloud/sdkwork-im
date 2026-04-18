import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Combobox } from './index';

afterEach(() => {
  cleanup();
});

describe('Combobox', () => {
  it('opens shared desktop options and updates the selected value', () => {
    const handleValueChange = vi.fn();

    render(
      <Combobox
        options={[
          { label: 'GPT-5.4', value: 'gpt-5.4' },
          { label: 'Claude Desktop', value: 'claude-desktop' },
        ]}
        placeholder="Select model"
        onValueChange={handleValueChange}
      />,
    );

    fireEvent.click(screen.getByRole('combobox', { name: 'Select model' }));
    fireEvent.click(screen.getByText('Claude Desktop'));

    expect(handleValueChange).toHaveBeenCalledWith('claude-desktop');
    expect(screen.getByRole('combobox')).toHaveTextContent('Claude Desktop');
  });

  it('renders a real clear button with governed slot markers instead of icon-only click chrome', () => {
    const handleValueChange = vi.fn();

    render(
      <Combobox
        clearable
        defaultValue="gpt-5.4"
        onValueChange={handleValueChange}
        options={[
          { label: 'GPT-5.4', value: 'gpt-5.4' },
          { label: 'Claude Desktop', value: 'claude-desktop' },
        ]}
        slotProps={{
          clearButton: {
            className: 'combobox-clear-probe',
            'data-slot-probe': 'combobox-clear-button',
          },
        }}
      />,
    );

    const clearButton = screen.getByRole('button', { name: 'Clear selection' });

    expect(clearButton).toHaveAttribute('data-slot', 'combobox-clear-button');
    expect(clearButton).toHaveClass('combobox-clear-probe');
    expect(clearButton).toHaveAttribute('data-slot-probe', 'combobox-clear-button');

    fireEvent.click(clearButton);

    expect(handleValueChange).toHaveBeenCalledWith('');
    expect(screen.getByRole('combobox')).toHaveTextContent('Select option');
  });
});
