# Topology profiles

Machine contract: [../../specs/topology.spec.json](../../specs/topology.spec.json)  
Greenfield plan: [../../docs/topology-greenfield.md](../../docs/topology-greenfield.md)

## Profiles

| File | Profile id | Use |
| --- | --- | --- |
| `self-hosted.split-services.development.env` | `self-hosted.split-services.development` | Default dev (`pnpm im:dev`) |
| `self-hosted.unified-process.development.env` | `self-hosted.unified-process.development` | CI smoke (`pnpm im:dev:unified`) |
| `self-hosted.split-services.production.env` | `self-hosted.split-services.production` | Self-hosted production |
| `cloud-hosted.split-services.production.env` | `cloud-hosted.split-services.production` | Cloud production |

## Default development binds

| Surface | Env key | Value |
| --- | --- | --- |
| Application ingress | `SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND` | `127.0.0.1:18079` |
| Application HTTP | `SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL` | `http://127.0.0.1:18079` |
| Platform gateway | `SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL` | `http://127.0.0.1:3900` |

Load order: `scripts/im-dev.mjs` and `scripts/im-server-dev.mjs` merge the selected profile before spawning services.

## Verification

```bash
node ../sdkwork-app-topology/scripts/sdkwork-topology.mjs validate --root ../.. --spec specs/topology.spec.json
pnpm test:topology-baggage
```
