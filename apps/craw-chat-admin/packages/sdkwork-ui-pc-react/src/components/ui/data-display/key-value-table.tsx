import * as React from 'react';
import { cn } from '../../../lib/utils';

export type KeyValueTone = 'default' | 'success' | 'warning' | 'danger';
export type KeyValueEmphasis = 'default' | 'strong';

export interface KeyValueTableRowData {
  id?: React.Key;
  label: React.ReactNode;
  value: React.ReactNode;
  tone?: KeyValueTone;
  emphasis?: KeyValueEmphasis;
  mono?: boolean;
}

export interface KeyValueTableProps extends React.TableHTMLAttributes<HTMLTableElement> {
  rows?: KeyValueTableRowData[];
  dense?: boolean;
}

export interface KeyValueTableRowProps extends React.ComponentPropsWithoutRef<'tr'> {
  tone?: KeyValueTone;
  emphasis?: KeyValueEmphasis;
}

export type KeyValueTableLabelProps = React.ComponentPropsWithoutRef<'th'>;

export interface KeyValueTableValueProps extends React.ComponentPropsWithoutRef<'td'> {
  mono?: boolean;
  tone?: KeyValueTone;
  emphasis?: KeyValueEmphasis;
}

const denseContext = React.createContext(false);

const valueToneClassName: Record<KeyValueTone, string> = {
  default: 'text-[var(--sdk-color-text-primary)]',
  success: 'text-[var(--sdk-color-state-success)]',
  warning: 'text-[var(--sdk-color-state-warning)]',
  danger: 'text-[var(--sdk-color-state-danger)]',
};

const valueEmphasisClassName: Record<KeyValueEmphasis, string> = {
  default: 'font-normal',
  strong: 'font-semibold',
};

const KeyValueTable = React.forwardRef<HTMLTableElement, KeyValueTableProps>(
  ({ className, dense = false, rows, children, ...props }, ref) => (
    <denseContext.Provider value={dense}>
      <table
        ref={ref}
        className={cn('w-full table-fixed border-collapse', className)}
        data-density={dense ? 'dense' : 'default'}
        data-sdk-ui="key-value-table"
        {...props}
      >
        <tbody>
          {rows?.map((row, index) => (
            <KeyValueTableRow
              key={row.id ?? index}
              emphasis={row.emphasis}
              tone={row.tone}
            >
              <KeyValueTableLabel>{row.label}</KeyValueTableLabel>
              <KeyValueTableValue
                emphasis={row.emphasis}
                mono={row.mono}
                tone={row.tone}
              >
                {row.value}
              </KeyValueTableValue>
            </KeyValueTableRow>
          ))}
          {children}
        </tbody>
      </table>
    </denseContext.Provider>
  ),
);

KeyValueTable.displayName = 'KeyValueTable';

const KeyValueTableRow = React.forwardRef<HTMLTableRowElement, KeyValueTableRowProps>(
  ({ className, tone = 'default', emphasis = 'default', ...props }, ref) => (
    <tr
      ref={ref}
      className={cn(
        'border-b border-[var(--sdk-color-border-default)] last:border-b-0',
        className,
      )}
      data-emphasis={emphasis}
      data-sdk-ui="key-value-table-row"
      data-tone={tone}
      {...props}
    />
  ),
);

KeyValueTableRow.displayName = 'KeyValueTableRow';

const KeyValueTableLabel = React.forwardRef<HTMLTableCellElement, KeyValueTableLabelProps>(
  ({ className, ...props }, ref) => {
    const dense = React.useContext(denseContext);
    return (
      <th
        ref={ref}
        className={cn(
          'w-56 pr-6 text-left align-top text-xs font-medium uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]',
          dense ? 'py-1.5' : 'py-2.5',
          className,
        )}
        data-sdk-ui="key-value-table-label"
        scope="row"
        {...props}
      />
    );
  },
);

KeyValueTableLabel.displayName = 'KeyValueTableLabel';

const KeyValueTableValue = React.forwardRef<HTMLTableCellElement, KeyValueTableValueProps>(
  ({ className, mono = false, tone = 'default', emphasis = 'default', ...props }, ref) => {
    const dense = React.useContext(denseContext);
    return (
      <td
        ref={ref}
        className={cn(
          'align-top text-sm break-words',
          valueToneClassName[tone],
          valueEmphasisClassName[emphasis],
          mono ? 'font-mono' : undefined,
          dense ? 'py-1.5' : 'py-2.5',
          className,
        )}
        data-sdk-ui="key-value-table-value"
        {...props}
      />
    );
  },
);

KeyValueTableValue.displayName = 'KeyValueTableValue';

export { KeyValueTable, KeyValueTableLabel, KeyValueTableRow, KeyValueTableValue };
