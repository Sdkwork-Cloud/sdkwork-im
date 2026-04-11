# start-local.ps1 Health Timeout Window Recalibration Design

## Problem

- The regression test intentionally shortens the PowerShell health wait to keep execution fast.
- A `5 x 100ms` launch budget was still below the observed Windows cold-start cost for the redirected probe process, so the test could kill the child before it wrote its marker.

## Decision

- Keep the synthetic timeout model, but widen the copied PowerShell readiness loop to `20 x 100ms`.
- Keep `wait_for_path` and the existing cleanup assertions unchanged.
- Leave the production lifecycle script unchanged.

## Rationale

- The test should measure rollback semantics, not scheduler noise.
- A 2-second synthetic startup budget remains far below the real 30-second production wait while covering current Windows process-launch latency.

## Non-Goals

- No production readiness-timeout changes.
- No Bash test redesign in this pass.
