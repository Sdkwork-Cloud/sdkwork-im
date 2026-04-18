import * as React from 'react';
import { AlertCircle, RotateCcw } from 'lucide-react';
import { cn } from '../../../lib/utils';

export type SettingsFieldResetHandler = () => void;

export interface SettingsFieldProps extends React.HTMLAttributes<HTMLDivElement> {
  actions?: React.ReactNode;
  controlId?: string;
  description?: React.ReactNode;
  error?: React.ReactNode;
  isModified?: boolean;
  label?: React.ReactNode;
  layout?: 'horizontal' | 'vertical';
  onReset?: SettingsFieldResetHandler;
  resetLabel?: string;
}

const SettingsField = React.forwardRef<HTMLDivElement, SettingsFieldProps>(
  (
    {
      actions,
      children,
      className,
      controlId,
      description,
      error,
      isModified = false,
      label,
      layout = 'horizontal',
      onReset,
      resetLabel = 'Reset setting',
      ...props
    },
    ref,
  ) => {
    const descriptionId = React.useId();
    const errorId = React.useId();
    const isVertical = layout === 'vertical';

    return (
      <div
        ref={ref}
        className={cn(
          'group relative rounded-[var(--sdk-radius-control)] p-1.5 transition-colors',
          isVertical ? 'flex flex-col gap-3' : 'flex items-start gap-6',
          className,
        )}
        data-layout={layout}
        data-modified={isModified ? 'true' : 'false'}
        data-sdk-ui="settings-field"
        {...props}
      >
        <span
          aria-hidden="true"
          className={cn(
            'absolute bottom-2 left-0 top-2 w-[3px] rounded-r-full transition-opacity',
            isModified
              ? 'bg-[var(--sdk-color-brand-primary)] opacity-100'
              : 'bg-transparent opacity-0',
          )}
        />

        {label || description || error ? (
          <div className={cn('min-w-0', isVertical ? 'w-full' : 'max-w-[40%] flex-1 pt-2')}>
            {label ? (
              controlId ? (
                <label
                  className={cn(
                    'block text-sm font-medium leading-none',
                    error
                      ? 'text-[var(--sdk-color-state-danger)]'
                      : 'text-[var(--sdk-color-text-primary)]',
                  )}
                  htmlFor={controlId}
                >
                  {label}
                </label>
              ) : (
                <div
                  className={cn(
                    'text-sm font-medium leading-none',
                    error
                      ? 'text-[var(--sdk-color-state-danger)]'
                      : 'text-[var(--sdk-color-text-primary)]',
                  )}
                >
                  {label}
                </div>
              )
            ) : null}

            {description ? (
              <div
                className="mt-1.5 text-xs leading-relaxed text-[var(--sdk-color-text-secondary)]"
                id={descriptionId}
              >
                {description}
              </div>
            ) : null}

            {error ? (
              <div
                className="mt-2 flex items-center gap-1.5 text-xs font-medium text-[var(--sdk-color-state-danger)]"
                id={errorId}
                role="alert"
              >
                <AlertCircle className="h-3.5 w-3.5 shrink-0" />
                <span>{error}</span>
              </div>
            ) : null}
          </div>
        ) : null}

        <div
          className={cn(
            'flex min-w-0 items-start gap-2',
            isVertical ? 'w-full' : 'flex-1 justify-end',
          )}
        >
          <div
            aria-describedby={description ? descriptionId : undefined}
            aria-errormessage={error ? errorId : undefined}
            className={cn('min-w-0', isVertical ? 'w-full' : 'w-full max-w-[32rem]')}
          >
            {children}
          </div>

          {actions ? <div className="flex shrink-0 items-center gap-1 pt-1">{actions}</div> : null}

          {onReset ? (
            <button
              aria-label={resetLabel}
              className={cn(
                'shrink-0 rounded-full p-1 text-[var(--sdk-color-text-muted)] transition-colors',
                isModified
                  ? 'opacity-100 hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-brand-primary)]'
                  : 'pointer-events-none opacity-0',
              )}
              onClick={onReset}
              type="button"
            >
              <RotateCcw className="h-3.5 w-3.5" />
            </button>
          ) : null}
        </div>
      </div>
    );
  },
);

SettingsField.displayName = 'SettingsField';

export { SettingsField };
