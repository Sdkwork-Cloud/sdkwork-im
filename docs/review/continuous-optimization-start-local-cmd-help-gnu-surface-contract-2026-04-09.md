# Continuous Optimization: Start Local CMD Help GNU-Surface Contract

## Context

- Current loop: continue Step 12 Windows operator seam tightening without claiming repo-wide closure.
- Surface: `bin/start-local.cmd`
- Contract: the Windows `.cmd` entrypoint must expose the same GNU-style named flags that operators are already taught to use.

## Gap

- `docs/部署/快速启动脚本.md` already documented `start-local` with `--release`, `--foreground`, and `--bind-addr`.
- `bin/start-local.cmd --help` only showed the PowerShell usage line from `bin/start-local.ps1`.
- That left a discoverability split:
  - docs taught GNU-style Windows flags
  - local `--help` did not confirm the `.cmd` contract

## Decision

- Keep the launcher flow unchanged for this loop.
- Patch only the `-Help` branch in `bin/start-local.ps1`.
- Add one explicit Windows `.cmd` usage line that surfaces `--release`, `--foreground`, and `--bind-addr`.

## Changed Files

- `bin/start-local.ps1`
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `docs/部署/快速启动脚本.md`
- `docs/review/continuous-optimization-start-local-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/step/continuous-optimization-start-local-cmd-help-gnu-surface-contract-2026-04-09.md`
- `docs/架构/09BF-start-local-cmd-help-gnu-surface-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BF-start-local-cmd-help-gnu-surface-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p craw-chat-cli --offline --test chat_cli_e2e_test test_start_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
```

- Failed before the patch because `start-local.cmd --help` only emitted:
  - `[-Release]`
  - `[-Foreground]`
  - `[-BindAddress <host:port>]`

Green:

```powershell
cargo test -p craw-chat-cli --offline --test chat_cli_e2e_test test_start_local_cmd_help_surfaces_gnu_style_named_flags -- --exact --nocapture
cmd /c .\bin\start-local.cmd --help
cargo test -p craw-chat-cli --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- This loop only closes help discoverability for `start-local.cmd`.
- Other Windows wrappers that still depend on `_cmd-forward-powershell.cmd` must be audited individually instead of assuming uniform behavior.
