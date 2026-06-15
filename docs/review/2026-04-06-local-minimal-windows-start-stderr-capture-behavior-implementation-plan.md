# local-minimal Windows start stderr capture behavior implementation plan

## 1. Objective

Close the remaining behavior-regression gap for Windows local lifecycle startup:

- `bin/start-local.ps1` already redirects both stdout and stderr.
- The repository already had a behavior test for stdout capture.
- The repository still lacked a deterministic behavior test for stderr capture.

This wave adds the missing stderr regression and records the current Linux shell verification limitation on this Windows runner.

## 2. Problem statement

Current operator quality depends on the documented log paths being trustworthy:

- `local-minimal-node.out.log` must receive child stdout.
- `local-minimal-node.err.log` must receive child stderr.

Without a behavior-level stderr regression, a future script change could silently break error log capture while text-contract assertions still pass.

## 3. Constraints

- Work inside `apps/sdkwork-im`.
- Keep production lifecycle behavior unchanged unless a real defect is proven.
- Prefer deterministic test fixtures over environment-sensitive system executables.
- Current Windows host does not provide a usable POSIX shell runtime for executing `bash`-based behavior tests:
  - `C:\Windows\System32\bash.exe` routes to WSL launcher and is not usable here.
  - Git Bash binaries currently fail with Win32 error 5 on process startup.

## 4. Planned implementation

1. Add an environment-aware Linux `restart-local.sh` behavior regression:
   - When no usable Bash runtime exists, skip instead of reporting a false failure.
   - Keep the text contract in place.
2. Add a Windows-only stderr behavior regression for `start-local.ps1`:
   - Create a temporary workspace.
   - Copy `start-local.ps1`, `install-local.ps1`, and `init-config-local.ps1`.
   - Stub `cargo.cmd` so the lifecycle script does not rebuild.
   - Compile a tiny probe executable with `rustc` that writes a fixed line to stderr and exits non-zero.
   - Run `start-local.ps1`.
   - Assert stdout log stays empty and stderr log contains the fixed stderr line.
3. Re-run the existing stdout behavior regression to confirm both channels remain covered.
4. Record the result in `/docs/review/` and `/docs/架构/`.

## 5. Verification plan

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_captures_background_process_stderr_into_documented_log_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_captures_background_process_stdout_into_documented_log_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

## 6. Exit criteria

- Windows stderr capture has a deterministic behavior regression.
- Windows stdout behavior regression still passes.
- Linux restart behavior regression remains present but self-identifies as skipped when no usable Bash runtime exists.
- Docs reflect both the operator standard and the current runner limitation.
