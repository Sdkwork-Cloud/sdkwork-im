import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const currentDir = path.dirname(fileURLToPath(import.meta.url));
const docsRoot = path.resolve(currentDir, "..");

export const overviewItems = [
  { text: "API Overview", link: "/api-reference/index" },
  { text: "Authentication and Errors", link: "/api-reference/auth-and-errors" },
  { text: "App API Overview", link: "/api-reference/app-api" },
  { text: "Platform API Overview", link: "/api-reference/platform-api" },
  { text: "IoT API Overview", link: "/api-reference/iot-api" },
  { text: "Control Plane Overview", link: "/api-reference/control-plane-api" },
];

export const groupedPages = [
  {
    text: "App API",
    pages: [
      { text: "Session and Realtime", link: "/api-reference/app/session-and-realtime" },
      { text: "Device Sync", link: "/api-reference/app/device-sync" },
      { text: "Conversations and Handoff", link: "/api-reference/app/conversations" },
      { text: "Membership and Read State", link: "/api-reference/app/membership-and-read-state" },
      { text: "Messages", link: "/api-reference/app/messages" },
      { text: "Media", link: "/api-reference/app/media" },
      { text: "Streams", link: "/api-reference/app/streams" },
      { text: "RTC", link: "/api-reference/app/rtc" },
    ],
  },
  {
    text: "Platform API",
    pages: [
      { text: "Notifications", link: "/api-reference/platform/notifications" },
      { text: "Automation", link: "/api-reference/platform/automation" },
      { text: "Audit", link: "/api-reference/platform/audit" },
      { text: "Operations", link: "/api-reference/platform/ops" },
      { text: "Provider Health", link: "/api-reference/platform/provider-health" },
    ],
  },
  {
    text: "IoT API",
    pages: [{ text: "Protocol and Health", link: "/api-reference/iot/protocol-and-health" }],
  },
  {
    text: "Control Plane API",
    pages: [
      { text: "Protocol Governance", link: "/api-reference/control-plane/protocol" },
      { text: "Provider Governance", link: "/api-reference/control-plane/providers" },
      { text: "Node Operations", link: "/api-reference/control-plane/nodes" },
    ],
  },
];

export const pageOperationGroups = {
  "/api-reference/app/session-and-realtime": [
    { text: "Health and Probes", anchors: ["get-healthz", "get-readyz"] },
    { text: "Session Lifecycle", anchors: ["resume-session", "disconnect-session"] },
    { text: "Presence", anchors: ["heartbeat-presence", "get-presence-me"] },
    {
      text: "Realtime Delivery",
      anchors: [
        "sync-realtime-subscriptions",
        "list-realtime-events",
        "ack-realtime-events",
        "realtime-websocket",
      ],
    },
  ],
  "/api-reference/app/device-sync": [
    { text: "Registration", anchors: ["register-device"] },
    { text: "Projection Sync Feed", anchors: ["get-device-sync-feed"] },
  ],
  "/api-reference/app/conversations": [
    {
      text: "Inbox and Provisioning",
      anchors: [
        "get-inbox",
        "create-conversation",
        "create-agent-dialog",
        "create-agent-handoff",
        "create-system-channel",
      ],
    },
    {
      text: "Conversation State",
      anchors: ["get-conversation-summary"],
    },
    {
      text: "Agent Handoff Flow",
      anchors: [
        "get-agent-handoff-state",
        "accept-agent-handoff",
        "resolve-agent-handoff",
        "close-agent-handoff",
      ],
    },
  ],
  "/api-reference/app/membership-and-read-state": [
    {
      text: "Membership",
      anchors: [
        "list-members",
        "add-member",
        "remove-member",
        "transfer-owner",
        "change-member-role",
        "leave-conversation",
      ],
    },
    { text: "Read Cursor", anchors: ["get-read-cursor", "update-read-cursor"] },
  ],
  "/api-reference/app/messages": [
    { text: "Timeline Read", anchors: ["get-timeline"] },
    {
      text: "Message Submission and Mutation",
      anchors: [
        "post-message",
        "publish-system-channel-message",
        "edit-message",
        "recall-message",
      ],
    },
  ],
  "/api-reference/app/media": [
    {
      text: "Upload Lifecycle",
      anchors: ["create-media-upload", "complete-media-upload"],
    },
    { text: "Asset Access", anchors: ["get-media", "get-media-download-url"] },
    { text: "Conversation Attachment", anchors: ["attach-media"] },
  ],
  "/api-reference/app/streams": [
    {
      text: "Session Lifecycle",
      anchors: ["open-stream", "checkpoint-stream", "complete-stream", "abort-stream"],
    },
    { text: "Frame Transport", anchors: ["append-stream-frame", "list-stream-frames"] },
  ],
  "/api-reference/app/rtc": [
    {
      text: "Session Lifecycle",
      anchors: [
        "create-rtc-session",
        "invite-rtc-session",
        "accept-rtc-session",
        "reject-rtc-session",
        "end-rtc-session",
      ],
    },
    { text: "Signals and Credentials", anchors: ["post-rtc-signal", "issue-rtc-participant-credential"] },
    { text: "Artifacts and Provider Integration", anchors: ["get-rtc-recording-artifact", "map-rtc-provider-callback"] },
  ],
  "/api-reference/platform/notifications": [
    { text: "Submission", anchors: ["request-notification"] },
    { text: "Read Models", anchors: ["list-notifications", "get-notification"] },
  ],
  "/api-reference/platform/automation": [
    { text: "Executions", anchors: ["request-automation-execution", "get-automation-execution"] },
  ],
  "/api-reference/platform/audit": [
    { text: "Write", anchors: ["record-audit-anchor"] },
    { text: "Read and Export", anchors: ["list-audit-records", "export-audit-bundle"] },
  ],
  "/api-reference/platform/ops": [
    { text: "Health and Topology", anchors: ["get-ops-health", "get-ops-cluster", "get-ops-lag", "get-ops-replay-status"] },
    { text: "Runtime and Provider Mirrors", anchors: ["get-ops-runtime-dir", "get-ops-provider-bindings", "get-ops-provider-binding-drift"] },
    { text: "Diagnostics", anchors: ["get-ops-diagnostics"] },
  ],
  "/api-reference/platform/provider-health": [
    {
      text: "Provider Probes",
      anchors: [
        "get-media-provider-health",
        "get-rtc-provider-health",
        "get-user-module-provider-health",
      ],
    },
  ],
  "/api-reference/iot/protocol-and-health": [
    {
      text: "Provider Health",
      anchors: ["get-iot-access-provider-health", "get-iot-protocol-provider-health"],
    },
    { text: "Protocol Traffic", anchors: ["ingest-iot-uplink", "ingest-iot-downlink"] },
  ],
  "/api-reference/control-plane/protocol": [
    { text: "Probe", anchors: ["get-control-healthz"] },
    { text: "Registry and Governance", anchors: ["get-protocol-registry", "get-protocol-governance"] },
  ],
  "/api-reference/control-plane/providers": [
    { text: "Registry and Effective Bindings", anchors: ["get-provider-registry", "get-provider-bindings", "upsert-provider-binding-policy"] },
    { text: "Policy History and Diff", anchors: ["get-provider-policy-history", "get-provider-policy-diff"] },
    { text: "Preview and Rollback", anchors: ["preview-provider-policy", "rollback-provider-policy"] },
  ],
  "/api-reference/control-plane/nodes": [
    { text: "Lifecycle", anchors: ["drain-node", "activate-node"] },
    { text: "Route Migration", anchors: ["migrate-node-routes"] },
  ],
};

export function markdownPathFor(pageLink) {
  return path.join(
    docsRoot,
    `${pageLink.replace(/^\//, "").replaceAll("/", path.sep)}.md`,
  );
}

export function formatOperationText(operationTitle) {
  const match = operationTitle.match(/^([A-Z]+)\s+(.+)$/);
  if (!match) {
    return operationTitle;
  }

  const [, method, route] = match;
  const cleanRoute = route.startsWith("/api/v1") ? route.slice("/api/v1".length) || "/" : route;
  return `${method} ${cleanRoute}`;
}

export function readOperations(pageLink) {
  const content = fs.readFileSync(markdownPathFor(pageLink), "utf8");

  return [...content.matchAll(/<a id="([^"]+)"><\/a>[\s\S]*?## `([^`]+)`/g)].map(
    ([, anchor, operationTitle]) => ({
      anchor,
      operationTitle,
      text: formatOperationText(operationTitle),
      sourceLink: `${pageLink}#${anchor}`,
    }),
  );
}

export function operationPageLink(pageLink, anchor) {
  const pageTail = pageLink.replace(/^\/api-reference\//, "");
  return `/api-reference/operations/${pageTail}/${anchor}`;
}

export function operationMarkdownPath(pageLink, anchor) {
  return path.join(
    docsRoot,
    "api-reference",
    "operations",
    ...pageLink.replace(/^\/api-reference\//, "").split("/"),
    `${anchor}.md`,
  );
}

function buildPageItems(page) {
  const operations = readOperations(page.link);
  const operationByAnchor = new Map(operations.map((operation) => [operation.anchor, operation]));
  const consumed = new Set();
  const configuredGroups = pageOperationGroups[page.link] ?? [];

  const items = [{ text: "Overview", link: page.link }];

  for (const group of configuredGroups) {
    const groupItems = group.anchors
      .map((anchor) => operationByAnchor.get(anchor))
      .filter(Boolean)
      .map((operation) => {
        consumed.add(operation.anchor);
        return { text: operation.text, link: operationPageLink(page.link, operation.anchor) };
      });

    if (groupItems.length > 0) {
      items.push({ text: group.text, items: groupItems });
    }
  }

  const remaining = operations
    .filter((operation) => !consumed.has(operation.anchor))
    .map((operation) => ({
      text: operation.text,
      link: operationPageLink(page.link, operation.anchor),
    }));

  if (remaining.length > 0) {
    items.push({
      text: configuredGroups.length > 0 ? "Other Operations" : "Operations",
      items: remaining,
    });
  }

  return items;
}

function buildGroupedSidebar() {
  return groupedPages.map((group) => ({
    text: group.text,
    items: group.pages.map((page) => ({
      text: page.text,
      items: buildPageItems(page),
    })),
  }));
}

function collectOperationLinks(items) {
  const links = [];

  for (const item of items) {
    if (item.link?.startsWith("/api-reference/operations/")) {
      links.push(item.link);
    }
    if (item.items) {
      links.push(...collectOperationLinks(item.items));
    }
  }

  return links;
}

const groupedSidebar = buildGroupedSidebar();

export const apiReferenceSidebar = [
  {
    text: "Overview",
    items: overviewItems,
  },
  ...groupedSidebar,
];

export const apiReferenceOperationLinks = collectOperationLinks(groupedSidebar);
