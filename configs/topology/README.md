# Topology profiles

Machine contract: [../../specs/topology.spec.json](../../specs/topology.spec.json)  
Platform standard: [../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md](../../../sdkwork-specs/APP_RUNTIME_TOPOLOGY_ADOPTION.md)

## Profiles

| File | Profile id | Use |
| --- | --- | --- |
| `standalone.unified-process.development.env` | `standalone.unified-process.development` | Default dev (`pnpm dev`) |
| `standalone.split-services.development.env` | `standalone.split-services.development` | Standalone split local integration |
| `standalone.unified-process.production.env` | `standalone.unified-process.production` | Standalone unified production |
| `standalone.split-services.production.env` | `standalone.split-services.production` | Standalone split production |
| `cloud.split-services.production.env` | `cloud.split-services.production` | Cloud production |

## Standalone gateway

Standalone profiles embed IAM and IM application ingress through `sdkwork-im-standalone-gateway`
on `application.public-ingress`. Client and platform SDK URLs collapse to the same bind.

| Command | Purpose |
| --- | --- |
| `pnpm gateway:run:standalone` | Run standalone gateway only |
| `pnpm gateway:build:standalone` | Build standalone gateway binary |

## Default development binds

| Surface | Env key | Standalone unified value |
| --- | --- | --- |
| Application ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `http://127.0.0.1:18079` |
| Platform gateway (collapsed) | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `http://127.0.0.1:18079` |

Load order: `scripts/im-dev.mjs` and `scripts/im-server-dev.mjs` merge the selected profile before spawning services.

## Verification

```bash
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root ../.. --spec specs/topology.spec.json
pnpm test:topology-baggage
pnpm test:sdkwork-im-pc-dev-command
```
