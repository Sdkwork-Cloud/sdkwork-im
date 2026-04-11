import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

const requiredPackages = [
  'sdkwork-craw-chat-admin-types',
  'sdkwork-craw-chat-admin-core',
  'sdkwork-craw-chat-admin-shell',
  'sdkwork-craw-chat-admin-admin-api',
  'sdkwork-craw-chat-admin-auth',
  'sdkwork-craw-chat-admin-overview',
  'sdkwork-craw-chat-admin-tenants',
  'sdkwork-craw-chat-admin-users',
  'sdkwork-craw-chat-admin-conversations',
  'sdkwork-craw-chat-admin-messages',
  'sdkwork-craw-chat-admin-groups',
  'sdkwork-craw-chat-admin-moderation',
  'sdkwork-craw-chat-admin-automation',
  'sdkwork-craw-chat-admin-announcements',
  'sdkwork-craw-chat-admin-realtime',
  'sdkwork-craw-chat-admin-system',
  'sdkwork-craw-chat-admin-settings',
];

test('standalone craw-chat-admin app root exists', () => {
  assert.equal(existsSync(path.join(appRoot, 'package.json')), true);
  assert.equal(existsSync(path.join(appRoot, 'pnpm-workspace.yaml')), true);
  assert.equal(existsSync(path.join(appRoot, 'turbo.json')), true);
  assert.equal(existsSync(path.join(appRoot, 'src', 'App.tsx')), true);
  assert.equal(existsSync(path.join(appRoot, 'src', 'main.tsx')), true);
  assert.equal(existsSync(path.join(appRoot, 'src-tauri', 'Cargo.toml')), true);
  assert.equal(existsSync(path.join(appRoot, 'src-tauri', 'src', 'main.rs')), true);
});

test('app root exposes standalone browser and tauri scripts', () => {
  const packageJsonSource = read('package.json');
  const packageJson = JSON.parse(packageJsonSource);

  assert.equal(typeof packageJson.scripts?.dev, 'string');
  assert.equal(typeof packageJson.scripts?.build, 'string');
  assert.equal(typeof packageJson.scripts?.typecheck, 'string');
  assert.equal(typeof packageJson.scripts?.preview, 'string');
  assert.equal(typeof packageJson.scripts?.['tauri:dev'], 'string');
  assert.equal(typeof packageJson.scripts?.['tauri:build'], 'string');
  assert.match(packageJsonSource, /craw-chat-admin/);
});

test('required packages exist under packages/', () => {
  for (const packageName of requiredPackages) {
    assert.equal(
      existsSync(path.join(appRoot, 'packages', packageName, 'package.json')),
      true,
      `missing ${packageName}`,
    );
  }
});

test('root app stays thin and mounts the shell package', () => {
  const app = read('src/App.tsx');
  const main = read('src/main.tsx');

  assert.match(app, /sdkwork-craw-chat-admin-shell/);
  assert.match(app, /AppRoot/);
  assert.match(main, /bootstrapShellRuntime/);
  assert.match(main, /@sdkwork\/ui-pc-react\/styles\.css/);
  assert.doesNotMatch(app, /ConversationsPage|MessagesPage|GroupsPage|ModerationPage/);
});

test('core route manifest formalizes IM product modules', () => {
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');

  assert.match(routeManifest, /sdkwork-craw-chat-admin-overview/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-conversations/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-messages/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-moderation/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-system/);
  assert.match(routeManifest, /requiredPermissions:/);
  assert.match(routeManifest, /capabilityTags:/);
  assert.match(routeManifest, /strategy: 'lazy'/);
  assert.match(coreIndex, /adminRouteManifest/);
  assert.doesNotMatch(
    routeManifest,
    /sdkwork-router-admin|sdkwork-craw-chat-admin-(apirouter|traffic|catalog|coupons|commercial|pricing)/,
  );
});

test('shell owns router and auth isolation', () => {
  const routes = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const shellHostStyles = read('packages/sdkwork-craw-chat-admin-shell/src/styles/shell-host.css');

  assert.match(routes, /AdminLoginPage/);
  assert.match(routes, /ROUTE_PATHS\.LOGIN/);
  assert.match(routes, /ROUTE_PATHS\.REGISTER/);
  assert.match(routes, /ROUTE_PATHS\.FORGOT_PASSWORD/);
  assert.match(layout, /Sidebar/);
  assert.match(layout, /AppHeader/);
  assert.match(layout, /data-sdk-shell="craw-chat-admin-desktop"/);
  assert.match(shellHostStyles, /\[data-sdk-shell='craw-chat-admin-desktop'\]/);
  assert.doesNotMatch(layout, /router-admin-desktop/);
  assert.doesNotMatch(shellHostStyles, /router-admin-desktop/);
  assert.doesNotMatch(layout, /Craw Chat Admin|API Router|Catalog/);
});

test('vite config serves the admin shell from /admin/', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /base:\s*'\/admin\/'/);
  assert.match(viteConfig, /port:\s*5173/);
  assert.match(viteConfig, /strictPort:\s*true/);
});

test('tsconfig mirrors router-admin ui type shims for root and grouped ui entries', () => {
  const tsconfig = read('tsconfig.json');
  const uiShim = read('src/types/sdkwork-ui-pc-react-shim.d.ts');

  assert.equal(existsSync(path.join(appRoot, 'src', 'types', 'sdkwork-ui-pc-react-shim.d.ts')), true);
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react":\s*\["src\/types\/sdkwork-ui-pc-react-shim\.d\.ts"\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/theme":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/theme\/index\.d\.ts"\s*\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/\*":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/\*"\s*\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/styles\.css":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/sdkwork-ui\.css"\s*\]/,
  );
  assert.match(uiShim, /export \* from '[^']*sdkwork-ui-pc-react\/dist\/index';/);
});

test('theme establishes a dedicated admin visual system', () => {
  const theme = read('src/theme.css');

  assert.match(theme, /--admin-font-sans:/);
  assert.match(theme, /--admin-font-display:/);
  assert.match(theme, /body::before/);
  assert.doesNotMatch(theme, /font-family:\s*ui-sans-serif,\s*system-ui/);
});

test('core i18n surface stays focused on craw-chat admin runtime concerns', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(i18n, /export const ADMIN_LOCALE_OPTIONS/);
  assert.match(i18n, /export function AdminI18nProvider/);
  assert.match(i18n, /export function useAdminI18n/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_APIROUTER_SURFACE_TRANSLATIONS/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_COMMERCIAL_ACCOUNT_TRANSLATIONS/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_PRICING_TRANSLATIONS/);
});

test('core workbench avoids router-admin commerce preload and catalog language', () => {
  const adminApiIndex = read('packages/sdkwork-craw-chat-admin-admin-api/src/index.ts');
  const workbench = read('packages/sdkwork-craw-chat-admin-core/src/workbench.tsx');
  const workbenchActions = read('packages/sdkwork-craw-chat-admin-core/src/workbenchActions.ts');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.doesNotMatch(
    adminApiIndex,
    /export \* from '\.\/commerce'|listCoupons|saveCoupon|deleteCoupon|listMarketingCouponTemplates|saveMarketingCouponTemplate|updateMarketingCouponTemplateStatus|listMarketingCampaignBudgets|saveMarketingCampaignBudget|updateMarketingCampaignBudgetStatus|listMarketingCouponCodes|saveMarketingCouponCode|updateMarketingCouponCodeStatus|listMarketingCouponReservations|listMarketingCouponRedemptions|listMarketingCouponRollbacks|listCommercialAccounts|getCommercialAccountBalance|listCommercialAccountBenefitLots|listCommercialAccountLedger|listCommercialAccountHolds|listCommercialRequestSettlements|listCommercialPricingPlans|createCommercialPricingPlan|updateCommercialPricingPlan|cloneCommercialPricingPlan|publishCommercialPricingPlan|scheduleCommercialPricingPlan|retireCommercialPricingPlan|synchronizeCommercialPricingLifecycle|listCommercialPricingRates|createCommercialPricingRate|updateCommercialPricingRate/,
  );
  assert.doesNotMatch(
    workbench,
    /listCoupons|listRecentCommerceOrders|listCommercePaymentEvents|listMarketingCoupon|listCommercialAccount|listCommercialPricing/,
  );
  assert.doesNotMatch(
    workbenchActions,
    /handleSaveCoupon|handleToggleCoupon|handleDeleteCoupon|handleUpdateMarketingCouponTemplateStatus|handleUpdateMarketingCampaignStatus|handleUpdateMarketingCampaignBudgetStatus|handleUpdateMarketingCouponCodeStatus|handleCreateCommercialPricingPlan|handleCreateCommercialPricingRate|handleUpdateCommercialPricingPlan|handleCloneCommercialPricingPlan|handlePublishCommercialPricingPlan|handleScheduleCommercialPricingPlan|handleRetireCommercialPricingPlan|handleSynchronizeCommercialPricingLifecycle|handleUpdateCommercialPricingRate/,
  );
  assert.doesNotMatch(
    workbenchSnapshot,
    /coupon-repository|No model catalog entries|routing catalog|Create or upsert models in Catalog|credentials in Catalog/,
  );
});

test('workspace snapshot stays focused on IM runtime data and excludes dormant commerce payloads', () => {
  const typesSource = read('packages/sdkwork-craw-chat-admin-types/src/index.ts');
  const workbench = read('packages/sdkwork-craw-chat-admin-core/src/workbench.tsx');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.match(typesSource, /marketingCampaigns:/);
  assert.doesNotMatch(
    typesSource,
    /export (interface|type) (CouponRecord|MarketingBenefitKind|MarketingStackingPolicy|MarketingSubjectScope|CouponTemplateStatus|CouponDistributionKind|CampaignBudgetStatus|CouponCodeStatus|CouponReservationStatus|CouponRedemptionStatus|CouponRollbackType|CouponRollbackStatus|CouponBenefitSpec|CouponRestrictionSpec|CouponTemplateRecord|CampaignBudgetRecord|CouponCodeRecord|CouponReservationRecord|CouponRedemptionRecord|CouponRollbackRecord|CommerceOrderStatus|CommerceSettlementStatus|CommercePaymentEventType|CommercePaymentEventProcessingStatus|CommerceOrderRecord|PaymentMethodRecord|PaymentMethodCredentialBindingRecord|CommercePaymentEventRecord|CommerceOrderAuditRecord|CommercialAccountType|CommercialAccountStatus|CommercialAccountBenefitType|CommercialAccountBenefitSourceType|CommercialAccountBenefitLotStatus|CommercialAccountHoldStatus|CommercialRequestSettlementStatus|CommercialAccountLedgerEntryType|CommercialAccountRecord|CommercialAccountLotBalanceSnapshot|CommercialAccountBalanceSnapshot|CommercialAccountSummary|CommercialAccountBenefitLotRecord|CommercialAccountHoldRecord|CommercialRequestSettlementRecord|CommercialAccountLedgerEntryRecord|CommercialAccountLedgerAllocationRecord|CommercialAccountLedgerHistoryEntry|CommercialPricingPlanRecord|CommercialPricingChargeUnit|CommercialPricingMethod|CommercialPricingRateRecord|CommercialPricingPlanCreateInput|CommercialPricingRateCreateInput|CommercialPricingLifecycleSynchronizationReport)/,
  );
  assert.doesNotMatch(typesSource, /export \* from '\.\/commercePayments';/);
  assert.doesNotMatch(typesSource, /coupons: CouponRecord\[]/);
  assert.doesNotMatch(typesSource, /couponTemplates: CouponTemplateRecord\[]/);
  assert.doesNotMatch(typesSource, /campaignBudgets: CampaignBudgetRecord\[]/);
  assert.doesNotMatch(typesSource, /couponCodes: CouponCodeRecord\[]/);
  assert.doesNotMatch(typesSource, /couponReservations: CouponReservationRecord\[]/);
  assert.doesNotMatch(typesSource, /couponRedemptions: CouponRedemptionRecord\[]/);
  assert.doesNotMatch(typesSource, /couponRollbacks: CouponRollbackRecord\[]/);
  assert.doesNotMatch(typesSource, /commerceOrders: CommerceOrderRecord\[]/);
  assert.doesNotMatch(typesSource, /commercePaymentEvents: CommercePaymentEventRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialAccounts: CommercialAccountSummary\[]/);
  assert.doesNotMatch(typesSource, /commercialAccountHolds: CommercialAccountHoldRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialAccountLedger: CommercialAccountLedgerHistoryEntry\[]/);
  assert.doesNotMatch(
    typesSource,
    /commercialRequestSettlements: CommercialRequestSettlementRecord\[]/,
  );
  assert.doesNotMatch(typesSource, /commercialPricingPlans: CommercialPricingPlanRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialPricingRates: CommercialPricingRateRecord\[]/);
  assert.doesNotMatch(workbench, /commerceOrders|commercePaymentEvents|couponTemplates|campaignBudgets|couponCodes|couponReservations|couponRedemptions|couponRollbacks|commercialAccounts|commercialAccountHolds|commercialAccountLedger|commercialRequestSettlements|commercialPricingPlans|commercialPricingRates/);
  assert.doesNotMatch(workbenchSnapshot, /coupons: \[]|couponTemplates: \[]|campaignBudgets: \[]|couponCodes: \[]|couponReservations: \[]|couponRedemptions: \[]|couponRollbacks: \[]|commerceOrders: \[]|commercePaymentEvents: \[]|commercialAccounts: \[]|commercialAccountHolds: \[]|commercialAccountLedger: \[]|commercialRequestSettlements: \[]|commercialPricingPlans: \[]|commercialPricingRates: \[]/);
});

test('app header avoids import meta asset resolution to stay worktree-safe', () => {
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');

  assert.doesNotMatch(header, /import\.meta\.url/);
  assert.doesNotMatch(header, /new URL\(/);
  assert.match(header, /dataSlot="app-header-search"/);
  assert.match(header, /Ctrl K/);
  assert.match(header, /setCommandPaletteOpen|openCommandPalette/);
  assert.match(header, /Open command center|Open command search/);
});

test('shell command center is implemented as an in-place command palette', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const commandPalette = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const store = read('packages/sdkwork-craw-chat-admin-core/src/store.ts');

  assert.match(layout, /CommandPalette/);
  assert.match(header, /setCommandPaletteOpen|openCommandPalette/);
  assert.match(commandPalette, /SearchCommandPalette/);
  assert.match(commandPalette, /adminRouteManifest/);
  assert.match(commandPalette, /prefetchSidebarRoute/);
  assert.match(commandPalette, /refreshWorkspace/);
  assert.match(commandPalette, /handleLogout/);
  assert.match(commandPalette, /Command center/);
  assert.match(shellIndex, /CommandPalette/);
  assert.match(store, /isCommandPaletteOpen/);
  assert.match(store, /commandSearchValue/);
});

test('shell exposes a persistent operations pulse drawer for cross-route continuity', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const operationsPulse = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const store = read('packages/sdkwork-craw-chat-admin-core/src/store.ts');

  assert.match(layout, /OperationsPulseDrawer/);
  assert.match(header, /dataSlot="app-header-pulse"/);
  assert.match(header, /openOperationsPulse|setOperationsPulseOpen/);
  assert.match(operationsPulse, /Operations pulse/);
  assert.match(operationsPulse, /Incident watch/);
  assert.match(operationsPulse, /Shift handoff/);
  assert.match(operationsPulse, /First response SLA/);
  assert.match(operationsPulse, /Reconnect watch/);
  assert.match(operationsPulse, /Retry queue/);
  assert.match(shellIndex, /OperationsPulseDrawer/);
  assert.match(store, /isOperationsPulseOpen/);
  assert.match(store, /openOperationsPulse/);
  assert.match(store, /closeOperationsPulse/);
});

test('shell exposes a persistent route context strip for active module governance', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const routeContext = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/RouteContextStrip.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');

  assert.match(layout, /RouteContextStrip/);
  assert.match(routeContext, /adminRouteManifest/);
  assert.match(routeContext, /adminRouteKeyFromPathname/);
  assert.match(routeContext, /Continuity cue/);
  assert.match(routeContext, /Required permissions/);
  assert.match(routeContext, /Capability tags/);
  assert.match(routeContext, /Open command center/);
  assert.match(routeContext, /Open operations pulse|Open pulse/);
  assert.match(routeContext, /Open settings center|Operations directory/);
  assert.match(shellIndex, /RouteContextStrip/);
});

test('legacy router-admin subapps are removed from IM module packages', () => {
  const removedPaths = [
    'packages/sdkwork-craw-chat-admin-overview/src/view-model.ts',
    'packages/sdkwork-craw-chat-admin-core/src/commercialPricing.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslations.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsCommercial.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsCore.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsRecovery.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsRouting.ts',
    'packages/sdkwork-craw-chat-admin-admin-api/src/commerce.ts',
    'packages/sdkwork-craw-chat-admin-users/src/page/OperatorUserDialog.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/PortalUserDialog.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogChannelDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogChannelModelDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogCredentialDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDialogs.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogModelPriceDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogProviderDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/useCatalogWorkspaceState.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponDialog.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/billingEventAnalytics.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayAccessPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayModelMappingsPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayRateLimitsPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayRoutesPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayUsagePage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/shared.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/useGatewayAccessWorkspaceState.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessForms.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyCreateDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyEditDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyGroupsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyRouteDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyUsageDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingEditorDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitPolicyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayProviderDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutingProfilesDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutingSnapshotsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/routingSnapshotAnalytics.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayApiKeyAccessService.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayOverlayStore.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayViewService.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/commercialOrderAuditDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/commercialOverviewSections.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/formatters.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/ledgerTimeline.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/orderAuditLookup.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/orderPaymentAudit.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentCredentialBindingsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentMethodDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentMethodManagerSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentOrderOperationsSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentReconciliationSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentRefundDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentShared.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentWebhookInboxSection.tsx',
  ];

  for (const relativePath of removedPaths) {
    assert.equal(
      existsSync(path.join(appRoot, relativePath)),
      false,
      `${relativePath} should be removed from the IM admin workspace`,
    );
  }
});

test('tenants module restores router-admin page decomposition for IM governance workflows', () => {
  const tenantPageFiles = [
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  ];

  for (const relativePath of tenantPageFiles) {
    assert.equal(
      existsSync(path.join(appRoot, relativePath)),
      true,
      `${relativePath} should exist for the tenant registry/detail workflow`,
    );
  }

  const tenantsIndex = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const registrySection = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  );
  const detailDrawer = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
  );
  const projectDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx');
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const plaintextApiKeyDialog = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
  );

  assert.match(tenantsIndex, /TenantsRegistrySection/);
  assert.match(tenantsIndex, /TenantsDetailDrawer/);
  assert.match(tenantsIndex, /TenantDialog/);
  assert.match(tenantsIndex, /ProjectDialog/);
  assert.match(tenantsIndex, /ApiKeyDialog/);
  assert.match(tenantsIndex, /PlaintextApiKeyDialog/);
  assert.match(tenantsIndex, /handleSaveTenant/);
  assert.match(tenantsIndex, /handleSaveProject/);
  assert.match(tenantsIndex, /handleCreateApiKey/);
  assert.match(registrySection, /DataTable/);
  assert.match(registrySection, /Issue key/);
  assert.match(registrySection, /New project/);
  assert.match(detailDrawer, /Key issuance ready|Key issuance guardrail/);
  assert.match(detailDrawer, /Issue key/);
  assert.match(projectDialog, /Project profile|New project|Save project/);
  assert.match(apiKeyDialog, /Issue key|Gateway key profile|Environment/);
  assert.match(plaintextApiKeyDialog, /Plaintext key|Copy key|Operator handoff/);
});

test('tenants page decomposition stays on the router-admin root ui entrypoint', () => {
  const tenantSources = [
    'packages/sdkwork-craw-chat-admin-tenants/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx',
  ];

  for (const relativePath of tenantSources) {
    const source = read(relativePath);

    assert.doesNotMatch(
      source,
      /@sdkwork\/ui-pc-react\/components\/ui(?:\/|')/,
      `${relativePath} should rely on the root ui entrypoint instead of grouped ui runtime imports`,
    );
    assert.match(
      source,
      /@sdkwork\/ui-pc-react'/,
      `${relativePath} should import from the root @sdkwork/ui-pc-react entrypoint`,
    );
  }
});
