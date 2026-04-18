import * as React from 'react';
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import * as feedback from './index';

function getActivityFeedComponent() {
  const ActivityFeed = (feedback as Record<string, unknown>).ActivityFeed as
    | React.ComponentType<Record<string, unknown>>
    | undefined;

  expect(ActivityFeed).toBeDefined();

  return ActivityFeed!;
}

describe('ActivityFeed', () => {
  it('exports the shared desktop activity feed surface', () => {
    expect((feedback as Record<string, unknown>).ActivityFeed).toBeDefined();
  });

  it('renders an empty state when there are no activity items', () => {
    const ActivityFeed = getActivityFeedComponent();

    render(
      <ActivityFeed
        emptyDescription="Workflow updates, review events, and automation history will appear here."
        items={[]}
        title="Recent activity"
      />,
    );

    expect(screen.getByText('Recent activity')).toBeInTheDocument();
    expect(screen.getByText('No activity yet')).toBeInTheDocument();
    expect(
      screen.getByText('Workflow updates, review events, and automation history will appear here.'),
    ).toBeInTheDocument();
  });

  it('renders activity items with timestamps, details, actions, and item selection', () => {
    const ActivityFeed = getActivityFeedComponent();
    const onItemSelect = vi.fn();

    render(
      <ActivityFeed
        items={[
          {
            description: 'The latest desktop bundle passed smoke validation and was published to QA.',
            id: 'release',
            meta: 'Release pipeline',
            timestamp: '2026-04-01 10:30',
            title: 'Build completed',
            tone: 'success',
            unread: true,
          },
          {
            actions: <button type="button">Open review</button>,
            content: <div>Approver: Desktop platform team</div>,
            description: 'A manual approval is required before promoting this build to production.',
            id: 'approval',
            timestamp: '2026-04-01 11:00',
            title: 'Approval requested',
            tone: 'warning',
          },
        ]}
        onItemSelect={onItemSelect}
        title="Activity"
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Build completed' }));

    expect(onItemSelect).toHaveBeenCalledWith('release');
    expect(screen.getAllByRole('listitem')).toHaveLength(2);
    expect(screen.getByText('Release pipeline')).toBeInTheDocument();
    expect(screen.getByText('2026-04-01 11:00')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Open review' })).toBeInTheDocument();
    expect(screen.getByText('Approver: Desktop platform team')).toBeInTheDocument();
    expect(
      screen.getByText('Build completed').closest('[data-sdk-ui="activity-feed-item"]'),
    ).toHaveAttribute('data-tone', 'success');
  });
});
