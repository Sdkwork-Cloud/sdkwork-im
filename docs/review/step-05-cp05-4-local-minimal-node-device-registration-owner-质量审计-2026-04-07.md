# Step 05 CP05-4 local-minimal-node device registration owner 质量审计 - 2026-04-07

## 1. 审计范围

- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/access.rs`
- `services/local-minimal-node/src/node/session.rs`
- `services/local-minimal-node/src/node/device_registration.rs`
- `services/local-minimal-node/tests/lib_structure_test.rs`

## 2. 审计结论

- 本轮增量通过了 red-green 结构验证、`local-minimal-node` 结构全量回归、包级全量回归和格式检查。
- 未发现新的阻断级行为回退。
- 这次通过的是增量级质量门，不是 `Step 05` 总体质量门。

## 3. 已验证证据

- `test_local_minimal_node_device_registration_owner_moves_out_of_access_impl`
- `cargo test -p local-minimal-node --test lib_structure_test --offline --target-dir target-cp054o-green-local-structure-full`
- `cargo test -p local-minimal-node --offline --target-dir target-cp054o-reg-local-minimal-node`
- `rustfmt --edition 2024 --check services/local-minimal-node/src/node.rs services/local-minimal-node/src/node/build.rs services/local-minimal-node/src/node/access.rs services/local-minimal-node/src/node/session.rs services/local-minimal-node/src/node/device_registration.rs`

## 4. 风险变化

- 已消除
  - `local-minimal-node/access.rs` 同时承担 access/auth 与 device registration owner 的混合职责
  - session/device 路径通过 access glue 直接编排 `realtime_cluster / presence / realtime / projection` 的边界漂移
- 仍保留
  - `CP05-4` 仍有 repo 级残余 seam 待收口
  - `Step 05 / 91 / 95 / 97 / Wave B / 93` 仍未通过
  - `session-gateway` 的 realtime 入口仍存在重复 route-current / register-device 入口 glue
