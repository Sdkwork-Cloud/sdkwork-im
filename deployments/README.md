# Deployments

## Purpose

Deployment descriptors, environment topology, packaging handoff files, infrastructure examples, and
deployment runbooks for Craw Chat.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Docker, Kubernetes, systemd, nginx, release handoff, and topology documentation.
- Deployment examples and non-secret environment templates.
- Runbooks for SaaS, private, local, and packaged deployment modes.

## Forbidden Content

- Live secrets, private keys, local override files, runtime databases, mutable service state, logs,
  caches, or user-private config.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DEPLOYMENT_SPEC.md`
- `../sdkwork-specs/RELEASE_SPEC.md`
- `../sdkwork-specs/RUNTIME_DIRECTORY_SPEC.md`

## Verification

Run deployment-specific checks and `pnpm run test:sdkwork-workspace-structure-standard` after root
layout changes.
