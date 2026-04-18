import * as React from 'react';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { AppShell, type AppShellProps } from '../app-shell/AppShell';
import { NavigationRail } from '../app-shell/NavigationRail';
import {
  DesktopAppHeader,
  type DesktopAppHeaderProps,
  type DesktopAppHeaderSlotProps,
} from './DesktopAppHeader';

type DesktopShellFrameShellProps = Omit<
  AppShellProps,
  keyof React.HTMLAttributes<HTMLDivElement> | 'className' | 'content' | 'footer' | 'header' | 'sidebar'
>;

type DesktopShellFrameHeaderProps = Pick<
  DesktopAppHeaderProps,
  | 'actions'
  | 'badge'
  | 'bordered'
  | 'brand'
  | 'brandMark'
  | 'center'
  | 'centerInteractive'
  | 'centerMaxWidth'
  | 'leading'
  | 'size'
  | 'subtitle'
  | 'title'
  | 'translucent'
  | 'windowController'
  | 'windowControlLabels'
  | 'windowControls'
>;

type DesktopShellFrameRootProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DesktopShellFrameShellSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DesktopShellFrameContentSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DesktopShellFrameNavigationSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'nav'>, 'children'>
>;

export interface DesktopShellFrameSlotProps extends DesktopAppHeaderSlotProps {
  content?: DesktopShellFrameContentSlotProps;
  header?: PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
  navigation?: DesktopShellFrameNavigationSlotProps;
  shell?: DesktopShellFrameShellSlotProps;
}

export interface DesktopShellFrameProps
  extends Omit<React.ComponentPropsWithoutRef<'div'>, 'children' | 'content' | 'title'>,
    DesktopShellFrameShellProps,
    DesktopShellFrameHeaderProps {
  content: React.ReactNode;
  navigation?: React.ReactNode;
  navigationFooter?: React.ReactNode;
  navigationHeader?: React.ReactNode;
  sidebar?: React.ReactNode;
  slotProps?: DesktopShellFrameSlotProps;
  statusBar?: React.ReactNode;
}

export const DesktopShellFrame = React.forwardRef<HTMLDivElement, DesktopShellFrameProps>(({
  actions,
  badge,
  bordered,
  brand,
  brandMark,
  center,
  centerInteractive,
  centerMaxWidth,
  className,
  content,
  id,
  leading,
  navigation,
  navigationFooter,
  navigationHeader,
  sidebar,
  sidebarWidth = 280,
  size,
  slotProps,
  statusBar,
  style,
  subtitle,
  theme,
  title,
  translucent,
  windowControlLabels,
  windowController,
  windowControls,
  ...props
}, ref) => {
  const hasHeader =
    brand !== undefined ||
    brandMark !== undefined ||
    title !== undefined ||
    subtitle !== undefined ||
    badge !== undefined ||
    leading !== undefined ||
    center !== undefined ||
    actions !== undefined ||
    windowController !== undefined ||
    windowControls !== undefined;

  const {
    bar: barSlotProps,
    brand: brandSlotProps,
    center: centerSlotProps,
    centerShell: centerShellSlotProps,
    content: contentSlotProps,
    header: headerSlotProps,
    leading: leadingSlotProps,
    navigation: navigationSlotProps,
    shell: shellSlotProps,
    subtitle: subtitleSlotProps,
    title: titleSlotProps,
    trailing: trailingSlotProps,
  } = slotProps ?? {};

  const resolvedSidebar =
    sidebar ??
    (navigation || navigationHeader || navigationFooter ? (
      <NavigationRail
        footer={navigationFooter}
        header={navigationHeader}
        {...navigationSlotProps}
      >
        {navigation}
      </NavigationRail>
    ) : undefined);

  return (
    <div
      ref={ref}
      {...mergePatternSlotProps<DesktopShellFrameRootProps>(
        { 'data-sdk-pattern': 'desktop-shell-frame' },
        {
          className,
          id,
          style,
          ...props,
        },
      )}
    >
      <AppShell
        {...shellSlotProps}
        content={
          <div
            {...mergePatternSlotProps<DesktopShellFrameContentSlotProps>(
              {
                className: 'h-full min-h-0 min-w-0',
                'data-sdk-region': 'desktop-shell-content',
              },
              contentSlotProps,
            )}
          >
            {content}
          </div>
        }
        footer={statusBar}
        header={
          hasHeader ? (
            <DesktopAppHeader
              actions={actions}
              badge={badge}
              bordered={bordered}
              brand={brand}
              brandMark={brandMark}
              center={center}
              centerInteractive={centerInteractive}
              centerMaxWidth={centerMaxWidth}
              leading={leading}
              size={size}
              slotProps={{
                bar: barSlotProps,
                brand: brandSlotProps,
                center: centerSlotProps,
                centerShell: centerShellSlotProps,
                leading: leadingSlotProps,
                subtitle: subtitleSlotProps,
                title: titleSlotProps,
                trailing: trailingSlotProps,
              }}
              subtitle={subtitle}
              title={title}
              translucent={translucent}
              windowControlLabels={windowControlLabels}
              windowController={windowController}
              windowControls={windowControls}
              {...headerSlotProps}
            />
          ) : undefined
        }
        sidebar={resolvedSidebar}
        sidebarWidth={sidebarWidth}
        theme={theme}
      />
    </div>
  );
});
DesktopShellFrame.displayName = 'DesktopShellFrame';
