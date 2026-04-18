import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { TagInput } from './index';

describe('TagInput', () => {
  it('adds and removes tags with desktop-friendly keyboard interactions', () => {
    const onValueChange = vi.fn();

    render(
      <TagInput
        aria-label="Tags"
        defaultValue={['design']}
        onValueChange={onValueChange}
        placeholder="Add tag"
        slotProps={{
          removeButton: {
            className: 'tag-input-remove-probe',
            'data-slot-probe': 'tag-input-remove',
          },
          tag: {
            className: 'tag-input-tag-probe',
            'data-slot-probe': 'tag-input-tag',
          },
        }}
      />,
    );

    const input = screen.getByPlaceholderText('Add tag');

    expect(document.body.querySelector('[data-slot="tag-input"]')).toBeInTheDocument();
    expect(input).toHaveAttribute('data-slot', 'tag-input-input');
    expect(screen.getByText('design').closest('[data-slot="tag-input-tag"]')).toHaveClass('tag-input-tag-probe');
    expect(screen.getByRole('button', { name: 'Remove design' })).toHaveAttribute('data-slot', 'tag-input-remove');
    expect(screen.getByRole('button', { name: 'Remove design' })).toHaveClass('tag-input-remove-probe');
    expect(screen.getByRole('button', { name: 'Remove design' })).toHaveAttribute(
      'data-slot-probe',
      'tag-input-remove',
    );

    fireEvent.change(input, { target: { value: 'react' } });
    fireEvent.keyDown(input, { key: 'Enter' });

    expect(onValueChange).toHaveBeenCalledWith(['design', 'react']);
    expect(screen.getByText('react')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Remove design' }));
    expect(onValueChange).toHaveBeenCalledWith(['react']);
  });
});
