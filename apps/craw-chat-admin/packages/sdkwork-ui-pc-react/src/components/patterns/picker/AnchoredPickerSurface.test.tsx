import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { AnchoredPickerSurface } from './AnchoredPickerSurface';

describe('AnchoredPickerSurface', () => {
  it('renders a reusable anchored picker shell with header, filters, footer, and body content', () => {
    render(
      <AnchoredPickerSurface
        actions={<button type="button">Refresh</button>}
        badge={<span>3 groups</span>}
        defaultOpen
        description="Choose a model family from the shared anchored picker shell."
        filters={<button type="button">OpenAI</button>}
        footer={<button type="button">Confirm</button>}
        resultsSummary="12 options"
        title="Choose model"
        trigger={<button type="button">Open picker</button>}
      >
        <div>Picker body</div>
      </AnchoredPickerSurface>,
    );

    expect(screen.getByText('Choose model')).toBeInTheDocument();
    expect(screen.getByText('3 groups')).toBeInTheDocument();
    expect(screen.getByText('12 options')).toBeInTheDocument();
    expect(
      screen.getByText('Choose a model family from the shared anchored picker shell.'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Refresh' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'OpenAI' })).toBeInTheDocument();
    expect(screen.getByText('Picker body')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Confirm' })).toBeInTheDocument();
  });

  it('renders loading, error, and empty states through the shared feedback layer', () => {
    const { rerender } = render(
      <AnchoredPickerSurface
        defaultOpen
        loading
        loadingLabel="Loading models..."
        title="Choose model"
        trigger={<button type="button">Open picker</button>}
      />,
    );

    expect(screen.getByText('Loading models...')).toBeInTheDocument();

    rerender(
      <AnchoredPickerSurface
        defaultOpen
        error="The provider list could not be loaded."
        errorTitle="Load failed"
        title="Choose model"
        trigger={<button type="button">Open picker</button>}
      />,
    );

    expect(screen.getByText('Load failed')).toBeInTheDocument();
    expect(screen.getByText('The provider list could not be loaded.')).toBeInTheDocument();

    rerender(
      <AnchoredPickerSurface
        defaultOpen
        empty
        emptyDescription="Try a different provider or connect a new account."
        emptyTitle="No models available"
        title="Choose model"
        trigger={<button type="button">Open picker</button>}
      />,
    );

    expect(screen.getByText('No models available')).toBeInTheDocument();
    expect(screen.getByText('Try a different provider or connect a new account.')).toBeInTheDocument();
  });

  it('supports non-scrollable custom bodies for composed anchored picker layouts', () => {
    render(
      <AnchoredPickerSurface
        bodyScrollable={false}
        defaultOpen
        maxHeight={360}
        slotProps={{
          body: {
            className: 'p-0',
          },
        }}
        title="Choose model"
        trigger={<button type="button">Open picker</button>}
      >
        <div
          className="h-[20rem]"
          data-testid="two-pane-body"
        >
          Two pane body
        </div>
      </AnchoredPickerSurface>,
    );

    const surface = document.querySelector('[data-sdk-pattern="anchored-picker-surface"]');
    const body = document.querySelector('[data-sdk-region="anchored-picker-body"]');

    expect(surface).toHaveStyle({ maxHeight: '360px' });
    expect(body).toHaveClass('p-0');
    expect(body).toHaveClass('overflow-hidden');
    expect(body).not.toHaveClass('overflow-y-auto');
    expect(screen.getByTestId('two-pane-body')).toBeInTheDocument();
  });
});
