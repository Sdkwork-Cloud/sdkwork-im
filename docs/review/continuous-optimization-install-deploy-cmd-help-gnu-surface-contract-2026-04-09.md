# Continuous Optimization: Install/Deploy CMD Help GNU-Surface Contract

## Context

- Current loop: continue Step 12 Windows operator seam tightening after the `start/status` and `user-module` fixes.
- Surfaces: `bin/install-local.cmd`, `bin/deploy-local.cmd`
- Contract: Windows `.cmd` entrypoints must expose the same GNU-style named flags that docs and operators already use.

## Confirmed Gap

- `cmd /c .\bin\install-local.cmd --help` only showed:
  - `Usage: powershell -ExecutionPolicy Bypass -File bin/install-local.ps1 [-Release] [-BindAddress <host:port>]`
- `cmd /c .\bin\deploy-local.cmd --help` only showed:
  - `Usage: powershell -ExecutionPolicy Bypass -File bin/deploy-local.ps1 [-ProfileName <local-minimal|local-default>] [-SkipSmoke] [-SmokeBaseUrl <url>]`
- That left the same drift already closed for `start-local.cmd` and `status-local.cmd`:
  - docs taught `--release`, `--bind-addr`, `--profile`, `--skip-smoke`, `--smoke-base-url`
  - local Windows help still surfaced only PowerShell syntax

## Decision

- Keep `_cmd-forward-powershell.cmd` and runtime execution flow unchanged.
- Patch only the `-Help` branch in `bin/install-local.ps1` and `bin/deploy-local.ps1`.
- Freeze the visible Windows help contract with two regression tests in `deployment_profile_test.rs`.

## Changed Files

- `bin/install-local.ps1`
- `bin/deploy-local.ps1`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
- `docs/review/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-install-deploy-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BI-install-deploy-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BI-install-deploy-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline cmd_help_surfaces_gnu_style_named_flags -- --nocapture
```

- Failed before the patch because both tests only saw PowerShell usage lines.

Green:

```powershell
cargo test -p local-minimal-node --offline cmd_help_surfaces_gnu_style_named_flags -- --nocapture
cmd /c .\bin\install-local.cmd --help
cmd /c .\bin\deploy-local.cmd --help
cargo test -p local-minimal-node --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop closes Windows help discoverability for `install-local.cmd` and `deploy-local.cmd`.
- The next higher-value gap is no longer operator help parity; it is deeper runtime/provider maturity:
  - invalid `user-module-external` bootstrap semantics and health reporting
  - real RTC / object-storage / IoT provider runtime depth beyond current contract baseline
