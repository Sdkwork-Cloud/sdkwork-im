# Profiles and Environment

## Profile Matrix

| Item | `local-minimal` | `local-default` |
| --- | --- | --- |
| Template | `deployments/templates/local-minimal.env.example` | `deployments/templates/local-default.env.example` |
| Primary config file | `.runtime/local-minimal/config/local-minimal.env` | `.runtime/local-default/config/local-default.env` |
| Default bind | `127.0.0.1:18090` | `127.0.0.1:18090` |
| Effective runtime dir by default | `.runtime/local-minimal` | Falls back to `.runtime/local-minimal` |
| Compose shape | Standalone definition | Extends `local-minimal` |

## Required Environment Variables

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
```

### `local-default.env.example`

```dotenv
CRAW_CHAT_BIND_ADDR=127.0.0.1:18090
CRAW_CHAT_RUNTIME_DIR=.runtime/local-minimal
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=replace-with-local-default-secret
```

That second template is explicit evidence that the current `local-default` profile still reuses the
`local-minimal` runtime contract.
