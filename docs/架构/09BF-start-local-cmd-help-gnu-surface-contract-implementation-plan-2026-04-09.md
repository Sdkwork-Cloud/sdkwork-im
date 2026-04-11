# 09BF Start Local CMD Help GNU-Surface Contract Implementation Plan

## Goal

Make `bin/start-local.cmd --help` surface the GNU-style Windows named flags that operators are expected to use.

## Implementation

1. Freeze the Windows help surface in `tools/chat-cli/tests/chat_cli_e2e_test.rs`
2. Patch the `-Help` branch in `bin/start-local.ps1`
3. Keep runtime startup behavior unchanged for this loop
4. Backwrite the operator quick-start doc and loop indexes

## Non-Goals

- No launcher refactor
- No runtime parameter-binding redesign
- No `_cmd-forward-powershell.cmd` redesign
- No unrelated local script changes
