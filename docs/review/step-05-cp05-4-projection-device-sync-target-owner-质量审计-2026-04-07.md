# Step 05 CP05-4 projection device-sync target owner 质量审计

## 1. 审计范围

- `services/projection-service/src/lib.rs`
- `services/projection-service/tests/lib_structure_test.rs`
- `services/projection-service/tests/timeline_projection_test.rs`

## 2. 审计结论

- 结论：本轮增量通过。
- 本轮把 `projection-service` 内部重复的 conversation active principal / registered device fanout 逻辑收回到单一 seam，减少了多处 handler 自行拼接 target 列表带来的 drift 风险。
- 新增 seam 没有改变既有 device-sync payload 结构，只收敛 target 解析与 fallback 语义，属于 `CP05-4` 的真实 owner 收口，而不是表面改名。

## 3. 证据

- 结构证据
  - `test_projection_service_exposes_conversation_device_sync_target_owner_seam`
- 行为证据
  - `test_device_sync_fanout_targets_for_conversation_include_active_members_and_fallback_devices`
- 回归证据
  - `cargo test -p projection-service --offline`
  - `rustfmt --edition 2024 --check services/projection-service/src/lib.rs services/projection-service/tests/lib_structure_test.rs services/projection-service/tests/timeline_projection_test.rs`

## 4. 已消除风险

- 消除了 `message / mutation / handoff / member governance` 各自维护 active principal -> registered device 解析逻辑的重复实现风险。
- 保留了 member governance 路径对 removed principal fallback 的既有语义，避免 owner seam 收口后误丢被影响成员的 sync feed。
- `lib.rs` 行数从本轮前的 929 行下降到 910 行，没有触碰 `Step 02` 的 1000 行红线。

## 5. 剩余风险与未闭环项

- `CP05-4` 仍有其它 projection / notification / multi-device sync owner seam 未完全收口，不能据此宣告 `CP05-4` 完成。
- `Step 05` 仍未完成总体验证，因此 `91 / 95 / 97` 以及 `Wave B / 93` 继续保持未通过状态。
