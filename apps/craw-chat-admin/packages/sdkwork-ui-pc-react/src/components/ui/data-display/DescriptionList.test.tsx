import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { DescriptionDetails, DescriptionItem, DescriptionList, DescriptionTerm } from './index';

describe('DescriptionList', () => {
  it('renders structured key-value information for desktop detail panels', () => {
    const { container } = render(
      <DescriptionList columns={2}>
        <DescriptionItem>
          <DescriptionTerm>Request ID</DescriptionTerm>
          <DescriptionDetails mono>req_123456</DescriptionDetails>
        </DescriptionItem>
        <DescriptionItem>
          <DescriptionTerm>Status</DescriptionTerm>
          <DescriptionDetails>Completed</DescriptionDetails>
        </DescriptionItem>
      </DescriptionList>,
    );

    expect(screen.getByText('Request ID')).toBeInTheDocument();
    expect(screen.getByText('req_123456')).toHaveClass('font-mono');
    expect(screen.getByText('Status')).toBeInTheDocument();
    expect(screen.getByText('Completed')).toBeInTheDocument();
    expect(container.querySelector('dl[data-sdk-ui="description-list"]')).toBeInTheDocument();
    expect(container.querySelector('[data-sdk-ui="description-item"]')).toBeInTheDocument();
  });
});
