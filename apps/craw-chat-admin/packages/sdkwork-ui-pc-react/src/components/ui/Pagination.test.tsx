import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from './pagination';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Pagination', () => {
  it('renders reusable desktop pagination controls with current-page semantics', () => {
    render(
      <Pagination>
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious href="/page/1" />
          </PaginationItem>
          <PaginationItem>
            <PaginationLink href="/page/1">1</PaginationLink>
          </PaginationItem>
          <PaginationItem>
            <PaginationLink href="/page/2" isActive>
              2
            </PaginationLink>
          </PaginationItem>
          <PaginationItem>
            <PaginationEllipsis />
          </PaginationItem>
          <PaginationItem>
            <PaginationNext href="/page/3" />
          </PaginationItem>
        </PaginationContent>
      </Pagination>,
    );

    expect(screen.getByRole('navigation', { name: 'pagination' })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Previous page' })).toHaveAttribute('href', '/page/1');
    expect(screen.getByRole('link', { name: 'Next page' })).toHaveAttribute('href', '/page/3');
    expect(screen.getByRole('link', { name: '2' })).toHaveAttribute('aria-current', 'page');
    expect(screen.getByRole('link', { name: '2' })).toHaveClass('rounded-[var(--sdk-radius-field)]');
    expect(screen.getByText('More pages')).toBeInTheDocument();
  });

  it('publishes stable slot markers for pagination anatomy surfaces', () => {
    render(
      <Pagination>
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious href="/page/1" />
          </PaginationItem>
          <PaginationItem>
            <PaginationLink href="/page/2" isActive>
              2
            </PaginationLink>
          </PaginationItem>
          <PaginationItem>
            <PaginationEllipsis />
          </PaginationItem>
          <PaginationItem>
            <PaginationNext href="/page/3" />
          </PaginationItem>
        </PaginationContent>
      </Pagination>,
    );

    expect(queryUi('pagination')).toHaveAttribute('data-slot', 'pagination');
    expect(queryUi('pagination-content')).toHaveAttribute('data-slot', 'pagination-content');
    expect(queryUi('pagination-item')).toHaveAttribute('data-slot', 'pagination-item');
    expect(screen.getByRole('link', { name: '2' })).toHaveAttribute('data-slot', 'pagination-link');
    expect(screen.getByRole('link', { name: 'Previous page' })).toHaveAttribute('data-slot', 'pagination-previous');
    expect(screen.getByRole('link', { name: 'Next page' })).toHaveAttribute('data-slot', 'pagination-next');
    expect(queryUi('pagination-ellipsis')).toHaveAttribute('data-slot', 'pagination-ellipsis');
  });
});
