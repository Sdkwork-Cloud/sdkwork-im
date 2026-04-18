import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  Combobox,
  DateInput,
  NumberInput,
  TagInput,
} from './components/ui';

const slotProbeClassName = 'sdk-ui-field-shell-slot-probe';
const slotProbeStyle = { outline: '1px solid transparent' } as const;

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('UI field shell slot props contract', () => {
  it('applies structured slotProps to owned NumberInput shell surfaces', () => {
    render(
      <NumberInput
        aria-label="Retries"
        defaultValue={3}
        slotProps={{
          decrementButton: {
            className: 'number-input-decrement-probe',
            'data-slot-probe': 'number-input-decrement',
          },
          incrementButton: {
            className: 'number-input-increment-probe',
            'data-slot-probe': 'number-input-increment',
          },
          root: {
            className: slotProbeClassName,
            'data-slot-probe': 'number-input-root',
            id: 'number-input-root',
            style: slotProbeStyle,
          },
          stepper: {
            className: 'number-input-stepper-probe',
            'data-slot-probe': 'number-input-stepper',
          },
        }}
      />,
    );

    expect(queryUi('number-input')).toHaveClass(slotProbeClassName);
    expect(queryUi('number-input')).toHaveAttribute('data-slot-probe', 'number-input-root');
    expect(queryUi('number-input')).toHaveAttribute('id', 'number-input-root');
    expect(queryUi('number-input')).toHaveStyle(slotProbeStyle);
    expect(queryUi('number-input-stepper')).toHaveClass('number-input-stepper-probe');
    expect(queryUi('number-input-stepper')).toHaveAttribute('data-slot-probe', 'number-input-stepper');
    expect(screen.getByRole('button', { name: 'Increase value' })).toHaveClass('number-input-increment-probe');
    expect(screen.getByRole('button', { name: 'Increase value' })).toHaveAttribute(
      'data-slot-probe',
      'number-input-increment',
    );
    expect(screen.getByRole('button', { name: 'Decrease value' })).toHaveClass('number-input-decrement-probe');
    expect(screen.getByRole('button', { name: 'Decrease value' })).toHaveAttribute(
      'data-slot-probe',
      'number-input-decrement',
    );
  });

  it('applies structured slotProps to owned DateInput shell surfaces', () => {
    render(
      <DateInput
        aria-label="Due date"
        slotProps={{
          calendarButton: {
            className: 'date-input-calendar-probe',
            'data-slot-probe': 'date-input-calendar-button',
          },
          root: {
            className: slotProbeClassName,
            'data-slot-probe': 'date-input-root',
            id: 'date-input-root',
            style: slotProbeStyle,
          },
        }}
      />,
    );

    expect(queryUi('date-input')).toHaveClass(slotProbeClassName);
    expect(queryUi('date-input')).toHaveAttribute('data-slot-probe', 'date-input-root');
    expect(queryUi('date-input')).toHaveAttribute('id', 'date-input-root');
    expect(queryUi('date-input')).toHaveStyle(slotProbeStyle);
    expect(queryUi('date-input-calendar-button')).toHaveClass('date-input-calendar-probe');
    expect(queryUi('date-input-calendar-button')).toHaveAttribute('data-slot-probe', 'date-input-calendar-button');
  });

  it('applies structured slotProps to owned TagInput shell surfaces', () => {
    render(
      <TagInput
        aria-label="Tags"
        defaultValue={['alpha']}
        slotProps={{
          removeButton: {
            className: 'tag-input-remove-probe',
            'data-slot-probe': 'tag-input-remove',
          },
          root: {
            className: slotProbeClassName,
            'data-slot-probe': 'tag-input-root',
            id: 'tag-input-root',
            style: slotProbeStyle,
          },
          tag: {
            className: 'tag-input-tag-probe',
            'data-slot-probe': 'tag-input-tag',
          },
        }}
      />,
    );

    expect(queryUi('tag-input')).toHaveClass(slotProbeClassName);
    expect(queryUi('tag-input')).toHaveAttribute('data-slot-probe', 'tag-input-root');
    expect(queryUi('tag-input')).toHaveAttribute('id', 'tag-input-root');
    expect(queryUi('tag-input')).toHaveStyle(slotProbeStyle);
    expect(screen.getByLabelText('Tags')).toHaveAttribute('data-slot', 'tag-input-input');
    expect(screen.getByText('alpha').closest('[data-slot="tag-input-tag"]')).toHaveClass('tag-input-tag-probe');
    expect(screen.getByText('alpha').closest('[data-slot="tag-input-tag"]')).toHaveAttribute(
      'data-slot-probe',
      'tag-input-tag',
    );
    expect(screen.getByRole('button', { name: 'Remove alpha' })).toHaveClass('tag-input-remove-probe');
    expect(screen.getByRole('button', { name: 'Remove alpha' })).toHaveAttribute(
      'data-slot-probe',
      'tag-input-remove',
    );
  });

  it('applies structured slotProps to owned Combobox shell surfaces', async () => {
    render(
      <Combobox
        aria-label="Model"
        clearable
        defaultValue="gpt-5.4"
        options={[
          { label: 'GPT-5.4', value: 'gpt-5.4' },
          { label: 'Claude Desktop', value: 'claude-desktop' },
        ]}
        slotProps={{
          clearButton: {
            className: 'combobox-clear-probe',
            'data-slot-probe': 'combobox-clear',
          },
          content: {
            className: 'combobox-content-probe',
            'data-slot-probe': 'combobox-content',
          },
          input: {
            className: 'combobox-input-probe',
            'data-slot-probe': 'combobox-input',
          },
          root: {
            className: slotProbeClassName,
            'data-slot-probe': 'combobox-root',
            id: 'combobox-root',
            style: slotProbeStyle,
          },
        }}
      />,
    );

    expect(queryUi('combobox')).toHaveClass(slotProbeClassName);
    expect(queryUi('combobox')).toHaveAttribute('data-slot-probe', 'combobox-root');
    expect(queryUi('combobox')).toHaveAttribute('id', 'combobox-root');
    expect(queryUi('combobox')).toHaveStyle(slotProbeStyle);
    expect(screen.getByRole('button', { name: 'Clear selection' })).toHaveClass('combobox-clear-probe');
    expect(screen.getByRole('button', { name: 'Clear selection' })).toHaveAttribute('data-slot-probe', 'combobox-clear');

    fireEvent.click(screen.getByRole('combobox'));

    await waitFor(() => {
      expect(queryUi('combobox-content')).toHaveClass('combobox-content-probe');
      expect(queryUi('combobox-content')).toHaveAttribute('data-slot-probe', 'combobox-content');
      expect(queryUi('combobox-input')).toHaveClass('combobox-input-probe');
      expect(queryUi('combobox-input')).toHaveAttribute('data-slot-probe', 'combobox-input');
    });
  });
});
