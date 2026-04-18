import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { KeyValueTable } from './index';

describe('KeyValueTable', () => {
  it('renders read-only label and value rows', () => {
    const { container } = render(
      <KeyValueTable
        rows={[
          { label: 'Request ID', value: 'req_123456', mono: true },
          { label: 'Status', value: 'Completed' },
        ]}
      />,
    );

    expect(screen.getByText('Request ID')).toBeInTheDocument();
    expect(screen.getByText('req_123456')).toHaveClass('font-mono');
    expect(screen.getByText('Status')).toBeInTheDocument();
    expect(screen.getByText('Completed')).toBeInTheDocument();
    expect(container.querySelector('table[data-sdk-ui="key-value-table"]')).toBeInTheDocument();
    expect(container.querySelector('[data-sdk-ui="key-value-table-row"]')).toBeInTheDocument();
  });

  it('supports tone and emphasis for important values', () => {
    const { container } = render(
      <KeyValueTable
        rows={[{ label: 'Risk', value: 'High', tone: 'danger', emphasis: 'strong' }]}
      />,
    );

    const row = container.querySelector('[data-sdk-ui="key-value-table-row"]');
    expect(row).toHaveAttribute('data-tone', 'danger');
    expect(row).toHaveAttribute('data-emphasis', 'strong');
    expect(screen.getByText('High')).toHaveClass('font-semibold');
  });

  it('supports dense layout for inspector surfaces', () => {
    const { container } = render(
      <KeyValueTable dense rows={[{ label: 'Model', value: 'gpt-5.4' }]} />,
    );

    expect(container.querySelector('table[data-density="dense"]')).toBeInTheDocument();
    expect(screen.getByText('Model')).toHaveClass('py-1.5');
    expect(screen.getByText('gpt-5.4')).toHaveClass('py-1.5');
  });
});
