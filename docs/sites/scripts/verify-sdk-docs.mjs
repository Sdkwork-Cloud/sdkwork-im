import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const issues = [];

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
    "SdkworkBackendClient",
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
    "SdkworkBackendClient",
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
  "@sdkwork/craw-chat-sdk",
  "must document @sdkwork/craw-chat-sdk as the official TypeScript consumer package",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "CrawChatSdkClient",
  "must document CrawChatSdkClient as the primary TypeScript client",
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
  "SdkworkBackendClient",
  "must document low-level generated access from the root TypeScript package",
);
requireIncludes(
  typescriptDocSource,
  typescriptDocPath,
  "new CrawChatSdkClient({",
  "must teach synchronous CrawChatSdkClient construction",
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
  "sdk.generated.inbox.getInbox()",
  "must document sdk.generated.inbox.getInbox() in the TypeScript guide",
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
  "import { SdkworkBackendClient } from '@sdkwork/craw-chat-backend-sdk';",
  "must not tell consumers to import SdkworkBackendClient from @sdkwork/craw-chat-backend-sdk",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "a generated transport package and a composed package built around `CrawChatClient`",
  "must not describe the TypeScript SDK as a two-package consumer model",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "composed/package.json` owns `@sdkwork/craw-chat-sdk`",
  "must not describe composed/package.json as the public TypeScript consumer package contract",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "CrawChatClient",
  "must not teach the removed CrawChatClient compatibility alias in the TypeScript guide",
);
requireExcludes(
  typescriptDocSource,
  typescriptDocPath,
  "CrawChatSdkClient.create(",
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
  "craw_chat_sdk",
  "must document craw_chat_sdk as the official Flutter consumer package",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "backend_sdk",
  "must document backend_sdk as the generated Flutter transport package",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "CrawChatClient",
  "must document CrawChatClient as the primary Flutter client",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "re-exports `backend_sdk`",
  "must explain that craw_chat_sdk re-exports backend_sdk",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "package:craw_chat_sdk/craw_chat_sdk.dart",
  "must point app consumers to the craw_chat_sdk package entrypoint",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "AuthApi",
  "must document that backend_sdk now exports AuthApi in the Flutter guide",
);
requireIncludes(
  flutterDocSource,
  flutterDocPath,
  "PortalApi",
  "must document that backend_sdk now exports PortalApi in the Flutter guide",
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
  "does not ship `sdk.connect(...)`",
  "must state that Flutter has no delivered websocket live runtime",
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
  "@sdkwork/craw-chat-sdk",
  "must list @sdkwork/craw-chat-sdk in the SDK naming model",
);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "craw_chat_sdk",
  "must list craw_chat_sdk in the SDK naming model",
);
requireIncludes(
  sdkIndexSource,
  sdkIndexPath,
  "SdkworkBackendClient",
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
  "sdk.generated.inbox.getInbox()",
  "must document sdk.generated.inbox.getInbox() in the app SDK guide",
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
  "@sdkwork/craw-chat-sdk",
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
  "SdkworkBackendClient",
  "must explain the current raw generated transport client naming model in language support",
);
requireIncludes(
  languageSupportSource,
  languageSupportPath,
  "HTTP coordination only",
  "must describe Flutter realtime as HTTP coordination only when websocket runtime is absent",
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
  "sdkwork-craw-chat-backend-sdk",
  "com.sdkwork:craw-chat-backend-sdk",
  "Sdkwork.CrawChat.BackendSdk",
  "CrawChatBackendSdk",
  "github.com/sdkwork/craw-chat-backend-sdk",
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
  "sdkwork-craw-chat-backend-sdk",
  "craw_chat_sdk",
  "Tier A",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(rustDocPath, rustDocSource);

const javaDocPath = "sdk/java-sdk.md";
const javaDocSource = readRequired(javaDocPath);
requireLanguageDocStructure(javaDocPath, javaDocSource, [
  "com.sdkwork:craw-chat-backend-sdk",
  "com.sdkwork:craw-chat-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(javaDocPath, javaDocSource);

const csharpDocPath = "sdk/csharp-sdk.md";
const csharpDocSource = readRequired(csharpDocPath);
requireLanguageDocStructure(csharpDocPath, csharpDocSource, [
  "Sdkwork.CrawChat.BackendSdk",
  "Sdkwork.CrawChat.Sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(csharpDocPath, csharpDocSource);

const swiftDocPath = "sdk/swift-sdk.md";
const swiftDocSource = readRequired(swiftDocPath);
requireLanguageDocStructure(swiftDocPath, swiftDocSource, [
  "CrawChatBackendSdk",
  "CrawChatSdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(swiftDocPath, swiftDocSource);

const kotlinDocPath = "sdk/kotlin-sdk.md";
const kotlinDocSource = readRequired(kotlinDocPath);
requireLanguageDocStructure(kotlinDocPath, kotlinDocSource, [
  "com.sdkwork:craw-chat-backend-sdk",
  "com.sdkwork:craw-chat-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(kotlinDocPath, kotlinDocSource);

const goDocPath = "sdk/go-sdk.md";
const goDocSource = readRequired(goDocPath);
requireLanguageDocStructure(goDocPath, goDocSource, [
  "github.com/sdkwork/craw-chat-backend-sdk",
  "github.com/sdkwork/craw-chat-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
]);
requireTransportLanguageApiMap(goDocPath, goDocSource);

const pythonDocPath = "sdk/python-sdk.md";
const pythonDocSource = readRequired(pythonDocPath);
requireLanguageDocStructure(pythonDocPath, pythonDocSource, [
  "sdkwork-craw-chat-backend-sdk",
  "sdkwork-craw-chat-sdk",
  "Tier B",
  "generated/server-openapi",
  "composed",
  "CrawChatSdkClient",
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
  "/sdk/admin-sdk",
]) {
  requireIncludes(boundaryDocSource, boundaryDocPath, marker, `must include ${marker}`);
}

const adminSdkPath = "sdk/admin-sdk.md";
const adminSdkSource = readRequired(adminSdkPath);
for (const marker of [
  "Choose This Family When",
  "Do Not Use The Admin SDK For",
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
  "@sdkwork/craw-chat-sdk",
  "must surface the official TypeScript SDK package on the docs home page",
);

if (issues.length > 0) {
  console.error(issues.join("\n"));
  process.exit(1);
}

console.log("Verified SDK documentation contract pages.");
