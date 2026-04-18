import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import {
  Controller,
  FormProvider,
  useFormContext,
  type ControllerProps,
  type FieldPath,
  type FieldValues,
} from 'react-hook-form';
import { cn } from '../../../lib/utils';
import {
  Field,
  FieldDescription,
  FieldError,
  FieldLabel,
} from '../field';

const Form = FormProvider as typeof FormProvider & { displayName?: string };
export type FormProps = React.ComponentProps<typeof FormProvider>;

type FormFieldContextValue<
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> = {
  name: TName;
};

const FormFieldContext = React.createContext<FormFieldContextValue | undefined>(undefined);

type FormItemContextValue = {
  id: string;
};

const FormItemContext = React.createContext<FormItemContextValue | undefined>(undefined);

export type FormFieldProps<
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
> = ControllerProps<TFieldValues, TName>;

type FormFieldComponent = <
  TFieldValues extends FieldValues = FieldValues,
  TName extends FieldPath<TFieldValues> = FieldPath<TFieldValues>,
>(props: FormFieldProps<TFieldValues, TName>) => React.JSX.Element;

const FormField: FormFieldComponent & { displayName?: string } = (props) => (
  <FormFieldContext.Provider value={{ name: props.name }}>
    <Controller {...props} />
  </FormFieldContext.Provider>
);

function useFormField() {
  const fieldContext = React.useContext(FormFieldContext);
  const itemContext = React.useContext(FormItemContext);
  const { getFieldState, formState } = useFormContext();

  if (!fieldContext) {
    throw new Error('useFormField must be used within <FormField>.');
  }

  if (!itemContext) {
    throw new Error('useFormField must be used within <FormItem>.');
  }

  const fieldState = getFieldState(fieldContext.name, formState);
  const { id } = itemContext;

  return {
    id,
    name: fieldContext.name,
    formDescriptionId: `${id}-description`,
    formItemId: `${id}-item`,
    formMessageId: `${id}-message`,
    ...fieldState,
  };
}

export type FormItemProps = React.HTMLAttributes<HTMLDivElement>;
export type FormLabelProps = React.ComponentPropsWithoutRef<typeof FieldLabel>;
export type FormControlProps = React.ComponentPropsWithoutRef<typeof Slot>;
export type FormDescriptionProps = React.HTMLAttributes<HTMLParagraphElement>;
export type FormMessageProps = React.HTMLAttributes<HTMLParagraphElement>;

const FormItem = React.forwardRef<HTMLDivElement, FormItemProps>(
  ({ children, className, ...props }, ref) => {
    const id = React.useId();
    const fieldContext = React.useContext(FormFieldContext);
    const { getFieldState, formState } = useFormContext();
    const error = fieldContext ? getFieldState(fieldContext.name, formState).error : undefined;

    return (
      <FormItemContext.Provider value={{ id }}>
        <div
          ref={ref}
          className={cn(className)}
          data-sdk-ui="form-item"
          {...props}
        >
          <Field invalid={!!error}>
            {children}
          </Field>
        </div>
      </FormItemContext.Provider>
    );
  },
);

FormItem.displayName = 'FormItem';

const FormLabel = React.forwardRef<
  React.ElementRef<typeof FieldLabel>,
  FormLabelProps
>(({ className, ...props }, ref) => {
  const { error, formItemId } = useFormField();

  return (
    <FieldLabel
      ref={ref}
      className={cn(error ? 'text-[var(--sdk-color-state-danger)]' : undefined, className)}
      data-sdk-ui="form-label"
      htmlFor={formItemId}
      {...props}
    />
  );
});

FormLabel.displayName = 'FormLabel';

const FormControl = React.forwardRef<
  React.ElementRef<typeof Slot>,
  FormControlProps
>(({ ...props }, ref) => {
  const { error, formDescriptionId, formItemId, formMessageId } = useFormField();

  return (
    <Slot
      ref={ref}
      aria-describedby={error ? `${formDescriptionId} ${formMessageId}` : formDescriptionId}
      aria-invalid={error ? 'true' : 'false'}
      data-sdk-ui="form-control"
      data-slot="form-control"
      id={formItemId}
      {...props}
    />
  );
});

FormControl.displayName = 'FormControl';

const FormDescription = React.forwardRef<HTMLParagraphElement, FormDescriptionProps>(
  ({ className, ...props }, ref) => {
    const { formDescriptionId } = useFormField();

    return (
      <FieldDescription
        ref={ref}
        className={className}
        data-sdk-ui="form-description"
        id={formDescriptionId}
        {...props}
      />
    );
  },
);

FormDescription.displayName = 'FormDescription';

const FormMessage = React.forwardRef<HTMLParagraphElement, FormMessageProps>(
  ({ children, className, ...props }, ref) => {
    const { error, formMessageId } = useFormField();
    const message = error?.message ? String(error.message) : children;

    if (!message) {
      return null;
    }

    return (
      <FieldError
        ref={ref}
        className={className}
        data-sdk-ui="form-message"
        id={formMessageId}
        {...props}
      >
        {message}
      </FieldError>
    );
  },
);

FormMessage.displayName = 'FormMessage';

export {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
  useFormField,
};
Form.displayName = 'Form';
FormField.displayName = 'FormField';
