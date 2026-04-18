import * as React from 'react';
import { cn } from '../../lib/utils';
import { Label, type LabelProps } from './label';

export type FieldOrientation = 'horizontal' | 'vertical';

interface FieldContextValue {
  disabled: boolean;
  invalid: boolean;
  required: boolean;
}

const FieldContext = React.createContext<FieldContextValue>({
  disabled: false,
  invalid: false,
  required: false,
});

export interface FieldProps extends React.HTMLAttributes<HTMLDivElement> {
  disabled?: boolean;
  invalid?: boolean;
  orientation?: FieldOrientation;
  required?: boolean;
}

export interface FieldLabelProps extends LabelProps {}

export interface FieldDescriptionProps extends React.HTMLAttributes<HTMLParagraphElement> {}

export interface FieldErrorProps extends React.HTMLAttributes<HTMLParagraphElement> {}

export interface FieldGroupProps extends React.HTMLAttributes<HTMLDivElement> {}

const Field = React.forwardRef<HTMLDivElement, FieldProps>(
  (
    {
      className,
      disabled = false,
      invalid = false,
      orientation = 'vertical',
      required = false,
      ...props
    },
    ref,
  ) => {
    return (
      <FieldContext.Provider value={{ disabled, invalid, required }}>
        <div
          ref={ref}
          className={cn(
            orientation === 'horizontal' ? 'flex items-start gap-4' : 'grid gap-2',
            className,
          )}
          data-disabled={disabled ? 'true' : 'false'}
          data-invalid={invalid ? 'true' : 'false'}
          data-orientation={orientation}
          data-required={required ? 'true' : 'false'}
          data-sdk-ui="field"
          data-slot="field"
          {...props}
        />
      </FieldContext.Provider>
    );
  },
);

const FieldLabel = React.forwardRef<React.ElementRef<typeof Label>, FieldLabelProps>(
  ({ children, className, ...props }, ref) => {
    const { disabled, invalid, required } = React.useContext(FieldContext);

    return (
      <Label
        ref={ref}
        className={cn(
          invalid ? 'text-[var(--sdk-color-state-danger)]' : undefined,
          disabled ? 'opacity-60' : undefined,
          className,
        )}
        data-disabled={disabled ? 'true' : 'false'}
        data-invalid={invalid ? 'true' : 'false'}
        data-required={required ? 'true' : 'false'}
        data-sdk-ui="field-label"
        data-slot="field-label"
        {...props}
      >
        <span className="inline-flex items-center gap-1.5">
          <span>{children}</span>
          {required ? (
            <span
              aria-hidden="true"
              className="text-[var(--sdk-color-state-danger)]"
            >
              *
            </span>
          ) : null}
        </span>
      </Label>
    );
  },
);

const FieldDescription = React.forwardRef<HTMLParagraphElement, FieldDescriptionProps>(
  ({ className, ...props }, ref) => {
    const { disabled } = React.useContext(FieldContext);

    return (
      <p
        ref={ref}
        className={cn(
          'text-sm text-[var(--sdk-color-text-secondary)]',
          disabled ? 'opacity-60' : undefined,
          className,
        )}
        data-disabled={disabled ? 'true' : 'false'}
        data-sdk-ui="field-description"
        data-slot="field-description"
        {...props}
      />
    );
  },
);

const FieldError = React.forwardRef<HTMLParagraphElement, FieldErrorProps>(
  ({ className, ...props }, ref) => (
    <p
      ref={ref}
      className={cn('text-sm font-medium text-[var(--sdk-color-state-danger)]', className)}
      data-sdk-ui="field-error"
      data-slot="field-error"
      {...props}
    />
  ),
);

const FieldGroup = React.forwardRef<HTMLDivElement, FieldGroupProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('grid gap-2', className)}
      data-sdk-ui="field-group"
      data-slot="field-group"
      {...props}
    />
  ),
);

Field.displayName = 'Field';
FieldDescription.displayName = 'FieldDescription';
FieldError.displayName = 'FieldError';
FieldGroup.displayName = 'FieldGroup';
FieldLabel.displayName = 'FieldLabel';

export {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
};
