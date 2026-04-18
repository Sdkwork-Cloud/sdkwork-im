import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { StatusBar, StatusBarItem, StatusBarSection } from './index';

describe('StatusBar', () => {
  it('renders desktop status sections and structured status items', () => {
    render(
      <StatusBar>
        <StatusBarSection>
          <StatusBarItem label="Branch" value="main" />
          <StatusBarItem>Ready</StatusBarItem>
        </StatusBarSection>
        <StatusBarSection align="end">
          <StatusBarItem label="Sync" value="Live" />
        </StatusBarSection>
      </StatusBar>,
    );

    expect(screen.getByRole('status')).toBeInTheDocument();
    expect(screen.getByText('Branch')).toBeInTheDocument();
    expect(screen.getByText('main')).toBeInTheDocument();
    expect(screen.getByText('Ready')).toBeInTheDocument();
    expect(screen.getByText('Sync')).toBeInTheDocument();
    expect(screen.getByText('Live')).toBeInTheDocument();
  });
});
