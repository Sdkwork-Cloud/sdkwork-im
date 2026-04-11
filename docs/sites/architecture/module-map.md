# Module Map

Understanding the workspace module map is the fastest way to understand where behavior lives and
which directories are stable enough to document as product surfaces.

## Top-level Directories

| Directory | Current responsibility |
| --- | --- |
| `adapters/` | Provider and storage adapters such as local disk, local memory, IoT access, IoT MQTT, object storage, and RTC providers |
| `crates/` | Shared contracts, CCP protocol crates, auth context, runtime links, route ownership models, and domain primitives |
| `services/` | App runtime services, control-plane API, operator services, and business subsystems |
| `tools/` | Local verification tools such as `chat-cli` and smoke workflows |
| `bin/` | PowerShell, Bash, and CMD lifecycle wrappers for local development and operations |
| `deployments/` | Dockerfile, Compose profiles, environment templates, and bootstrap scripts |
| `sdks/` | App and admin SDK workspaces, generation wrappers, and release-catalog metadata |
| `docs/` | Historical docs plus the current VitePress site under `docs/sites` |
| `apps/` | Frontend workspace directories that currently exist but are not documented as mature product surfaces |

## Key Services

| Service | Responsibility |
| --- | --- |
| `local-minimal-node` | App-facing HTTP node that assembles the current default runtime |
| `conversation-runtime` | Conversation, membership, message, and handoff behavior |
| `session-gateway` | Session resume, presence, realtime route ownership, disconnect fences, and websocket handling |
| `projection-service` | Inbox, timeline, summary, and read-model projection support |
| `media-service` | Media upload lifecycle, lookup, attachment, and provider-aware download URLs |
| `streaming-service` | Stream sessions, frames, checkpoints, completion, and abort flow |
| `rtc-signaling-service` | RTC session lifecycle, signaling, credentials, artifacts, and provider interactions |
| `notification-service` | Notification task submission and retrieval |
| `automation-service` | Automation execution submission and retrieval |
| `audit-service` | Audit record storage and export |
| `ops-service` | Health, cluster, lag, diagnostics, runtime-dir, and provider-binding views |
| `control-plane-api` | Protocol governance, provider governance, and node lifecycle API |

## Key Contract and Protocol Crates

| Crate group | Why it matters |
| --- | --- |
| `craw-chat-contract-*` | Business and transport contracts for app-facing surfaces |
| `craw-chat-ccp-*` | CCP binding, codec, control, core, and registry surfaces |
| `im-platform-contracts` | Provider registry, effective binding, and platform integration contracts |
| `im-auth-context` | Shared auth-context parsing for bearer and trusted-header flows |
| `craw-chat-runtime-*` | Runtime linking and route-ownership contracts |
| `im-domain-*` | Core domain and event-level models reused by services |

## What The Docs Deliberately Do Not Overstate

- `apps/craw-chat-admin` and `apps/craw-chat-portal` are not documented as complete products.
- SDK workspaces are documented separately from actual release status.
- OpenAPI authority currently exists for the app SDK workspace, but not as a checked-in admin SDK
  source under `sdks/sdkwork-craw-chat-sdk-admin/`.

That distinction is important for a mature documentation standard: directory presence alone is not
treated as product delivery.
