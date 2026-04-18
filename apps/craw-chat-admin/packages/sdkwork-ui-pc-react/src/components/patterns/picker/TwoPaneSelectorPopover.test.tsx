import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { TwoPaneSelectorPopover } from './TwoPaneSelectorPopover';

interface ProviderSection {
  id: string;
  name: string;
}

interface ProviderModel {
  badge?: string;
  description?: string;
  id: string;
  name: string;
}

const sections: Array<{ items: ProviderModel[]; section: ProviderSection }> = [
  {
    section: { id: 'openai', name: 'OpenAI' },
    items: [
      { id: 'gpt-4.1', name: 'GPT-4.1', description: 'General purpose reasoning model.', badge: 'Default' },
      { id: 'o4-mini', name: 'o4-mini', description: 'Fast assistant model.' },
    ],
  },
  {
    section: { id: 'anthropic', name: 'Anthropic' },
    items: [{ id: 'claude-sonnet-4', name: 'Claude Sonnet 4', description: 'Balanced writing and coding model.' }],
  },
];

afterEach(() => {
  cleanup();
});

describe('TwoPaneSelectorPopover', () => {
  it('supports a controlled active section so apps can coordinate anchored pickers with external state', () => {
    render(
      <TwoPaneSelectorPopover
        activeSectionId="anthropic"
        defaultOpen
        getItemDescription={(item) => item.description}
        getSectionLabel={(section) => section.name}
        sections={sections}
        trigger={<button type="button">Choose model</button>}
        value="gpt-4.1"
      />,
    );

    expect(screen.getByText('Claude Sonnet 4')).toBeInTheDocument();
    expect(screen.queryByText('GPT-4.1')).not.toBeInTheDocument();
  });

  it('defaults the active section from the current value and lets users browse the second pane', () => {
    render(
      <TwoPaneSelectorPopover
        defaultOpen
        getItemDescription={(item) => item.description}
        getSectionLabel={(section) => section.name}
        sections={sections}
        trigger={<button type="button">Choose model</button>}
        value="gpt-4.1"
      />,
    );

    expect(screen.getByText('OpenAI')).toBeInTheDocument();
    expect(screen.getByText('Anthropic')).toBeInTheDocument();
    expect(document.querySelector('[data-sdk-pattern="anchored-picker-surface"]')).not.toBeNull();
    expect(screen.getByText('GPT-4.1')).toBeInTheDocument();
    expect(screen.queryByText('Claude Sonnet 4')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'OpenAI' })).toHaveAttribute(
      'data-slot',
      'two-pane-selector-section-trigger',
    );
    expect(screen.getByRole('button', { name: 'OpenAI' })).toHaveClass(
      'rounded-[var(--sdk-radius-field)]',
    );
    const gptOption = screen.getByText('GPT-4.1').closest('button');

    expect(gptOption).not.toBeNull();
    expect(gptOption).toHaveAttribute(
      'data-slot',
      'two-pane-selector-item-trigger',
    );
    expect(gptOption).toHaveClass(
      'rounded-[var(--sdk-radius-field)]',
    );

    fireEvent.click(screen.getByRole('button', { name: 'Anthropic' }));

    expect(screen.getByText('Claude Sonnet 4')).toBeInTheDocument();
    expect(screen.queryByText('GPT-4.1')).not.toBeInTheDocument();
  });

  it('emits the selected item and closes the popover when choosing an option', () => {
    const handleOpenChange = vi.fn();
    const handleValueChange = vi.fn();

    render(
      <TwoPaneSelectorPopover
        defaultOpen
        getSectionLabel={(section) => section.name}
        onOpenChange={handleOpenChange}
        onValueChange={handleValueChange}
        sections={sections}
        trigger={<button type="button">Choose model</button>}
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Anthropic' }));
    fireEvent.click(screen.getByRole('button', { name: 'Claude Sonnet 4' }));

    expect(handleValueChange).toHaveBeenCalledWith(
      'claude-sonnet-4',
      sections[1].items[0],
      sections[1].section,
    );
    expect(handleOpenChange).toHaveBeenCalledWith(false);
    expect(screen.queryByText('Claude Sonnet 4')).not.toBeInTheDocument();
  });

  it('renders a shared empty state when the active section has no items', () => {
    render(
      <TwoPaneSelectorPopover
        defaultActiveSectionId="empty"
        defaultOpen
        emptyDescription="Connect a provider or choose a different section."
        emptyTitle="No models available"
        getSectionId={(section) => section.id}
        getSectionLabel={(section) => section.name}
        sections={[
          { section: { id: 'empty', name: 'Local' }, items: [] },
          ...sections,
        ]}
        trigger={<button type="button">Choose model</button>}
      />,
    );

    expect(screen.getByText('No models available')).toBeInTheDocument();
    expect(screen.getByText('Connect a provider or choose a different section.')).toBeInTheDocument();
  });

  it('supports custom section and item rendering for branded selectors', () => {
    const handleValueChange = vi.fn();

    render(
      <TwoPaneSelectorPopover
        defaultOpen
        onValueChange={handleValueChange}
        renderItem={({ item, select, selected }) => (
          <button
            aria-pressed={selected}
            onClick={select}
            type="button"
          >
            Use {item.name}
          </button>
        )}
        renderSection={({ active, section, setActiveSection }) => (
          <button
            aria-pressed={active}
            onClick={setActiveSection}
            type="button"
          >
            {section.name}
          </button>
        )}
        sections={sections}
        trigger={<button type="button">Choose model</button>}
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Anthropic' }));
    fireEvent.click(screen.getByRole('button', { name: 'Use Claude Sonnet 4' }));

    expect(handleValueChange).toHaveBeenCalledWith(
      'claude-sonnet-4',
      sections[1].items[0],
      sections[1].section,
    );
  });
});
