import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  ActivityFeed,
  NotificationCenter,
  RichTree,
  Stepper,
  StepperItem,
  Timeline,
  WorkspaceTabs,
} from './components/ui';

const slotProbeClassName = 'sdk-ui-collection-slot-probe';
const slotProbeStyle = { outline: '1px solid transparent' } as const;

function queryRegion(region: string) {
  return document.body.querySelector(`[data-sdk-region="${region}"]`);
}

afterEach(() => {
  cleanup();
});

describe('UI collection surface contract', () => {
  it('applies structured collection surface props to RichTree owned item surfaces', () => {
    render(
      <RichTree
        checkable
        defaultExpandedIds={['workspace']}
        getItemProps={(item) =>
          item.id === 'workspace'
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'rich-tree-item',
                id: 'rich-tree-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={(item) =>
          item.id === 'workspace'
            ? {
                actions: {
                  className: 'rich-tree-actions-probe',
                  'data-slot-probe': 'rich-tree-actions',
                },
                checkbox: {
                  className: 'rich-tree-checkbox-probe',
                  'data-slot-probe': 'rich-tree-checkbox',
                },
                content: {
                  className: 'rich-tree-content-probe',
                  'data-slot-probe': 'rich-tree-content',
                },
                toggle: {
                  className: 'rich-tree-toggle-probe',
                  'data-slot-probe': 'rich-tree-toggle',
                },
              }
            : undefined
        }
        items={[
          {
            id: 'workspace',
            label: 'Workspace',
            children: [{ id: 'readme', label: 'README.md' }],
          },
        ]}
        renderActions={(item) => <button type="button">Open {item.id}</button>}
        slotProps={{
          tree: {
            className: 'rich-tree-root-probe',
            'data-slot-probe': 'rich-tree-root',
            id: 'rich-tree-root',
            style: slotProbeStyle,
          },
        }}
      />,
    );

    expect(queryRegion('rich-tree-tree')).toHaveClass('rich-tree-root-probe');
    expect(queryRegion('rich-tree-tree')).toHaveAttribute('data-slot-probe', 'rich-tree-root');
    expect(queryRegion('rich-tree-tree')).toHaveAttribute('id', 'rich-tree-root');
    expect(queryRegion('rich-tree-tree')).toHaveStyle(slotProbeStyle);
    expect(screen.getByRole('treeitem', { name: /workspace/i })).toHaveClass(slotProbeClassName);
    expect(screen.getByRole('treeitem', { name: /workspace/i })).toHaveAttribute('data-slot-probe', 'rich-tree-item');
    expect(screen.getByRole('treeitem', { name: /workspace/i })).toHaveAttribute('id', 'rich-tree-item');
    expect(screen.getByRole('treeitem', { name: /workspace/i })).toHaveStyle(slotProbeStyle);
    expect(screen.getByLabelText('Collapse Workspace')).toHaveClass('rich-tree-toggle-probe');
    expect(screen.getByLabelText('Collapse Workspace')).toHaveAttribute('data-slot-probe', 'rich-tree-toggle');
    expect(screen.getByLabelText('Check item Workspace')).toHaveClass('rich-tree-checkbox-probe');
    expect(screen.getByLabelText('Check item Workspace')).toHaveAttribute('data-slot-probe', 'rich-tree-checkbox');
    expect(screen.getByText('Workspace').closest('[data-sdk-region="rich-tree-item-content"]')).toHaveClass(
      'rich-tree-content-probe',
    );
    expect(screen.getByText('Workspace').closest('[data-sdk-region="rich-tree-item-content"]')).toHaveAttribute(
      'data-slot-probe',
      'rich-tree-content',
    );
    expect(screen.getByRole('button', { name: 'Open workspace' }).closest('[data-sdk-region="rich-tree-item-actions"]')).toHaveClass(
      'rich-tree-actions-probe',
    );
    expect(screen.getByRole('button', { name: 'Open workspace' }).closest('[data-sdk-region="rich-tree-item-actions"]')).toHaveAttribute(
      'data-slot-probe',
      'rich-tree-actions',
    );
  });

  it('applies repeated item props to Timeline exact item surfaces', () => {
    render(
      <Timeline
        getItemProps={(item) =>
          item.id === 'submitted'
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'timeline-item',
                id: 'timeline-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={(item) =>
          item.id === 'submitted'
            ? {
                connector: {
                  className: 'timeline-connector-probe',
                  'data-slot-probe': 'timeline-connector',
                },
                content: {
                  className: 'timeline-content-probe',
                  'data-slot-probe': 'timeline-content',
                },
                indicator: {
                  className: 'timeline-indicator-probe',
                  'data-slot-probe': 'timeline-indicator',
                },
              }
            : undefined
        }
        items={[
          {
            description: 'Funds were requested for the default payout account.',
            id: 'submitted',
            status: 'done',
            timestamp: '2026-03-31 09:00',
            title: 'Submitted request',
          },
          {
            id: 'review',
            status: 'current',
            title: 'Finance review',
          },
        ]}
      />,
    );

    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')).toHaveClass(slotProbeClassName);
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')).toHaveAttribute(
      'data-slot-probe',
      'timeline-item',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')).toHaveAttribute(
      'id',
      'timeline-item',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')).toHaveStyle(slotProbeStyle);
    expect(screen.getByText('Submitted request').closest('[data-sdk-region="timeline-item-content"]')).toHaveClass(
      'timeline-content-probe',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-region="timeline-item-content"]')).toHaveAttribute(
      'data-slot-probe',
      'timeline-content',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')?.querySelector('[data-sdk-region="timeline-item-indicator"]')).toHaveClass(
      'timeline-indicator-probe',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')?.querySelector('[data-sdk-region="timeline-item-indicator"]')).toHaveAttribute(
      'data-slot-probe',
      'timeline-indicator',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')?.querySelector('[data-sdk-region="timeline-item-connector"]')).toHaveClass(
      'timeline-connector-probe',
    );
    expect(screen.getByText('Submitted request').closest('[data-sdk-ui="timeline-item"]')?.querySelector('[data-sdk-region="timeline-item-connector"]')).toHaveAttribute(
      'data-slot-probe',
      'timeline-connector',
    );
  });

  it('applies collection and item slot props to ActivityFeed surfaces', () => {
    render(
      <ActivityFeed
        getItemProps={(item) =>
          item.id === 'release'
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'activity-feed-item',
                id: 'activity-feed-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={(item) =>
          item.id === 'approval'
            ? {
                actions: {
                  className: 'activity-feed-actions-probe',
                  'data-slot-probe': 'activity-feed-actions',
                },
                connector: {
                  className: 'activity-feed-connector-probe',
                  'data-slot-probe': 'activity-feed-connector',
                },
                indicator: {
                  className: 'activity-feed-indicator-probe',
                  'data-slot-probe': 'activity-feed-indicator',
                },
                panel: {
                  className: 'activity-feed-panel-probe',
                  'data-slot-probe': 'activity-feed-panel',
                },
              }
            : undefined
        }
        headerActions={<button type="button">Filter</button>}
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
            description: 'A manual approval is required before promoting this build to production.',
            id: 'approval',
            timestamp: '2026-04-01 11:00',
            title: 'Approval requested',
            tone: 'warning',
          },
          {
            id: 'published',
            timestamp: '2026-04-01 12:30',
            title: 'Published',
            tone: 'success',
          },
        ]}
        slotProps={{
          header: {
            className: 'activity-feed-header-probe',
            'data-slot-probe': 'activity-feed-header',
          },
          headerActions: {
            className: 'activity-feed-header-actions-probe',
            'data-slot-probe': 'activity-feed-header-actions',
          },
          list: {
            className: 'activity-feed-list-probe',
            'data-slot-probe': 'activity-feed-list',
          },
        }}
        title="Activity"
      />,
    );

    expect(queryRegion('activity-feed-header')).toHaveClass('activity-feed-header-probe');
    expect(queryRegion('activity-feed-header')).toHaveAttribute('data-slot-probe', 'activity-feed-header');
    expect(queryRegion('activity-feed-header-actions')).toHaveClass('activity-feed-header-actions-probe');
    expect(queryRegion('activity-feed-header-actions')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-header-actions',
    );
    expect(queryRegion('activity-feed-list')).toHaveClass('activity-feed-list-probe');
    expect(queryRegion('activity-feed-list')).toHaveAttribute('data-slot-probe', 'activity-feed-list');
    expect(screen.getByText('Build completed').closest('[data-sdk-ui="activity-feed-item"]')).toHaveClass(slotProbeClassName);
    expect(screen.getByText('Build completed').closest('[data-sdk-ui="activity-feed-item"]')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-item',
    );
    expect(screen.getByText('Build completed').closest('[data-sdk-ui="activity-feed-item"]')).toHaveAttribute(
      'id',
      'activity-feed-item',
    );
    expect(screen.getByText('Build completed').closest('[data-sdk-ui="activity-feed-item"]')).toHaveStyle(slotProbeStyle);
    expect(screen.getByText('Approval requested').closest('[data-sdk-region="activity-feed-item-panel"]')).toHaveClass(
      'activity-feed-panel-probe',
    );
    expect(screen.getByText('Approval requested').closest('[data-sdk-region="activity-feed-item-panel"]')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-panel',
    );
    expect(screen.getByText('Approval requested').closest('[data-sdk-ui="activity-feed-item"]')?.querySelector('[data-sdk-region="activity-feed-item-indicator"]')).toHaveClass(
      'activity-feed-indicator-probe',
    );
    expect(screen.getByText('Approval requested').closest('[data-sdk-ui="activity-feed-item"]')?.querySelector('[data-sdk-region="activity-feed-item-indicator"]')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-indicator',
    );
    expect(screen.getByText('Approval requested').closest('[data-sdk-ui="activity-feed-item"]')?.querySelector('[data-sdk-region="activity-feed-item-connector"]')).toHaveClass(
      'activity-feed-connector-probe',
    );
    expect(screen.getByText('Approval requested').closest('[data-sdk-ui="activity-feed-item"]')?.querySelector('[data-sdk-region="activity-feed-item-connector"]')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-connector',
    );
    expect(screen.getByRole('button', { name: 'Open review' }).closest('[data-sdk-region="activity-feed-item-actions"]')).toHaveClass(
      'activity-feed-actions-probe',
    );
    expect(screen.getByRole('button', { name: 'Open review' }).closest('[data-sdk-region="activity-feed-item-actions"]')).toHaveAttribute(
      'data-slot-probe',
      'activity-feed-actions',
    );
  });

  it('applies collection and item slot props to NotificationCenter surfaces', () => {
    render(
      <NotificationCenter
        clearLabel="Clear all notifications"
        getItemProps={(item) =>
          item.id === 'review'
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'notification-center-item',
                id: 'notification-center-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={(item) =>
          item.id === 'review'
            ? {
                action: {
                  className: 'notification-center-action-probe',
                  'data-slot-probe': 'notification-center-action',
                },
                body: {
                  className: 'notification-center-body-probe',
                  'data-slot-probe': 'notification-center-body',
                },
                description: {
                  className: 'notification-center-description-probe',
                  'data-slot-probe': 'notification-center-description',
                },
                icon: {
                  className: 'notification-center-icon-probe',
                  'data-slot-probe': 'notification-center-icon',
                },
                timeLabel: {
                  className: 'notification-center-time-probe',
                  'data-slot-probe': 'notification-center-time',
                },
                title: {
                  className: 'notification-center-title-probe',
                  'data-slot-probe': 'notification-center-title',
                },
                unreadIndicator: {
                  className: 'notification-center-unread-probe',
                  'data-slot-probe': 'notification-center-unread',
                },
              }
            : undefined
        }
        items={[
          {
            content: 'A manual review is required before release.',
            id: 'review',
            timeLabel: '5m ago',
            title: 'Review requested',
            tone: 'warning',
            unread: true,
            actionHref: 'https://example.com/review',
            actionLabel: 'View review',
          },
        ]}
        onClear={() => undefined}
        onMarkAllAsRead={() => undefined}
        onItemSelect={() => undefined}
        slotProps={{
          header: {
            className: 'notification-center-header-probe',
            'data-slot-probe': 'notification-center-header',
          },
          headerActions: {
            className: 'notification-center-header-actions-probe',
            'data-slot-probe': 'notification-center-header-actions',
          },
          list: {
            className: 'notification-center-list-probe',
            'data-slot-probe': 'notification-center-list',
          },
          viewport: {
            className: 'notification-center-viewport-probe',
            'data-slot-probe': 'notification-center-viewport',
          },
        }}
      />,
    );

    expect(queryRegion('notification-center-header')).toHaveClass('notification-center-header-probe');
    expect(queryRegion('notification-center-header')).toHaveAttribute('data-slot-probe', 'notification-center-header');
    expect(queryRegion('notification-center-header-actions')).toHaveClass('notification-center-header-actions-probe');
    expect(queryRegion('notification-center-header-actions')).toHaveAttribute(
      'data-slot-probe',
      'notification-center-header-actions',
    );
    expect(queryRegion('notification-center-viewport')).toHaveClass('notification-center-viewport-probe');
    expect(queryRegion('notification-center-viewport')).toHaveAttribute(
      'data-slot-probe',
      'notification-center-viewport',
    );
    expect(queryRegion('notification-center-list')).toHaveClass('notification-center-list-probe');
    expect(queryRegion('notification-center-list')).toHaveAttribute('data-slot-probe', 'notification-center-list');
    expect(screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')).toHaveClass(
      slotProbeClassName,
    );
    expect(screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')).toHaveAttribute(
      'data-slot-probe',
      'notification-center-item',
    );
    expect(screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')).toHaveAttribute(
      'id',
      'notification-center-item',
    );
    expect(screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')).toHaveStyle(
      slotProbeStyle,
    );
    expect(screen.getByText('Review requested').closest('[data-sdk-region="notification-center-item-title"]')).toHaveClass(
      'notification-center-title-probe',
    );
    expect(screen.getByText('Review requested').closest('[data-sdk-region="notification-center-item-title"]')).toHaveAttribute(
      'data-slot-probe',
      'notification-center-title',
    );
    expect(
      screen
        .getByText('A manual review is required before release.')
        .closest('[data-sdk-region="notification-center-item-description"]'),
    ).toHaveClass('notification-center-description-probe');
    expect(
      screen
        .getByText('A manual review is required before release.')
        .closest('[data-sdk-region="notification-center-item-description"]'),
    ).toHaveAttribute('data-slot-probe', 'notification-center-description');
    expect(screen.getByText('5m ago').closest('[data-sdk-region="notification-center-item-time-label"]')).toHaveClass(
      'notification-center-time-probe',
    );
    expect(
      screen.getByText('5m ago').closest('[data-sdk-region="notification-center-item-time-label"]'),
    ).toHaveAttribute('data-slot-probe', 'notification-center-time');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')?.querySelector(
        '[data-sdk-region="notification-center-item-icon"]',
      ),
    ).toHaveClass('notification-center-icon-probe');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')?.querySelector(
        '[data-sdk-region="notification-center-item-icon"]',
      ),
    ).toHaveAttribute('data-slot-probe', 'notification-center-icon');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-region="notification-center-item-body"]'),
    ).toHaveClass('notification-center-body-probe');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-region="notification-center-item-body"]'),
    ).toHaveAttribute('data-slot-probe', 'notification-center-body');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')?.querySelector(
        '[data-sdk-region="notification-center-item-unread-indicator"]',
      ),
    ).toHaveClass('notification-center-unread-probe');
    expect(
      screen.getByText('Review requested').closest('[data-sdk-ui="notification-center-item"]')?.querySelector(
        '[data-sdk-region="notification-center-item-unread-indicator"]',
      ),
    ).toHaveAttribute('data-slot-probe', 'notification-center-unread');
    expect(screen.getByRole('link', { name: 'View review' })).toHaveClass('notification-center-action-probe');
    expect(screen.getByRole('link', { name: 'View review' })).toHaveAttribute(
      'data-slot-probe',
      'notification-center-action',
    );
  });

  it('applies collection-wide and per-item slot props to Stepper exact item surfaces', () => {
    render(
      <Stepper
        aria-label="Publish workflow"
        getItemProps={({ index }) =>
          index === 1
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'stepper-item',
                id: 'stepper-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={({ index }) =>
          index === 1
            ? {
                indicator: {
                  className: 'stepper-indicator-probe',
                  'data-slot-probe': 'stepper-indicator',
                },
              }
            : undefined
        }
        orientation="vertical"
      >
        <StepperItem status="complete" title="Configure" />
        <StepperItem
          description="Validate assets and release notes."
          slotProps={{
            connector: {
              className: 'stepper-connector-probe',
              'data-slot-probe': 'stepper-connector',
            },
            content: {
              className: 'stepper-content-probe',
              'data-slot-probe': 'stepper-content',
            },
          }}
          status="current"
          title="Review"
        />
        <StepperItem status="upcoming" title="Publish" />
      </Stepper>,
    );

    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')).toHaveClass(slotProbeClassName);
    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')).toHaveAttribute(
      'data-slot-probe',
      'stepper-item',
    );
    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')).toHaveAttribute('id', 'stepper-item');
    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')).toHaveStyle(slotProbeStyle);
    expect(screen.getByText('2').closest('[data-sdk-region="stepper-item-indicator"]')).toHaveClass(
      'stepper-indicator-probe',
    );
    expect(screen.getByText('2').closest('[data-sdk-region="stepper-item-indicator"]')).toHaveAttribute(
      'data-slot-probe',
      'stepper-indicator',
    );
    expect(screen.getByText('Review').closest('[data-sdk-region="stepper-item-content"]')).toHaveClass(
      'stepper-content-probe',
    );
    expect(screen.getByText('Review').closest('[data-sdk-region="stepper-item-content"]')).toHaveAttribute(
      'data-slot-probe',
      'stepper-content',
    );
    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')?.querySelector('[data-sdk-region="stepper-item-connector"]')).toHaveClass(
      'stepper-connector-probe',
    );
    expect(screen.getByText('Review').closest('[data-sdk-ui="stepper-item"]')?.querySelector('[data-sdk-region="stepper-item-connector"]')).toHaveAttribute(
      'data-slot-probe',
      'stepper-connector',
    );
  });

  it('applies collection-wide and per-item slot props to WorkspaceTabs exact item surfaces', () => {
    render(
      <WorkspaceTabs
        actions={<button type="button">Open palette</button>}
        getItemProps={({ item }) =>
          item.id === 'notes'
            ? {
                className: slotProbeClassName,
                'data-slot-probe': 'workspace-tab-item',
                id: 'workspace-tab-item',
                style: slotProbeStyle,
              }
            : undefined
        }
        getItemSlotProps={({ item }) =>
          item.id === 'notes'
            ? {
                closeButton: {
                  className: 'workspace-tab-close-probe',
                  'data-slot-probe': 'workspace-tab-close',
                },
                label: {
                  className: 'workspace-tab-label-probe',
                  'data-slot-probe': 'workspace-tab-label',
                },
                meta: {
                  className: 'workspace-tab-meta-probe',
                  'data-slot-probe': 'workspace-tab-meta',
                },
                modifiedIndicator: {
                  className: 'workspace-tab-modified-probe',
                  'data-slot-probe': 'workspace-tab-modified',
                },
                trigger: {
                  className: 'workspace-tab-trigger-probe',
                  'data-slot-probe': 'workspace-tab-trigger',
                },
              }
            : undefined
        }
        items={[
          { id: 'notes', label: 'Notes.md', meta: 'Draft', modified: true },
          { id: 'memory', label: 'Memory.md' },
        ]}
        onCloseTab={() => undefined}
        onValueChange={() => undefined}
        slotProps={{
          actions: {
            className: 'workspace-tabs-actions-probe',
            'data-slot-probe': 'workspace-tabs-actions',
          },
          tabList: {
            className: 'workspace-tabs-list-probe',
            'data-slot-probe': 'workspace-tabs-list',
          },
        }}
        value="notes"
      />,
    );

    expect(queryRegion('workspace-tabs-tablist')).toHaveClass('workspace-tabs-list-probe');
    expect(queryRegion('workspace-tabs-tablist')).toHaveAttribute('data-slot-probe', 'workspace-tabs-list');
    expect(queryRegion('workspace-tabs-actions')).toHaveClass('workspace-tabs-actions-probe');
    expect(queryRegion('workspace-tabs-actions')).toHaveAttribute('data-slot-probe', 'workspace-tabs-actions');
    expect(screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')).toHaveClass(slotProbeClassName);
    expect(screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')).toHaveAttribute(
      'data-slot-probe',
      'workspace-tab-item',
    );
    expect(screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')).toHaveAttribute(
      'id',
      'workspace-tab-item',
    );
    expect(screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')).toHaveStyle(slotProbeStyle);
    expect(screen.getByRole('tab', { name: /notes\.md/i })).toHaveClass('workspace-tab-trigger-probe');
    expect(screen.getByRole('tab', { name: /notes\.md/i })).toHaveAttribute(
      'data-slot-probe',
      'workspace-tab-trigger',
    );
    expect(screen.getByText('Notes.md').closest('[data-sdk-region="workspace-tab-label"]')).toHaveClass(
      'workspace-tab-label-probe',
    );
    expect(screen.getByText('Notes.md').closest('[data-sdk-region="workspace-tab-label"]')).toHaveAttribute(
      'data-slot-probe',
      'workspace-tab-label',
    );
    expect(screen.getByText('Draft').closest('[data-sdk-region="workspace-tab-meta"]')).toHaveClass(
      'workspace-tab-meta-probe',
    );
    expect(screen.getByText('Draft').closest('[data-sdk-region="workspace-tab-meta"]')).toHaveAttribute(
      'data-slot-probe',
      'workspace-tab-meta',
    );
    expect(
      screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')?.querySelector(
        '[data-sdk-region="workspace-tab-modified-indicator"]',
      ),
    ).toHaveClass('workspace-tab-modified-probe');
    expect(
      screen.getByText('Notes.md').closest('[data-sdk-ui="workspace-tab"]')?.querySelector(
        '[data-sdk-region="workspace-tab-modified-indicator"]',
      ),
    ).toHaveAttribute('data-slot-probe', 'workspace-tab-modified');
    expect(screen.getByRole('button', { name: 'Close Notes.md' })).toHaveClass('workspace-tab-close-probe');
    expect(screen.getByRole('button', { name: 'Close Notes.md' })).toHaveAttribute(
      'data-slot-probe',
      'workspace-tab-close',
    );
  });
});
