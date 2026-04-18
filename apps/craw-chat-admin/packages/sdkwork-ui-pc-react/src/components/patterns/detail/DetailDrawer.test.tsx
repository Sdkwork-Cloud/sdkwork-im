import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import {
  DescriptionDetails,
  DescriptionItem,
  DescriptionList,
  DescriptionTerm,
} from '../../ui/data-display';
import {
  DetailDrawer,
  DetailDrawerMetric,
  DetailDrawerMetrics,
  DetailDrawerSection,
} from './DetailDrawer';

describe('DetailDrawer', () => {
  it('renders a reusable list-detail drawer shell with sections, metrics, and footer actions', () => {
    render(
      <DetailDrawer
        description="Shared inspection flow for desktop apps."
        footer={<button type="button">Close panel</button>}
        open
        summary={<div>Provider account is healthy and synced.</div>}
        title="Provider account"
      >
        <DetailDrawerMetrics>
          <DetailDrawerMetric helper="Last 30 days" label="Requests" value="12,430" />
          <DetailDrawerMetric label="Latency" tone="warning" value="280ms" />
        </DetailDrawerMetrics>
        <DetailDrawerSection description="Primary metadata shared across app detail drawers." title="Overview">
          <DescriptionList columns={2}>
            <DescriptionItem>
              <DescriptionTerm>Provider</DescriptionTerm>
              <DescriptionDetails>OpenAI</DescriptionDetails>
            </DescriptionItem>
            <DescriptionItem>
              <DescriptionTerm>Mode</DescriptionTerm>
              <DescriptionDetails>Production</DescriptionDetails>
            </DescriptionItem>
          </DescriptionList>
        </DetailDrawerSection>
      </DetailDrawer>,
    );

    expect(screen.getByText('Provider account')).toBeInTheDocument();
    expect(screen.getByText('Shared inspection flow for desktop apps.')).toBeInTheDocument();
    expect(screen.getByText('Provider account is healthy and synced.')).toBeInTheDocument();
    expect(screen.getByText('Requests')).toBeInTheDocument();
    expect(screen.getByText('12,430')).toBeInTheDocument();
    expect(screen.getByText('Overview')).toBeInTheDocument();
    expect(screen.getByText('Primary metadata shared across app detail drawers.')).toBeInTheDocument();
    expect(screen.getByText('OpenAI')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Close panel' })).toBeInTheDocument();
  });

  it('surfaces the shared drawer close affordance through the Radix open-state callback', () => {
    const onOpenChange = vi.fn();

    render(
      <DetailDrawer
        description="Reusable detail drawer close behavior."
        onOpenChange={onOpenChange}
        open
        title="Audit details"
      >
        <DetailDrawerSection title="Activity">
          <div>One item</div>
        </DetailDrawerSection>
      </DetailDrawer>,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Close' }));

    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it('does not emit a Radix description warning when the optional description is omitted', () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const errorSpy = vi.spyOn(console, 'error').mockImplementation(() => undefined);

    render(
      <DetailDrawer
        open
        title="Audit details"
      >
        <DetailDrawerSection title="Activity">
          <div>One item</div>
        </DetailDrawerSection>
      </DetailDrawer>,
    );

    const consoleOutput = [...warnSpy.mock.calls.flat(), ...errorSpy.mock.calls.flat()].join(' ');

    expect(consoleOutput).not.toContain('Missing `Description`');

    warnSpy.mockRestore();
    errorSpy.mockRestore();
  });
});
