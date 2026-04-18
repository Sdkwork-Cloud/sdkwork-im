import { render, screen, within } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { MarkdownViewer } from './index';

describe('MarkdownViewer', () => {
  it('renders markdown headings and paragraph content', () => {
    render(<MarkdownViewer content={'# AI Console\n## Session Notes\nLatest response summary.'} />);

    expect(screen.getByRole('heading', { level: 1, name: 'AI Console' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { level: 2, name: 'Session Notes' })).toBeInTheDocument();
    expect(screen.getByText('Latest response summary.')).toBeInTheDocument();
  });

  it('renders markdown lists and fenced code blocks', () => {
    render(
      <MarkdownViewer
        content={'- First item\n- Second item\n\n```tsx\nconst answer = 42;\n```\n'}
      />,
    );

    const list = screen.getByRole('list');
    expect(within(list).getByText('First item')).toBeInTheDocument();
    expect(within(list).getByText('Second item')).toBeInTheDocument();
    expect(screen.getByText('const answer = 42;')).toBeInTheDocument();
    expect(screen.getByText('tsx')).toBeInTheDocument();
  });

  it('shows an empty markdown state when content is blank', () => {
    render(<MarkdownViewer content={'   '} />);

    expect(screen.getByText('No markdown content')).toBeInTheDocument();
  });
});
