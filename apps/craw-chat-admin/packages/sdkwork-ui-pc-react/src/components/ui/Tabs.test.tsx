import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './tabs';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Tabs', () => {
  it('renders the shared active tab panel from the provided default value', () => {
    render(
      <Tabs defaultValue="settings">
        <TabsList>
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>
        <TabsContent value="overview">Overview panel</TabsContent>
        <TabsContent value="settings">Settings panel</TabsContent>
      </Tabs>,
    );

    expect(screen.getByText('Settings panel')).toBeVisible();
    expect(screen.queryByText('Overview panel')).not.toBeInTheDocument();
  });

  it('publishes stable slot markers for tab anatomy surfaces', () => {
    render(
      <Tabs defaultValue="settings">
        <TabsList>
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>
        <TabsContent value="overview">Overview panel</TabsContent>
        <TabsContent value="settings">Settings panel</TabsContent>
      </Tabs>,
    );

    expect(queryUi('tabs')).toHaveAttribute('data-slot', 'tabs');
    expect(queryUi('tabs-list')).toHaveAttribute('data-slot', 'tabs-list');
    expect(screen.getByRole('tab', { name: 'Overview' })).toHaveAttribute('data-slot', 'tabs-trigger');
    expect(screen.getByText('Settings panel').closest('[data-sdk-ui="tabs-content"]')).toHaveAttribute(
      'data-slot',
      'tabs-content',
    );
  });
});
