import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { StatCard } from './stat-card';

describe('StatCard', () => {
  it('renders desktop metric summaries with label, value, and change text', () => {
    render(
      <StatCard
        change="+12.4%"
        description="Compared with last week"
        label="Active Agents"
        value="128"
      />,
    );

    expect(screen.getByText('Active Agents')).toBeInTheDocument();
    expect(screen.getByText('128')).toBeInTheDocument();
    expect(screen.getByText('+12.4%')).toBeInTheDocument();
    expect(screen.getByText('Compared with last week')).toBeInTheDocument();
  });
});
