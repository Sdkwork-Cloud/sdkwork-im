import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { DataTable, type DataTableProps } from '../../ui/data-display';
import { WorkspacePanel, type WorkspacePanelProps } from '../workspace/WorkspacePanel';
import {
  ManagementWorkbench,
  type ManagementWorkbenchProps,
} from './ManagementWorkbench';

export interface CrudWorkbenchTableProps<T>
  extends Omit<DataTableProps<T>, 'description' | 'title'> {
  className?: string;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

export interface CrudWorkbenchEditorProps
  extends Omit<WorkspacePanelProps, 'className'> {
  className?: string;
}

export interface CrudWorkbenchSlotProps {
  main?: PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'section'>, 'children'>>;
  stack?: PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
}

export interface CrudWorkbenchProps<T>
  extends Omit<ManagementWorkbenchProps, 'main'> {
  editor?: CrudWorkbenchEditorProps;
  slotProps?: CrudWorkbenchSlotProps;
  table: CrudWorkbenchTableProps<T>;
}

export function CrudWorkbench<T>({
  detail,
  editor,
  slotProps,
  table,
  ...props
}: CrudWorkbenchProps<T>) {
  const {
    className: tableClassName,
    description: tableDescription,
    title: tableTitle,
    ...tableProps
  } = table;

  return (
    <ManagementWorkbench
      {...props}
      data-sdk-pattern="crud-workbench"
      detail={detail}
      main={{
        ...slotProps?.main,
        description: tableDescription,
        title: tableTitle,
        children: (
          <div
            {...mergePatternSlotProps<PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>>(
              {
                className: 'flex h-full min-h-0 flex-col gap-4',
                'data-sdk-region': 'crud-workbench-stack',
              },
              slotProps?.stack,
            )}
          >
            <DataTable
              {...tableProps}
              className={cn('min-h-0', tableClassName)}
            />
            {editor ? <WorkspacePanel {...editor} className={cn('shrink-0', editor.className)} /> : null}
          </div>
        ),
      }}
    />
  );
}
CrudWorkbench.displayName = 'CrudWorkbench';
