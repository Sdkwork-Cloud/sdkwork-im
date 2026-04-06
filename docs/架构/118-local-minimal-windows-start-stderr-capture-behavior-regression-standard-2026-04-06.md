# 118. local-minimal Windows start stderr capture behavior regression standard

## 1. Standard

For `bin/start-local.ps1`, the documented operator contract is:

- child stdout must land in `.runtime/local-minimal/logs/local-minimal-node.out.log`
- child stderr must land in `.runtime/local-minimal/logs/local-minimal-node.err.log`
- both log paths printed by the script must match the files that actually receive output

This is a behavior contract, not only a text contract.

## 2. Required regression coverage

The repository must keep two Windows behavior regressions:

1. a stdout capture regression
2. a stderr capture regression

Both regressions must:

- create an isolated temporary workspace
- reuse the real lifecycle script
- avoid rebuilding the real service binary
- assert the documented log file paths receive the expected stream content

## 3. Fixture standard

Do not depend on ad hoc system console programs as stderr fixtures when a deterministic executable can be generated instead.

Preferred fixture rule:

- compile a tiny purpose-built executable for the test case
- make it write one fixed line to the target stream
- make it exit quickly so the lifecycle failure path stays covered

Reason:

- system console tools can vary under rename, hidden-window execution, or background process launch
- deterministic probes reduce false failures and false confidence

## 4. Cross-platform note

Linux lifecycle behavior tests should run when a usable Bash runtime exists.

If a Windows runner resolves `bash` only to unusable WSL/MSYS launchers, Linux shell behavior regressions may be marked as environment-aware skip on that runner. This does not remove the requirement to execute them on at least one shell-capable environment.

## 5. Operational reason

During startup failures, stderr is often the first and only useful diagnosis channel. Losing the stderr log breaks:

- operator troubleshooting
- support handoff
- deployment automation diagnostics
- restart incident analysis

Therefore stderr capture must remain regression-guarded at the same level as stdout capture.
