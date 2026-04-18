import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './card';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Card', () => {
  it('publishes stable slot markers for card anatomy surfaces', () => {
    render(
      <Card>
        <CardHeader>
          <CardTitle>Workspace Summary</CardTitle>
          <CardDescription>Framework-owned summary shell.</CardDescription>
        </CardHeader>
        <CardContent>Card body</CardContent>
        <CardFooter>
          <button type="button">Open</button>
        </CardFooter>
      </Card>,
    );

    expect(queryUi('card')).toHaveAttribute('data-slot', 'card');
    expect(queryUi('card-header')).toHaveAttribute('data-slot', 'card-header');
    expect(screen.getByText('Workspace Summary').closest('[data-sdk-ui="card-title"]')).toHaveAttribute(
      'data-slot',
      'card-title',
    );
    expect(screen.getByText('Framework-owned summary shell.').closest('[data-sdk-ui="card-description"]')).toHaveAttribute(
      'data-slot',
      'card-description',
    );
    expect(queryUi('card-content')).toHaveAttribute('data-slot', 'card-content');
    expect(queryUi('card-footer')).toHaveAttribute('data-slot', 'card-footer');
  });
});
