import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function assertModuleSurface(relativePath, patterns) {
  assert.equal(existsSync(path.join(appRoot, relativePath)), true, `missing ${relativePath}`);
  const source = read(relativePath);

  for (const pattern of patterns) {
    assert.match(source, pattern, `${relativePath} missing ${String(pattern)}`);
  }
}

test('overview exposes IM operations posture and hotspots', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-overview/src/index.tsx', [
    /Message throughput/,
    /Moderation backlog/,
    /Online users/,
    /Hot conversations|hot conversations/i,
    /Tenant load|Command board/,
    /Incident watch|Shift handoff/,
  ]);
});

test('workspace operations modules expose tenant, user, and group governance', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx', [
    /Tenant posture/,
    /Organizations|Workspace/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-users/src/index.tsx', [
    /Device posture/,
    /Activation|Ban/,
    /Recovery review|Risk watchlist/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-groups/src/index.tsx', [
    /Group directory/,
    /Membership posture/,
  ]);
});

test('conversation governance modules expose lifecycle, audit, and moderation', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-conversations/src/index.tsx', [
    /Conversation lifecycle/,
    /Handoff|Archive|Freeze/,
    /Handoff SLA|Freeze candidates/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-messages/src/index.tsx', [
    /Message audit/,
    /Export evidence|Search transcript/,
    /Recall review|Retention guardrails/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-moderation/src/index.tsx', [
    /Report queue/,
    /Keyword policy|Blocklist/,
    /Disposition matrix|First response SLA/,
  ]);
});

test('automation, announcements, realtime, and system stay first-class', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-automation/src/index.tsx', [
    /Bot registry/,
    /Automation runs/,
    /Run history|Retry queue/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-announcements/src/index.tsx', [
    /Broadcast tasks/,
    /Delivery posture/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-realtime/src/index.tsx', [
    /Realtime sessions/,
    /RTC posture|Gateway health/,
    /Reconnect watch|Failover window/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-system/src/index.tsx', [
    /Protocol governance/,
    /Compatibility matrix/,
    /Protocol change gate|Rollout risks/,
  ]);
});

test('settings run through an operator-grade settings center without router-admin residue', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/index.tsx', [
    /from '\.\/Settings'/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/Settings.tsx', [
    /SettingsCenter/,
    /Operator workspace|IM operator workspace/,
    /moderation|transcript|realtime|incident/i,
    /Search shortcuts|Operations directory/,
    /Workspace Ops|Conversation Governance|System/,
  ]);

  const settingsSource = read('packages/sdkwork-craw-chat-admin-settings/src/Settings.tsx');
  const routeSource = read('packages/sdkwork-craw-chat-admin-core/src/routes.ts');

  assert.doesNotMatch(settingsSource, /control plane|router-admin|claw-studio/i);
  assert.doesNotMatch(routeSource, /Control Plane/);
  assert.match(routeSource, /Operations|Workspace Ops/);
});

test('shell command center exposes route launch and operator quick actions', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx', [
    /Command center/,
    /Quick actions/,
    /Route launch/,
    /Workspace refresh/,
    /Sign out/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/GeneralSettings.tsx', [
    /Command center/,
    /Operations pulse/,
    /Quick actions|Search shortcuts/,
  ]);
});

test('shell operations pulse keeps incident and handoff risk visible outside overview', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx', [
    /Operations pulse/,
    /Incident watch/,
    /Shift handoff/,
    /First response SLA/,
    /Reconnect watch/,
    /Retry queue/,
    /Rollout risks/,
  ]);
});

test('shell route context strip keeps module governance and continuity cues visible', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/RouteContextStrip.tsx', [
    /Continuity cue/,
    /Required permissions/,
    /Capability tags/,
    /handoff|retention|retry|rollout|delivery|posture/i,
    /Command center|Open command center/,
    /Operations pulse|Open operations pulse/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/GeneralSettings.tsx', [
    /Route context strip/,
    /Continuity cue|Capability tags/,
    /Command center|Operations pulse/,
  ]);
});

test('user-facing IM copy avoids router and commerce fallback residue', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const realtime = read('packages/sdkwork-craw-chat-admin-realtime/src/index.tsx');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.doesNotMatch(tenants, /US commerce/);
  assert.doesNotMatch(automation, /Routing bot|Completed routing sweep/);
  assert.doesNotMatch(realtime, /provider routes|route drain/i);
  assert.doesNotMatch(
    workbenchSnapshot,
    /operator control plane|messaging control plane|The control plane has no published model bindings|quota posture|providers have no credential coverage|admin control plane instead of local mock state/i,
  );
});

test('zh-CN translations cover shell continuity, settings center, and live workspace status strings', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const commandPalette = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx',
  );

  assert.match(i18n, /'Continuity cue':/);
  assert.match(i18n, /'Capability tags':/);
  assert.match(i18n, /'Required permissions':/);
  assert.match(i18n, /'Open settings center':/);
  assert.match(i18n, /'Settings center':/);
  assert.match(i18n, /'Route context strip':/);
  assert.match(i18n, /'Choose the operator workspace language\. Dates, numbers, and shared shell copy follow this setting immediately\.':/);
  assert.match(i18n, /'Current shell posture for the IM operator workspace\.':/);
  assert.match(i18n, /'Theme mode':/);
  assert.match(i18n, /'Theme color':/);
  assert.match(i18n, /'Sidebar mode':/);
  assert.match(i18n, /'Hidden nav items':/);
  assert.match(i18n, /'IM operator':/);
  assert.match(i18n, /'No published channel capabilities are available\. Review integrations before opening live message traffic\.':/);
  assert.match(i18n, /'Connector credentials are missing':/);
  assert.match(
    i18n,
    /upstream connectors have no credential coverage\. Rotate or create credentials before opening live message traffic\./,
  );
  assert.match(i18n, /projects have exhausted their traffic budget\./);
  assert.match(i18n, /'Live admin sync is backend-backed':/);
  assert.doesNotMatch(commandPalette, /è·¯/);
});

test('zh-CN translations cover auth, sidebar, and advanced settings panels', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(i18n, /'Create operator access':/);
  assert.match(i18n, /'Recover access':/);
  assert.match(i18n, /'Request operator access and enter the IM operator workspace after an existing admin provisions your identity\.':/);
  assert.match(i18n, /'Password reset links are not enabled for this workspace yet\. Continue back to sign in with your operator email\.':/);
  assert.match(i18n, /'Operator session':/);
  assert.match(i18n, /'Open app to scan':/);
  assert.match(i18n, /'Open the SDKWork app and scan this code to continue without typing credentials while the operator command post stays protected\.':/);
  assert.match(i18n, /'Local dev credentials are prefilled: \{email\} \/ \{password\}\.':/);
  assert.match(i18n, /'Already have an account\?':/);
  assert.match(i18n, /'No account\?':/);
  assert.match(i18n, /'Back to login':/);
  assert.match(i18n, /'Continue with':/);
  assert.match(i18n, /'Create account':/);
  assert.match(i18n, /'Create a password':/);
  assert.match(i18n, /'Enter your password':/);
  assert.match(i18n, /'Open user menu':/);
  assert.match(i18n, /'Close user menu':/);
  assert.match(i18n, /'Sign in to manage the IM operator workspace':/);
  assert.match(i18n, /'Expand sidebar':/);
  assert.match(i18n, /'Collapse sidebar':/);
  assert.match(i18n, /'Theme posture':/);
  assert.match(i18n, /'Choose how the shell follows light, dark, or system appearance\.':/);
  assert.match(i18n, /'Bright shell with frosted content panes\.':/);
  assert.match(i18n, /'Low-glare operator shell tuned for long moderation and transcript review sessions\.':/);
  assert.match(i18n, /'Follow the device preference automatically\.':/);
  assert.match(i18n, /'Accent':/);
  assert.match(i18n, /'Accent preset':/);
  assert.match(i18n, /'Sidebar behavior':/);
  assert.match(i18n, /'Compact navigation':/);
  assert.match(i18n, /'Visible routes':/);
  assert.match(i18n, /'shell continuity':/);
  assert.match(i18n, /'workspace persistence':/);
  assert.match(i18n, /'Content region':/);
  assert.match(i18n, /'single workspace surface':/);
  assert.match(i18n, /'Appearance':/);
  assert.match(i18n, /'General':/);
  assert.match(i18n, /'Navigation':/);
  assert.match(i18n, /'Operator workspace':/);
  assert.match(i18n, /'Operator shell continuity':/);
  assert.match(i18n, /'IM operator settings center':/);
  assert.match(i18n, /'Search settings':/);
  assert.match(i18n, /'No settings match your search':/);
  assert.match(i18n, /'Try a different keyword or browse the navigation without a search term\.':/);
  assert.match(i18n, /'Clear filters':/);
  assert.match(i18n, /'Shell':/);
  assert.match(i18n, /'Live':/);
});

test('zh-CN translations cover overview and primary workspace governance modules', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const keys = [
    'Ops',
    'Identity',
    'Membership',
    'Broadcast',
    'Lifecycle',
    'Audit',
    'Safety',
    'Bots',
    'Governance',
    'Preferences',
    'Message throughput, moderation backlog, and operator shortcuts',
    'Tenant posture, organizations, and workspace governance',
    'Portal identities, operator accounts, and device posture',
    'Group directory, channel governance, and membership posture',
    'Broadcast tasks, system notices, and delivery posture',
    'Conversation lifecycle, handoff, archive, and freeze posture',
    'Message audit, transcript search, export, and evidence review',
    'Report queues, keyword policy, and escalation governance',
    'Bot registry, automation runs, and operator workflows',
    'Realtime sessions, RTC posture, and gateway health',
    'Protocol governance, compatibility matrix, and runtime health',
    'Theme mode, theme color, and sidebar preferences',
    'IM command center',
    'Message throughput',
    'Moderation backlog',
    'Online users',
    'Hot conversations',
    'Incident watch',
    'Command board',
    'Tenant load',
    'Rolling admission volume across core messaging, audit, and automation lanes.',
    'Open reports awaiting moderator assignment, review, or escalation sign-off.',
    'Authenticated operator and portal identities currently active across the workspace.',
    'Live conversation lanes that should remain pinned for operator watch.',
    'High-volume workspace queue with active moderator attention.',
    'Cross-tenant complaints and compliance escalations.',
    'Priority concierge threads with manual handoff SLA.',
    'Creator onboarding, media approvals, and revenue notices.',
    'Abuse escalations are rolling into the next operator shift with two frozen conversations pending owner confirmation.',
    'Session health is stable and the live message plane is accepting sustained traffic.',
    'Broadcast and automation systems are available for intervention playbooks if needed.',
    'Tenant governance',
    'Workspace governance',
    'Organizations',
    'Tenant posture',
    'Tenant directory',
    'Organizations currently represented in the IM estate.',
    'Workspace environments connected to the operator shell.',
    'Organizations with dedicated moderation coverage: {count}',
    'Workspace handoff templates aligned for every active tenant cluster.',
    'Regional escalation paths can be reviewed before new launches are approved.',
    '{count} active tenant projects are contributing to the current operator load.',
    '{count} workspace environments attached.',
    'Identity control',
    'Activation',
    'Ban',
    'Recovery review',
    'Device posture',
    'Risk watchlist',
    'Operators',
    'Portal users',
    'Identity roster',
    'Activation ready',
    'Ban or recovery review',
    'Operator identities with direct access to the admin workspace.',
    'Portal accounts participating in messaging, groups, and announcements.',
    'Device posture remains visible when support, abuse, and activation workflows converge.',
    'Password reset plus device rebind requested after suspicious logout.',
    'Operator lockout follows repeated OTP drift across mobile and web.',
    'Inactive session needs manual reactivation before the next moderation shift.',
    'Shared device fingerprint overlaps with previously frozen identities.',
    'Night-shift access spike exceeds the workspace baseline.',
    'Repeated recovery attempts are landing from three network regions.',
    'Web sessions remain the dominant support channel during moderation peaks.',
    'Mobile login recovery is the main source of temporary operator escalations.',
    'Conversations',
    'Lifecycle governance',
    'Handoff',
    'Archive',
    'Freeze',
    'Priority queues',
    'Handoff SLA',
    'Conversation lifecycle',
    'Handoff queue',
    'Archived threads',
    'Lifecycle board',
    'Freeze candidates',
    'Conversations with active routing or assignment changes in progress.',
    'Conversation records already sealed for evidence or long-term archive retention.',
    'Pinned lanes keep active handoff and freeze candidates visible without leaving the primary queue view.',
    'VIP and compliance queues target sub-5-minute first ownership changes.',
    'Standard support threads can wait 15 minutes before escalation is required.',
    'Unowned freeze candidates bypass the normal SLA and page trust operators immediately.',
    'Handoff keeps cross-region service, abuse review, and VIP escalations from stalling.',
    'Archive policies reduce operator noise once a conversation reaches a stable terminal state.',
    'Freeze is reserved for legal hold, fraud investigation, or severe trust and safety events.',
    'Transcript contains open abuse signals and should pause member writes.',
    'Cross-tenant escalation requires evidence preservation before rerouting.',
    'Legal hold requested while the handoff owner is still unresolved.',
    '{count} active conversation events.',
    'Message compliance',
    'Search transcript',
    'Export evidence',
    'Recall review',
    'Export controls',
    'Retention guardrails',
    'Message audit',
    'Indexed transcripts',
    'Evidence bundles',
    'Audit queue',
    'Message audit posture across searchable, reviewable transcript slices.',
    'Transcript segments indexed for quick policy and incident review.',
    'Evidence packages prepared for export or downstream compliance tooling.',
    'Evidence exports should be deliberate, limited, and traceable to a clear operator case.',
    'Search transcript before export so evidence scope is minimized.',
    'Attach retention reason and review ticket before releasing a transcript bundle.',
    'Use evidence export only for moderation, legal hold, or enterprise compliance workflows.',
    'Retention guardrails stop operators from mixing investigation, legal hold, and routine search workflows.',
    'Default retention remains searchable but not exportable without a linked case owner.',
    'Legal hold supersedes recall requests until counsel clears the thread.',
    'Expired evidence bundles are deleted on schedule and recreated only from audited searches.',
    'Operator requested urgent withdrawal after a PII leak was confirmed.',
    'Recall requires moderator approval before the transcript can be re-exported.',
    'Cross-region deletion waits for evidence hold confirmation.',
    'Threads currently visible to operators for lifecycle review and intervention.',
    '{count} indexed message units ready for Search transcript review.',
  ];

  for (const key of keys) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('zh-CN translations cover secondary governance, safety, and live operations modules', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const keys = [
    'Community governance',
    'Membership posture',
    'Group directory',
    'Managed groups',
    'Group directory volume currently visible to the operator shell.',
    'Members participating across the most active managed groups.',
    'Groups that should remain under active governance during campaign or moderation events.',
    'Membership posture stays healthy when ownership, review cadence, and pinned operators remain explicit.',
    'High-density groups should keep at least one primary moderator and one backup owner.',
    'Broadcast-only groups can be separated from conversational groups when risk rises.',
    'Dormant groups should be archived before they become a blind spot for compliance review.',
    '{count} active members under review.',
    'Safety operations',
    'First response SLA',
    'Disposition matrix',
    'Keyword policy',
    'Blocklist',
    'Report queue',
    'Reviewed today',
    'Escalation ready',
    'Open reports waiting for first response, escalation, or policy confirmation.',
    'Queues with enough context to move from intake into definitive action.',
    'First response SLA makes sure every new report lands in front of a human or automated owner within the correct risk window.',
    'Self-harm, child safety, and active fraud queues target under 3 minutes.',
    'Enterprise abuse and impersonation reports target under 10 minutes.',
    'Low-confidence keyword matches can batch until the next queue sweep without blocking higher-risk work.',
    'Keyword policy should be versioned, reviewed, and distributed with clear ownership.',
    'High-risk patterns route directly into human review instead of silent suppression.',
    'Region-specific policies can be layered without breaking baseline safety coverage.',
    'Blocklist posture is reserved for deterministic abuse, fraud, or legal hold scenarios.',
    'Temporary blocks expire with case review by default to prevent silent overreach.',
    'Permanent blocks require an audit trail, owner, and reactivation pathway.',
    'Retain visibility, enrich evidence, and keep the user active while confidence is still low.',
    'Stop new writes immediately when fraud, coercion, or compromise becomes probable.',
    'Apply only when policy breach is deterministic and the audit trail is complete.',
    'Signals already triaged by an operator during the current operating window.',
    'Outbound comms',
    'Delivery posture',
    'Broadcast tasks',
    'Scheduled',
    'Active delivery',
    'Broadcast jobs currently visible to campaign and operator teams.',
    'Notices already scheduled for future delivery windows.',
    'Announcements actively moving through live delivery or acknowledgement workflows.',
    'Delivery posture keeps fan-out quality and operator confidence visible before large announcements ship.',
    'Scheduled tasks are isolated from high-risk queues until policy review is complete.',
    'Delivery failures should promote directly into operator remediation workflows.',
    'Critical notices remain pinned until every priority workspace acknowledges receipt.',
    'Workflow orchestration',
    'Automation runs',
    'Run history',
    'Retry queue',
    'Bot registry',
    'Workflow actions',
    'Running',
    'Standby',
    'Automation runs remain useful only when outcome, trigger, and owner are explicit.',
    'Policy sweeps are scheduled around broadcast and moderation spikes.',
    'Bot-triggered handoffs are logged before they reach human queues.',
    'Replay and retry controls stay operator-owned even when bots initiate the action.',
    'Retry queue isolates automation that needs human judgment before another run is allowed to touch production queues.',
    'Confidence drops, missing ownership, and stale inputs are the main retry blockers.',
    'Retries inherit the original audit trail so operators can compare pre and post states.',
    'High-risk retries remain disabled during active incidents unless an owner explicitly overrides them.',
    'Bots and workflow engines currently registered for operator-supervised use.',
    'Live automation runs that are actively shaping queue posture or operator tasks.',
    'Registered capabilities that can be attached to lifecycle, moderation, or announcement flows.',
    'Completed queue hygiene sweep for high-priority inboxes without manual overrides.',
    'Paused after confidence dropped below the moderation threshold.',
    'Finished announcement targeting refresh and queued a follow-up diff.',
    'Live transport',
    'Gateway health',
    'Failover window',
    'Realtime sessions',
    'RTC posture',
    'Session monitor',
    'Offline',
    'Reconnect watch',
    '{count} live transport edges are reporting healthy realtime posture.',
    'Gateway health remains pinned because realtime incidents are usually transport incidents first.',
    'Session drains, failovers, and reconnect storms should be reviewed before broadcast launches.',
    'Failover window defines when operators can drain edges without breaking active large-room or RTC traffic.',
    'Planned cutovers are limited to low-broadcast windows with checkpoint sync confirmed.',
    'Emergency failovers require reconnect capacity and moderation transport coverage to stay green.',
    'VIP and enterprise rooms are pinned until a replacement edge reports healthy fan-out.',
    'Realtime sessions currently visible to the operator shell.',
    'RTC posture tracks live signaling and media-layer readiness.',
    'Gateway health shows whether live transport remains ready for operator-driven fan-out.',
    'Reconnect rate is elevated after an edge rebalance; keep new room joins throttled.',
    'Session rebinds are healthy but should stay under observation during broadcast peaks.',
    'Edge posture is stable, but cross-region fallback remains armed.',
    'Platform governance',
    'Runtime watch',
    'Protocol change gate',
    'Rollout risks',
    'Protocol governance',
    'Compatibility matrix',
    'Runtime health',
    'Supported',
    'Pilot',
    'Desktop pilot lag',
    'Cross-region cutover',
    'Moderation policy sync',
    '{count} runtime surfaces are currently reported into the admin workspace.',
    'Protocol changes should only ship when runtime health, compatibility, and fallback posture are all explicit.',
    'Provider health and transport health should remain green before protocol cutovers.',
    'Protocol change gate is the release checkpoint that protects clients, transport, and operators from out-of-sync semantics.',
    'Compatibility matrix, runtime health, and fallback readiness must all pass before a new wire contract opens.',
    'Rollouts are staged by client surface so operator tools never outrun end-user transports.',
    'Emergency reversions stay pre-approved with the previous protocol bundle pinned and ready.',
    'Protocol governance keeps transport, auth, and moderation contracts aligned.',
    'Compatibility matrix coverage across the major client and operator surfaces.',
    'Runtime nodes and services currently participating in the admin health model.',
    'Desktop clients still trail the latest protocol framing and must remain behind the gate.',
    'Route ownership migration depends on clean checkpoint replication before transport flags can flip.',
    'Keyword and recall semantics must ship alongside protocol changes to avoid operator mismatch.',
    'Desktop review consoles are reserved for heavy evidence and transcript workflows.',
  ];

  for (const key of keys) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('automation and announcements localize fallback status labels instead of rendering raw English state', () => {
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const announcements = read('packages/sdkwork-craw-chat-admin-announcements/src/index.tsx');

  assert.doesNotMatch(automation, /status: 'Running'/);
  assert.doesNotMatch(automation, /status: 'Standby'/);
  assert.match(automation, /t\('Running'\)/);
  assert.match(automation, /t\('Standby'\)/);

  assert.doesNotMatch(announcements, /\{task\.status\}/);
  assert.match(announcements, /resolveAnnouncementStatusLabel/);
});
