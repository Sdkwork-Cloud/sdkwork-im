# local-minimal Windows start stderr capture behavior review cycle

## 1. Review scope

- [deployment_profile_test.rs](D:/javasource/spring-ai-plus/spring-ai-plus-business/apps/craw-chat/services/local-minimal-node/tests/deployment_profile_test.rs)
- [start-local.ps1](D:/javasource/spring-ai-plus/spring-ai-plus-business/apps/craw-chat/bin/start-local.ps1)
- [restart-local.sh](D:/javasource/spring-ai-plus/spring-ai-plus-business/apps/craw-chat/bin/restart-local.sh)

## 2. Findings

### 2.1 Missing deterministic stderr behavior regression

The Windows lifecycle start path already redirected both stdout and stderr, but only stdout had a behavior-level regression.

Risk:

- A later edit could preserve `-RedirectStandardError` text while breaking the actual stderr file contract.
- Operators would lose the documented error log path during early-start failures.

### 2.2 Initial stderr fixture choice was not stable enough

The first stderr probe used a copied system executable (`findstr.exe`).

Observed result:

- The probe looked suitable when invoked directly.
- Under the hidden/background start path, the produced log output was not stable enough for a deterministic regression.

Root cause:

- System console tools are not a strong fixture for this contract under rename/background execution.
- The regression needs a purpose-built executable that writes to stderr through the standard handle in a predictable way.

### 2.3 Current Windows runner cannot execute Bash-based behavior tests reliably

Observed evidence:

- `C:\Windows\System32\bash.exe` resolves to the Microsoft Bash launcher and is unusable here.
- Git Bash binaries fail with Win32 error 5 during startup.

Impact:

- Linux `restart-local.sh` behavior cannot be executed on this runner right now.
- The test must be environment-aware instead of producing false negatives.

## 3. Changes made

1. Added `resolve_usable_bash()` so Linux shell behavior regressions only execute when a real Bash runtime is available.
2. Added `test_restart_local_sh_propagates_stop_failure_before_starting_new_instance` as an environment-aware behavior regression.
3. Added `test_start_local_ps1_captures_background_process_stderr_into_documented_log_file`.
4. Replaced the unstable stderr fixture with a tiny Rust probe executable compiled on demand with `rustc`.

## 4. Verification evidence

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_captures_background_process_stderr_into_documented_log_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_start_local_ps1_captures_background_process_stdout_into_documented_log_file -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`

Observed result:

- stderr capture regression passed
- stdout capture regression passed
- deployment asset contract passed

## 5. Current stage

The local-minimal operator hardening work is now stronger in two ways:

- Windows startup log capture is behavior-covered for both stdout and stderr.
- Linux restart stop-failure propagation is now represented by a behavior test that will execute automatically once a usable Bash runtime exists.

## 6. Next actions

1. Run the full `local-minimal-node` package test suite after formatting.
2. Review `restart-local.ps1`, `status-local.ps1`, and `stop-local.ps1` for the next operator-facing behavior gap.
3. Add a shell-capable CI or workstation lane so Linux lifecycle regressions stop relying on environment-aware skip behavior.
