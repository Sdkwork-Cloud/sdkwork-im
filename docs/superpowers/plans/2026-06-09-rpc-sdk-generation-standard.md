# RPC SDK Generation Standard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add SDKWork root standards for RPC SDK families and extend `sdkwork-sdk-generator` so RPC SDKs can be generated automatically from proto contracts and RPC manifests.

**Architecture:** Keep `RPC_SPEC.md` as the language-neutral contract standard, add a dedicated `RPC_SDK_WORKSPACE_SPEC.md` for proto/RPC SDK workspace layout and generation rules, and update adjacent specs so RPC SDKs become first-class SDKWork SDK families. Extend `@sdkwork/sdk-generator` with a protocol dimension: HTTP generation continues to use OpenAPI, while RPC generation loads proto plus a SDKWork RPC manifest, delegates protobuf compilation to Buf/protoc-compatible generation, and writes SDKWork package scaffolds, wrappers, manifests, README files, and verification reports.

**Tech Stack:** SDKWork root specs, TypeScript/Node 18, Commander CLI, Vitest, Buf/protoc ecosystem, existing `@sdkwork/sdk-generator` output-sync/control-plane patterns.

---

## Non-Regression Requirement

The new RPC/gRPC SDK generation capability must not weaken, rename, replace, or change the default behavior of existing OpenAPI/HTTP SDK generation.

Hard requirements:

- Existing `sdkgen generate` commands without `--protocol` continue to behave exactly as HTTP/OpenAPI generation.
- Existing HTTP CLI flags, generated output paths, `sdkwork-sdk.json`, `.sdkwork/sdkwork-generator-*.json`, `custom/`, package scaffolds, README behavior, version resolution, output sync, and dry-run fingerprints remain compatible.
- RPC generation uses an isolated branch selected by `--protocol rpc`; HTTP generation must not load proto manifests, require `--proto-root`, require Buf/protoc, or emit RPC control-plane files.
- All existing generator tests must pass before claiming the RPC addition is complete.
- Add focused non-regression tests that run a representative legacy HTTP generation command and assert the generated file set and control-plane files are unchanged.
- The first RPC implementation must be additive. If a shared type or runner must change, tests must prove the HTTP path sees the same config defaults and output as before.

## Scope And Boundaries

This is a multi-repository standards and tooling change:

- Standards root: `../sdkwork-specs`
- Generator root: `../sdkwork-sdk-generator`
- Plan/documentation root: `E:/sdkwork-space/craw-chat`

This plan does not implement Craw Chat IM RPC services. It creates the standards and generator capability needed for applications such as Craw Chat to define and generate RPC SDKs later.

Do not hand-edit generated SDK output. Do not replace existing HTTP OpenAPI SDK generation. Do not implement a custom protobuf compiler in `sdkwork-sdk-generator`; use standard Buf/protoc tooling as the contract compiler and let `sdkgen` own SDKWork layout, metadata, wrapper, and verification semantics.

## File Structure

### Standards Files

- Create: `../sdkwork-specs/RPC_SDK_WORKSPACE_SPEC.md`  
  Owns proto workspace layout, RPC SDK family naming, RPC manifest shape, generation workflow, generated-output boundaries, and verification.

- Modify: `../sdkwork-specs/README.md`  
  Adds the new standard to the foundation contracts and task matrix.

- Modify: `../sdkwork-specs/RPC_SPEC.md`  
  Clarifies that `.proto` remains the RPC source of truth and points physical SDK workspace/generation rules to `RPC_SDK_WORKSPACE_SPEC.md`.

- Modify: `../sdkwork-specs/SDK_SPEC.md`  
  Makes RPC SDKs first-class SDK families generated from proto plus RPC manifest, and requires no raw fallback.

- Modify: `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`  
  Keeps OpenAPI HTTP SDK workspace authority, and delegates RPC physical layout to `RPC_SDK_WORKSPACE_SPEC.md`.

- Modify: `../sdkwork-specs/RUST_RPC_SPEC.md`  
  Aligns Rust proto/server/client crate generation with the new RPC SDK workspace standard.

- Modify: `../sdkwork-specs/SECURITY_SPEC.md`  
  Adds RPC SDK metadata auth generation and mTLS/reflection access-control requirements.

- Modify: `../sdkwork-specs/OBSERVABILITY_SPEC.md`  
  Adds generated RPC SDK trace/deadline/operation metadata expectations.

- Modify: `../sdkwork-specs/TEST_SPEC.md`  
  Adds executable checks for RPC SDK workspace, proto generation, manifest coverage, and generated client compilation.

- Modify: `../sdkwork-specs/QUALITY_GATE_SPEC.md`  
  Adds gate expectations for public RPC SDK and streaming changes.

- Modify: `../sdkwork-specs/MIGRATION_SPEC.md`  
  Adds RPC SDK version and client migration requirements.

- Modify: `../sdkwork-specs/DOCUMENTATION_SPEC.md`  
  Adds required RPC SDK README content.

### Generator Files

- Modify: `../sdkwork-sdk-generator/src/framework/types.ts`  
  Adds protocol-aware config types: `SdkProtocol`, `RpcGeneratorConfig`, `RpcManifest`, and input metadata.

- Modify: `../sdkwork-sdk-generator/src/cli.ts`  
  Adds CLI flags for RPC generation without breaking existing HTTP generation.

- Modify: `../sdkwork-sdk-generator/src/cli-runner.ts`  
  Splits command execution into HTTP and RPC paths while keeping report/output-sync behavior.

- Modify: `../sdkwork-sdk-generator/src/index.ts`  
  Exports RPC generation APIs and keeps existing HTTP exports stable.

- Create: `../sdkwork-sdk-generator/src/rpc/manifest.ts`  
  Loads, validates, and normalizes `sdkwork.rpc.manifest.json`.

- Create: `../sdkwork-sdk-generator/src/rpc/proto-input.ts`  
  Resolves proto roots, proto files, import roots, and package list.

- Create: `../sdkwork-sdk-generator/src/rpc/buf-config.ts`  
  Generates or validates Buf generation config inputs used by SDKWork RPC SDK generation.

- Create: `../sdkwork-sdk-generator/src/rpc/rpc-generation-runner.ts`  
  Orchestrates RPC generation for one language and returns `GeneratorResult`.

- Create: `../sdkwork-sdk-generator/src/rpc/rpc-readme-generator.ts`  
  Writes SDKWork RPC SDK README examples for endpoint, TLS/mTLS, metadata auth, deadlines, idempotency, and unary calls.

- Create: `../sdkwork-sdk-generator/src/rpc/rpc-package-scaffold.ts`  
  Writes package-level scaffold files outside generated protobuf output.

- Create: `../sdkwork-sdk-generator/src/rpc/rpc-control-plane.ts`  
  Writes SDKWork RPC generation manifest/report files outside generated proto-owned output.

- Create: `../sdkwork-sdk-generator/src/rpc/language-registry.ts`  
  Defines supported RPC language targets and their Buf/protoc plugin strategy.

- Create: `../sdkwork-sdk-generator/src/rpc/verification.ts`  
  Produces verification plan entries for proto lint, breaking checks, generated client compile, and manifest coverage.

- Create: `../sdkwork-sdk-generator/src/rpc/*.test.ts`  
  Focused Vitest tests for manifest validation, CLI routing, dry-run output, and generated scaffold paths.

- Modify: `../sdkwork-sdk-generator/src/cli-output.ts` and `../sdkwork-sdk-generator/src/execution-report.ts` if needed  
  Adds protocol/RPC metadata to success output and reports while preserving HTTP output.

- Modify: `../sdkwork-sdk-generator/README.md`  
  Documents RPC SDK generation commands and limitations.

## CLI Design

Keep existing HTTP command compatible:

```powershell
node bin/sdkgen.js generate `
  --input .\openapi\sdkwork-im-open-api.sdkgen.yaml `
  --output .\sdks\sdkwork-im-sdk\sdkwork-im-sdk-typescript\generated\server-openapi `
  --name SdkworkIm `
  --language typescript `
  --standard-profile sdkwork-v3
```

Add RPC support through the same `generate` command with an explicit protocol:

```powershell
node bin/sdkgen.js generate `
  --protocol rpc `
  --input .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-typescript `
  --name SdkworkImRpc `
  --sdk-name sdkwork-im-rpc-sdk `
  --language typescript `
  --package-name @sdkwork/im-rpc-sdk
```

Add a dedicated alias only if CLI clarity requires it:

```powershell
node bin/sdkgen.js generate-rpc `
  --manifest .\sdks\sdkwork-im-rpc-sdk\rpc\sdkwork-im-rpc.manifest.json `
  --proto-root .\proto `
  --output .\sdks\sdkwork-im-rpc-sdk\sdkwork-im-rpc-sdk-rust `
  --name SdkworkImRpc `
  --language rust
```

Recommended first implementation: support `generate --protocol rpc`; add `generate-rpc` later only if users find the command too ambiguous.

---

### Task 1: Write RPC SDK Workspace Standard

**Files:**
- Create: `../sdkwork-specs/RPC_SDK_WORKSPACE_SPEC.md`
- Modify: `../sdkwork-specs/README.md`
- Test: Manual standards checklist in `../sdkwork-specs/README.md`

- [ ] **Step 1: Draft the new standard**

Create `RPC_SDK_WORKSPACE_SPEC.md` with these sections:

```md
# RPC SDK Workspace And Proto Generation Detail Standard

- Version: 1.0
- Scope: proto contract workspace layout, RPC SDK family naming, RPC manifest shape, generated RPC SDK output, multi-language generation, SDKWork RPC generation verification
- Related: RPC_SPEC.md, SDK_SPEC.md, RUST_RPC_SPEC.md, SECURITY_SPEC.md, OBSERVABILITY_SPEC.md, TEST_SPEC.md

## 1. Authority
## 2. Standard Workspace Shape
## 3. Proto Contract Packages
## 4. RPC SDK Family Naming
## 5. RPC Manifest
## 6. Generation Workflow
## 7. Generated Output Boundaries
## 8. Language Baseline
## 9. Application Integration
## 10. Verification
## 11. Acceptance Checklist
```

- [ ] **Step 2: Include the canonical workspace shape**

Add this normative shape:

```text
<application-root>/
  proto/
    sdkwork/common/v1/
    sdkwork/<domain>/app/v3/
    sdkwork/<domain>/backend/v3/
    sdkwork/<domain>/internal/v1/
  sdks/
    sdkwork-<domain>-rpc-sdk/
      README.md
      .sdkwork-assembly.json
      rpc/
        sdkwork-<domain>-rpc.manifest.json
      proto/
      generated/
      sdkwork-<domain>-rpc-sdk-<language>/
      specs/
        README.md
        component.spec.json
```

- [ ] **Step 3: Define RPC SDK family naming**

Add rules:

```text
sdkwork-<domain>-rpc-sdk
sdkwork-<domain>-rpc-sdk-typescript
sdkwork-<domain>-rpc-sdk-rust
sdkwork-<domain>-rpc-sdk-go
```

Permit approved aliases such as `sdkwork-im-rpc-sdk` only when the manifest links the alias to canonical `domain=communication`.

- [ ] **Step 4: Define generation workflow**

Add the standard flow:

```text
proto contracts
  -> RPC manifest
  -> proto lint / breaking check
  -> sdkgen --protocol rpc
  -> Buf/protoc language generation
  -> SDKWork package scaffold and wrappers
  -> SDKWork RPC generation reports
  -> generated client compile and smoke tests
```

- [ ] **Step 5: Update `README.md` index**

In `../sdkwork-specs/README.md`, add `RPC_SDK_WORKSPACE_SPEC.md` near `RPC_SPEC.md` and `SDK_SPEC.md`, and state that RPC SDK generation starts from `RPC_SPEC.md` then `RPC_SDK_WORKSPACE_SPEC.md`.

- [ ] **Step 6: Verify**

Run:

```powershell
rg -n "RPC_SDK_WORKSPACE_SPEC|RPC SDK" ..\sdkwork-specs\README.md ..\sdkwork-specs\RPC_SDK_WORKSPACE_SPEC.md
```

Expected: new standard is indexed and discoverable.

- [ ] **Step 7: Commit**

```powershell
git -C ..\sdkwork-specs add README.md RPC_SDK_WORKSPACE_SPEC.md
git -C ..\sdkwork-specs commit -m "docs: add rpc sdk workspace standard"
```

---

### Task 2: Cross-Link Existing Standards

**Files:**
- Modify: `../sdkwork-specs/RPC_SPEC.md`
- Modify: `../sdkwork-specs/SDK_SPEC.md`
- Modify: `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- Modify: `../sdkwork-specs/RUST_RPC_SPEC.md`

- [ ] **Step 1: Update `RPC_SPEC.md`**

Add a section after Source Of Truth:

```md
## RPC SDK Generation

RPC SDK package layout, RPC SDK family naming, proto generation workspace, language output placement, SDKWork RPC generation manifests, and generator verification are governed by `RPC_SDK_WORKSPACE_SPEC.md`.

Rules:

- RPC SDKs MUST be generated from proto contracts and the SDKWork RPC manifest.
- RPC SDK generation MUST NOT hand-edit generated protobuf output.
- Missing RPC client capability MUST be fixed by updating proto contracts and the RPC manifest, then regenerating.
```

- [ ] **Step 2: Update `SDK_SPEC.md`**

In the SDK source-of-truth section, add:

```md
- RPC SDKs are first-class SDKWork SDK families. They are generated from proto contracts governed by `RPC_SPEC.md` and workspace rules governed by `RPC_SDK_WORKSPACE_SPEC.md`.
- RPC SDK generation MAY be orchestrated by `@sdkwork/sdk-generator` / `sdkgen`, but protobuf compilation MUST use standard Buf/protoc-compatible tooling.
- RPC SDK consumers MUST NOT add raw HTTP, raw gRPC stubs, manual metadata auth, local DTO forks, or generated-output edits to bypass missing RPC SDK methods.
```

- [ ] **Step 3: Update `SDK_WORKSPACE_GENERATION_SPEC.md`**

Add a clear boundary note:

```md
This file governs OpenAPI HTTP SDK workspace layout. RPC SDK physical layout and proto generation are governed by `RPC_SDK_WORKSPACE_SPEC.md`.
```

- [ ] **Step 4: Update `RUST_RPC_SPEC.md`**

Add or tighten:

```md
Rust RPC proto crates generated through `sdkgen --protocol rpc` MUST keep generated protobuf output isolated from handwritten server adapters. The proto crate owns generated types and clients; the RPC server crate owns context mapping, service binding, and error mapping only.
```

- [ ] **Step 5: Verify cross-links**

Run:

```powershell
rg -n "RPC_SDK_WORKSPACE_SPEC|sdkgen --protocol rpc|first-class SDKWork SDK families" ..\sdkwork-specs
```

Expected: links appear only in relevant standards and do not duplicate large rule bodies.

- [ ] **Step 6: Commit**

```powershell
git -C ..\sdkwork-specs add RPC_SPEC.md SDK_SPEC.md SDK_WORKSPACE_GENERATION_SPEC.md RUST_RPC_SPEC.md
git -C ..\sdkwork-specs commit -m "docs: align rpc sdk generation standards"
```

---

### Task 3: Update Security, Observability, Test, Quality, Migration, And Documentation Specs

**Files:**
- Modify: `../sdkwork-specs/SECURITY_SPEC.md`
- Modify: `../sdkwork-specs/OBSERVABILITY_SPEC.md`
- Modify: `../sdkwork-specs/TEST_SPEC.md`
- Modify: `../sdkwork-specs/QUALITY_GATE_SPEC.md`
- Modify: `../sdkwork-specs/MIGRATION_SPEC.md`
- Modify: `../sdkwork-specs/DOCUMENTATION_SPEC.md`

- [ ] **Step 1: Add RPC SDK security rules**

In `SECURITY_SPEC.md`, add:

```md
- Generated RPC SDK clients MUST support SDKWork metadata providers for `authorization`, `access-token`, `traceparent`, `idempotency-key`, and `x-request-hash`.
- Application and backend code MUST inject metadata providers through SDK/bootstrap infrastructure instead of assembling raw metadata in business modules.
- RPC SDK examples for protected methods MUST show metadata provider setup, not hard-coded tokens.
```

- [ ] **Step 2: Add RPC SDK observability rules**

In `OBSERVABILITY_SPEC.md`, add:

```md
- Generated RPC SDKs SHOULD expose deadline and trace metadata options.
- RPC client wrappers SHOULD propagate `traceparent` and record package, service, method, operationId, status, deadline, and duration where the language runtime supports it.
```

- [ ] **Step 3: Add RPC SDK tests**

In `TEST_SPEC.md`, add required checks:

```text
proto lint
proto breaking check
RPC manifest coverage
sdkgen --protocol rpc dry-run
generated client compile
metadata provider example check
unary smoke test
```

- [ ] **Step 4: Add quality gate risk mapping**

In `QUALITY_GATE_SPEC.md`, add:

```md
Public RPC SDK generation changes are High risk. Streaming RPC, auth metadata changes, package naming changes, or generator-owned output boundary changes are Critical risk.
```

- [ ] **Step 5: Add migration rules**

In `MIGRATION_SPEC.md`, add:

```md
RPC SDK migrations MUST name proto package, service, method, message, field, generated language package versions, affected consumers, compatibility window, and rollback or forward-fix plan.
```

- [ ] **Step 6: Add documentation rules**

In `DOCUMENTATION_SPEC.md`, add required RPC SDK README sections:

```text
proto packages
service catalog
generated languages
endpoint and TLS/mTLS configuration
metadata auth
deadline and cancellation
idempotent write example
error/status mapping
verification commands
```

- [ ] **Step 7: Verify**

Run:

```powershell
rg -n "RPC SDK|sdkgen --protocol rpc|metadata provider|proto breaking" ..\sdkwork-specs\SECURITY_SPEC.md ..\sdkwork-specs\OBSERVABILITY_SPEC.md ..\sdkwork-specs\TEST_SPEC.md ..\sdkwork-specs\QUALITY_GATE_SPEC.md ..\sdkwork-specs\MIGRATION_SPEC.md ..\sdkwork-specs\DOCUMENTATION_SPEC.md
```

Expected: every touched lifecycle spec has concise RPC SDK rules.

- [ ] **Step 8: Commit**

```powershell
git -C ..\sdkwork-specs add SECURITY_SPEC.md OBSERVABILITY_SPEC.md TEST_SPEC.md QUALITY_GATE_SPEC.md MIGRATION_SPEC.md DOCUMENTATION_SPEC.md
git -C ..\sdkwork-specs commit -m "docs: add rpc sdk lifecycle gates"
```

---

### Task 4: Add Protocol-Aware Generator Types

**Files:**
- Modify: `../sdkwork-sdk-generator/src/framework/types.ts`
- Test: `../sdkwork-sdk-generator/src/framework/types.test.ts` if present, otherwise add tests in Task 5 and Task 6

- [ ] **Step 1: Add protocol and RPC types**

In `src/framework/types.ts`, add:

```ts
export type SdkProtocol = 'http' | 'rpc';

export interface RpcServiceManifest {
  schemaVersion: 1;
  kind: 'sdkwork.rpc.manifest';
  domain: string;
  sdkFamily: string;
  services: RpcServiceDefinition[];
}

export interface RpcServiceDefinition {
  package: string;
  service: string;
  surface: 'app' | 'backend' | 'internal' | 'common';
  methods: RpcMethodDefinition[];
}

export interface RpcMethodDefinition {
  method: string;
  operationId: string;
  auth: string;
  idempotency: 'none' | 'optional' | 'required';
  streaming: 'unary' | 'server' | 'client' | 'bidi';
  owner: string;
  compatibility: string;
}

export interface RpcInputSpec {
  manifestPath: string;
  protoRoot: string;
  protoFiles: string[];
  importRoots: string[];
  manifest: RpcServiceManifest;
}
```

- [ ] **Step 2: Extend `GeneratorConfig` safely**

Add:

```ts
protocol?: SdkProtocol;
rpc?: RpcInputSpec;
```

Keep default behavior equivalent to `protocol: 'http'`.

- [ ] **Step 3: Run typecheck**

Run:

```powershell
npm run lint
```

Expected: PASS or only pre-existing unrelated failures. If failures occur from this task, fix before continuing.

- [ ] **Step 4: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/framework/types.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: add rpc generator contract types"
```

---

### Task 5: Add RPC Manifest Loader And Validation

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/manifest.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/manifest.test.ts`

- [ ] **Step 1: Write failing tests**

Create `manifest.test.ts` with tests for:

```ts
it('accepts a valid sdkwork rpc manifest');
it('rejects a manifest without sdkwork.rpc.manifest kind');
it('rejects duplicate service method entries');
it('rejects methods without operationId');
it('rejects unsupported streaming values');
it('normalizes app/backend/internal/common surfaces');
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
npx vitest run src/rpc/manifest.test.ts
```

Expected: FAIL because `src/rpc/manifest.ts` does not exist.

- [ ] **Step 3: Implement manifest loader**

Implement:

```ts
export function loadRpcManifest(path: string): RpcServiceManifest;
export function validateRpcManifest(value: unknown): RpcServiceManifest;
export function listRpcManifestMethods(manifest: RpcServiceManifest): RpcMethodDefinition[];
```

Validation rules:

- `kind` must be `sdkwork.rpc.manifest`.
- `schemaVersion` must be `1`.
- `domain` and `sdkFamily` are required.
- `package`, `service`, `method`, and `operationId` are required.
- Service/method pairs must be unique.
- `streaming` must be one of `unary`, `server`, `client`, `bidi`.
- `idempotency` must be one of `none`, `optional`, `required`.

- [ ] **Step 4: Run tests**

Run:

```powershell
npx vitest run src/rpc/manifest.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/manifest.ts src/rpc/manifest.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: validate rpc sdk manifests"
```

---

### Task 6: Add Proto Input Resolution

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/proto-input.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/proto-input.test.ts`

- [ ] **Step 1: Write failing tests**

Test these behaviors:

```ts
it('resolves proto files from manifest package names and proto root');
it('accepts explicit proto file list');
it('rejects missing proto root');
it('rejects missing proto files');
it('keeps import roots deterministic and absolute internally');
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
npx vitest run src/rpc/proto-input.test.ts
```

Expected: FAIL because implementation is missing.

- [ ] **Step 3: Implement resolver**

Implement:

```ts
export interface ResolveRpcProtoInputOptions {
  manifestPath: string;
  protoRoot: string;
  protoFiles?: string[];
  importRoots?: string[];
}

export function resolveRpcProtoInput(options: ResolveRpcProtoInputOptions): RpcInputSpec;
```

Rules:

- Resolve all paths to absolute paths internally.
- Store user-facing report paths as relative when possible.
- Fail fast when `protoRoot` does not exist.
- Fail fast when explicit `protoFiles` are missing.
- If no explicit proto files are passed, discover `*.proto` under the packages listed by manifest services.

- [ ] **Step 4: Run tests**

Run:

```powershell
npx vitest run src/rpc/proto-input.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/proto-input.ts src/rpc/proto-input.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: resolve rpc proto inputs"
```

---

### Task 7: Add RPC Language Registry And Buf Strategy

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/language-registry.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/language-registry.test.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/buf-config.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/buf-config.test.ts`

- [ ] **Step 1: Write failing language registry tests**

Require first wave languages:

```ts
expect(getRpcLanguageTargets().map((entry) => entry.language)).toEqual([
  'typescript',
  'go',
  'java',
  'python',
  'rust',
]);
```

This keeps first implementation focused. Add Dart, Kotlin, Swift, C#, Flutter later once package scaffolds are proven.

- [ ] **Step 2: Write failing Buf config tests**

Test:

```ts
it('creates deterministic buf generation config for typescript');
it('creates deterministic buf generation config for go');
it('throws for unsupported rpc language targets');
```

- [ ] **Step 3: Implement registry**

Define:

```ts
export interface RpcLanguageTarget {
  language: Language;
  displayName: string;
  packageManager?: 'npm' | 'go' | 'maven' | 'cargo' | 'pip';
  generatedRoot: string;
  plugins: RpcGeneratorPlugin[];
}
```

- [ ] **Step 4: Implement Buf config builder**

Implement:

```ts
export function createBufGenerateConfig(options: {
  language: Language;
  outputPath: string;
  packageName?: string;
  namespace?: string;
}): string;
```

It should produce YAML text, not execute Buf yet.

- [ ] **Step 5: Run tests**

Run:

```powershell
npx vitest run src/rpc/language-registry.test.ts src/rpc/buf-config.test.ts
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/language-registry.ts src/rpc/language-registry.test.ts src/rpc/buf-config.ts src/rpc/buf-config.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: add rpc language generation registry"
```

---

### Task 8: Add RPC Package Scaffold And README Generation

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-package-scaffold.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-package-scaffold.test.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-readme-generator.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-readme-generator.test.ts`

- [ ] **Step 1: Write failing scaffold tests**

Test that TypeScript RPC SDK scaffold includes:

```text
package.json
README.md
src/index.ts
src/metadata.ts
src/deadline.ts
src/idempotency.ts
```

Test that Rust RPC SDK scaffold includes:

```text
Cargo.toml
README.md
src/lib.rs
```

- [ ] **Step 2: Write failing README tests**

Assert README includes:

```text
proto package list
service catalog
endpoint
TLS/mTLS
authorization
access-token
deadline
idempotency-key
unary call example
verification commands
```

- [ ] **Step 3: Implement scaffold generator**

Return `GeneratedFile[]` with scaffold ownership:

```ts
ownership: 'scaffold',
overwriteStrategy: 'if-missing'
```

Generated protobuf output remains in language-specific generated roots.

- [ ] **Step 4: Implement README generator**

Use manifest services and methods to render service catalog and examples.

- [ ] **Step 5: Run tests**

Run:

```powershell
npx vitest run src/rpc/rpc-package-scaffold.test.ts src/rpc/rpc-readme-generator.test.ts
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/rpc-package-scaffold.ts src/rpc/rpc-package-scaffold.test.ts src/rpc/rpc-readme-generator.ts src/rpc/rpc-readme-generator.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: scaffold rpc sdk packages"
```

---

### Task 9: Add RPC Generation Runner Dry-Run

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-generation-runner.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-generation-runner.test.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-control-plane.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/rpc-control-plane.test.ts`

- [ ] **Step 1: Write failing dry-run tests**

Use a test manifest and temporary proto files. Assert that dry-run generation returns files for:

```text
buf.gen.yaml
README.md
package scaffold
.sdkwork/sdkwork-rpc-generator-manifest.json
.sdkwork/sdkwork-rpc-generator-report.json
```

- [ ] **Step 2: Implement runner without executing Buf**

First implementation should create deterministic config and scaffolds only. It must not require Buf installed for unit tests.

Implement:

```ts
export async function generateRpcSdk(config: GeneratorConfig, rpc: RpcInputSpec): Promise<GeneratorResult>;
```

- [ ] **Step 3: Implement control-plane files**

Write files outside generated protobuf transport:

```text
.sdkwork/sdkwork-rpc-generator-manifest.json
.sdkwork/sdkwork-rpc-generator-report.json
```

Do not reuse HTTP `sdkwork-sdk.json` semantics until standard approves a shared manifest shape.

- [ ] **Step 4: Run tests**

Run:

```powershell
npx vitest run src/rpc/rpc-generation-runner.test.ts src/rpc/rpc-control-plane.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/rpc-generation-runner.ts src/rpc/rpc-generation-runner.test.ts src/rpc/rpc-control-plane.ts src/rpc/rpc-control-plane.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: generate rpc sdk scaffolds"
```

---

### Task 10: Wire CLI Protocol Routing

**Files:**
- Modify: `../sdkwork-sdk-generator/src/cli.ts`
- Modify: `../sdkwork-sdk-generator/src/cli-runner.ts`
- Modify: `../sdkwork-sdk-generator/src/cli-output.ts` if needed
- Modify: `../sdkwork-sdk-generator/src/index.ts`
- Test: `../sdkwork-sdk-generator/src/cli-runner.test.ts`
- Test: `../sdkwork-sdk-generator/src/cli-runner-rpc.test.ts`
- Test: `../sdkwork-sdk-generator/src/cli-runner-http-non-regression.test.ts`

- [ ] **Step 1: Write failing CLI tests**

Add tests for:

```ts
it('defaults generate protocol to http');
it('routes --protocol rpc to rpc generation');
it('requires --proto-root for rpc generation');
it('rejects --standard-profile sdkwork-v3 for rpc unless mapped to rpc standard');
it('keeps existing http generate behavior unchanged');
```

- [ ] **Step 2: Write HTTP non-regression tests**

Create `src/cli-runner-http-non-regression.test.ts` with a representative existing OpenAPI fixture. Assert:

```ts
it('keeps generate defaulting to http without requiring rpc options');
it('does not create rpc control-plane files for http generation');
it('keeps http sdkwork control-plane files under generated output');
it('keeps dry-run fingerprint stable for the legacy http command fixture');
```

The test must call `runGenerateCommand` without `protocol`, `protoRoot`, `protoFiles`, or `importRoots`.

- [ ] **Step 3: Add CLI options**

In `generate` command, add:

```ts
.option('--protocol <protocol>', 'SDK protocol (http or rpc)', 'http')
.option('--proto-root <path>', 'RPC proto root')
.option('--proto-file <path...>', 'RPC proto files')
.option('--import-root <path...>', 'RPC proto import roots')
```

- [ ] **Step 4: Split runner**

In `cli-runner.ts`:

```ts
if (options.protocol === 'rpc') {
  return runRpcGenerateCommand(options);
}
return runHttpGenerateCommand(options);
```

Preserve existing exported `runGenerateCommand` by making it protocol-aware.

- [ ] **Step 5: Preserve HTTP defaults explicitly**

Ensure the runner uses:

```ts
const protocol = options.protocol || 'http';
```

HTTP execution must not read RPC-only options or validate RPC inputs. RPC execution must be the only path that reads `protoRoot`, `protoFiles`, `importRoots`, or calls RPC manifest loading.

- [ ] **Step 6: Export RPC APIs**

In `index.ts`, export:

```ts
export * from './rpc/manifest.js';
export * from './rpc/proto-input.js';
export * from './rpc/rpc-generation-runner.js';
```

- [ ] **Step 7: Run tests**

Run:

```powershell
npx vitest run src/cli-runner.test.ts src/cli-runner-rpc.test.ts src/cli-runner-http-non-regression.test.ts
```

Expected: PASS.

- [ ] **Step 8: Run broad generator test**

Run:

```powershell
npm test
```

Expected: PASS.

- [ ] **Step 9: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/cli.ts src/cli-runner.ts src/cli-output.ts src/index.ts src/cli-runner.test.ts src/cli-runner-rpc.test.ts src/cli-runner-http-non-regression.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: add rpc sdk generation cli"
```

---

### Task 11: Add Real Buf Execution Behind An Explicit Option

**Files:**
- Modify: `../sdkwork-sdk-generator/src/rpc/rpc-generation-runner.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/buf-executor.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/buf-executor.test.ts`
- Modify: `../sdkwork-sdk-generator/src/cli.ts`

- [ ] **Step 1: Write failing executor tests**

Test:

```ts
it('builds a deterministic buf generate command');
it('does not execute external commands in dry-run');
it('fails clearly when buf is missing and execution is requested');
```

- [ ] **Step 2: Add execution option**

Add CLI option:

```ts
.option('--execute-proto-generator', 'Run Buf/protoc generation for RPC output')
```

Default remains scaffold-only/dry-run safe until project dependencies are pinned.

- [ ] **Step 3: Implement `buf-executor.ts`**

Use Node child process APIs with explicit argument arrays, not shell strings.

Do not pass secrets or tokens.

- [ ] **Step 4: Integrate executor**

If `executeProtoGenerator` is true and `dryRun` is false, run Buf generation after writing or staging `buf.gen.yaml`.

- [ ] **Step 5: Run tests**

Run:

```powershell
npx vitest run src/rpc/buf-executor.test.ts src/rpc/rpc-generation-runner.test.ts
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/buf-executor.ts src/rpc/buf-executor.test.ts src/rpc/rpc-generation-runner.ts src/cli.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: execute rpc proto generation"
```

---

### Task 12: Add RPC Verification Plan Support

**Files:**
- Create: `../sdkwork-sdk-generator/src/rpc/verification.ts`
- Create: `../sdkwork-sdk-generator/src/rpc/verification.test.ts`
- Modify: `../sdkwork-sdk-generator/src/verification-plan.ts`
- Modify: `../sdkwork-sdk-generator/src/verification-plan.test.ts`

- [ ] **Step 1: Write failing verification tests**

Expected RPC verification plan entries:

```text
buf lint
buf breaking
sdkgen --protocol rpc dry-run
generated client compile
manifest coverage
unary smoke test
```

- [ ] **Step 2: Implement RPC verification helper**

Implement:

```ts
export function createRpcVerificationPlan(input: RpcInputSpec, config: GeneratorConfig): VerificationPlan;
```

- [ ] **Step 3: Integrate with existing verification plan**

Keep HTTP plans unchanged. Add RPC branch only when `protocol === 'rpc'`.

- [ ] **Step 4: Run tests**

Run:

```powershell
npx vitest run src/rpc/verification.test.ts src/verification-plan.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add src/rpc/verification.ts src/rpc/verification.test.ts src/verification-plan.ts src/verification-plan.test.ts
git -C ..\sdkwork-sdk-generator commit -m "feat: add rpc sdk verification plans"
```

---

### Task 13: Document RPC SDK Generator Usage

**Files:**
- Modify: `../sdkwork-sdk-generator/README.md`
- Modify: `../sdkwork-sdk-generator/src/readme-cli-contract.test.ts` if existing README tests require updates

- [ ] **Step 1: Add README section**

Add:

```md
## RPC SDK Generation

Use `sdkgen generate --protocol rpc` to generate SDKWork RPC SDK package scaffolds and Buf/protoc-compatible generation configuration from proto contracts and `sdkwork.rpc.manifest` files.
```

Include example command for TypeScript and Rust.

- [ ] **Step 2: Document current limitation**

State:

```md
`sdkgen` orchestrates SDKWork RPC generation. It does not replace Protocol Buffers, Buf, protoc, or language-specific gRPC plugins.
```

- [ ] **Step 3: Run README tests**

Run:

```powershell
npx vitest run src/readme-cli-contract.test.ts
```

Expected: PASS. Update tests only if they validate supported command docs.

- [ ] **Step 4: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add README.md src/readme-cli-contract.test.ts
git -C ..\sdkwork-sdk-generator commit -m "docs: document rpc sdk generation"
```

---

### Task 14: End-To-End Dry-Run Fixture

**Files:**
- Create: `../sdkwork-sdk-generator/test/fixtures/rpc/communication/proto/sdkwork/communication/app/v3/message_service.proto`
- Create: `../sdkwork-sdk-generator/test/fixtures/rpc/communication/rpc/sdkwork-communication-rpc.manifest.json`
- Create: `../sdkwork-sdk-generator/test/rpc-generate-dry-run.test.ts`

- [ ] **Step 1: Add fixture proto**

Create minimal `MessageService` with one unary method:

```proto
syntax = "proto3";

package sdkwork.communication.app.v3;

service MessageService {
  rpc CreateMessage(CreateMessageRequest) returns (CreateMessageResponse);
}

message CreateMessageRequest {
  string conversation_id = 1;
  string text = 2;
}

message CreateMessageResponse {
  string message_id = 1;
}
```

- [ ] **Step 2: Add fixture manifest**

```json
{
  "schemaVersion": 1,
  "kind": "sdkwork.rpc.manifest",
  "domain": "communication",
  "sdkFamily": "sdkwork-communication-rpc-sdk",
  "services": [
    {
      "package": "sdkwork.communication.app.v3",
      "service": "MessageService",
      "surface": "app",
      "methods": [
        {
          "method": "CreateMessage",
          "operationId": "messages.create",
          "auth": "app-session",
          "idempotency": "required",
          "streaming": "unary",
          "owner": "communication-open-api",
          "compatibility": "v3"
        }
      ]
    }
  ]
}
```

- [ ] **Step 3: Write E2E dry-run test**

Run CLI through test helper and assert output includes:

```text
buf.gen.yaml
README.md
.sdkwork/sdkwork-rpc-generator-manifest.json
src/metadata.ts
```

- [ ] **Step 4: Run E2E test**

Run:

```powershell
npx vitest run test/rpc-generate-dry-run.test.ts
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C ..\sdkwork-sdk-generator add test/fixtures/rpc test/rpc-generate-dry-run.test.ts
git -C ..\sdkwork-sdk-generator commit -m "test: cover rpc sdk dry-run generation"
```

---

### Task 15: Final Verification

**Files:**
- No new files expected.

- [ ] **Step 1: Verify standards links**

Run:

```powershell
rg -n "RPC_SDK_WORKSPACE_SPEC|sdkgen --protocol rpc|RPC SDK" ..\sdkwork-specs
```

Expected: relevant specs reference RPC SDK generation without duplicated conflicting bodies.

- [ ] **Step 2: Verify generator typecheck and tests**

Run:

```powershell
npm run lint
npm test
```

Expected: PASS.

- [ ] **Step 3: Verify legacy HTTP CLI dry-run still works**

Run from `../sdkwork-sdk-generator`:

```powershell
node bin/sdkgen.js generate `
  -i .\test-openapi.json `
  -o .\tmp-out-http-non-regression `
  -n Demo `
  -l typescript `
  --dry-run
```

Expected: CLI reports the same HTTP/OpenAPI SDK generation flow as before, does not require RPC flags, and does not emit `.sdkwork/sdkwork-rpc-generator-*` files.

- [ ] **Step 4: Verify RPC CLI dry-run**

Run from `../sdkwork-sdk-generator`:

```powershell
node bin/sdkgen.js generate `
  --protocol rpc `
  --input .\test\fixtures\rpc\communication\rpc\sdkwork-communication-rpc.manifest.json `
  --proto-root .\test\fixtures\rpc\communication\proto `
  --output .\tmp\rpc-sdk-dry-run `
  --name SdkworkCommunicationRpc `
  --sdk-name sdkwork-communication-rpc-sdk `
  --language typescript `
  --package-name @sdkwork/communication-rpc-sdk `
  --dry-run
```

Expected: CLI reports planned RPC SDK files and does not write output.

- [ ] **Step 5: Inspect git status**

Run:

```powershell
git -C ..\sdkwork-specs status --short
git -C ..\sdkwork-sdk-generator status --short
git status --short
```

Expected: only planned changes or committed changes remain.

- [ ] **Step 6: Record completion evidence**

Update the final response with:

```text
Standards verification:
- rg ...

Generator verification:
- npm run lint
- npm test
- sdkgen generate ... legacy HTTP --dry-run
- sdkgen generate --protocol rpc ... --dry-run
```

Do not claim generated RPC SDKs compile in real languages until Buf/protoc execution and language-specific compile checks have actually run.

---

## Follow-Up Work After This Plan

After this plan is complete, create a separate implementation plan for Craw Chat IM RPC contracts:

- `proto/sdkwork/communication/app/v3/message_service.proto`
- `proto/sdkwork/communication/app/v3/conversation_service.proto`
- `sdks/sdkwork-im-rpc-sdk/rpc/sdkwork-im-rpc.manifest.json`
- `crates/sdkwork-im-rpc-proto-rust`
- `crates/sdkwork-im-rpc-rust`

That follow-up should use `superpowers:test-driven-development` and `superpowers:verification-before-completion`, because it will implement actual generated SDK and Rust RPC behavior.
