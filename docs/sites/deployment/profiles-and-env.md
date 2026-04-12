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
| `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS` | Optional strict issuer match. When set, public bearer `iss` must equal this value. |
| `CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD` | Optional strict audience match. When set, public bearer `aud` must include this value. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS` | Per-tenant request ceiling for `/api/v1/conversations/shared-channel-links/sync` inside each process (clamped to `1..10000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS` | Sliding window used by the shared-channel sync per-tenant limiter (clamped to `1..3600`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS` | Maximum active tenant buckets retained by the in-process shared-channel sync limiter (default `10000`, clamped to `1..200000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS` | Outbound HTTP timeout (milliseconds) for control-plane shared-channel sync trigger dispatch. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED` | Enables periodic stale-claim reclaim for pending shared-channel sync requests (`1/true/yes/on`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS` | Scheduler tick interval in milliseconds for stale-claim reclaim scans (clamped to `1000..600000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS` | Adds bounded per-tick jitter for stale-claim scheduler sleeps to reduce multi-node synchronized scan spikes (default `250`, clamped to `0..5000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT` | Number of background dispatch workers that execute shared-channel sync outbound HTTP requests (capped at 128). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY` | Bounded in-memory queue capacity for shared-channel sync dispatch tasks (capped at 65,536); full queue returns backpressure errors. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS` | Retention window for delivered shared-channel sync ledger items before prune (default `2592000000`, capped at `31536000000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES` | Maximum delivered shared-channel sync ledger entries kept in durable state (default `200000`, capped at `2000000`). |
| `CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP` | Emergency local-test override for non-HTTPS shared-channel sync targets. Keep `false` in non-local environments. |
| `CRAW_CHAT_RUNTIME_PROFILE` | Runtime profile name (`local-minimal`/`local-default`/etc.). Remote `http://` override is only honored for local profiles. |

### Shared-channel sync target variables

| Variable | Purpose |
| --- | --- |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_TARGET_BASE_URL` | Enables standalone control-plane sync dispatch to conversation-runtime public HTTP route. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS` | Caps each tenant's in-process sync request budget per window (`1..10000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS` | Defines per-tenant sync limiter window size in seconds (`1..3600`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS` | Bounds active per-tenant limiter buckets to prevent in-memory amplification (`1..200000`, default `10000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS` | Caps outbound shared-channel sync request/response wait time to fail fast on transport stalls (max 60,000ms). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED` | Turns on automatic stale pending-claim reclaim scans without operator-triggered repair calls. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS` | Controls how often the reclaim scheduler checks pending shared-channel sync leases (clamped to `1000..600000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS` | Adds bounded jitter to each scheduler sleep tick (`0..5000`, default `250`) to avoid synchronized reclaim spikes in multi-instance deployments. |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT` | Tunes shared-channel sync dispatch throughput by increasing worker parallelism (max 128). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY` | Caps dispatch backlog size and enforces backpressure when queue capacity is exhausted (max 65,536). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS` | Caps delivered-ledger retention duration used by pruning (`0 < value <= 31536000000`, default `2592000000`). |
| `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES` | Caps delivered-ledger durable entry count used by pruning (`0 < value <= 2000000`, default `200000`). |
| `CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP` | Allows `http://` target only for controlled local testing; production should use HTTPS. |
| `CRAW_CHAT_RUNTIME_PROFILE` | Must be an explicit local profile (`local-minimal`/`local-default`/`local`/`dev`/`test`/`ci`) before remote `http://` override is accepted. |

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
CRAW_CHAT_RUNTIME_PROFILE=local-minimal
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=replace-with-local-minimal-secret
CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP=true
CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS=900
CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS=
CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD=
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS=120
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS=60
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS=10000
CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS=5000
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED=true
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS=30000
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS=250
CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT=4
CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY=1024
CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS=2592000000
CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES=200000
CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP=false
```

### `local-default.env.example`

```dotenv
CRAW_CHAT_BIND_ADDR=127.0.0.1:18090
CRAW_CHAT_RUNTIME_DIR=.runtime/local-minimal
CRAW_CHAT_RUNTIME_PROFILE=local-default
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=replace-with-local-default-secret
CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP=true
CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS=900
CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS=
CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD=
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS=120
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS=60
CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS=10000
CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS=5000
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED=true
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS=30000
CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS=250
CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT=4
CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY=1024
CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS=2592000000
CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES=200000
CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP=false
```

That second template is explicit evidence that the current `local-default` profile still reuses the
`local-minimal` runtime contract.
