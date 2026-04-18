import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  apiReferenceOperationLinks,
  groupedPages,
  markdownPathFor,
  operationMarkdownPath,
  operationPageLink,
  readOperations,
} from "../.vitepress/api-reference-sidebar.mjs";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const issues = [];
const sourceOperationLinks = [];
const forbiddenPublicPortalCredentialMarkers = [
  "ops_demo",
  "Portal#2026",
  "acct_portal_demo",
];

const blockPattern =
  /<a id="([^"]+)"><\/a>[\s\S]*?<section class="api-op">([\s\S]*?)(?=<a id="|$)/g;

function read(relativePath) {
  return fs.readFileSync(path.join(docsRoot, relativePath), "utf8");
}

function requireIncludes(source, relativePath, needle, message) {
  if (!source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

function requireExcludes(source, relativePath, needle, message) {
  if (source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

function verifySourceOperationBlock(filePath, anchor, block) {
  const titleMatch = block.match(/## `([^`]+)`/);
  const title = titleMatch?.[1] ?? anchor;
  const relativePath = path.relative(docsRoot, filePath).replaceAll("\\", "/");
  const location = `${relativePath}#${anchor}`;
  const metaMatch = block.match(/<div class="api-meta-grid">([\s\S]*?)<\/div>\s*(?=\n### )/);
  const route = title.replace(/^[A-Z]+\s+/, "");
  const isPost = title.startsWith("POST ");
  const hasPathParams = title.includes("{");
  const isOpenProbe = route === "/healthz" || route === "/readyz";

  if (!block.includes("operationId:")) {
    issues.push(`${location}: missing operationId`);
  }

  if (!/### Response `\d+`/.test(block)) {
    issues.push(`${location}: missing explicit response section`);
  }

  if (hasPathParams && !block.includes("### Path Parameters")) {
    issues.push(`${location}: missing path parameter table`);
  }

  if (
    isPost &&
    !block.includes("### Request Body") &&
    !/does not require a JSON request body/i.test(block) &&
    !/does not accept a JSON request body/i.test(block)
  ) {
    issues.push(`${location}: missing request body section or explicit no-body note`);
  }

  if (!metaMatch) {
    issues.push(`${location}: missing api-meta-grid`);
  } else {
    for (const label of ["Security", "SDK", "Permission", "Success"]) {
      if (!metaMatch[1].includes(`<strong>${label}</strong>`)) {
        issues.push(`${location}: missing api-meta-grid label "${label}"`);
      }
    }

    if (
      (relativePath.includes("docs/sites/api-reference/platform/") ||
        relativePath.includes("docs/sites/api-reference/iot/")) &&
      metaMatch[1].includes("`sdkwork-craw-chat-sdk`")
    ) {
      issues.push(
        `${location}: platform and IoT operation docs must not claim sdkwork-craw-chat-sdk as the SDK surface`,
      );
    }

    if (!isOpenProbe && metaMatch[1].includes("trusted headers")) {
      issues.push(
        `${location}: public Security metadata must describe the bearer-auth contract only; keep trusted-header details in shared auth docs or endpoint notes`,
      );
    }
  }

  if (!isOpenProbe && !block.includes("### Error Responses")) {
    issues.push(`${location}: missing error responses section`);
  }
}

function collectMarkdownFiles(root) {
  const entries = fs.readdirSync(root, { withFileTypes: true });
  const files = [];

  for (const entry of entries) {
    if (entry.name === "node_modules") {
      continue;
    }

    const entryPath = path.join(root, entry.name);

    if (entry.isDirectory()) {
      files.push(...collectMarkdownFiles(entryPath));
      continue;
    }

    if (entry.isFile() && path.extname(entry.name) === ".md") {
      files.push(entryPath);
    }
  }

  return files;
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    const filePath = markdownPathFor(page.link);
    if (!fs.existsSync(filePath)) {
      issues.push(`${page.link}: missing source overview page`);
      continue;
    }

    const content = fs.readFileSync(filePath, "utf8");
    let match;
    while ((match = blockPattern.exec(content))) {
      const [, anchor, block] = match;
      verifySourceOperationBlock(filePath, anchor, block);
      sourceOperationLinks.push(operationPageLink(page.link, anchor));
    }
  }
}

const expectedSidebarLinks = new Set(sourceOperationLinks);
const sidebarLinks = [...apiReferenceOperationLinks];
const sidebarLinkSet = new Set(sidebarLinks);

for (const link of sidebarLinks) {
  const markdownPath = path.join(docsRoot, `${link.replace(/^\//, "")}.md`);
  if (!fs.existsSync(markdownPath)) {
    issues.push(`sidebar ${link}: missing operation markdown page`);
    continue;
  }

  const markdownContent = fs.readFileSync(markdownPath, "utf8");
  if (!markdownContent.includes("<section class=\"api-op")) {
    issues.push(`sidebar ${link}: missing operation section wrapper`);
  }
  if (!/### Response `\d+`/.test(markdownContent)) {
    issues.push(`sidebar ${link}: missing explicit response section`);
  }
}

for (const link of expectedSidebarLinks) {
  if (!sidebarLinkSet.has(link)) {
    issues.push(`${link}: missing sidebar entry`);
  }
}

for (const markdownFile of collectMarkdownFiles(docsRoot)) {
  const content = fs.readFileSync(markdownFile, "utf8");
  const relativePath = path.relative(docsRoot, markdownFile).replaceAll("\\", "/");

  for (const marker of forbiddenPublicPortalCredentialMarkers) {
    if (content.includes(marker)) {
      issues.push(
        `${relativePath}: contains retired public portal credential marker "${marker}"`,
      );
    }
  }
}

for (const group of groupedPages) {
  for (const page of group.pages) {
    for (const operation of readOperations(page.link)) {
      const operationPath = operationMarkdownPath(page.link, operation.anchor);
      if (!fs.existsSync(operationPath)) {
        issues.push(`${operationPath}: missing generated operation page`);
        continue;
      }

      const content = fs.readFileSync(operationPath, "utf8");
      if (!content.includes(`# \`${operation.operationTitle}\``)) {
        issues.push(`${operationPath}: missing operation title heading`);
      }
      if (!content.includes(`href="${page.link}"`)) {
        issues.push(`${operationPath}: missing overview backlink`);
      }
    }
  }
}

const appApiPath = "api-reference/app-api.md";
const appApiSource = read(appApiPath);
requireIncludes(
  appApiSource,
  appApiPath,
  "@sdkwork/craw-chat-sdk",
  "must point TypeScript consumers to the official root package",
);
requireIncludes(
  appApiSource,
  appApiPath,
  "craw_chat_sdk",
  "must point Flutter consumers to the official consumer package",
);
requireExcludes(
  appApiSource,
  appApiPath,
  "consumer-layer decision between generated transport and composed clients",
  "must not describe the app SDK choice as a generated-versus-composed decision",
);

for (const relativePath of [
  "api-reference/app/conversations.md",
  "api-reference/app/device-sync.md",
  "api-reference/app/messages.md",
  "api-reference/app/media.md",
  "api-reference/app/rtc.md",
  "api-reference/app/session-and-realtime.md",
  "api-reference/app/streams.md",
]) {
  const source = read(relativePath);
  requireIncludes(
    source,
    relativePath,
    "@sdkwork/craw-chat-sdk",
    "must reference the official TypeScript root package when linking SDK usage",
  );
  requireIncludes(
    source,
    relativePath,
    "craw_chat_sdk",
    "must reference the official Flutter consumer package when linking SDK usage",
  );
}

const conversationsPath = "api-reference/app/conversations.md";
const conversationsSource = read(conversationsPath);
requireIncludes(
  conversationsSource,
  conversationsPath,
  "sdk.conversations.createAgentDialog(...)",
  "must document sdk.conversations.createAgentDialog(...) on the conversations API page",
);
requireIncludes(
  conversationsSource,
  conversationsPath,
  "sdk.conversations.getAgentHandoffState(...)",
  "must document sdk.conversations.getAgentHandoffState(...) on the conversations API page",
);
requireIncludes(
  conversationsSource,
  conversationsPath,
  "sdk.generated.inbox.getInbox()",
  "must document sdk.generated.inbox.getInbox() on the conversations API page",
);
requireExcludes(
  conversationsSource,
  conversationsPath,
  "composed TypeScript client",
  "must not describe the TypeScript SDK as a composed client",
);

const messagesPath = "api-reference/app/messages.md";
const messagesSource = read(messagesPath);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.uploadAndSendMessage(...)",
  "must document sdk.uploadAndSendMessage(...) on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.decodeMessage(...)",
  "must document sdk.decodeMessage(...) on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.createTextMessage(...)",
  "must document sdk.createTextMessage(...) on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.send(message)",
  "must document sdk.send(message) on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "createLocationMessage(...)",
  "must document standard rich message builders on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "createAiImageGenerationMessage(...)",
  "must document AI message builders on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "createWorkflowEventMessage(...)",
  "must document workflow and agent-era message builders on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.conversations.postText(...)",
  "must document sdk.conversations.postText(...) on the messages API page",
);
requireIncludes(
  messagesSource,
  messagesPath,
  "sdk.conversations.publishSystemText(...)",
  "must document sdk.conversations.publishSystemText(...) on the messages API page",
);
requireExcludes(
  messagesSource,
  messagesPath,
  "Composed helpers",
  "must not describe shared SDK helpers only as composed helpers",
);

const mediaPath = "api-reference/app/media.md";
const mediaSource = read(mediaPath);
requireIncludes(
  mediaSource,
  mediaPath,
  "sdk.media.uploadAndComplete(...)",
  "must document sdk.media.uploadAndComplete(...) on the media API page",
);
requireIncludes(
  mediaSource,
  mediaPath,
  "sdk.media.attachText(...)",
  "must document sdk.media.attachText(...) on the media API page",
);
requireExcludes(
  mediaSource,
  mediaPath,
  "Both app SDK languages expose media helpers on the composed client",
  "must not describe media helpers as living on a generic composed client across languages",
);

const rtcPath = "api-reference/app/rtc.md";
const rtcSource = read(rtcPath);
requireIncludes(
  rtcSource,
  rtcPath,
  "sdk.rtc.create(...)",
  "must document sdk.rtc.create(...) on the RTC API page",
);
requireIncludes(
  rtcSource,
  rtcPath,
  "sdk.rtc.postJsonSignal(...)",
  "must document sdk.rtc.postJsonSignal(...) on the RTC API page",
);
requireIncludes(
  rtcSource,
  rtcPath,
  "live.signals.onRtcSession(...)",
  "must document live.signals.onRtcSession(...) as the inbound RTC signaling path",
);
requireIncludes(
  rtcSource,
  rtcPath,
  "sdk.rtc.issueParticipantCredential(...)",
  "must document sdk.rtc.issueParticipantCredential(...) on the RTC API page",
);
requireIncludes(
  rtcSource,
  rtcPath,
  "sdk.rtc.getRecordingArtifact(...)",
  "must document sdk.rtc.getRecordingArtifact(...) on the RTC API page",
);
requireExcludes(
  rtcSource,
  rtcPath,
  "Both app SDK languages currently expose RTC helpers on the composed client",
  "must not describe RTC helpers as living on a generic composed client across languages",
);
requireExcludes(
  rtcSource,
  rtcPath,
  "`sdkwork-craw-chat-sdk` / rtc",
  "must not use the retired sdkwork-craw-chat-sdk / rtc meta label on the RTC API page",
);

const sessionRealtimePath = "api-reference/app/session-and-realtime.md";
const sessionRealtimeSource = read(sessionRealtimePath);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "sdk.sync.ack(...)",
  "must document sdk.sync.ack(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "context.ack()",
  "must document context.ack() on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.messages.on(...)",
  "must document live.messages.on(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.data.on(...)",
  "must document live.data.on(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.signals.on(...)",
  "must document live.signals.on(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.events.on(...)",
  "must document live.events.on(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.lifecycle.onStateChange(...)",
  "must document live.lifecycle.onStateChange(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.lifecycle.onError(...)",
  "must document live.lifecycle.onError(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "sdk.generated.session.disconnect(...)",
  "must document sdk.generated.session.disconnect(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "sdk.generated.presence.heartbeat(...)",
  "must document sdk.generated.presence.heartbeat(...) on the session and realtime API page",
);
requireIncludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "sdk.generated.realtime.listRealtimeEvents(...)",
  "must document sdk.generated.realtime.listRealtimeEvents(...) on the session and realtime API page",
);
requireExcludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "Generated and composed app clients cover the HTTP resume, sync, poll, and ACK flows",
  "must not describe app SDK coverage as generated-versus-composed clients",
);
requireExcludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.onMessage(...)",
  "must not teach live.onMessage(...) on the session and realtime API page",
);
requireExcludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.onData(...)",
  "must not teach live.onData(...) on the session and realtime API page",
);
requireExcludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "live.onSignal(...)",
  "must not teach live.onSignal(...) on the session and realtime API page",
);
requireExcludes(
  sessionRealtimeSource,
  sessionRealtimePath,
  "`sdkwork-craw-chat-sdk` / session",
  "must not use the retired sdkwork-craw-chat-sdk / session meta label on the session and realtime API page",
);

const deviceSyncPath = "api-reference/app/device-sync.md";
const deviceSyncSource = read(deviceSyncPath);
requireIncludes(
  deviceSyncSource,
  deviceSyncPath,
  "sdk.generated.device.register(...)",
  "must document sdk.generated.device.register(...) on the device sync API page",
);
requireIncludes(
  deviceSyncSource,
  deviceSyncPath,
  "sdk.generated.device.getDeviceSyncFeed(...)",
  "must document sdk.generated.device.getDeviceSyncFeed(...) on the device sync API page",
);
requireExcludes(
  deviceSyncSource,
  deviceSyncPath,
  "SDK device modules",
  "must not imply a dedicated semantic device module on the device sync API page",
);
requireExcludes(
  deviceSyncSource,
  deviceSyncPath,
  "`sdkwork-craw-chat-sdk` / device-sync",
  "must not use the retired sdkwork-craw-chat-sdk / device-sync meta label on the device sync API page",
);

const streamsPath = "api-reference/app/streams.md";
const streamsSource = read(streamsPath);
requireIncludes(
  streamsSource,
  streamsPath,
  "sdk.generated.stream.open(...)",
  "must document sdk.generated.stream.open(...) on the streams API page",
);
requireIncludes(
  streamsSource,
  streamsPath,
  "sdk.generated.stream.appendStreamFrame(...)",
  "must document sdk.generated.stream.appendStreamFrame(...) on the streams API page",
);
requireExcludes(
  streamsSource,
  streamsPath,
  "Composed app SDKs expose stream helpers and builders on top of these routes",
  "must not describe stream helpers only in terms of composed SDKs",
);
requireExcludes(
  streamsSource,
  streamsPath,
  "`sdkwork-craw-chat-sdk` / streams",
  "must not use the retired sdkwork-craw-chat-sdk / streams meta label on the streams API page",
);

const membershipPath = "api-reference/app/membership-and-read-state.md";
const membershipSource = read(membershipPath);
requireIncludes(
  membershipSource,
  membershipPath,
  "sdk.conversations.listMembers(...)",
  "must document sdk.conversations.listMembers(...) on the membership API page",
);
requireIncludes(
  membershipSource,
  membershipPath,
  "sdk.conversations.updateReadCursor(...)",
  "must document sdk.conversations.updateReadCursor(...) on the membership API page",
);
requireExcludes(
  membershipSource,
  membershipPath,
  "/ membership",
  "must not refer to a nonexistent membership SDK module on the membership API page",
);

const portalAuthPath = "api-reference/app/portal-and-auth.md";
const portalAuthSource = read(portalAuthPath);
requireIncludes(
  portalAuthSource,
  portalAuthPath,
  "@sdkwork/craw-chat-sdk",
  "must describe the TypeScript portal surface through the official root package",
);
requireIncludes(
  portalAuthSource,
  portalAuthPath,
  "craw_chat_sdk",
  "must describe the current Flutter portal gap through the official Flutter package naming",
);
requireIncludes(
  portalAuthSource,
  portalAuthPath,
  "backend_sdk",
  "must describe the current Flutter portal gap through the generated package naming",
);
requireExcludes(
  portalAuthSource,
  portalAuthPath,
  "generated and composed layers",
  "must not describe TypeScript or Flutter package surfaces as generic generated-and-composed layers",
);

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log(
  `Verified ${groupedPages.reduce((sum, group) => sum + group.pages.length, 0)} source API pages, ${sourceOperationLinks.length} operation pages, and ${sidebarLinks.length} sidebar entries.`,
);
