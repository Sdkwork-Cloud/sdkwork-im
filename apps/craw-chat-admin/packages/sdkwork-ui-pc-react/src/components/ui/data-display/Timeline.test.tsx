import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Timeline } from './index';

describe('Timeline', () => {
  it('renders ordered desktop timeline items with timestamps and descriptions', () => {
    render(
      <Timeline
        items={[
          {
            id: 'submitted',
            title: 'Submitted request',
            description: 'Funds were requested for the default payout account.',
            status: 'done',
            timestamp: '2026-03-31 09:00',
          },
          {
            id: 'review',
            title: 'Finance review',
            description: 'The finance team is validating the payout details.',
            status: 'current',
            timestamp: '2026-03-31 09:15',
          },
        ]}
      />,
    );

    expect(screen.getByRole('list')).toBeInTheDocument();
    expect(screen.getAllByRole('listitem')).toHaveLength(2);
    expect(screen.getByText('Submitted request')).toBeInTheDocument();
    expect(screen.getByText('Funds were requested for the default payout account.')).toBeInTheDocument();
    expect(screen.getByText('2026-03-31 09:15')).toBeInTheDocument();
  });

  it('supports custom item content and semantic status mapping', () => {
    render(
      <Timeline
        items={[
          {
            id: 'transfer',
            title: 'Transferred',
            content: <button type="button">View receipt</button>,
            status: 'success',
          },
          {
            id: 'exception',
            title: 'Needs attention',
            description: 'The receiving account must be reverified.',
            status: 'error',
          },
        ]}
      />,
    );

    expect(screen.getByRole('button', { name: 'View receipt' })).toBeInTheDocument();
    expect(screen.getByText('Transferred').closest('[data-sdk-ui="timeline-item"]')).toHaveAttribute('data-status', 'success');
    expect(screen.getByText('Needs attention').closest('[data-sdk-ui="timeline-item"]')).toHaveAttribute('data-status', 'danger');
  });
});
