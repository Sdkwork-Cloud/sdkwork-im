# 150BF Start Local CMD Help GNU-Surface Contract Design

## Problem

The Windows `start-local.cmd` wrapper is a documented operator entrypoint, but its local `--help` output only surfaced PowerShell-style arguments.
That created a discoverability mismatch:

- docs taught `--release`, `--foreground`, and `--bind-addr`
- local Windows help only showed `-Release`, `-Foreground`, and `-BindAddress`

## Decision

- Keep the current startup flow unchanged.
- Define GNU-style help discoverability as part of the visible Windows `.cmd` operator contract.
- Add one explicit `.cmd` usage line to `bin/start-local.ps1` help output.

## Rationale

- This is the minimum change that makes the published Windows operator surface truthful.
- It keeps the contract near the script that owns help generation.
- It avoids a broader wrapper redesign while still closing the real user-facing gap.

## Boundary

- This design only covers `start-local.cmd --help` discoverability on Windows.
- It does not redefine the generic strategy for unrelated wrappers.
