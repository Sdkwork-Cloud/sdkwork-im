import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Avatar, AvatarFallback } from './avatar';

describe('Avatar', () => {
  it('renders fallback content inside the shared avatar surface', () => {
    render(
      <Avatar>
        <AvatarFallback>SW</AvatarFallback>
      </Avatar>,
    );

    expect(screen.getByText('SW')).toBeInTheDocument();
  });
});
