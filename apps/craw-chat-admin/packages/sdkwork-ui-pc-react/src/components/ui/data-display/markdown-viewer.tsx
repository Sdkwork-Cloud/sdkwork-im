import * as React from 'react';
import { cn } from '../../../lib/utils';

interface MarkdownHeadingBlock {
  type: 'heading';
  level: 1 | 2 | 3 | 4 | 5 | 6;
  content: string;
}

interface MarkdownParagraphBlock {
  type: 'paragraph';
  content: string;
}

interface MarkdownListBlock {
  type: 'unordered-list' | 'ordered-list';
  items: string[];
}

interface MarkdownCodeBlock {
  type: 'code';
  language: string | null;
  content: string;
}

interface MarkdownQuoteBlock {
  type: 'quote';
  content: string;
}

interface MarkdownDividerBlock {
  type: 'divider';
}

type MarkdownBlock =
  | MarkdownHeadingBlock
  | MarkdownParagraphBlock
  | MarkdownListBlock
  | MarkdownCodeBlock
  | MarkdownQuoteBlock
  | MarkdownDividerBlock;

type MarkdownHeadingTag = 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';

export interface MarkdownViewerEmptyProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
}

const MarkdownViewerEmpty = React.forwardRef<HTMLDivElement, MarkdownViewerEmptyProps>(
  ({ children = 'No markdown content', className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('text-sm text-[var(--sdk-color-text-muted)]', className)}
      data-sdk-ui="markdown-viewer-empty"
      {...props}
    >
      {children}
    </div>
  ),
);

MarkdownViewerEmpty.displayName = 'MarkdownViewerEmpty';

export interface MarkdownViewerProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'content'> {
  content?: string | null;
  emptyContent?: React.ReactNode;
}

function parseMarkdownBlocks(content: string): MarkdownBlock[] {
  const lines = content.replace(/\r\n/g, '\n').split('\n');
  const blocks: MarkdownBlock[] = [];
  let index = 0;

  while (index < lines.length) {
    const line = lines[index];
    const trimmedLine = line.trim();

    if (!trimmedLine) {
      index += 1;
      continue;
    }

    if (trimmedLine.startsWith('```')) {
      const language = trimmedLine.slice(3).trim() || null;
      const codeLines: string[] = [];
      index += 1;

      while (index < lines.length && !lines[index].trim().startsWith('```')) {
        codeLines.push(lines[index]);
        index += 1;
      }

      if (index < lines.length && lines[index].trim().startsWith('```')) {
        index += 1;
      }

      blocks.push({
        type: 'code',
        language,
        content: codeLines.join('\n'),
      });
      continue;
    }

    const headingMatch = /^(#{1,6})\s+(.+)$/.exec(trimmedLine);
    if (headingMatch) {
      blocks.push({
        type: 'heading',
        level: headingMatch[1].length as 1 | 2 | 3 | 4 | 5 | 6,
        content: headingMatch[2].trim(),
      });
      index += 1;
      continue;
    }

    if (/^\s{0,3}([-*_])(\s*\1){2,}\s*$/.test(trimmedLine)) {
      blocks.push({ type: 'divider' });
      index += 1;
      continue;
    }

    if (/^>\s?/.test(trimmedLine)) {
      const quoteLines: string[] = [];
      while (index < lines.length && /^>\s?/.test(lines[index].trim())) {
        quoteLines.push(lines[index].trim().replace(/^>\s?/, ''));
        index += 1;
      }
      blocks.push({ type: 'quote', content: quoteLines.join(' ') });
      continue;
    }

    if (/^[-*+]\s+/.test(trimmedLine)) {
      const items: string[] = [];
      while (index < lines.length && /^[-*+]\s+/.test(lines[index].trim())) {
        items.push(lines[index].trim().replace(/^[-*+]\s+/, ''));
        index += 1;
      }
      blocks.push({ type: 'unordered-list', items });
      continue;
    }

    if (/^\d+\.\s+/.test(trimmedLine)) {
      const items: string[] = [];
      while (index < lines.length && /^\d+\.\s+/.test(lines[index].trim())) {
        items.push(lines[index].trim().replace(/^\d+\.\s+/, ''));
        index += 1;
      }
      blocks.push({ type: 'ordered-list', items });
      continue;
    }

    const paragraphLines: string[] = [];
    while (
      index < lines.length &&
      lines[index].trim() &&
      !lines[index].trim().startsWith('```') &&
      !/^(#{1,6})\s+/.test(lines[index].trim()) &&
      !/^[-*+]\s+/.test(lines[index].trim()) &&
      !/^\d+\.\s+/.test(lines[index].trim()) &&
      !/^>\s?/.test(lines[index].trim()) &&
      !/^\s{0,3}([-*_])(\s*\1){2,}\s*$/.test(lines[index].trim())
    ) {
      paragraphLines.push(lines[index].trim());
      index += 1;
    }

    blocks.push({ type: 'paragraph', content: paragraphLines.join(' ') });
  }

  return blocks;
}

function renderBlock(block: MarkdownBlock, key: string) {
  if (block.type === 'heading') {
    const HeadingTag = `h${block.level}` as MarkdownHeadingTag;
    const headingClassNameMap: Record<MarkdownHeadingBlock['level'], string> = {
      1: 'text-2xl',
      2: 'text-xl',
      3: 'text-lg',
      4: 'text-base',
      5: 'text-sm',
      6: 'text-sm',
    };

    return (
      <HeadingTag
        className={cn(
          'font-semibold text-[var(--sdk-color-text-primary)]',
          headingClassNameMap[block.level],
          block.level >= 5 && 'uppercase tracking-wide text-[var(--sdk-color-text-secondary)]',
        )}
        key={key}
      >
        {block.content}
      </HeadingTag>
    );
  }

  if (block.type === 'paragraph') {
    return (
      <p className="text-sm leading-6 text-[var(--sdk-color-text-secondary)]" key={key}>
        {block.content}
      </p>
    );
  }

  if (block.type === 'unordered-list' || block.type === 'ordered-list') {
    const ListTag = block.type === 'ordered-list' ? 'ol' : 'ul';
    return (
      <ListTag
        className={cn(
          'ml-5 space-y-1 text-sm leading-6 text-[var(--sdk-color-text-secondary)]',
          block.type === 'ordered-list' ? 'list-decimal' : 'list-disc',
        )}
        key={key}
      >
        {block.items.map((item, itemIndex) => (
          <li key={`${key}-item-${itemIndex}`}>{item}</li>
        ))}
      </ListTag>
    );
  }

  if (block.type === 'quote') {
    return (
      <blockquote
        className="border-l-2 border-[var(--sdk-color-border-default)] pl-3 text-sm italic leading-6 text-[var(--sdk-color-text-secondary)]"
        key={key}
      >
        {block.content}
      </blockquote>
    );
  }

  if (block.type === 'divider') {
    return <hr className="border-[var(--sdk-color-border-default)]" key={key} />;
  }

  return block.type === 'code' ? (
    <div
      className="overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)]"
      data-sdk-ui="markdown-viewer-code-block"
      key={key}
    >
      {block.language ? (
        <div className="border-b border-[var(--sdk-color-border-default)] px-3 py-1.5 text-xs font-medium uppercase tracking-wide text-[var(--sdk-color-text-muted)]">
          {block.language}
        </div>
      ) : null}
      <pre className="overflow-x-auto px-3 py-2 text-xs leading-5 text-[var(--sdk-color-text-primary)]">
        <code>{block.content}</code>
      </pre>
    </div>
  ) : null;
}

const MarkdownViewer = React.forwardRef<HTMLDivElement, MarkdownViewerProps>(
  ({ className, content, emptyContent, ...props }, ref) => {
    const normalizedContent = content?.trim() ?? '';
    const blocks = normalizedContent ? parseMarkdownBlocks(normalizedContent) : [];

    return (
      <div
        ref={ref}
        className={cn(
          'rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 shadow-[var(--sdk-shadow-soft)]',
          className,
        )}
        data-sdk-ui="markdown-viewer"
        {...props}
      >
        {blocks.length > 0 ? (
          <div className="space-y-3">{blocks.map((block, index) => renderBlock(block, `markdown-block-${index}`))}</div>
        ) : (
          <MarkdownViewerEmpty>{emptyContent}</MarkdownViewerEmpty>
        )}
      </div>
    );
  },
);

MarkdownViewer.displayName = 'MarkdownViewer';

export { MarkdownViewer, MarkdownViewerEmpty };
