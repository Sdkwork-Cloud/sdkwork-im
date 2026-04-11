# Step 12: User-Module Provider Health HTTP Surface

## Goal

- Expose user-module provider health directly to operators.

## Loop

1. Reproduce missing route with `GET /api/v1/user-module/provider-health`.
2. Freeze local healthy and external-unavailable contracts in HTTP tests.
3. Add route, handler, and `AppState` accessor.
4. Surface minimal user-module provider details in `ProviderHealthSnapshot`.
5. Add env-test serialization to remove parallel leakage.
6. Re-run format and package verification.

## Exit

- local provider health returns `200` + `user-module-local`
- unavailable external provider health returns `200` + `status=unavailable`
- env-driven user-module tests are stable inside the same test binary
