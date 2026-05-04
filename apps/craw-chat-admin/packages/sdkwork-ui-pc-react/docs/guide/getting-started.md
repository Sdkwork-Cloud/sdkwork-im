# Getting Started

## Install

```bash
pnpm install
```

If your app uses the `components/ui/form` domain, align `react-hook-form` with the package peer dependency in the consuming app.

To consume the package directly from the repository main branch instead of a registry release:

```bash
pnpm add "https://<git-host>/<org>/spring-ai-plus.git#main&path:/sdkwork-ui/sdkwork-ui-pc-react"
```

The package runs `prepare` on git installs so `dist` is built before the dependency is linked into the consumer app.

## Run The Core Checks

```bash
pnpm test
pnpm build
pnpm docs:build
```

## Consume The Package

```tsx
import '@sdkwork/ui-pc-react/styles.css';
import { AppShell, Button, PageHeader } from '@sdkwork/ui-pc-react';
```

For desktop forms, keep orchestration in `form` and standalone controls in `data-entry`:

```tsx
import {
  FilterBar,
  FilterBarActions,
  FilterBarSection,
  Form,
  FormActions,
  FormControl,
  FormField,
  FormGrid,
  FormItem,
  FormLabel,
  FormMessage,
  FormSection,
} from '@sdkwork/ui-pc-react/components/ui/form';
import { Input } from '@sdkwork/ui-pc-react/components/ui/data-entry';
```

For desktop headers, editor chrome, overflow actions, and split workspaces:

```tsx
import { ActionMenuButton, Button, IconButton, ToolbarButton } from '@sdkwork/ui-pc-react/components/ui/actions';
import { EmptySearch, Toaster, toast } from '@sdkwork/ui-pc-react/components/ui/feedback';
import { Panel, PanelGroup, PanelResizeHandle, Toolbar, ToolbarGroup, ToolbarSpacer } from '@sdkwork/ui-pc-react/components/ui/layout';
import { MoreHorizontal, Search, SlidersHorizontal } from 'lucide-react';

export function SearchWorkspace() {
  return (
    <>
      <Toolbar aria-label="Search workspace actions">
        <ToolbarGroup>
          <IconButton aria-label="Search notes" variant="ghost">
            <Search className="h-4 w-4" />
          </IconButton>
          <ToolbarButton aria-label="Toggle filters" pressed shortcut="Ctrl+Shift+F">
            <SlidersHorizontal className="h-4 w-4" />
          </ToolbarButton>
        </ToolbarGroup>
        <ToolbarSpacer />
        <ToolbarGroup>
          <ActionMenuButton
            aria-label="Open bulk actions"
            items={[{ key: 'archive', label: 'Archive selection' }]}
          >
            <MoreHorizontal className="h-4 w-4" />
            More
          </ActionMenuButton>
          <Button onClick={() => toast.success('Created new note')}>New note</Button>
        </ToolbarGroup>
      </Toolbar>
      <PanelGroup direction="horizontal">
        <Panel defaultSize={26} minSize={20}>
          <div>Sidebar</div>
        </Panel>
        <PanelResizeHandle withHandle />
        <Panel defaultSize={74}>
          <EmptySearch keyword="agent workflow" onClear={() => undefined} />
        </Panel>
      </PanelGroup>
      <Toaster />
    </>
  );
}
```
