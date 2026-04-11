# 150BI Install/Deploy CMD Help GNU-Surface Contract Design

## Problem

The Windows `install-local.cmd` and `deploy-local.cmd` wrappers are documented operator entrypoints, but their local `--help` output only surfaced PowerShell-style arguments.
That created a discoverability mismatch:

- docs taught GNU-style Windows flags
- local Windows help only showed PowerShell syntax

## Decision

- Keep the current forwarding flow unchanged.
- Define GNU-style help discoverability as part of the visible Windows `.cmd` operator contract.
- Add one explicit `.cmd` usage line to each help branch in `bin/install-local.ps1` and `bin/deploy-local.ps1`.

## Rationale

- This is the minimum change that makes the published Windows operator surface truthful.
- It keeps help generation beside the scripts that own the contract.
- It closes the real user-facing gap without introducing another wrapper layer or a generic redesign.

## Boundary

- This design only covers `install-local.cmd --help` and `deploy-local.cmd --help` discoverability on Windows.
- It does not redefine the global help strategy for every remaining wrapper.
