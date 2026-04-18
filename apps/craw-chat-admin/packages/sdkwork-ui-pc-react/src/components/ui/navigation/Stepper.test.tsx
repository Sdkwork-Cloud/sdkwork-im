import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Stepper, StepperItem } from './index';

describe('Stepper', () => {
  it('renders desktop progress steps with status-aware states and automatic numbering', () => {
    render(
      <Stepper aria-label="Install steps">
        <StepperItem description="Prepare the runtime environment" status="complete" title="Prepare" />
        <StepperItem description="Authorize desktop permissions" status="current" title="Authorize" />
        <StepperItem description="Validate the final setup" title="Review" />
      </Stepper>,
    );

    const items = screen.getAllByRole('listitem');

    expect(screen.getByRole('list', { name: 'Install steps' })).toHaveAttribute('data-orientation', 'horizontal');
    expect(items).toHaveLength(3);
    expect(items[0]).toHaveAttribute('data-status', 'complete');
    expect(items[1]).toHaveAttribute('data-status', 'current');
    expect(items[2]).toHaveAttribute('data-status', 'upcoming');
    expect(screen.getByText('Prepare')).toBeInTheDocument();
    expect(screen.getByText('Authorize')).toBeInTheDocument();
    expect(screen.getByText('Review')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('supports vertical workflow layouts for wizard-style desktop forms', () => {
    render(
      <Stepper aria-label="Wizard steps" orientation="vertical">
        <StepperItem meta="Required" status="complete" title="Connect provider" />
        <StepperItem description="Paste the server URL and headers." status="current" title="Configure endpoint" />
      </Stepper>,
    );

    expect(screen.getByRole('list', { name: 'Wizard steps' })).toHaveAttribute('data-orientation', 'vertical');
    expect(screen.getByText('Required')).toBeInTheDocument();
    expect(screen.getByText('Paste the server URL and headers.')).toBeInTheDocument();
  });
});
