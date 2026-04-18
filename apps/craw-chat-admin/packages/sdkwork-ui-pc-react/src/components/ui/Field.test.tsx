import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel, Input } from './index';

describe('Field', () => {
  it('renders a standardized field shell with stable slot and state markers', () => {
    const { container } = render(
      <Field invalid required>
        <FieldLabel htmlFor="workspace-name">Workspace name</FieldLabel>
        <FieldGroup>
          <Input id="workspace-name" placeholder="SDKWORK Desktop" />
        </FieldGroup>
        <FieldDescription>Used in the desktop shell and project switchers.</FieldDescription>
        <FieldError>Workspace name is required.</FieldError>
      </Field>,
    );

    const field = container.querySelector('[data-sdk-ui="field"]');

    expect(field).toHaveAttribute('data-slot', 'field');
    expect(field).toHaveAttribute('data-invalid', 'true');
    expect(field).toHaveAttribute('data-required', 'true');
    expect(screen.getByText('Workspace name').closest('[data-slot="field-label"]')).toBeInTheDocument();
    expect(
      screen.getByText('Used in the desktop shell and project switchers.').closest('[data-slot="field-description"]'),
    ).toBeInTheDocument();
    expect(screen.getByText('Workspace name is required.').closest('[data-slot="field-error"]')).toBeInTheDocument();
  });
});
