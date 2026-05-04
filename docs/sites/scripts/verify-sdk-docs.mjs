import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const issues = [];
const removedLegacyTsCompatClientName = ['Craw', 'Chat', 'Client'].join('');
const removedGeneratedClientName = ['Im', 'Generated', 'Client'].join('');
const removedGeneratedPackageName = '@sdkwork/im-sdk-generated';
const removedGeneratedClientImport = `import { ${removedGeneratedClientName} } from '${removedGeneratedPackageName}';`;
const removedLegacyTsTwoPackageNarrative =
  `a generated transport package and a composed package built around \`${removedLegacyTsCompatClientName}\``;

function read(relativePath) {
  const absolutePath = path.join(docsRoot, relativePath);
  return fs.readFileSync(absolutePath, "utf8");
}

function readRequired(relativePath) {
  const absolutePath = path.join(docsRoot, relativePath);
  if (!fs.existsSync(absolutePath)) {
    issues.push(`${relativePath}: file is required`);
    return "";
  }
  return fs.readFileSync(absolutePath, "utf8");
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

function requireLanguageDocStructure(relativePath, source, markers) {
  for (const marker of [
    "Current Delivery Reality",
    "Package Contract",
    "Raw generated client",
    "What Is Not Shipped Yet",
    "generated/server-openapi/README.md",
    "repo contract",
    "sdk-gen.ps1",
    "sdk-verify.ps1",
    "ImTransportClient",
    ...markers,
  ]) {
    requireIncludes(
      source,
      relativePath,
      marker,
      `must include ${marker} in the language SDK guide`,
    );
  }
}

function requireTransportLanguageApiMap(relativePath, source) {
  for (const marker of [
    "API Reference Map",
    "What To Read Next",
    "generated/server-openapi/README.md",
    "ImTransportClient",
    "/api-reference/app/portal-and-auth",
    "/api-reference/app/conversations",
    "/api-reference/app/membership-and-read-state",
    "/api-reference/app/messages",
    "/api-reference/app/media",
    "/api-reference/app/session-and-realtime",
    "/api-reference/app/device-sync",
    "/api-reference/app/rtc",
    "/api-reference/app/streams",
  ]) {
    requireIncludes(
      source,
      relativePath,
      marker,
      `must include ${marker} in the language API reference map`,
    );
  }
}

const typescriptDocPath = "sdk/typescript-sdk.md";
const typescriptDocSource = read(typescriptDocPath);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "@sdkwork/im-sdk",
  "must document @sdkwork/im-sdk as the official TypeScript consumer package",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "ImSdkClient",
  "must document ImSdkClient as the primary TypeScript client",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "src/generated/**",
  "must document src/generated/** as the assembled generated transport boundary",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "generated/server-openapi",
  "must document generated/server-openapi as the generator-owned authoring boundary",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "browser and Node.js",
  "must document browser and Node.js runtime targets",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.session.resume(...)",
  "must document sdk.session.resume(...) as the root TypeScript session transport entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.presence.getPresenceMe()",
  "must document sdk.presence.getPresenceMe() as the root TypeScript presence transport entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.realtime.listRealtimeEvents(...)",
  "must document sdk.realtime.listRealtimeEvents(...) as the root TypeScript realtime transport entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.device.register(...)",
  "must document sdk.device.register(...) as the root TypeScript device transport entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.stream.open(...)",
  "must document sdk.stream.open(...) as the root TypeScript stream transport entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "new ImSdkClient({",
  "must teach synchronous ImSdkClient construction",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.createTextMessage(...)",
  "must document sdk.createTextMessage(...) as the primary TypeScript message builder entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.send(...)",
  "must document sdk.send(...) as the primary TypeScript message delivery entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.uploadAndSendMessage(...)",
  "must document sdk.uploadAndSendMessage(...) as the primary TypeScript upload-complete-send shortcut",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.decodeMessage(...)",
  "must document sdk.decodeMessage(...) as the primary TypeScript message decode entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.connect(...)",
  "must document sdk.connect(...) as the live receive entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.messages.on(...)",
  "must document live.messages.on(...) as the primary TypeScript inbound message entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.messages.onConversation(...)",
  "must document live.messages.onConversation(...) as the scoped TypeScript inbound message entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.data.on(...)",
  "must document live.data.on(...) as the primary TypeScript inbound data entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.signals.on(...)",
  "must document live.signals.on(...) as the primary TypeScript inbound signal entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.signals.onRtcSession(...)",
  "must document live.signals.onRtcSession(...) as the scoped TypeScript inbound RTC signal entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.events.on(...)",
  "must document live.events.on(...) as the raw TypeScript inbound event entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.lifecycle.onStateChange(...)",
  "must document live.lifecycle.onStateChange(...) as the TypeScript live lifecycle entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.lifecycle.onError(...)",
  "must document live.lifecycle.onError(...) as the TypeScript live error entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.lifecycle.getState()",
  "must document live.lifecycle.getState() as the TypeScript live state snapshot helper",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.sync.catchUp(...)",
  "must document sdk.sync.catchUp(...) as the durable replay entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.sync.ack(...)",
  "must document sdk.sync.ack(...) as the explicit durable ACK entrypoint",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.conversations.createAgentDialog(...)",
  "must document sdk.conversations.createAgentDialog(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.conversations.postText(...)",
  "must document sdk.conversations.postText(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.conversations.publishSystemText(...)",
  "must document sdk.conversations.publishSystemText(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.inbox.getInbox()",
  "must document sdk.inbox.getInbox() in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.media.uploadAndComplete(...)",
  "must document sdk.media.uploadAndComplete(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.media.attachText(...)",
  "must document sdk.media.attachText(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "context.ack()",
  "must document context.ack() as the per-event ACK helper",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "createAiImageGenerationMessage",
  "must document createAiImageGenerationMessage as the AI image generation builder",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "createAiVideoGenerationMessage",
  "must document createAiVideoGenerationMessage as the AI video generation builder",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "createAgentMessage",
  "must document createAgentMessage as the standard agent message builder",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.rtc.postJsonSignal(...)",
  "must document sdk.rtc.postJsonSignal(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "rtcMode",
  "must use the current rtcMode field in the TypeScript RTC guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "signalingStreamId",
  "must use signalingStreamId in the TypeScript RTC guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.rtc.issueParticipantCredential(...)",
  "must document sdk.rtc.issueParticipantCredential(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.rtc.getRecordingArtifact(...)",
  "must document sdk.rtc.getRecordingArtifact(...) in the TypeScript guide",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "What To Read Next",
  "must include What To Read Next in the TypeScript guide",
);
for (const marker of [
  "/api-reference/app/portal-and-auth",
  "/api-reference/app/conversations",
  "/api-reference/app/membership-and-read-state",
  "/api-reference/app/messages",
  "/api-reference/app/media",
  "/api-reference/app/session-and-realtime",
  "/api-reference/app/device-sync",
  "/api-reference/app/rtc",
  "/api-reference/app/streams",
]) {
  requireIncludes(
    typescriptDocSource,
    typescriptDocPath,
    marker,
    `must include ${marker} in the TypeScript SDK guide`,
  );
}
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  removedGeneratedClientImport,
  `must not tell consumers to import ${removedGeneratedClientName} from ${removedGeneratedPackageName}`,
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "@sdkwork-internal/im-sdk-generated",
  "must not expose the internal generated package alias in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "@sdkwork/im-sdk-generated",
  "must not mention the unsupported @sdkwork/im-sdk-generated package identity in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  removedGeneratedClientName,
  `must not document ${removedGeneratedClientName} in the TypeScript guide`,
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "createGeneratedClient",
  "must not document createGeneratedClient in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "sdk.generated.",
  "must not document sdk.generated.* in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  removedLegacyTsTwoPackageNarrative,
  "must not describe the TypeScript SDK as a two-package consumer model",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "composed/package.json` owns `@sdkwork/im-sdk`",
  "must not describe composed/package.json as the public TypeScript consumer package contract",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  removedLegacyTsCompatClientName,
  "must not teach the removed TypeScript compatibility alias in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "ImSdkClient.create(",
  "must not teach async static create() in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "createReceiver()",
  "must not teach createReceiver() in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "createWebSocketReceiver()",
  "must not teach createWebSocketReceiver() in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onMessage(",
  "must not teach the legacy live.onMessage(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onConversationMessage(",
  "must not teach the legacy live.onConversationMessage(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onData(",
  "must not teach the legacy live.onData(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onSignal(",
  "must not teach the legacy live.onSignal(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onRawEvent(",
  "must not teach the legacy live.onRawEvent(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onStateChange(",
  "must not teach the legacy live.onStateChange(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "live.onError(",
  "must not teach the legacy live.onError(...) API in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "`connecting`, `connected`, `error`, and `closed`",
  "must not document an impossible connecting lifecycle state in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "createAiImage`",
  "must not teach the removed createAiImage builder in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "createAiVideo`",
  "must not teach the removed createAiVideo builder in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "participantIds",
  "must not teach the removed participantIds RTC invite example in the TypeScript guide",
);

const flutterDocPath = "sdk/flutter-sdk.md";
const flutterDocSource = read(flutterDocPath);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "im_sdk",
  "must document im_sdk as the official Flutter consumer package",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "im_sdk_generated",
  "must document im_sdk_generated as the generated Flutter transport package",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "ImSdkClient",
  "must document ImSdkClient as the primary Flutter client",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "re-exports `im_sdk_generated`",
  "must explain that im_sdk re-exports im_sdk_generated",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "package:im_sdk/im_sdk.dart",
  "must point app consumers to the im_sdk package entrypoint",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "AuthApi",
  "must document that im_sdk_generated now exports AuthApi in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "PortalApi",
  "must document that im_sdk_generated now exports PortalApi in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "client.auth",
  "must document mounted client.auth in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "client.portal",
  "must document mounted client.portal in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "sdk.auth",
  "must document sdk.auth in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "sdk.portal",
  "must document sdk.portal in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "sdk.connect(...)",
  "must document sdk.connect(...) as the delivered Flutter websocket live runtime entrypoint",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "ImWebSocketAuthOptions.automatic()",
  "must document the standard Flutter websocket auth strategy",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "does not yet ship `sdk.createXxxMessage()`",
  "must describe the current Flutter message-builder parity gap precisely",
);
requireExcludes(
  flutterDocSource,
  flutterDocPath,
  "choose between the generated transport and",
  "must not frame Flutter as a neutral generated-versus-composed package choice for most consumers",
);
requireExcludes(
  flutterDocSource,
  flutterDocPath,
  "both generated and composed clients",
  "must prefer concrete Flutter client names over generic generated/composed client wording",
);
requireExcludes(
  flutterDocSource,
  flutterDocPath,
  "layered package model",
  "must not lead with generic layered package wording when the official Flutter package names are known",
);

const sdkIndexPath = "sdk/index.md";
const sdkIndexSource = read(sdkIndexPath);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "Official consumer package",
  "must distinguish official consumer packages from internal generated boundaries",
);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "@sdkwork/im-sdk",
  "must list @sdkwork/im-sdk in the SDK naming model",
);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "im_sdk",
  "must list im_sdk in the SDK naming model",
);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "ImTransportClient",
  "must explain the current raw generated client naming model in the SDK overview",
);
for (const marker of [
  "Choose By Scenario",
  "richest app-facing SDK today",
  "Flutter UI",
  "generated transport only",
  "control-plane or governance tooling",
]) {
  requireIncludes(
    sdkIndexSource,
    sdkIndexPath,
    marker,
    `must include ${marker} in the SDK overview scenario guide`,
  );
}
for (const marker of [
  "/sdk/rust-sdk",
  "/sdk/java-sdk",
  "/sdk/csharp-sdk",
  "/sdk/swift-sdk",
  "/sdk/kotlin-sdk",
  "/sdk/go-sdk",
  "/sdk/python-sdk",
  "/sdk/generator-boundary",
  "/api-reference/app/portal-and-auth",
  "/api-reference/app/conversations",
  "/api-reference/app/messages",
  "/api-reference/app/media",
  "/api-reference/app/session-and-realtime",
  "/api-reference/app/rtc",
  "/api-reference/app/streams",
  "/api-reference/control-plane/protocol",
  "/api-reference/control-plane/providers",
  "/api-reference/control-plane/nodes",
  "Tier A",
  "Tier B",
]) {
  requireIncludes(
    sdkIndexSource,
    sdkIndexPath,
    marker,
    `must include ${marker} in the multilanguage SDK overview`,
  );
}
requireExcludes(
  sdkIndexSource,
  sdkIndexPath,
  "Available in generated and composed layers",
  "must not describe the TypeScript portal surface as generic generated/composed layers",
);
requireExcludes(
  sdkIndexSource,
  sdkIndexPath,
  "checked-in generated/composed layers",
  "must not describe the Flutter portal gap as generic generated/composed layers",
);

const appSdkPath = "sdk/app-sdk.md";
const appSdkSource = read(appSdkPath);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "Official consumer package",
  "must document the TypeScript single-package standard",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "src/generated/**",
  "must document the assembled TypeScript generated boundary",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "/openapi/craw-chat-app.openapi.yaml",
  "must document the live service schema export endpoint",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.createAiImageGenerationMessage(...)",
  "must document sdk.createAiImageGenerationMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.createAiVideoGenerationMessage(...)",
  "must document sdk.createAiVideoGenerationMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.createAgentMessage(...)",
  "must document sdk.createAgentMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.createTextMessage(...)",
  "must document sdk.createTextMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.send(...)",
  "must document sdk.send(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.uploadAndSendMessage(...)",
  "must document sdk.uploadAndSendMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.decodeMessage(...)",
  "must document sdk.decodeMessage(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "live.messages.on(...)",
  "must document live.messages.on(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "live.signals.onRtcSession(...)",
  "must document live.signals.onRtcSession(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "live.lifecycle.onStateChange(...)",
  "must document live.lifecycle.onStateChange(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.sync.ack(...)",
  "must document sdk.sync.ack(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.conversations.createAgentDialog(...)",
  "must document sdk.conversations.createAgentDialog(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.conversations.postText(...)",
  "must document sdk.conversations.postText(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.inbox.getInbox()",
  "must document sdk.inbox.getInbox() in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.session.resume(...)",
  "must document sdk.session.resume(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.device.register(...)",
  "must document sdk.device.register(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.stream.open(...)",
  "must document sdk.stream.open(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.media.uploadAndComplete(...)",
  "must document sdk.media.uploadAndComplete(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.media.attachText(...)",
  "must document sdk.media.attachText(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "context.ack()",
  "must document context.ack() in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.rtc.postJsonSignal(...)",
  "must document sdk.rtc.postJsonSignal(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.rtc.issueParticipantCredential(...)",
  "must document sdk.rtc.issueParticipantCredential(...) in the app SDK guide",
);
requireIncludes(
  appSdkSource,
  appSdkPath,
  "sdk.rtc.getRecordingArtifact(...)",
  "must document sdk.rtc.getRecordingArtifact(...) in the app SDK guide",
);
for (const marker of [
  "Choose This Family When",
  "Do Not Start Here When",
  "browser or Node.js app runtime",
  "Flutter app runtime",
  "governance or control-plane tooling",
  "/api-reference/app/portal-and-auth",
  "/api-reference/app/conversations",
  "/api-reference/app/messages",
  "/api-reference/app/media",
  "/api-reference/app/session-and-realtime",
  "/api-reference/app/rtc",
  "/api-reference/app/streams",
]) {
  requireIncludes(
    appSdkSource,
    appSdkPath,
    marker,
    `must include ${marker} in the app SDK entry guide`,
  );
}
requireExcludes(
  appSdkSource,
  appSdkPath,
  "checked-in generated and composed layers do not yet expose dedicated auth or portal modules",
  "must describe the Flutter portal gap through official package names instead of generic layers",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "@sdkwork-internal/im-sdk-generated",
  "must not expose the internal generated package alias in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  removedGeneratedClientName,
  `must not document ${removedGeneratedClientName} in the app SDK guide`,
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "sdk.generated.",
  "must not document sdk.generated.* in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "sdk.messages.createAiImage(...)",
  "must not teach the removed sdk.messages.createAiImage(...) builder in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "sdk.messages.createAiVideo(...)",
  "must not teach the removed sdk.messages.createAiVideo(...) builder in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "live.onMessage(",
  "must not teach the legacy live.onMessage(...) API in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "live.onData(",
  "must not teach the legacy live.onData(...) API in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "live.onSignal(",
  "must not teach the legacy live.onSignal(...) API in the app SDK guide",
);
requireExcludes(
  appSdkSource,
  appSdkPath,
  "`connecting`, `connected`, `error`, and `closed`",
  "must not document an impossible connecting lifecycle state in the app SDK guide",
);

const languageSupportPath = "sdk/language-support.md";
const languageSupportSource = read(languageSupportPath);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "Official consumer package",
  "must describe official consumer packages explicitly",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "src/generated/**",
  "must mention the assembled TypeScript generated boundary",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "@sdkwork/im-sdk",
  "must identify the official TypeScript consumer package",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "sdk.connect(...)",
  "must describe sdk.connect(...) as the TypeScript live runtime entrypoint",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "Message-first builders (`createXxxMessage`, `send`, `decodeMessage`)",
  "must track the message-first builder parity explicitly",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "sdk.auth",
  "must describe Flutter auth support through sdk.auth in language support",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "sdk.portal",
  "must describe Flutter portal support through sdk.portal in language support",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "client.auth",
  "must describe Flutter auth support through client.auth in language support",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "client.portal",
  "must describe Flutter portal support through client.portal in language support",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "ImTransportClient",
  "must explain the current raw generated transport client naming model in language support",
);
requireExcludes(
  languageSupportSource,
  languageSupportPath,
  removedGeneratedClientName,
  `must not document the removed ${removedGeneratedClientName} name in language support`,
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "Realtime WebSocket adapter | Yes | Yes",
  "must record that both TypeScript and Flutter currently ship websocket live runtimes",
);
for (const marker of [
  "Rust",
  "Java",
  "C#",
  "Swift",
  "Kotlin",
  "Go",
  "Python",
  "Tier A",
  "Tier B",
  "sdkwork-im-sdk-generated",
  "com.sdkwork:im-sdk-generated",
  "Sdkwork.Im.Sdk.Generated",
  "ImSdkGenerated",
  "github.com/sdkwork/im-sdk-generated",
  "generated/server-openapi",
  "composed",
  "What To Read Next",
  "/sdk/java-sdk",
  "/sdk/csharp-sdk",
  "/sdk/swift-sdk",
  "/sdk/kotlin-sdk",
  "/sdk/go-sdk",
  "/sdk/python-sdk",
]) {
  requireIncludes(
    languageSupportSource,
    languageSupportPath,
    marker,
    `must include ${marker} in the multilanguage support matrix`,
  );
}
requireExcludes(
  languageSupportSource,
  languageSupportPath,
  "standardized receive and decode helpers",
  "must not imply Flutter already ships the TypeScript receive and decode surface",
);

const rustDocPath = "sdk/rust-sdk.md";
const rustDocSource = readRequired(rustDocPath);
requireLanguageDocStructure(rustDocPath, rustDocSource, [
  "sdkwork-im-sdk-generated",
  "im_sdk",
  "Tier A",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(rustDocPath, rustDocSource);

const javaDocPath = "sdk/java-sdk.md";
const javaDocSource = readRequired(javaDocPath);
requireLanguageDocStructure(javaDocPath, javaDocSource, [
  "com.sdkwork:im-sdk-generated",
  "com.sdkwork:im-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(javaDocPath, javaDocSource);

const csharpDocPath = "sdk/csharp-sdk.md";
const csharpDocSource = readRequired(csharpDocPath);
requireLanguageDocStructure(csharpDocPath, csharpDocSource, [
  "Sdkwork.Im.Sdk.Generated",
  "Sdkwork.Im.Sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(csharpDocPath, csharpDocSource);

const swiftDocPath = "sdk/swift-sdk.md";
const swiftDocSource = readRequired(swiftDocPath);
requireLanguageDocStructure(swiftDocPath, swiftDocSource, [
  "ImSdkGenerated",
  "ImSdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(swiftDocPath, swiftDocSource);

const kotlinDocPath = "sdk/kotlin-sdk.md";
const kotlinDocSource = readRequired(kotlinDocPath);
requireLanguageDocStructure(kotlinDocPath, kotlinDocSource, [
  "com.sdkwork:im-sdk-generated",
  "com.sdkwork:im-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(kotlinDocPath, kotlinDocSource);

const goDocPath = "sdk/go-sdk.md";
const goDocSource = readRequired(goDocPath);
requireLanguageDocStructure(goDocPath, goDocSource, [
  "github.com/sdkwork/im-sdk-generated",
  "github.com/sdkwork/im-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(goDocPath, goDocSource);

const pythonDocPath = "sdk/python-sdk.md";
const pythonDocSource = readRequired(pythonDocPath);
requireLanguageDocStructure(pythonDocPath, pythonDocSource, [
  "sdkwork-im-sdk-generated",
  "sdkwork-im-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "ImSdkClient",
]);
requireTransportLanguageApiMap(pythonDocPath, pythonDocSource);

const boundaryDocPath = "sdk/generator-boundary.md";
const boundaryDocSource = readRequired(boundaryDocPath);
for (const marker of [
  "generated/server-openapi",
  "composed",
  "src/generated/**",
  "WebSocket",
  "live runtime",
  "/openapi/craw-chat-app.openapi.yaml",
  "What To Read Next",
  "/sdk/app-sdk",
  "/sdk/typescript-sdk",
  "/sdk/language-support",
  "/sdk/control-plane-sdk",
]) {
  requireIncludes(boundaryDocSource, boundaryDocPath, marker, `must include ${marker}`);
}

const adminSdkPath = "sdk/control-plane-sdk.md";
const adminSdkSource = readRequired(adminSdkPath);
for (const marker of [
  "Choose This Family When",
  "Do Not Use The Control-Plane SDK For",
  "Control Plane Reference Map",
  "protocol registry",
  "provider governance",
  "node lifecycle",
  "app-runtime chat flows",
  "Use the App SDK instead",
  "/api-reference/control-plane-api",
  "/api-reference/control-plane/protocol",
  "/api-reference/control-plane/providers",
  "/api-reference/control-plane/nodes",
  "/api-reference/auth-and-errors",
]) {
  requireIncludes(
    adminSdkSource,
    adminSdkPath,
    marker,
    `must include ${marker} in the admin SDK entry guide`,
  );
}

const homePagePath = "index.md";
const homePageSource = read(homePagePath);
requireIncludes(
  homePageSource,
  homePagePath,
  "@sdkwork/im-sdk",
  "must surface the official TypeScript SDK package on the docs home page",
);

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log("Verified SDK documentation contract pages.");
