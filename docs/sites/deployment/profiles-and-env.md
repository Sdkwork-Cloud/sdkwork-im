# Profiles and Environment

## Profile Matrix

| Item | `local-minimal` | `local-default` |
| --- | --- | --- |
| Template | `deployments/templates/local-minimal.env.example` | `deployments/templates/local-default.env.example` |
| Primary config file | `.runtime/local-minimal/config/local-minimal.env` | `.runtime/local-default/config/local-default.env` |
| Default bind | `127.0.0.1:18090` | `127.0.0.1:18090` |
| Effective runtime dir by default | `.runtime/local-minimal` | Falls back to `.runtime/local-minimal` |
| Compose shape | Standalone definition | Extends `local-minimal` |

## Baseline Environment Variables

| Variable | Purpose |
| --- | --- |
| `CRAW_CHAT_BIND_ADDR` | Listener bind address |
| `CRAW_CHAT_RUNTIME_DIR` | Managed runtime directory |
| `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET` | Public HS256 bearer signing secret |

### Why `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET` matters

It is required for:

- public bearer validation in `build_public_app()`
- locally generated smoke tokens
- public app access in Docker Compose

If it is missing, the local start scripts reject startup.

## Security Hardening Variables

For public or commercial deployments, keep these enabled and explicit:

| Variable | Purpose |
| --- | --- |
| `CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP` | Requires `exp` in public bearer tokens (`1/true/yes/on`). |
| `CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS` | Rejects public bearer tokens whose lifetime exceeds this maximum. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS` | Per-tenant request ceiling for `/api/v1/conversations/shared-channel-links/sync` inside each process. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS` | Sliding window used by the shared-channel sync per-tenant limiter. |
| `CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP` | Emergency local-test override for non-HTTPS shared-channel sync targets. Keep `false` in non-local environments. |

### Shared-channel sync target variables

| Variable | Purpose |
| --- | --- |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_TARGET_BASE_URL` | Enables standalone control-plane sync dispatch to conversation-runtime public HTTP route. |
| `CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP` | Allows `http://` target only for controlled local testing; production should use HTTPS. |

## Optional Provider-related Variables

The current repository also uses a user-module provider selection boundary:

| Variable | Purpose |
| --- | --- |
| `CRAW_CHAT_USER_MODULE_PROVIDER` | Selects the user-module provider mode |
| `CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH` | Catalog path for the external user-module provider |
| `CRAW_CHAT_USER_MODULE_EXTERNAL_SYSTEM` | External system identifier for that provider |

When `CRAW_CHAT_USER_MODULE_PROVIDER=external` is selected without the required external catalog
path, the provider-health surface reports the external mode as unavailable.

## Template Contents

### `local-minimal.env.example`

```dotenv
CRAW_CHAT_BIND_ADDR=127.0.0.1:18090
CRAW_CHAT_RUNTIME_DIR=.runtime/local-minimal
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=replace-with-local-minimal-secret
CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP=true
CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS=900
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS=120
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS=60
CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP=false
```

### `local-default.env.example`

```dotenv
CRAW_CHAT_BIND_ADDR=127.0.0.1:18090
CRAW_CHAT_RUNTIME_DIR=.runtime/local-minimal
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=replace-with-local-default-secret
CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP=true
CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS=900
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS=120
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS=60
CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP=false
```

That second template is explicit evidence that the current `local-default` profile still reuses the
`local-minimal` runtime contract.
