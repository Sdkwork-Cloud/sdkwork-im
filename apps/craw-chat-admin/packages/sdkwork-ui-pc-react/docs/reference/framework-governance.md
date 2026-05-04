# Framework Governance

SDKWORK UI PC React is governed as a framework rather than a component bucket.

The package keeps these contracts executable:

- package export maps are derived from `build/package-contract.ts`
- public barrels stay as star re-export entrypoints
- runtime imports in built files stay inside declared dependencies and peer dependencies
- docs import only published package subpaths and real exported symbols
- reusable components expose stable semantic props, display names, and framework metadata where required by tests

Run:

```bash
pnpm test
pnpm typecheck
pnpm build
```
