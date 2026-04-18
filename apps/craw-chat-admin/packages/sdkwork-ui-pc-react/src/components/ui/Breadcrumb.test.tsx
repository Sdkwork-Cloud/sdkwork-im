import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  Breadcrumb,
  BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from './breadcrumb';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Breadcrumb', () => {
  it('renders desktop breadcrumb navigation with semantic current page output', () => {
    render(
      <Breadcrumb>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink href="/workspace">Workspace</BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbLink href="/workspace/settings">Settings</BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbPage>Desktop Shell</BreadcrumbPage>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>,
    );

    expect(screen.getByRole('navigation', { name: 'breadcrumb' })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Workspace' })).toHaveAttribute('href', '/workspace');
    expect(screen.getByRole('link', { name: 'Settings' })).toHaveAttribute('href', '/workspace/settings');
    expect(screen.getByText('Desktop Shell')).toHaveAttribute('aria-current', 'page');
  });

  it('publishes stable slot markers for breadcrumb anatomy surfaces', () => {
    render(
      <Breadcrumb>
        <BreadcrumbList>
          <BreadcrumbItem>
            <BreadcrumbLink href="/workspace">Workspace</BreadcrumbLink>
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbEllipsis />
          </BreadcrumbItem>
          <BreadcrumbSeparator />
          <BreadcrumbItem>
            <BreadcrumbPage>Desktop Shell</BreadcrumbPage>
          </BreadcrumbItem>
        </BreadcrumbList>
      </Breadcrumb>,
    );

    expect(queryUi('breadcrumb')).toHaveAttribute('data-slot', 'breadcrumb');
    expect(queryUi('breadcrumb-list')).toHaveAttribute('data-slot', 'breadcrumb-list');
    expect(queryUi('breadcrumb-item')).toHaveAttribute('data-slot', 'breadcrumb-item');
    expect(screen.getByRole('link', { name: 'Workspace' })).toHaveAttribute('data-slot', 'breadcrumb-link');
    expect(queryUi('breadcrumb-separator')).toHaveAttribute('data-slot', 'breadcrumb-separator');
    expect(screen.getByText('Desktop Shell')).toHaveAttribute('data-slot', 'breadcrumb-page');
    expect(queryUi('breadcrumb-ellipsis')).toHaveAttribute('data-slot', 'breadcrumb-ellipsis');
  });
});
