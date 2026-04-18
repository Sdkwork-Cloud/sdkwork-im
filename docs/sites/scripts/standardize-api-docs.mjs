import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");
const apiRoot = path.join(docsRoot, "api-reference");

const markdownFiles = [];

function walk(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      walk(fullPath);
      continue;
    }
    if (entry.name.endsWith(".md")) {
      markdownFiles.push(fullPath);
    }
  }
}

function normalizePath(filePath) {
  return filePath.replaceAll("\\", "/");
}

function pageKey(filePath) {
  const normalized = normalizePath(filePath);
  const marker = "/api-reference/";
  const index = normalized.lastIndexOf(marker);
  return index === -1 ? normalized : normalized.slice(index + marker.length).replace(/\.md$/, "");
}

function inferSuccessLabel(method, route, block) {
  const firstResponse = block.match(/### Response `(\d+)`/);
  if (!firstResponse) {
    return method === "GET" && route.endsWith("/ws")
      ? "101 Switching Protocols"
      : "Response";
  }

  const status = firstResponse[1];
  const schemaMatch = block.match(
    new RegExp(`### Response \`${status}\`[\\s\\S]*?<ApiSchemaTable schema="([^"]+)" \\/>`),
  );
  if (schemaMatch) {
    return `${status} ${schemaMatch[1]}`;
  }

  if (status === "101") {
    return "101 Switching Protocols";
  }

  return status;
}

function sdkLabel(page, route, method) {
  const labels = {
    "app/session-and-realtime": "`@sdkwork/craw-chat-sdk` / session, presence, and realtime helpers",
    "app/device-sync": "`@sdkwork/craw-chat-sdk` / generated device-sync transport",
    "app/conversations": route === "/api/v1/inbox"
      ? "`@sdkwork/craw-chat-sdk` / `sdk.generated.inbox.getInbox()`"
      : "`@sdkwork/craw-chat-sdk` / `sdk.conversations`",
    "app/membership-and-read-state": "`@sdkwork/craw-chat-sdk` / `sdk.conversations`",
    "app/messages": "`@sdkwork/craw-chat-sdk` / `sdk.messages`",
    "app/media": "`@sdkwork/craw-chat-sdk` / `sdk.media`",
    "app/streams": "`@sdkwork/craw-chat-sdk` / generated stream transport",
    "app/rtc": "`@sdkwork/craw-chat-sdk` / `sdk.rtc`",
    "platform/notifications": "No standalone published SDK family",
    "platform/automation": "No standalone published SDK family",
    "platform/audit": "No standalone published SDK family",
    "platform/ops": "No standalone published SDK family",
    "platform/provider-health": "No standalone published SDK family",
    "iot/protocol-and-health": "No standalone published SDK family",
    "control-plane/protocol": "`sdkwork-craw-chat-sdk-admin` / protocol-governance",
    "control-plane/providers": "`sdkwork-craw-chat-sdk-admin` / provider-governance",
    "control-plane/nodes": "`sdkwork-craw-chat-sdk-admin` / node-operations",
  };
  if (page === "app/session-and-realtime") {
    if (route === "/healthz" || route === "/readyz") {
      return "Direct HTTP probe";
    }
    if (route === "/api/v1/sessions/resume") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.connect({ deviceId })`, `sdk.generated.session.resume(...)`";
    }
    if (route === "/api/v1/sessions/disconnect") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.session.disconnect(...)`";
    }
    if (route === "/api/v1/presence/heartbeat") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.presence.heartbeat(...)`";
    }
    if (route === "/api/v1/presence/me") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.auth.me()`, `sdk.generated.presence.getPresenceMe()`";
    }
    if (route === "/api/v1/realtime/subscriptions/sync") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.connect(...)`, `sdk.generated.realtime.syncRealtimeSubscriptions(...)`";
    }
    if (route === "/api/v1/realtime/events") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.sync.catchUp(...)`, `sdk.generated.realtime.listRealtimeEvents(...)`";
    }
    if (route === "/api/v1/realtime/events/ack") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.sync.ack(...)`, `context.ack()`, `sdk.generated.realtime.ackRealtimeEvents(...)`";
    }
    if (route.endsWith("/ws")) {
      return "`@sdkwork/craw-chat-sdk` / `sdk.connect(...)`";
    }
  }

  if (page === "app/device-sync") {
    if (route === "/api/v1/devices/register") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.device.register(...)`";
    }
    return "`@sdkwork/craw-chat-sdk` / `sdk.generated.device.getDeviceSyncFeed(...)`";
  }

  if (page === "app/streams") {
    if (route === "/api/v1/streams") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.open(...)`";
    }
    if (route.endsWith("/frames") && method === "GET") {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.listStreamFrames(...)`";
    }
    if (route.endsWith("/frames")) {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.appendStreamFrame(...)`";
    }
    if (route.endsWith("/checkpoint")) {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.checkpoint(...)`";
    }
    if (route.endsWith("/complete")) {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.complete(...)`";
    }
    if (route.endsWith("/abort")) {
      return "`@sdkwork/craw-chat-sdk` / `sdk.generated.stream.abort(...)`";
    }
  }

  return labels[page] ?? "`sdkwork-craw-chat-sdk`";
}

function securityLabel(route, page) {
  if (route === "/healthz" || route === "/readyz") {
    return "Open endpoint";
  }

  return "Bearer token";
}

function permissionLabel(page, method, route) {
  if (route === "/healthz" || route === "/readyz") {
    return "Not required";
  }

  switch (page) {
    case "app/session-and-realtime":
      if (route.endsWith("/ws")) {
        return "Authenticated principal; active device route is prepared before upgrade.";
      }
      return "Authenticated principal; device ownership and session binding are enforced where required.";
    case "app/device-sync":
      if (route.endsWith("/sync-feed")) {
        return "Registered device owner.";
      }
      return "Authenticated principal; `deviceId` must match the bound auth context when present.";
    case "app/conversations":
      if (route === "/api/v1/inbox") {
        return "Authenticated principal.";
      }
      if (
        route === "/api/v1/conversations" ||
        route === "/api/v1/conversations/agent-dialogs" ||
        route === "/api/v1/conversations/agent-handoffs" ||
        route === "/api/v1/conversations/system-channels"
      ) {
        return "Authenticated principal.";
      }
      if (route.endsWith("/agent-handoff/accept")) {
        return "Active conversation member with `handoff.accept` authority.";
      }
      if (route.endsWith("/agent-handoff/resolve")) {
        return "Active conversation member with `handoff.resolve` authority.";
      }
      if (route.endsWith("/agent-handoff/close")) {
        return "Active conversation member with `handoff.close` authority.";
      }
      return "Active conversation member.";
    case "app/membership-and-read-state":
      if (method === "GET" || route.endsWith("/members/leave")) {
        return "Active conversation member.";
      }
      return "Conversation-bound write access.";
    case "app/messages":
      return method === "GET"
        ? "Active conversation member."
        : "Conversation-bound write access.";
    case "app/media":
      if (route.endsWith("/attach")) {
        return "Authenticated principal with media asset ownership and target conversation write access.";
      }
      return "Authenticated principal with media asset ownership checks.";
    case "app/streams":
      if (route === "/api/v1/streams") {
        return "Conversation `stream.open` capability or device stream permission.";
      }
      if (method === "GET") {
        return "Conversation member or registered device read scope.";
      }
      if (route.endsWith("/checkpoint")) {
        return "Conversation `stream.checkpoint` capability or device stream permission.";
      }
      if (route.endsWith("/complete")) {
        return "Conversation `stream.complete` capability or device stream permission.";
      }
      if (route.endsWith("/abort")) {
        return "Conversation `stream.abort` capability or device stream permission.";
      }
      return "Conversation `stream.append` capability or device stream permission.";
    case "app/rtc":
      if (route === "/api/v1/rtc/sessions") {
        return "Conversation `rtc.create` capability when the session is bound to a conversation.";
      }
      if (route.endsWith("/invite")) {
        return "Conversation `rtc.invite` capability.";
      }
      if (route.endsWith("/accept")) {
        return "Conversation `rtc.accept` capability.";
      }
      if (route.endsWith("/reject")) {
        return "Conversation `rtc.reject` capability.";
      }
      if (route.endsWith("/end")) {
        return "Conversation `rtc.end` capability.";
      }
      if (route.endsWith("/signals")) {
        return "Conversation `rtc.signal` capability.";
      }
      if (route.endsWith("/credentials")) {
        return "Conversation `rtc.issue_credential` capability.";
      }
      if (route.endsWith("/artifacts/recording")) {
        return "Conversation `rtc.artifact` capability.";
      }
      return "Authenticated principal; provider callback mapping is validated by the RTC runtime.";
    case "platform/notifications":
      return method === "POST"
        ? "Own recipient scope or `notification.write` for delegated sends."
        : "Current recipient scope.";
    case "platform/automation":
      return method === "POST" ? "`automation.execute`" : "`automation.read`";
    case "platform/audit":
      return method === "POST" ? "`audit.write`" : "`audit.read`";
    case "platform/ops":
      return "`ops.read`";
    case "platform/provider-health":
      return "Authenticated principal.";
    case "iot/protocol-and-health":
      if (method === "GET") {
        return "Authenticated principal.";
      }
      if (route.endsWith("/uplink")) {
        return "Registered bound device actor.";
      }
      return "Registered device scope with `device.command.send`.";
    case "control-plane/protocol":
    case "control-plane/providers":
    case "control-plane/nodes":
      return method === "GET"
        ? "`control.read` or `control.write`"
        : "`control.write`";
    default:
      return "Authenticated principal.";
  }
}

function metaGrid(method, route, block, filePath) {
  const page = pageKey(filePath);
  const cards = [
    ["Security", securityLabel(route, page)],
    ["SDK", sdkLabel(page, route, method)],
    ["Permission", permissionLabel(page, method, route)],
    ["Success", `\`${inferSuccessLabel(method, route, block)}\``],
  ];

  return [
    '<div class="api-meta-grid">',
    ...cards.map(
      ([label, value]) =>
        `  <div class="api-meta-card"><strong>${label}</strong><span>${value}</span></div>`,
    ),
    "</div>",
  ].join("\n");
}

function appReadErrors() {
  return [
    ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
    [
      "403",
      "`conversation_permission_denied`, `device_permission_denied`, `permission_denied`",
      "The caller is not allowed to access the target resource.",
    ],
    ["404", "`*_not_found`", "The requested resource does not exist."],
    [
      "409",
      "`reconnect_required`, `disconnect_fence_conflict`, `conflict`",
      "Current runtime state blocks the read or handshake flow.",
    ],
    ["503", "`*_unavailable`", "A required subsystem or provider is unavailable."],
  ];
}

function appWriteErrors() {
  return [
    ["400", "`invalid_request`, `validation_error`", "The request payload or parameters are invalid."],
    ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
    [
      "403",
      "`conversation_permission_denied`, `device_permission_denied`, `permission_denied`",
      "The caller is not allowed to mutate the target resource.",
    ],
    ["404", "`*_not_found`", "The requested resource does not exist."],
    [
      "409",
      "`reconnect_required`, `disconnect_fence_conflict`, `conflict`",
      "Current runtime state blocks the mutation.",
    ],
    ["503", "`*_unavailable`", "A required subsystem or provider is unavailable."],
  ];
}

function errorRows(page, method, route) {
  if (route === "/healthz" || route === "/readyz") {
    return [];
  }

  switch (page) {
    case "platform/ops":
      return [
        ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
        ["403", "`permission_denied`", "The caller lacks `ops.read`."],
        ["503", "`*_unavailable`", "Operational diagnostics are temporarily unavailable."],
      ];
    case "platform/audit":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The audit anchor payload is invalid."],
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks `audit.write`."],
          ]
        : [
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks `audit.read`."],
          ];
    case "platform/automation":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The automation execution request is invalid."],
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks `automation.execute`."],
            ["409", "`automation_execution_conflict`", "The execution id conflicts with an existing request."],
            ["503", "`automation_store_unavailable`, `journal_unavailable`", "Automation persistence is unavailable."],
          ]
        : [
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks `automation.read`."],
            ["404", "`automation_execution_not_found`", "The requested automation execution does not exist."],
            ["503", "`automation_store_unavailable`", "Automation persistence is unavailable."],
          ];
    case "platform/notifications":
      return method === "POST"
        ? [
            ["400", "`invalid_request`, `validation_error`", "The notification request is invalid."],
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks delegated notification authority."],
            ["409", "`notification_conflict`", "The idempotent notification request conflicts with existing state."],
          ]
        : [
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller is not allowed to read the target notification scope."],
            ["404", "`notification_not_found`", "The requested notification task does not exist."],
          ];
    case "platform/provider-health":
      return [
        ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
        ["503", "`*_unavailable`", "The provider health source is unavailable."],
      ];
    case "iot/protocol-and-health":
      return method === "GET"
        ? [
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["503", "`*_unavailable`", "The provider health source is unavailable."],
          ]
        : route.endsWith("/uplink")
          ? [
              ["400", "`device_id_missing`, `device_id_mismatch`, `invalid_request`", "The uplink payload or bound device id is invalid."],
              ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
              ["403", "`device_permission_denied`", "The caller is not an authorized device actor."],
              ["404", "`device_not_found`", "The target device is not registered."],
              ["503", "`*_unavailable`", "The IoT protocol adapter is unavailable."],
            ]
          : [
              ["400", "`device_id_missing`, `device_id_mismatch`, `invalid_request`", "The downlink payload or device id is invalid."],
              ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
              ["403", "`device_permission_denied`", "The caller lacks `device.command.send` or device ownership."],
              ["404", "`device_not_found`", "The target device is not registered."],
              ["503", "`*_unavailable`", "The IoT protocol adapter is unavailable."],
            ];
    case "control-plane/protocol":
    case "control-plane/providers":
    case "control-plane/nodes":
      return method === "GET"
        ? [
            ["400", "`invalid_request`", "Query or path parameters are invalid."],
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks the required control-plane permission."],
            ["503", "`*_unavailable`", "The governance snapshot or provider runtime is unavailable."],
          ]
        : [
            ["400", "`invalid_request`, `invalid_provider_policy`", "The mutation payload is invalid."],
            ["401", "`missing_authorization`, `invalid_token`", "Authentication failed."],
            ["403", "`permission_denied`", "The caller lacks `control.write`."],
            ["404", "`*_not_found`, `provider_plugin_not_found`", "The requested node, plugin, or target resource does not exist."],
            ["409", "`*_conflict`, `provider_policy_conflict`", "Current control-plane state blocks the mutation."],
            ["503", "`*_unavailable`", "The governance snapshot or provider runtime is unavailable."],
          ];
    default:
      return method === "GET" ? appReadErrors() : appWriteErrors();
  }
}

function buildErrorResponses(method, route, filePath) {
  const rows = errorRows(pageKey(filePath), method, route);
  if (!rows.length) {
    return "";
  }

  return `### Error Responses

| HTTP | \`code\` | Description |
| --- | --- | --- |
${rows.map(([status, code, description]) => `| \`${status}\` | ${code} | ${description} |`).join("\n")}`;
}

function insertOrReplaceMetaGrid(block, meta) {
  if (/\n<div class="api-meta-grid">[\s\S]*?(?=\n### )/.test(block)) {
    return block.replace(/\n<div class="api-meta-grid">[\s\S]*?(?=\n### )/, `\n${meta}\n`);
  }

  return block.replace(/\n### /, `\n\n${meta}\n\n### `);
}

function ensureNoBodyRequestSection(block) {
  const mentionsNoJsonBody =
    /No JSON request body is required/i.test(block) ||
    /does not accept a JSON request body/i.test(block) ||
    /does not require a JSON request body/i.test(block);

  if (!mentionsNoJsonBody || block.includes("### Request Body")) {
    return block;
  }

  return block.replace(
    /\n### Response /,
    `\n### Request Body\n\nNone. This operation does not accept a JSON request body.\n\n### Response `,
  );
}

function normalizeBlock(block, filePath) {
  const titleMatch = block.match(/## `([A-Z]+) ([^`]+)`/);
  if (!titleMatch) {
    return block;
  }

  const [, method, route] = titleMatch;
  let nextBlock = block;
  const meta = metaGrid(method, route, nextBlock, filePath);

  nextBlock = insertOrReplaceMetaGrid(nextBlock, meta);

  if (method === "POST") {
    nextBlock = ensureNoBodyRequestSection(nextBlock);
  }

  if (!nextBlock.includes("### Error Responses")) {
    const errors = buildErrorResponses(method, route, filePath);
    if (errors) {
      nextBlock = nextBlock.replace(/\n<\/section>\s*$/, `\n\n${errors}\n\n</section>\n`);
    }
  }

  return nextBlock;
}

walk(apiRoot);

const blockPattern =
  /<a id="([^"]+)"><\/a>[\s\S]*?<section class="api-op">[\s\S]*?(?=<a id="|$)/g;

for (const filePath of markdownFiles) {
  const original = fs.readFileSync(filePath, "utf8");
  const updated = original.replace(blockPattern, (block) => normalizeBlock(block, filePath));
  if (updated !== original) {
    fs.writeFileSync(filePath, updated);
  }
}

console.log(`Standardized ${markdownFiles.length} API markdown files.`);
