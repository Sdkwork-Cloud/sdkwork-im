# SDKWORK UI PC React

`@sdkwork/ui-pc-react` is the shared PC application UI framework for SDKWORK desktop and desktop-first React applications.

The default visual baseline is the `claw-studio` desktop design language: zinc-neutral surfaces, theme-color-driven brand emphasis with a default lobster accent, compact control radii, restrained panel radii, and soft layered shadows.

It standardizes:

- shadcn-style UI primitives
- a dedicated desktop `form` domain
- reusable `navigation` primitives
- command, action-menu, split-button, toolbar button, bulk action bar, menubar, `Stepper`, toolbar, modal, confirm dialog, drawer, `RichTree` plus the compatibility `Tree`, upload family with paste, directory intake, rejection feedback, and item actions, combobox, date input, datetime input, date-range field and picker abstractions, number input, segmented control, tag input, context menu, and hover card desktop abstractions
- a shared resizable panel layout system for desktop workspaces, sidebars, and inspector panes
- a reusable desktop `StatusBar` layer for window-bottom status, branch, sync, and mode indicators
- settings editors, sidebar sections, closable workspace tabs, list-filter bars, markdown rendering, key-value detail tables, and structured detail description lists for desktop search, table, drawer, and modal workflows
- semantic icon actions, toolbar toggles, search-result empty states, and tag editing flows for desktop workflows
- standardized `ActivityFeed`, `InlineAlert`, `NotificationCenter`, `Toaster`, and `toast` feedback APIs aligned to current PC app usage
- a shared desktop shell family with `DesktopWindowControls`, `DesktopTitleBar`, `DesktopAppHeader`, `DesktopShellFrame`, `SectionHeader`, `SettingsCenter`, `DirtyStateBar`, `DetailDrawer`, `OperationDrawer`, `AnchoredPickerSurface`, `PickerDialog`, `EntityPickerDialog`, `PickerSelectionFooter`, `TwoPaneSelectorPopover`, `InspectorRail`, `ListDetailWorkspace`, `ManagementWorkbench`, `CrudWorkbench`, `SearchCommandPalette`, `WorkspaceScaffold`, and `RestartRequiredNotice`
- semantic SDKWORK theme tokens
- desktop application shell patterns
- framework-aligned component directory categories
- a publishable pnpm package build
- framework documentation with VitePress

The current shared baseline includes actions with `SplitButton`, `ActionMenuButton`, and `BulkActionBar`, data-entry primitives including `TagInput`, `DateTimeInput`, `DateRangeField`, `DateTimeRangeField`, and the popover-based `DateRangePicker`, a typed upload family with replacement, clear-all, paste, directory support, inline rejection feedback, and preview/download/retry item actions, richer data-display primitives including `CollectionGrid`, `DataTable`, `RichTree`, `Timeline`, `MarkdownViewer`, and `KeyValueTable`, layout toolbars plus `StatusBar`, split-workspace surfaces, navigation primitives including `Menubar`, `Stepper`, and closable workspace tabs, overlays, a feedback domain with `ActivityFeed`, inline alerts, notification-center panels, and themed toast notifications, a desktop form system with settings-specific field orchestration, list-filter infrastructure, structured detail presentation primitives, advanced desktop interaction components, and desktop shell or workbench patterns including reusable `DesktopShellFrame`, `InspectorRail`, `SettingsCenter`, `DirtyStateBar`, `DetailDrawer`, `OperationDrawer`, `AnchoredPickerSurface`, `PickerDialog`, `EntityPickerDialog`, `PickerSelectionFooter`, `TwoPaneSelectorPopover`, `ListDetailWorkspace`, `ManagementWorkbench`, `CrudWorkbench`, `SearchCommandPalette`, and `WorkspaceScaffold`.

## Framework Governance

The package is now governed by explicit framework contracts rather than ad hoc component conventions.

- Theme tokens are a closed contract and are audited for undeclared usage.
- Rendered components expose stable `data-sdk-ui`, `data-sdk-pattern`, or `data-sdk-region` markers.
- Reusable components follow shared authoring rules for props, refs, display names, and semantics.

See `docs/reference/framework-governance.md` for the framework rulebook.

## Scope

The package is designed to serve the audited PC React application family across:

- `claw-studio`
- `magic-studio-v2`
- `sdkwork-notes/sdkwork-notes-pc-react`
- `sdkwork-chat-pc-react`
- `sdkwork-cloud-portal`
- `sdkwork-desktop-settings`
- `sdkwork-drive/sdkwork-drive-pc-react`
- `openchat/app/openchat-react-pc`
- `sdkwork-backend-react-web/packages/sdkwork-react-backend-desktop`

## Commands

```bash
pnpm install
pnpm test
pnpm build
pnpm docs:dev
pnpm docs:build
```

## Install From Git

This package can be consumed directly from the repository main branch. Because the package declares a `prepare` script, git installs build `dist` before the dependency is linked into the consumer app.

```bash
pnpm add "https://<git-host>/<org>/spring-ai-plus.git#main&path:/spring-ai-plus-business/apps/sdkwork-ui/sdkwork-ui-pc-react"
```

After installation, consume it like any other package:

```tsx
import '@sdkwork/ui-pc-react/styles.css';
import { Button, DataTable } from '@sdkwork/ui-pc-react';
```

## Package Exports

- `@sdkwork/ui-pc-react`
- `@sdkwork/ui-pc-react/theme`
- `@sdkwork/ui-pc-react/components/ui`
- `@sdkwork/ui-pc-react/components/ui/actions`
- `@sdkwork/ui-pc-react/components/ui/data-entry`
- `@sdkwork/ui-pc-react/components/ui/data-display`
- `@sdkwork/ui-pc-react/components/ui/form`
- `@sdkwork/ui-pc-react/components/ui/feedback`
- `@sdkwork/ui-pc-react/components/ui/layout`
- `@sdkwork/ui-pc-react/components/ui/navigation`
- `@sdkwork/ui-pc-react/components/ui/overlays`
- `@sdkwork/ui-pc-react/components/ui/catalog`
- `@sdkwork/ui-pc-react/components/patterns`
- `@sdkwork/ui-pc-react/components/patterns/app-shell`
- `@sdkwork/ui-pc-react/components/patterns/command`
- `@sdkwork/ui-pc-react/components/patterns/desktop-shell`
- `@sdkwork/ui-pc-react/components/patterns/detail`
- `@sdkwork/ui-pc-react/components/patterns/picker`
- `@sdkwork/ui-pc-react/components/patterns/settings`
- `@sdkwork/ui-pc-react/components/patterns/system`
- `@sdkwork/ui-pc-react/components/patterns/workbench`
- `@sdkwork/ui-pc-react/components/patterns/workspace`
- `@sdkwork/ui-pc-react/styles.css`

All published subpath exports ship both runtime JS and matching `.d.ts` entries so TypeScript consumers can safely import by domain.

## Theme System

The theme model is `preset + deep overrides`.

- `CLAW_LIGHT_THEME` and `CLAW_DARK_THEME` are the canonical `claw-studio` presets.
- `SDKWORK_LIGHT_THEME` and `SDKWORK_DARK_THEME` are compatibility aliases that currently point to the claw preset baseline.
- `themeColor` selects the accent palette for claw tokens. The shared baseline now defaults to `lobster` and also supports `tech-blue`, `green-tech`, `rose`, `violet`, and `zinc`.
- `createSdkworkTheme()` accepts semantic overrides for `brand`, `surface`, `text`, `border`, `state`, `radius`, and `shadow`.

```tsx
import {
  CLAW_LIGHT_THEME,
  SdkworkThemeProvider,
  createSdkworkTheme,
} from '@sdkwork/ui-pc-react';

const oceanTheme = createSdkworkTheme({
  preset: 'claw',
  colorMode: 'light',
  brand: {
    primary: '#0f766e',
  },
  radius: {
    panel: '2rem',
  },
});

export function ExampleThemeScope() {
  return (
    <SdkworkThemeProvider
      defaultTheme={CLAW_LIGHT_THEME.colorMode}
      overrides={oceanTheme}
    >
      <div>Scoped theme area.</div>
    </SdkworkThemeProvider>
  );
}
```

## Usage

```tsx
import '@sdkwork/ui-pc-react/styles.css';
import {
  AppShell,
  Button,
  EmptySearch,
  IconButton,
  Panel,
  PanelGroup,
  PanelResizeHandle,
  PageHeader,
  SdkworkThemeProvider,
  ToolbarButton,
  Toaster,
  Toolbar,
  ToolbarGroup,
  ToolbarSpacer,
  WorkspacePanel,
  toast,
} from '@sdkwork/ui-pc-react';
import { Search, Settings2 } from 'lucide-react';

export function ExamplePage() {
  return (
    <SdkworkThemeProvider defaultTheme="dark">
      <AppShell
        header={
          <PageHeader
            title="SDKWORK Workspace"
            description="Shared PC shell layout"
            actions={
              <Toolbar aria-label="Workspace actions" className="min-w-[24rem]">
                <ToolbarGroup>
                  <IconButton aria-label="Search workspace" variant="ghost">
                    <Search className="h-4 w-4" />
                  </IconButton>
                  <ToolbarButton aria-label="Toggle inspector" shortcut="Ctrl+.">
                    <Settings2 className="h-4 w-4" />
                  </ToolbarButton>
                </ToolbarGroup>
                <ToolbarSpacer />
                <ToolbarGroup>
                  <Button variant="secondary">Filters</Button>
                  <Button onClick={() => toast.success('Workspace ready')}>Run</Button>
                </ToolbarGroup>
              </Toolbar>
            }
          />
        }
        content={
          <div className="p-6">
            <PanelGroup direction="horizontal">
              <Panel defaultSize={28} minSize={20}>
                <WorkspacePanel title="Navigation">Desktop navigation</WorkspacePanel>
              </Panel>
              <PanelResizeHandle withHandle />
              <Panel defaultSize={72}>
                <WorkspacePanel title="Search Results">
                  <EmptySearch keyword="agent workflow" onClear={() => undefined} />
                </WorkspacePanel>
              </Panel>
            </PanelGroup>
          </div>
        }
      />
      <Toaster />
    </SdkworkThemeProvider>
  );
}
```

## Design Standard

The framework uses:

- Radix primitives where accessibility matters
- shadcn-style component composition
- Tailwind 4 during framework build to ship prebuilt CSS
- semantic tokens instead of app-local color names
- `claw-studio` as the default desktop visual reference
- low-coupling desktop patterns rather than business-bound components

## Internal Directory Model

`src/components/ui` is organized by the same broad categories used by leading React UI frameworks:

- `actions`
- `data-entry`
- `form`
- `data-display`
- `feedback`
- `layout`
- `navigation`
- `overlays`

The directory contract is domain-first. Richer families are authored inside their domain folders instead of being added as more flat top-level files:

```text
src/components/ui
  actions/
    action-menu-button.tsx
    bulk-action-bar.tsx
    command.tsx
    icon-button.tsx
    split-button.tsx
    toolbar-button.tsx
    index.ts
  data-entry/
    combobox.tsx
    date-input.tsx
    date-range-picker.tsx
    upload/
      file-upload.tsx
      upload-dropzone.tsx
      upload-item.tsx
      upload-presets.tsx
      upload-types.ts
      upload-utils.ts
      index.ts
    number-input.tsx
    segmented-control.tsx
    tag-input.tsx
    index.ts
  data-display/
    data-grid.tsx
    data-table.tsx
    description-list.tsx
    key-value-table.tsx
    markdown-viewer.tsx
    timeline.tsx
    tree.tsx
    index.ts
  feedback/
    activity-feed.tsx
    empty-search.tsx
    inline-alert.tsx
    notification-center.tsx
    toast.tsx
    index.ts
  form/
    filter-bar.tsx
    form.tsx
    form-layout.tsx
    settings-field.tsx
    settings-section.tsx
    index.ts
  layout/
    panel-group.tsx
    sidebar-section.tsx
    status-bar.tsx
    toolbar.tsx
    index.ts
  navigation/
    menubar.tsx
    stepper.tsx
    workspace-tabs.tsx
    index.ts
  overlays/
    context-menu.tsx
    drawer.tsx
    hover-card.tsx
    modal.tsx
    index.ts
```

Each domain barrel is now authoritative for its own public runtime surface.

`form` owns field orchestration and desktop form layouts:

- `Form`, `FormField`, `FormItem`, `FormLabel`, `FormControl`, `FormDescription`, `FormMessage`
- `FormSection`, `FormGrid`, `FormActions`
- `SettingsField`, `SettingsSection`
- `FilterBar`, `FilterBarSection`, `FilterBarActions`

`data-entry` owns standalone controls and typed input families:

- `Input`
- `Textarea`
- `Checkbox`
- `Switch`
- `Select`
- `Label`
- `RadioGroup`
- `Slider`
- `Combobox`
- `DateInput`
- `DateTimeInput`
- `DateRangeField`
- `DateTimeRangeField`
- `DateRangePicker`
- `FileUpload`
- `ImageUpload`
- `VideoUpload`
- `AudioUpload`
- `DocumentUpload`
- `NumberInput`
- `SegmentedControl`
- `TagInput`

The upload family now covers local and remote desktop intake flows:

- `allowPaste`, `directory`, `clearable`, and single-file replacement ergonomics
- inline rejection rendering through `showRejections` and `rejectionTitle`
- item-level `onPreviewItem`, `onDownloadItem`, and `onRetryItem` hooks for server-backed queues

`navigation` owns reusable cross-page wayfinding primitives:

- `Breadcrumb`, `BreadcrumbLink`, `BreadcrumbPage`
- `Menubar`
- `Stepper`, `StepperItem`
- `Pagination`, `PaginationLink`, `PaginationPrevious`, `PaginationNext`
- `WorkspaceTab`, `WorkspaceTabs`

`layout` owns reusable workspace structure that is lighter than app shells and heavier than raw `div` wrappers:

- `Card`, `CardHeader`, `CardContent`, `CardFooter`
- `PanelGroup`, `Panel`, `PanelResizeHandle`
- `StatusBar`, `StatusBarSection`, `StatusBarItem`
- `Toolbar`, `ToolbarGroup`, `ToolbarSpacer`, `ToolbarSeparator`
- `SidebarSection`
- `ScrollArea`
- `Separator`

`data-display` now also covers core desktop information surfaces:

- `CollectionGrid`
- `DataTable`
- `RichTree`
- `DescriptionList`, `DescriptionItem`, `DescriptionTerm`, `DescriptionDetails`
- `KeyValueTable`, `KeyValueTableRow`, `KeyValueTableLabel`, `KeyValueTableValue`
- `MarkdownViewer`, `MarkdownViewerEmpty`
- `Table`, `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`
- `Timeline`
- `RichTree`
- `Tree`
- `StatusBadge`
- `StatCard`

`actions` and `overlays` now also cover desktop command surfaces and secondary work panes:

- `Button`, `IconButton`, `ToolbarButton`
- `ActionMenuButton`
- `BulkActionBar`
- `SplitButton`
- `Command`, `CommandDialog`, `CommandInput`, `CommandItem`
- `Modal`, `ModalContent`, `ModalHeader`, `ModalBody`, `ModalFooter`, `ConfirmDialog`
- `Drawer`, `DrawerContent`, `DrawerHeader`, `DrawerBody`, `DrawerFooter`
- `ContextMenu`, `ContextMenuContent`, `ContextMenuItem`
- `HoverCard`, `HoverCardTrigger`, `HoverCardContent`

`feedback` now standardizes both inline states and global notifications:

- `ActivityFeed`, `ActivityFeedItem`
- `EmptyState`, `LoadingBlock`, `StatusNotice`, `Progress`, `Skeleton`
- `EmptySearch`
- `InlineAlert`
- `NotificationCenter`, `NotificationCenterItem`
- `Toaster`, `SdkworkToaster`
- `toast`, `sdkToast`

The toast layer is intentionally `sonner`-compatible so audited PC apps can migrate from direct `sonner` imports or app-local toast stores without changing their overall notification calling pattern.

Application composition stays under `src/components/patterns`.

`patterns` now follows the same domain-first structure as the primitive layer:

```text
src/components/patterns
  app-shell/
    AppShell.tsx
    NavigationRail.tsx
    PageHeader.tsx
    index.ts
  desktop-shell/
    DesktopWindowControls.tsx
    DesktopTitleBar.tsx
    DesktopAppHeader.tsx
    DesktopShellFrame.tsx
    index.ts
  command/
    SearchCommandPalette.tsx
    index.ts
  detail/
    DetailDrawer.tsx
    OperationDrawer.tsx
    index.ts
  picker/
    AnchoredPickerSurface.tsx
    PickerDialog.tsx
    EntityPickerDialog.tsx
    PickerSelectionFooter.tsx
    TwoPaneSelectorPopover.tsx
    index.ts
  settings/
    SettingsCenter.tsx
    index.ts
  system/
    RestartRequiredNotice.tsx
    index.ts
  workbench/
    CrudWorkbench.tsx
    ManagementWorkbench.tsx
    index.ts
  workspace/
    InspectorRail.tsx
    WorkspaceScaffold.tsx
    WorkspacePanel.tsx
    InspectorPanel.tsx
    ListDetailWorkspace.tsx
    SectionHeader.tsx
    index.ts
  feedback.tsx
```

The top-level pattern files remain as compatibility re-export facades, but new composition families should land inside their owning pattern domain.

The current implementation covers the highest-frequency shared controls first, including `Avatar`, `Tabs`, `CollectionGrid`, `DataTable`, `RichTree`, `DescriptionList`, `KeyValueTable`, `MarkdownViewer`, `Table`, `Timeline`, `Tree`, `StatusBadge`, `StatCard`, `ActivityFeed`, `ActivityFeedItem`, `Progress`, `Skeleton`, `InlineAlert`, `EmptySearch`, `NotificationCenter`, `NotificationCenterItem`, `Toaster`, `toast`, `Popover`, `DropdownMenu`, `Tooltip`, `RadioGroup`, `Slider`, `Button`, `IconButton`, `ToolbarButton`, `ActionMenuButton`, `BulkActionBar`, `SplitButton`, `Command`, `Toolbar`, `PanelGroup`, `Panel`, `PanelResizeHandle`, `SidebarSection`, `StatusBar`, `Modal`, `ConfirmDialog`, `Drawer`, `ContextMenu`, `HoverCard`, `Combobox`, `DateInput`, `DateTimeInput`, `DateRangeField`, `DateTimeRangeField`, `DateRangePicker`, the full upload family (`FileUpload`, `ImageUpload`, `VideoUpload`, `AudioUpload`, `DocumentUpload`), `NumberInput`, `SegmentedControl`, `TagInput`, `Menubar`, `Stepper`, `StepperItem`, `WorkspaceTab`, `WorkspaceTabs`, the shared form system, core navigation primitives, and the desktop pattern family (`DesktopWindowControls`, `DesktopTitleBar`, `DesktopAppHeader`, `DesktopShellFrame`, `SectionHeader`, `SettingsCenter`, `DirtyStateBar`, `DetailDrawer`, `OperationDrawer`, `AnchoredPickerSurface`, `PickerDialog`, `EntityPickerDialog`, `PickerSelectionFooter`, `TwoPaneSelectorPopover`, `InspectorRail`, `ListDetailWorkspace`, `ManagementWorkbench`, `CrudWorkbench`, `SearchCommandPalette`, `WorkspaceScaffold`, and `RestartRequiredNotice`). The picker stack now layers `AnchoredPickerSurface` beneath `TwoPaneSelectorPopover` so future `DatePicker`, `PresetRangePicker`, `TreeSelect`, and `TransferList` patterns can converge on one anchored shell. The docs now position the next-wave catalog around richer popup date picking, filter tabs and chips, `TreeSelect`, `TransferList`, explorer shells, and higher-order settings or navigation workbench families.

## Documentation

The framework site lives in [`docs`](./docs) and is built with VitePress.
