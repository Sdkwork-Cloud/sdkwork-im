import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { NotificationCenter } from './index';

describe('NotificationCenter', () => {
  it('renders unread notifications and exposes panel actions', () => {
    const onClear = vi.fn();
    const onItemSelect = vi.fn();
    const onMarkAllAsRead = vi.fn();

    render(
      <NotificationCenter
        items={[
          {
            content: 'Workspace build completed successfully.',
            id: '1',
            timeLabel: 'just now',
            title: 'Build finished',
            tone: 'success',
            unread: true,
          },
        ]}
        onClear={onClear}
        onItemSelect={onItemSelect}
        onMarkAllAsRead={onMarkAllAsRead}
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Mark all as read' }));
    expect(onMarkAllAsRead).toHaveBeenCalledTimes(1);

    fireEvent.click(screen.getByText('Build finished'));
    expect(onItemSelect).toHaveBeenCalledWith('1');

    fireEvent.click(screen.getByRole('button', { name: 'Clear notifications' }));
    expect(onClear).toHaveBeenCalledTimes(1);
  });
});
