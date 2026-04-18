import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Progress } from './progress';
import { Skeleton } from './skeleton';

describe('feedback primitives', () => {
  it('renders a semantic skeleton placeholder', () => {
    render(<Skeleton className="h-8 w-24" data-testid="skeleton" />);

    expect(screen.getByTestId('skeleton')).toHaveClass('animate-pulse');
  });

  it('renders progress value on the shared indicator', () => {
    render(<Progress value={72} />);

    expect(screen.getByRole('progressbar')).toHaveAttribute('aria-valuenow', '72');
    expect(screen.getByTestId('sdk-progress-indicator')).toHaveStyle('width: 72%');
  });
});
