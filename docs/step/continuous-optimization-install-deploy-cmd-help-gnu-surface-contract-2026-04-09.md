# Continuous Optimization: Install/Deploy CMD Help GNU-Surface Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- `install-local` and `deploy-local` are still part of the documented Windows operator path.
- Their `.cmd` launchers already accepted GNU-style switches through the forwarder.
- The remaining real gap was discoverability: local `--help` still exposed only PowerShell syntax.

## Closure Target

1. Add Windows regressions for `bin/install-local.cmd --help` and `bin/deploy-local.cmd --help`.
2. Reproduce the real help-surface drift first.
3. Patch only the help text surface.
4. Backwrite review, step, architecture, and indexes for this loop.

## Actual Delivery

- Added `test_install_local_cmd_help_surfaces_gnu_style_named_flags`
- Added `test_deploy_local_cmd_help_surfaces_gnu_style_named_flags`
- Reproduced the missing `.cmd` help lines before patching
- Added GNU-style `.cmd` usage lines to the `-Help` branches in `bin/install-local.ps1` and `bin/deploy-local.ps1`
- Backwrote review and architecture docs for this loop

## Verification

```powershell
cargo test -p local-minimal-node --offline cmd_help_surfaces_gnu_style_named_flags -- --nocapture
cmd /c .\bin\install-local.cmd --help
cmd /c .\bin\deploy-local.cmd --help
cargo test -p local-minimal-node --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Continue from the now smaller backlog:
  - validate failure behavior for invalid `principal-profile-external-catalog` runtime selection
  - deepen real provider/plugin adapters instead of only tightening operator seams
