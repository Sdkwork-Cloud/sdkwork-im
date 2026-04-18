import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { LoadingBlock } from './feedback';

describe('pattern feedback helpers', () => {
  it('uses an ASCII loading label by default', () => {
    render(<LoadingBlock />);

    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
});
