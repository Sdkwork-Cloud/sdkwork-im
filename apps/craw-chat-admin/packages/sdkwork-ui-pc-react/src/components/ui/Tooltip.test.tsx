import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from './tooltip';

describe('Tooltip', () => {
  it('renders shared tooltip content when opened', () => {
    render(
      <TooltipProvider>
        <Tooltip open>
          <TooltipTrigger>Hover</TooltipTrigger>
          <TooltipContent>Tooltip content</TooltipContent>
        </Tooltip>
      </TooltipProvider>,
    );

    expect(screen.getAllByText('Tooltip content')).not.toHaveLength(0);
    expect(screen.getByRole('tooltip')).toHaveTextContent('Tooltip content');
  });
});
