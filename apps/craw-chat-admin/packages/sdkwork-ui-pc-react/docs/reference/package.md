# Package Reference

## Commands

```bash
pnpm install
pnpm test
pnpm build
pnpm typecheck
pnpm docs:dev
pnpm docs:build
```

## Governance

Framework standards are defined in `/reference/framework-governance`.

## Exports

All documented subpath exports publish both runtime JavaScript and matching declaration files, so TypeScript consumers can import from root or by domain without losing types.

### Root

```ts
import {
  ActivityFeed,
  ActionMenuButton,
  AnchoredPickerSurface,
  AppShell,
  BulkActionBar,
  Button,
  CollectionGrid,
  DataTable,
  DesktopShellFrame,
  DetailDrawer,
  EmptySearch,
  IconButton,
  ListDetailWorkspace,
  ManagementWorkbench,
  PageHeader,
  SearchCommandPalette,
  SdkworkThemeProvider,
  ToolbarButton,
  Toaster,
  WorkspaceScaffold,
  toast,
} from '@sdkwork/ui-pc-react';
```

### Actions

```ts
import {
  ActionMenuButton,
  BulkActionBar,
  Button,
  IconButton,
  SplitButton,
  ToolbarButton,
} from '@sdkwork/ui-pc-react/components/ui/actions';
```

### Data Entry

```ts
import {
  Combobox,
  DateInput,
  DateRangePicker,
  FileUpload,
  Input,
  NumberInput,
  SegmentedControl,
  TagInput,
} from '@sdkwork/ui-pc-react/components/ui/data-entry';
```

### Data Display

```ts
import {
  CollectionGrid,
  DataTable,
  DescriptionList,
  KeyValueTable,
  MarkdownViewer,
  Timeline,
  Tree,
} from '@sdkwork/ui-pc-react/components/ui/data-display';
```

### Form

```ts
import {
  FilterBar,
  Form,
  FormField,
  FormLayout,
  SettingsField,
  SettingsSection,
} from '@sdkwork/ui-pc-react/components/ui/form';
```

### Layout

```ts
import {
  Panel,
  PanelGroup,
  PanelResizeHandle,
  SidebarSection,
  StatusBar,
  Toolbar,
} from '@sdkwork/ui-pc-react/components/ui/layout';
```

### Navigation

```ts
import {
  Menubar,
  Pagination,
  Stepper,
  WorkspaceTabs,
} from '@sdkwork/ui-pc-react/components/ui/navigation';
```

### Overlays

```ts
import {
  ContextMenu,
  Drawer,
  HoverCard,
  Modal,
} from '@sdkwork/ui-pc-react/components/ui/overlays';
```

### Feedback

```ts
import {
  ActivityFeed,
  EmptySearch,
  InlineAlert,
  NotificationCenter,
  Toaster,
  toast,
} from '@sdkwork/ui-pc-react/components/ui/feedback';
```

### Patterns

```ts
import {
  AppShell,
  DesktopShellFrame,
  DetailDrawer,
  EntityPickerDialog,
  ListDetailWorkspace,
  ManagementWorkbench,
  OperationDrawer,
  PickerDialog,
  SearchCommandPalette,
  SettingsCenter,
  WorkspaceScaffold,
} from '@sdkwork/ui-pc-react/components/patterns';
```

### Theme And Styles

```ts
import '@sdkwork/ui-pc-react/styles.css';
import {
  CLAW_DARK_THEME,
  CLAW_LIGHT_THEME,
  SdkworkThemeProvider,
  createSdkworkTheme,
} from '@sdkwork/ui-pc-react/theme';
```
