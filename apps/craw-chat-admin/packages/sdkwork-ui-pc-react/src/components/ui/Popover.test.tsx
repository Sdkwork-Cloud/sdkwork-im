import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { Popover, PopoverAnchor, PopoverContent, PopoverTrigger } from './popover';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Popover', () => {
  it('renders shared popover content when opened', () => {
    render(
      <Popover defaultOpen>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent>Popover content</PopoverContent>
      </Popover>,
    );

    expect(screen.getByText('Popover content')).toBeInTheDocument();
  });

  it('publishes stable slot markers for popover anatomy surfaces', () => {
    render(
      <Popover defaultOpen>
        <PopoverAnchor>Anchor</PopoverAnchor>
        <PopoverTrigger>Open</PopoverTrigger>
        <PopoverContent forceMount>Popover content</PopoverContent>
      </Popover>,
    );

    expect(queryUi('popover-anchor')).toHaveAttribute('data-slot', 'popover-anchor');
    expect(queryUi('popover-trigger')).toHaveAttribute('data-slot', 'popover-trigger');
    expect(queryUi('popover-content')).toHaveAttribute('data-slot', 'popover-content');
  });
});
