#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';

function read(rootDir, relativePath) {
  return readFileSync(path.join(rootDir, relativePath), 'utf8');
}

export function verifySdkSiteDocs(options = {}) {
  const rootDir = options.rootDir || path.resolve(import.meta.dirname, '..', '..', '..');
  const failures = [];

  const docsPackageSource = read(rootDir, 'docs/sites/package.json');
  const vitepressConfigSource = read(rootDir, 'docs/sites/.vitepress/config.mjs');
  const apiReferenceSidebarSource = read(rootDir, 'docs/sites/.vitepress/api-reference-sidebar.mjs');
  const architectureOverviewSource = read(rootDir, 'docs/sites/architecture/overview.md');
  const runtimeTopologySource = read(rootDir, 'docs/sites/architecture/runtime-topology.md');
  const moduleMapSource = read(rootDir, 'docs/sites/architecture/module-map.md');
  const deploymentIndexSource = read(rootDir, 'docs/sites/deployment/index.md');
  const dockerSource = read(rootDir, 'docs/sites/deployment/docker.md');
  const localBinarySource = read(rootDir, 'docs/sites/deployment/local-binary.md');
  const runtimeOperationsSource = read(rootDir, 'docs/sites/deployment/runtime-operations.md');
  const serverLifecycleSource = read(rootDir, 'docs/sites/deployment/server-lifecycle.md');
  const indexSource = read(rootDir, 'docs/sites/sdk/index.md');
  const appSource = read(rootDir, 'docs/sites/sdk/app-sdk.md');
  const adminSource = read(rootDir, 'docs/sites/sdk/admin-sdk.md');
  const managementSource = read(rootDir, 'docs/sites/sdk/management-sdk.md');
  const languageSupportSource = read(rootDir, 'docs/sites/sdk/language-support.md');
  const sdkReleaseCatalog = JSON.parse(
    read(rootDir, 'artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json'),
  );
  const siteHomeSource = read(rootDir, 'docs/sites/index.md');
  const gettingStartedIndexSource = read(rootDir, 'docs/sites/getting-started/index.md');
  const featureOverviewSource = read(rootDir, 'docs/sites/features/index.md');
  const capabilitiesSource = read(rootDir, 'docs/sites/features/capabilities.md');
  const cliReferenceSource = read(rootDir, 'docs/sites/reference/cli-and-scripts.md');
  const apiReferenceIndexSource = read(rootDir, 'docs/sites/api-reference/index.md');
  const authAndErrorsSource = read(rootDir, 'docs/sites/api-reference/auth-and-errors.md');
  const appApiSource = read(rootDir, 'docs/sites/api-reference/app-api.md');
  const platformApiSource = read(rootDir, 'docs/sites/api-reference/platform-api.md');
  const iotApiSource = read(rootDir, 'docs/sites/api-reference/iot-api.md');
  const controlPlaneApiSource = read(rootDir, 'docs/sites/api-reference/control-plane-api.md');
  const quickStartSource = read(rootDir, 'docs/sites/getting-started/quick-start.md');
  const runtimeDirectorySource = read(rootDir, 'docs/sites/reference/runtime-directory.md');

  const docsPackage = JSON.parse(docsPackageSource);
  const docsBuildScript = docsPackage.scripts?.['docs:build'] || '';
  const docsDevScript = docsPackage.scripts?.['docs:dev'] || '';
  const docsGenerateScript = docsPackage.scripts?.['docs:generate'] || '';
  const docsPreviewScript = docsPackage.scripts?.['docs:preview'] || '';
  const docsVerifyScript = docsPackage.scripts?.['docs:verify'] || '';

  const expectedDocsTasks = new Map([
    ['docs:generate', [docsGenerateScript, 'generate']],
    ['docs:dev', [docsDevScript, 'dev']],
    ['docs:build', [docsBuildScript, 'build']],
    ['docs:preview', [docsPreviewScript, 'preview']],
    ['docs:verify', [docsVerifyScript, 'verify']],
  ]);

  for (const [scriptName, [scriptValue, taskName]] of expectedDocsTasks) {
    if (!scriptValue.includes(`./scripts/run-docs-task.mjs ${taskName}`)) {
      failures.push(`docs/sites/package.json must route ${scriptName} through scripts/run-docs-task.mjs ${taskName}.`);
    }
    if (!scriptValue.includes('npm_node_execpath')) {
      failures.push(`docs/sites/package.json must provide an npm_node_execpath fallback for ${scriptName}.`);
    }
  }

  if (!vitepressConfigSource.includes('{ text: "Management SDK", link: "/sdk/management-sdk" }')) {
    failures.push('docs/sites/.vitepress/config.mjs must include the Management SDK page in the SDK sidebar.');
  }
  if (!vitepressConfigSource.includes('{ text: "Server Lifecycle", link: "/deployment/server-lifecycle" }')) {
    failures.push('docs/sites/.vitepress/config.mjs must include the Server Lifecycle page in the deployment sidebar.');
  }
  if (!apiReferenceSidebarSource.includes('{ text: "Gateway OpenAPI", link: "/api-reference/gateway-openapi" }')) {
    failures.push('docs/sites/.vitepress/api-reference-sidebar.mjs must include the Gateway OpenAPI page in the API overview sidebar.');
  }

  const expectedCliEntries = [
    '`npm run docs:generate`',
    '`npm run docs:dev`',
    '`npm run docs:build`',
    '`npm run docs:preview`',
    '`npm run docs:verify`',
    '`scripts/run-docs-task.cmd verify`',
    '`powershell -ExecutionPolicy Bypass -File scripts/run-docs-task.ps1 verify`',
    '`sh scripts/run-docs-task.sh verify`',
    'Install docs dependencies first:',
  ];

  for (const expectedEntry of expectedCliEntries) {
    if (!cliReferenceSource.includes(expectedEntry)) {
      failures.push(`docs/sites/reference/cli-and-scripts.md must mention ${expectedEntry}.`);
    }
  }

  if (!/Run `npm ci` inside `docs\/sites` before any `docs:\*` task\./.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must explicitly instruct users to run npm ci inside docs/sites before docs tasks.');
  }
  if (!/route through `scripts\/run-docs-task\.mjs` and keep an/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must explain that npm scripts route through scripts/run-docs-task.mjs and keep an npm_node_execpath fallback.');
  }
  if (!/`docs:generate`, `docs:verify`, `docs:build`, and `docs:dev` first standardize the overview API markdown and then regenerate operation pages/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must document that generate, verify, build, and dev first standardize source API markdown and then regenerate operation pages.');
  }
  if (!/`docs:verify` and `docs:build` also run API and SDK docs verification before invoking VitePress build-time work/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must document that docs:verify and docs:build perform API and SDK docs verification before VitePress build-time work.');
  }
  if (!/`docs:preview` serves the already built site and does not mutate docs content/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must document that docs:preview only serves the previously built site.');
  }
  if (!/direct `.cmd` and `.ps1` wrappers remain the cleanest explicit entrypoints/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must explain that the direct Windows wrappers are the cleanest explicit entrypoints.');
  }
  if (!/use `docs:verify` in-place and run the VitePress commands from a normal local terminal/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must document the restricted-shell VitePress limitation and docs:verify fallback.');
  }
  for (const expectedServerEntry of [
    '`install-server.*`',
    '`init-config-server.*`',
    '`init-storage-server.*`',
    '`verify-server.*`',
    '`start-server.*`',
    '`install-service-server.*`',
    '`plan-release-server.*`',
  ]) {
    if (!cliReferenceSource.includes(expectedServerEntry)) {
      failures.push(`docs/sites/reference/cli-and-scripts.md must mention ${expectedServerEntry}.`);
    }
  }
  if (!/craw-chat-server --config <config-root>\/server\.yaml/.test(cliReferenceSource)) {
    failures.push('docs/sites/reference/cli-and-scripts.md must document the canonical craw-chat-server startup contract.');
  }

  if (/all four artifacts have `generationStatus = template_only_pending_generation`/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must not claim all SDK artifacts are template-only.');
  }
  if (!/The repository currently defines three SDK families with different consumers, contracts, and release/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must describe the repository as having three SDK families.');
  }
  if (!/For day-to-day engineering, treat the checked-in SDK workspaces and their `.sdkwork-assembly\.json`/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must describe checked-in workspaces and assembly files as the current engineering truth.');
  }
  if (!/### Release Snapshot/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must keep a dedicated release snapshot section.');
  }
  if (!/state = generated_pending_publication/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must document the generated_pending_publication release-catalog state.');
  }
  if (!/generationStatus = generated/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must document that the tracked release-catalog artifacts are generated.');
  }
  if (!/sdkwork-craw-chat-sdk-management` has materialized TypeScript and Flutter package workspaces/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must describe the management family as TypeScript-and-Flutter materialized.');
  }
  if (!/Generated symbols must be consumed through package root entrypoints only\./.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must document the package-root-only consumption rule.');
  }
  if (!/App SDK consumers target `local-minimal-node` during direct local development and the unified\s+`craw-chat-server` \/ `web-gateway` public origin in packaged installs\./.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must summarize app SDK endpoint targeting across local and packaged installs.');
  }
  if (!/Admin SDK consumers can target `control-plane-api` directly during standalone governance\s+development, but packaged installs should switch to the unified gateway public origin\./.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must summarize admin SDK endpoint targeting across standalone and packaged installs.');
  }
  if (!/Management SDK consumers target the deployed surface that serves `\/api\/admin\/\*`; in packaged\s+installs that is also the unified gateway public origin\./.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must summarize management SDK endpoint targeting.');
  }

  for (const [label, source, expectedPackage] of [
    ['app', appSource, '@sdkwork/craw-chat-backend-sdk'],
    ['admin', adminSource, '@sdkwork/craw-chat-admin-backend-sdk'],
    ['management', managementSource, '@sdkwork/craw-chat-management-backend-sdk'],
  ]) {
    if (!source.includes(expectedPackage)) {
      failures.push(`docs/sites/sdk/${label}-sdk.md must mention ${expectedPackage}.`);
    }
    if (!/generated\/server-openapi\/src\/\*/.test(source)) {
      failures.push(`docs/sites/sdk/${label}-sdk.md must explicitly forbid generated/server-openapi/src/* private path imports.`);
    }
  }

  if (!/CrawChatSdkClient/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must name the primary TypeScript client as CrawChatSdkClient.');
  }
  if (/createCrawChatClient/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must not mention the removed createCrawChatClient TypeScript compatibility helper.');
  }
  if (!/new CrawChatSdkClient\(\{/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must document synchronous new CrawChatSdkClient({ ... }) construction for TypeScript consumers.');
  }
  if (/backendConfig/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must not document backendConfig as the public TypeScript app client surface.');
  }
  if (!/For `local-minimal-node` development, set `baseUrl` to the node origin such as\s+`http:\/\/127\.0\.0\.1:18090`\./.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must explain that local app SDK development points baseUrl at the local-minimal-node origin.');
  }
  if (!/For packaged installs, set `baseUrl` to the unified `craw-chat-server` \/ `web-gateway` public\s+origin documented in \[Gateway OpenAPI\]\(\/api-reference\/gateway-openapi\) and\s+\[Server Lifecycle\]\(\/deployment\/server-lifecycle\)\./.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must direct packaged app SDK consumers to the unified gateway public origin.');
  }
  if (!/The live websocket handshake at `GET \/api\/v1\/realtime\/ws` uses that same public origin/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must explain that the live realtime websocket handshake uses the same public origin as the HTTP app API.');
  }
  if (!/node \.\/sdks\/sdkwork-craw-chat-sdk\/bin\/verify-sdk\.mjs/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must document the app SDK verification command.');
  }
  if (!/composed smoke test for `CrawChatSdkClient`/.test(appSource)) {
    failures.push('docs/sites/sdk/app-sdk.md must document the CrawChatSdkClient smoke test in the verification path.');
  }
  if (!/CrawChatAdminSdkClient\.create\(\{[\s\S]*baseUrl:/.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must document CrawChatAdminSdkClient.create({ baseUrl: ... }) for TypeScript consumers.');
  }
  if (!/createCrawChatAdminSdkClient/.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must mention createCrawChatAdminSdkClient for TypeScript consumers.');
  }
  if (!/craw_chat_admin_backend_sdk/.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must mention the generated Flutter admin backend package.');
  }
  if (!/CrawChatAdminSdkClient/.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must mention the primary Flutter admin client as CrawChatAdminSdkClient.');
  }
  if (!/For standalone governance development, `baseUrl` can point directly at `control-plane-api`,\s+which defaults to `http:\/\/127\.0\.0\.1:18081`\./.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must explain the standalone control-plane baseUrl target.');
  }
  if (!/For packaged installs, point `baseUrl` at the unified `craw-chat-server` \/ `web-gateway`\s+public origin; the gateway proxies control-plane routes on the same external port as the other\s+operator-facing HTTP surfaces\./.test(adminSource)) {
    failures.push('docs/sites/sdk/admin-sdk.md must direct packaged admin SDK consumers to the unified gateway public origin.');
  }
  if (!/CrawChatSdkManagementClient/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must name the primary TypeScript client as CrawChatSdkManagementClient.');
  }
  if (!/createCrawChatSdkManagementClient/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must mention createCrawChatSdkManagementClient for TypeScript consumers.');
  }
  if (!/craw_chat_management_backend_sdk/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must mention the generated Flutter management backend package.');
  }
  if (!/CrawChatManagementClient/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must mention the primary Flutter management client as CrawChatManagementClient.');
  }
  if (!/CrawChatSdkManagementClient\.create\(\{ backendConfig \}\)/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must document CrawChatSdkManagementClient.create({ backendConfig }) for TypeScript consumers.');
  }
  if (!/Point `baseUrl` at the deployed surface that serves the checked-in `\/api\/admin\/\*` contract\./.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must explain that baseUrl targets the deployed /api/admin/* surface.');
  }
  if (!/In packaged installs, that is the unified `craw-chat-server` \/ `web-gateway` origin\./.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must explain that packaged management SDK consumers target the unified gateway origin.');
  }

  if (/before the language generators are wired/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must not describe management generation as not yet wired.');
  }
  if (!/generated TypeScript transport package is materialized as/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must describe the materialized management TypeScript transport package.');
  }
  if (!/Verify the materialized management SDK workspace/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must document verification of the materialized management SDK workspace.');
  }
  if (!/generated Flutter transport package is materialized as/.test(managementSource)) {
    failures.push('docs/sites/sdk/management-sdk.md must describe the materialized management Flutter transport package.');
  }

  const expectedLanguageRows = [
    '| App | TypeScript | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript` | Workspace materialized, locally verifiable, publication still pending |',
    '| App | Flutter | `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter` | Workspace materialized, locally verifiable, publication still pending |',
    '| Admin | TypeScript | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript` | Workspace materialized, locally verifiable, publication still pending |',
    '| Admin | Flutter | `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter` | Workspace materialized, locally verifiable, publication still pending |',
    '| Management | TypeScript | `sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-typescript` | Workspace materialized, locally verifiable, publication still pending |',
    '| Management | Flutter | `sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-flutter` | Workspace materialized, locally verifiable, publication still pending |',
  ];

  for (const row of expectedLanguageRows) {
    if (!languageSupportSource.includes(row)) {
      failures.push(`docs/sites/sdk/language-support.md is missing expected workspace matrix row: ${row}`);
    }
  }
  if (!/This page separates two questions:/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must distinguish repository-usable workspaces from historical release publication state.');
  }
  if (!/## Practical Interpretation/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must include a practical interpretation section.');
  }
  if (!/app TypeScript uses generated `@sdkwork\/craw-chat-backend-sdk` and composed `@sdkwork\/craw-chat-sdk`/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must list the materialized app TypeScript generated and composed package roots.');
  }
  if (!/admin TypeScript uses generated `@sdkwork\/craw-chat-admin-backend-sdk` and composed `@sdkwork\/craw-chat-admin-sdk`/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must list the materialized admin TypeScript generated and composed package roots.');
  }
  if (!/management TypeScript uses generated `@sdkwork\/craw-chat-management-backend-sdk` and composed `@sdkwork\/craw-chat-sdk-management`/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must list the materialized management TypeScript generated and composed package roots.');
  }
  if (!/admin Flutter uses generated `craw_chat_admin_backend_sdk` and composed `craw_chat_admin_sdk`/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must list the materialized admin Flutter generated and composed package roots.');
  }
  if (!/management Flutter uses generated `craw_chat_management_backend_sdk` and composed `craw_chat_sdk_management`/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must list the materialized management Flutter generated and composed package roots.');
  }
  if (!/management SDK consumers target the deployed `\/api\/admin\/\*` surface; in packaged installs that\s+surface is also reached through the unified gateway public origin/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must summarize management runtime entry targeting.');
  }
  if (!/admin TypeScript and Flutter are available as checked-in workspaces/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must explain that admin TypeScript and Flutter are both available as checked-in workspaces.');
  }
  if (!/management TypeScript and Flutter are available as checked-in workspaces/.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must explain that management TypeScript and Flutter are both available as checked-in workspaces.');
  }

  const expectedArtifactRows = sdkReleaseCatalog.sdkArtifacts.map(
    (artifact) =>
      `| \`${artifact.id}\` | ${artifact.audience} | ${artifact.language} | \`${artifact.generationStatus}\` | \`${artifact.releaseStatus}\` |`,
  );
  for (const row of expectedArtifactRows) {
    if (!languageSupportSource.includes(row)) {
      failures.push(`docs/sites/sdk/language-support.md is missing expected release row: ${row}`);
    }
  }
  if (!/The current release catalog and the checked-in workspaces agree that all six language lines are\s+generated locally and remain unpublished\./.test(languageSupportSource)) {
    failures.push('docs/sites/sdk/language-support.md must explain that the release catalog and checked-in workspaces now agree on generated-but-unpublished SDK state.');
  }

  if (/SDK workspace structure is already checked in/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must not describe SDK delivery as only workspace structure being checked in.');
  }
  if (!/App, Admin, and Management SDK families all have materialized TypeScript and Flutter workspaces in-repo/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must summarize the current materialized SDK workspaces.');
  }
  if (!/services\/web-gateway`, the unified external entrypoint that publishes the canonical/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must describe services/web-gateway as the unified external entrypoint and packaged server source.');
  }
  if (!/Use \[Server Lifecycle\]\(\/deployment\/server-lifecycle\)/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must include the Server Lifecycle page in the recommended reading path.');
  }
  if (/admin SDK workspace with frozen audience and language boundaries but no checked-in admin OpenAPI source yet/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must not describe the admin SDK as lacking a checked-in OpenAPI source.');
  }
  if (!/admin SDK workspace with checked-in OpenAPI authority plus materialized TypeScript and Flutter package lines/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must describe the admin SDK as having checked-in authority and materialized TypeScript and Flutter package lines.');
  }
  if (!/management SDK workspace with checked-in OpenAPI authority, derived sdkgen input, and materialized TypeScript and Flutter package lines/.test(siteHomeSource)) {
    failures.push('docs/sites/index.md must describe the management SDK as having materialized TypeScript and Flutter package lines.');
  }

  if (!/Operator-console admin SDK \| Checked-in authority, derived sdkgen input, assembly snapshot, and materialized TypeScript and Flutter generated\/composed packages now live under `sdks\/sdkwork-craw-chat-sdk-management\/`/.test(capabilitiesSource)) {
    failures.push('docs/sites/features/capabilities.md must describe the operator-console SDK as TypeScript-and-Flutter materialized.');
  }
  if (!/admin console already consumes the management SDK TypeScript packages through its compatibility layer/.test(capabilitiesSource)) {
    failures.push('docs/sites/features/capabilities.md must explain that the admin console already consumes the management SDK TypeScript packages through its compatibility layer.');
  }
  if (!/App TypeScript and Flutter packages \| Both app language workspaces are materialized and locally verifiable/.test(capabilitiesSource)) {
    failures.push('docs/sites/features/capabilities.md must describe the app SDK workspaces as materialized and locally verifiable.');
  }
  if (!/Admin TypeScript and Flutter packages \| Both admin language workspaces are materialized and locally verifiable; release publication is still pending/.test(capabilitiesSource)) {
    failures.push('docs/sites/features/capabilities.md must describe the admin SDK as materialized across both TypeScript and Flutter.');
  }
  if (/Frozen workspace boundary/.test(featureOverviewSource)) {
    failures.push('docs/sites/features/index.md must not describe SDK boundaries as frozen planned-delivery shape.');
  }
  if (!/Materialized SDK boundary: checked-in OpenAPI authority, generated packages, or assembled/.test(featureOverviewSource)) {
    failures.push('docs/sites/features/index.md must describe materialized SDK boundaries in terms of checked-in authority or generated packages.');
  }
  if (!/Scaffolded SDK boundary: the workspace contract is reserved and named, but one or more language/.test(featureOverviewSource)) {
    failures.push('docs/sites/features/index.md must distinguish scaffold-only SDK boundaries from materialized ones.');
  }

  if (!/documentation is easiest to understand through five architectural lenses/.test(architectureOverviewSource)) {
    failures.push('docs/sites/architecture/overview.md must describe the architecture through five lenses including the unified gateway.');
  }
  if (!/Unified server binary \| `services\/web-gateway` with `\[\[bin\]\] name = "craw-chat-server"`/.test(architectureOverviewSource)) {
    failures.push('docs/sites/architecture/overview.md must document services/web-gateway as the source of the craw-chat-server binary.');
  }
  if (!/GET \/openapi\.json`, `GET \/openapi\/index\.json`, and/.test(architectureOverviewSource)) {
    failures.push('docs/sites/architecture/overview.md must document the unified gateway discovery endpoints.');
  }
  if (!/web-gateway \/ craw-chat-server \(:18080 by server template\)/.test(runtimeTopologySource)) {
    failures.push('docs/sites/architecture/runtime-topology.md must include the unified web-gateway / craw-chat-server topology.');
  }
  if (!/startup contract: `craw-chat-server --config <config-root>\/server\.yaml`/.test(runtimeTopologySource)) {
    failures.push('docs/sites/architecture/runtime-topology.md must document the canonical craw-chat-server startup contract.');
  }
  if (!/`web-gateway` \| Unified external entrypoint, aggregate OpenAPI export, service-schema proxies, rendered docs, and canonical `craw-chat-server` binary/.test(moduleMapSource)) {
    failures.push('docs/sites/architecture/module-map.md must list web-gateway as the unified external entrypoint and packaged server source.');
  }
  if (!/Checked-in OpenAPI authority now exists for the app, admin, and management SDK workspaces/.test(moduleMapSource)) {
    failures.push('docs/sites/architecture/module-map.md must describe checked-in OpenAPI authority for app, admin, and management SDK workspaces.');
  }

  if (!/Unified server lifecycle \| `bin\/install-server\.\*`, `bin\/start-server\.\*`, `bin\/verify-server\.\*`/.test(gettingStartedIndexSource)) {
    failures.push('docs/sites/getting-started/index.md must list the unified server lifecycle runtime mode.');
  }
  if (!/Want the packaged server install contract: \[Server Lifecycle\]\(\/deployment\/server-lifecycle\)/.test(gettingStartedIndexSource)) {
    failures.push('docs/sites/getting-started/index.md must link to the Server Lifecycle page.');
  }
  if (!/`sdkwork-craw-chat-sdk` maps to the app-facing routes. Local development points at\s+`local-minimal-node`; packaged installs point at the unified `craw-chat-server` \/\s+`web-gateway` public origin\./.test(gettingStartedIndexSource)) {
    failures.push('docs/sites/getting-started/index.md must summarize app SDK entry targets across local and packaged installs.');
  }
  if (!/`sdkwork-craw-chat-sdk-admin` maps to governance and control-plane routes. Standalone governance\s+development can point directly at `control-plane-api`; packaged installs should switch to the\s+unified gateway public origin\./.test(gettingStartedIndexSource)) {
    failures.push('docs/sites/getting-started/index.md must summarize admin SDK entry targets across standalone and packaged installs.');
  }
  if (!/`sdkwork-craw-chat-sdk-management` maps to the deployed `\/api\/admin\/\*` surface. In packaged\s+installs that surface is also reached through the unified gateway public origin\./.test(gettingStartedIndexSource)) {
    failures.push('docs/sites/getting-started/index.md must summarize management SDK entry targets.');
  }
  if (!/If you need the packaged single-port server contract instead of the local development profile, use/.test(quickStartSource)) {
    failures.push('docs/sites/getting-started/quick-start.md must direct packaged-server readers to the Server Lifecycle page.');
  }
  if (!/For local app-SDK integration against this development profile, use `baseUrl = http:\/\/127\.0\.0\.1:18090`\./.test(quickStartSource)) {
    failures.push('docs/sites/getting-started/quick-start.md must tell local app SDK consumers to target the local-minimal-node origin.');
  }

  if (!/Unified server lifecycle \| `bin\/install-server\.\*`, `bin\/init-config-server\.\*`, `bin\/init-storage-server\.\*`, `bin\/verify-server\.\*`, `bin\/install-service-server\.\*`, `bin\/start-server\.\*`/.test(deploymentIndexSource)) {
    failures.push('docs/sites/deployment/index.md must document the unified server lifecycle deployment mode.');
  }
  if (!/the canonical binary is `craw-chat-server`/.test(deploymentIndexSource)) {
    failures.push('docs/sites/deployment/index.md must document the canonical craw-chat-server binary in the current server boundary section.');
  }
  if (!/Server Lifecycle\]\(\/deployment\/server-lifecycle\)/.test(deploymentIndexSource)) {
    failures.push('docs/sites/deployment/index.md must link to the Server Lifecycle page.');
  }

  if (!/The formal packaged server contract is centered on the unified `web-gateway` service and the/.test(serverLifecycleSource)) {
    failures.push('docs/sites/deployment/server-lifecycle.md must describe the unified web-gateway as the packaged server contract center.');
  }
  if (!/startup command: `craw-chat-server --config <config-root>\/server\.yaml`/.test(serverLifecycleSource)) {
    failures.push('docs/sites/deployment/server-lifecycle.md must document the canonical craw-chat-server startup command.');
  }
  if (!/GET \/openapi\/runtime-summary\.json/.test(serverLifecycleSource)) {
    failures.push('docs/sites/deployment/server-lifecycle.md must document the runtime-summary endpoint.');
  }
  if (!/artifacts\/releases\/wave-d-2026-04-08\/server\/release-gate\.json/.test(serverLifecycleSource)) {
    failures.push('docs/sites/deployment/server-lifecycle.md must mention the machine-readable server release-gate bundle.');
  }
  if (!/it is not the\s+formal packaged `craw-chat-server` install contract/.test(localBinarySource)) {
    failures.push('docs/sites/deployment/local-binary.md must distinguish the local binary workflow from the formal craw-chat-server install contract.');
  }
  if (!/Switch to \[Server Lifecycle\]\(\/deployment\/server-lifecycle\)/.test(localBinarySource)) {
    failures.push('docs/sites/deployment/local-binary.md must direct packaged-server readers to the Server Lifecycle page.');
  }
  if (!/It is not the\s+formal packaged `craw-chat-server` install contract/.test(dockerSource)) {
    failures.push('docs/sites/deployment/docker.md must distinguish Docker local validation from the formal craw-chat-server install contract.');
  }
  if (!/For the single-port packaged server, config\s+root layout, PostgreSQL baseline, and service-management wrappers, use/.test(dockerSource)) {
    failures.push('docs/sites/deployment/docker.md must direct packaged-server readers to Server Lifecycle for config-root, PostgreSQL, and service wrappers.');
  }
  if (!/production-style install shape with the unified `web-gateway` entrypoint, runtime\s+OpenAPI discovery, or PostgreSQL-backed storage configuration, switch to/.test(dockerSource)) {
    failures.push('docs/sites/deployment/docker.md must state that production-style web-gateway and PostgreSQL installs belong to Server Lifecycle.');
  }
  if (!/It does not replace the formal packaged `craw-chat-server` contract/.test(runtimeOperationsSource)) {
    failures.push('docs/sites/deployment/runtime-operations.md must distinguish local runtime-dir operations from the packaged craw-chat-server contract.');
  }
  if (!/PostgreSQL is the frozen storage baseline and operators manage a config root/.test(runtimeOperationsSource)) {
    failures.push('docs/sites/deployment/runtime-operations.md must document PostgreSQL and config-root as the packaged-server operations baseline.');
  }
  if (!/diagnostic tooling for development profiles only/.test(runtimeOperationsSource)) {
    failures.push('docs/sites/deployment/runtime-operations.md must position local runtime restore commands as development-profile diagnostics for packaged-server readers.');
  }

  if (!/sdks\/sdkwork-craw-chat-sdk\/openapi\/craw-chat-app\.openapi\.yaml/.test(appApiSource)) {
    failures.push('docs/sites/api-reference/app-api.md must document the checked-in app OpenAPI authority contract path.');
  }
  if (!/In packaged installs, this same app-facing HTTP surface is exposed through the unified `craw-chat-server` \/ `web-gateway` public origin rather than a separate public app-node port\./.test(appApiSource)) {
    failures.push('docs/sites/api-reference/app-api.md must explain that packaged installs expose the app API through the unified gateway origin.');
  }
  if (!/Public deployments of `craw-chat-server` \/ `web-gateway`, `local-minimal-node`, and/.test(authAndErrorsSource)) {
    failures.push('docs/sites/api-reference/auth-and-errors.md must describe bearer auth across craw-chat-server/web-gateway and the underlying services.');
  }

  if (!/SDK families remain boundary-specific: `sdkwork-craw-chat-sdk` maps to the public app API, `sdkwork-craw-chat-sdk-admin` maps to control-plane governance, and `sdkwork-craw-chat-sdk-management` maps to the unified gateway's `\/api\/admin\/\*` operator surface\./.test(apiReferenceIndexSource)) {
    failures.push('docs/sites/api-reference/index.md must document the app/admin/management SDK family split.');
  }
  if (!/For packaged installs, start with \[Gateway OpenAPI\]\(\/api-reference\/gateway-openapi\): app, platform, IoT, control-plane, and `\/api\/admin\/\*` discovery all converge on the same unified public origin\./.test(apiReferenceIndexSource)) {
    failures.push('docs/sites/api-reference/index.md must direct packaged-install readers to Gateway OpenAPI as the unified public-origin discovery entrypoint.');
  }
  if (!/Operator Console Admin API \(`\/api\/admin\/\*`\) \| `sdkwork-craw-chat-sdk-management` \| Checked-in authority plus materialized TypeScript and Flutter generated\/composed packages exist; the admin console already consumes this family through its TypeScript compatibility layer \|?/.test(indexSource)) {
    failures.push('docs/sites/sdk/index.md must describe the operator-console API group as already backed by materialized management SDK TypeScript and Flutter packages.');
  }

  if (!/Operator and `\/api\/admin\/\*` platform surfaces are intended for `sdkwork-craw-chat-sdk-management`; control-plane governance stays in `sdkwork-craw-chat-sdk-admin`; neither belongs in the public `sdkwork-craw-chat-sdk` surface\./.test(platformApiSource)) {
    failures.push('docs/sites/api-reference/platform-api.md must map platform and control-plane boundaries to the correct SDK families.');
  }
  if (!/In packaged installs, these routes are still reached through the unified `craw-chat-server` \/ `web-gateway` public origin even though the implementation remains on the app-node side of the runtime\./.test(platformApiSource)) {
    failures.push('docs/sites/api-reference/platform-api.md must explain that packaged installs reach platform routes through the unified gateway origin.');
  }
  if (!/The local `platform\/\*` routes documented on this page do not currently have a standalone published SDK family\./.test(platformApiSource)) {
    failures.push('docs/sites/api-reference/platform-api.md must explain that local platform routes do not currently have a standalone published SDK family.');
  }
  if (!/The routes documented on this page do not currently have a standalone published SDK family\./.test(iotApiSource)) {
    failures.push('docs/sites/api-reference/iot-api.md must explain that IoT routes do not currently have a standalone published SDK family.');
  }
  if (!/In packaged installs, these IoT routes are still published through the unified `craw-chat-server` \/ `web-gateway` public origin rather than a separate public device-ingress port\./.test(iotApiSource)) {
    failures.push('docs/sites/api-reference/iot-api.md must explain that packaged installs expose IoT routes through the unified gateway origin.');
  }

  if (/does not yet include a checked-in admin OpenAPI authority file/.test(controlPlaneApiSource)) {
    failures.push('docs/sites/api-reference/control-plane-api.md must not claim the admin SDK lacks a checked-in OpenAPI authority file.');
  }
  if (!/sdks\/sdkwork-craw-chat-sdk-admin\/openapi\/craw-chat-control-plane\.openapi\.json/.test(controlPlaneApiSource)) {
    failures.push('docs/sites/api-reference/control-plane-api.md must document the checked-in admin OpenAPI authority contract path.');
  }
  if (!/sdkwork-craw-chat-sdk-admin/.test(controlPlaneApiSource)) {
    failures.push('docs/sites/api-reference/control-plane-api.md must name the admin SDK family.');
  }
  if (!/Standalone governance development can call `control-plane-api` directly, but packaged installs expose the same governance routes through the unified `craw-chat-server` \/ `web-gateway` public origin\./.test(controlPlaneApiSource)) {
    failures.push('docs/sites/api-reference/control-plane-api.md must explain the standalone-versus-packaged access model for governance routes.');
  }

  if (!/These scripts help validate a local node; application integrations should move to the public `sdkwork-craw-chat-sdk` packages instead of scripting raw HTTP once the node is healthy\./.test(quickStartSource)) {
    failures.push('docs/sites/getting-started/quick-start.md must position local verification tools as complementary to the public app SDK.');
  }
  if (!/formal packaged `craw-chat-server` config contract/.test(runtimeDirectorySource)) {
    failures.push('docs/sites/reference/runtime-directory.md must distinguish the local runtime directory contract from the packaged craw-chat-server config contract.');
  }
  if (!/PostgreSQL is the frozen storage baseline through `storage\/postgresql\.yaml`/.test(runtimeDirectorySource)) {
    failures.push('docs/sites/reference/runtime-directory.md must mention the PostgreSQL server baseline for the packaged server path.');
  }

  return failures;
}

const isCli = process.argv[1] && path.resolve(process.argv[1]) === import.meta.filename;

if (isCli) {
  const failures = verifySdkSiteDocs();
  if (failures.length > 0) {
    console.error('[docs/sites/sdk] SDK site docs verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[docs/sites/sdk] SDK site docs verification passed.');
}
