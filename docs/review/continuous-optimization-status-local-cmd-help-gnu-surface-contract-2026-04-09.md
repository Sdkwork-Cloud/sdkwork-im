# Continuous Optimization: Status Local CMD Help GNU-Surface Contract

## Context

- Current loop: continue Step 12 Windows operator seam tightening without claiming repo-wide closure.
- Surface: `bin/status-local.cmd`
- Contract: the Windows `.cmd` help surface must expose the GNU-style named flags that operators use in local runtime inspection flows.

## Gap

- `docs/部署/快速启动脚本.md` already documented `status-local` with `--profile` and `--runtime-dir`.
- `bin/status-local.cmd --help` only showed the PowerShell usage line from `bin/status-local.ps1`.
- That left a local discoverability split for Windows operators.

## Decision

- Keep the runtime status logic unchanged for this loop.
- Patch only the `-Help` branch in `bin/status-local.ps1`.
- Add one explicit Windows `.cmd` usage line for `--profile` and `--runtime-dir`.

## Changed Files

- `bin/status-local.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/快速启动脚本.md`
- `docs/review/continuous-optimization-status-local-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-status-local-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BG-status-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BG-status-local-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p craw-chat-cli --offline --test chat_cli_e2e_test test_status_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `status-local.cmd --help` only emitted the PowerShell usage line.

Green:

```powershell
cargo test -p craw-chat-cli --offline --test chat_cli_e2e_test test_status_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cmd /c .\bin\status-local.cmd --help
cargo test -p craw-chat-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes help discoverability for `status-local.cmd`.
- Other Windows wrappers such as `install-local.cmd` and `deploy-local.cmd` still need independent audits.
