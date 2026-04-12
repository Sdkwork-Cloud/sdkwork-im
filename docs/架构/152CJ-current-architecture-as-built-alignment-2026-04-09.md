# 152CJ - 当前架构 As-Built 对齐

## Loop74 增量（2026-04-11）
- `shared-channel sync` 当前新增 `dead-letter metadata parity assertion lock seam`。
- `test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor` 当前已显式锁定 dead-letter inventory item 的：
  - `leaseStatus = unclaimed`
  - `takeoverEligible = false`
  - `legacyTakeoverRequired = false`
  - `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt = null`
- 本轮复核后的真实情况是 pending/dead-letter inventory 已通过共享 helper 返回同一套 lease/takeover metadata；Loop74 补的是 dedicated regression lock，而不是新的 runtime policy。
- 当前剩余主缺口从 `dead-letter metadata parity 是否真实存在` 继续收敛为 `richer batch remediation / suggested-action contract / stale-age policy / SLA semantics`。

## Loop75 增量（2026-04-11）
- `shared-channel sync` 当前新增 `conflict suggested-action contract seam`。
- `social_shared_channel_sync_conflict_details(...)` 当前新增 `suggestedAction`，并把最小语汇收敛为：
  - `wait_for_owner_release_or_expiry`
  - `takeover_pending_request`
  - `takeover_with_legacy_override`
- claim conflictItems 与 republish/release/takeover conflict details 当前复用同一条 suggested-action helper。
- stale foreign claim 当前虽然仍会阻止普通 targeted claim，但当前会显式提示 operator 改走 `takeover_pending_request`。
- 本轮没有引入新的 stale policy engine；补的是 machine-readable remediation hint，而不是 scheduler / timeout reclaim。
- 当前剩余主缺口已从 `richer batch remediation / suggested-action contract / stale-age policy / SLA semantics` 继续收敛为 `automatic stale detection / timeout reclaim / scheduler`。

## 1. 目标
- 用最短口径描述仓库当前已实现事实、未闭环事实与 owner。
- 只写 code / test / doc 已验证内容，不用目标态替代现状。
- 闭环结论只使用：`not_closed / local_closure / step_closure / wave_closure / release_closure`。

## 2. 当前真实模块层
- 契约层：`crates/craw-chat-contract-*`、`crates/craw-chat-ccp-*`、`crates/im-platform-contracts`、`crates/im-auth-context`
- 领域层：`crates/im-domain-core`、`crates/im-domain-events`、`crates/im-time`
- 运行时服务：`services/session-gateway`、`services/conversation-runtime`、`services/projection-service`、`services/streaming-service`、`services/rtc-signaling-service`
- 控制面与运维：`services/control-plane-api`、`services/audit-service`、`services/ops-service`、`services/notification-service`、`services/automation-service`、`services/media-service`
- 最小装配与工具：`services/local-minimal-node`、`tools/chat-cli`
- 适配层：`adapters/local-memory`、`adapters/local-disk`、`adapters/object-storage-s3`、`adapters/rtc-*`、`adapters/iot-*`

## 3. 当前已落地主能力
1. `conversation / message / read-cursor / edit / recall / reaction / pin`
2. `session / presence / websocket / route ownership / reconnect`
3. `provider registry / provider policy / protocol governance`
4. `projection / notification / ops / audit`
5. social 最小 truth：
   - `friend_request / friendship / user_block / direct_chat`
   - `external_connection / external_member_link / shared_channel_policy`
   - `domain contract + event contract + minimal control-plane write path`
   - 默认仍可运行于进程内内存态；命中 `CRAW_CHAT_RUNTIME_DIR` 或显式 runtime-dir builder 时会物化 `state/social-state.json` 与 `state/social-commit-journal.json`
   - `contacts / interaction projections`
6. device / IoT / external access：
   - `AgentSubject / DeviceSubject -> Sender` 统一主体模型
   - `session-gateway` resume / heartbeat 注入 device access provider
   - `local-minimal-node` register / owner binding / device stream 主链
   - `GET /api/v1/devices/{device_id}/twin`
   - `POST /api/v1/devices/{device_id}/twin/desired`
   - `POST /api/v1/devices/{device_id}/twin/reported`
   - `/api/v1/iot/access/provider-health`
   - `/api/v1/iot/protocol/provider-health`
   - uplink -> `device.telemetry`
   - downlink -> `device.command`
   - runtime-dir managed `device-twin-state.json`
7. `conversation-runtime` 已落地：
   - `ConversationPolicy / policy_epoch`
   - `ConversationBusinessBinding`
   - `conversation.policy_applied`
   - `POST /api/v1/conversations`
   - `POST /api/v1/conversations/threads`
   - `POST /api/v1/conversations/direct-chats/bindings`
   - `GET /api/v1/conversations/{conversation_id}/binding`
   - `GET /api/v1/conversations/{conversation_id}/messages`
   - `direct_chat` 绑定已走专用 runtime 入口与回放恢复
   - `thread` 现已作为最小子会话 runtime truth：必须锚定到 `group conversation + root message`，并通过 `businessType = thread / businessId = rootMessageId` 暴露 binding
   - `thread` 创建时，若 root message author 与 creator 不同且仍是 parent conversation 的 active member，则会把 root author 一并 materialize 为 thread active member，使现有 projection / notification fanout 可以消费同一 active principal 集合
   - `retention_policy_ref` 当前会派生 `conversation.policy_applied` 与后续 conversation/message mutation commit 的 `retention_class`
  - `history_visibility` 当前真实支持已扩展为 `joined / world_readable / invited / shared`
- `shared` 当前已覆盖“显式 shared/external 锚点 + 历史读取 + message-posted fanout / projection rebuild + system-owned shared-channel linked-member sync seam + control-plane ready-pair auto-trigger seam + local-minimal embedded real runtime consumer + standalone control-plane public HTTP runtime consumer”的最小 truth；当前剩余缺口已收敛到 cross-service stale-claim / lease / SLA boundary
   - `notification-service` 的 `message.posted` fanout 当前已通过 `projection-service` auth-context seam 覆盖 `joined active members + shared-history-visible linked members`；projection replay / rebuild 会保留同一 recipient truth
8. `S10 = step_closure`
9. `S11 = step_closure`
10. `S12 = step_closure`
11. `S13 = step_closure`
12. `S14 = step_closure`
13. current global gate: `release_closure = no` because `S07` is not yet `step_closure`

## 4. 当前 social As-Built 口径
- truth 已覆盖：`friend_request + friendship + direct_chat + external_connection + external_member_link + shared_channel_policy`
- runtime truth 形态：
  - 默认 builder 仍走内存态 `SocialControlState + MemoryCommitJournal`
  - runtime-dir builder 会落地 `state/social-state.json` durable snapshot 与 `state/social-commit-journal.json` append-only journal
  - 新写 journal 已补齐 `friend_request.requestMessage` 与 `user_block.expiresAt`；旧 journal 缺字段时 replay 按 `None` 降级
  - `friend_request / friendship / user_block / direct_chat / external_* / shared_*` 写链路已统一复用同一持久化骨架
- external collaboration 当前最小 truth：
  - `external_connection` 只能跨租户建立
  - `external_member_link` 必须依赖 active `external_connection`
  - 新写入的 `external_member_link` 现在会持久化 `localActorKind`；legacy 缺失该字段的 truth 仍按降级态保留
  - `shared_channel_policy` 必须依赖 active `external_connection`
  - `shared_channel_policy` 只允许 `history_visibility = shared`
  - `control-plane-api` 现在会在 `external_member_link / shared_channel_policy` ready pair 成立时，通过注入的 `SharedChannelLinkedMemberSyncTrigger` 自动 dispatch shared-channel sync request
  - `local-minimal-node` 默认/公开装配面当前会 merge embedded `/api/v1/control/*` surface，并把该 trigger 注入为同进程 `ConversationRuntime` consumer
  - standalone `control-plane-api` 当前也可通过 `CRAW_CHAT_SHARED_CHANNEL_SYNC_TARGET_BASE_URL` 装配 public HTTP trigger，并以 public bearer 身份调用 standalone `conversation-runtime` 的 `/api/v1/conversations/shared-channel-links/sync`
- `control-plane-api` 当前会把 shared-channel sync 的未投递请求 durable 记入 `SocialControlState.pending_shared_channel_sync_requests`；当 trigger 缺失或 dispatch 失败时，social durable truth 继续保持 committed，而 backlog 会随 runtime-dir snapshot 一并持久化
- 当同一 request 连续失败达到固定阈值时，control-plane 当前会把它从 `pending_shared_channel_sync_requests` 转入 `dead_letter_shared_channel_sync_requests`；dead-lettered request 不再进入 `next healthy ready-pair write` 自动重试队列，也不再被 `repair-shared-channel-sync` 继续消费
- 当 trigger 恢复健康且后续再次出现新的 ready pair 写入时，dispatch 路径当前会先重放已有 pending backlog，再投递本次新请求；成功项会沿成功路径 best-effort 清理 backlog
- `dispatch_shared_channel_sync_requests(...)` 当前会在系统 dispatch / backlog queue evaluation 前主动回收 stale pending claim metadata；这条 write-triggered normalization seam 即使在 `trigger_unconfigured` 分支也会先执行，但在系统 idle 且没有后续写入时仍不会自动触发
- 控制面新增 `POST /api/v1/control/social/runtime/repair-shared-channel-sync`；operator 可显式重放 pending backlog，且 `repair-derived-snapshot` / `repair-social-runtime-dir` replay journal 时都会保留 pending backlog 与 dead-letter backlog；repair response 与 aggregate/report surface 当前也会显式暴露 dead-letter 计数
- 控制面新增 `GET /api/v1/control/social/runtime/pending-shared-channel-sync`；operator 当前可读取 pending inventory，并拿到稳定的 `requestKey`、原始 request payload、`failureCount`、`lastError`、`ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`
- 控制面新增 `POST /api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted`；operator 当前可先按 `requestKey` claim 选中的 pending request，pending/dead-letter inventory item 也会显式暴露 `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`
- 控制面新增 `POST /api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted`；operator 当前可按 `requestKey` 定点把选中的 pending request 显式 republish 到 remote runtime；成功项会从 pending backlog 清理，未选中的 pending request 保持不动
- `republish_pending_shared_channel_sync_targeted(...)` 当前会在 dispatch 前校验 owner contract；foreign-owned 或 unclaimed request 会得到 `409 shared_channel_sync_owner_conflict`；对于 same-owner stale request，targeted republish 当前会先刷新 `claimedAt / leaseExpiresAt`，再进入 dispatch / failure persistence，或在 `trigger_unconfigured` 早返回前持久化刷新后的状态
- 控制面新增 `POST /api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted`；owner operator 当前可显式释放 selected pending request 的 owner 元数据、`claimedAt` 与 `leaseExpiresAt`，使 request 返回 unowned pool；foreign-owned request 的 release 会得到 `409 shared_channel_sync_owner_conflict`
- 控制面新增 `POST /api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted`；operator 当前可显式接管 selected `foreign-owned` pending request，并刷新 `claimedAt / leaseExpiresAt`
- `PendingSharedChannelSyncRequest` 当前已具备 durable `claimedAt / leaseExpiresAt`；首次 owner claim 会写入 UTC RFC3339 毫秒时间戳与固定 `15m` lease deadline，同 owner 重复 claim 在 lease 仍然 active 时会保留原元数据，但 same-owner stale re-claim 现在会刷新 `claimedAt / leaseExpiresAt`；这仍然只是 stale-claim metadata seam，而不是 stale-age policy / SLA 执行
- 控制面新增 `GET /api/v1/control/social/runtime/dead-letter-shared-channel-sync`；operator 当前可读取 dead-letter inventory，并拿到稳定的 `requestKey`、原始 request payload、`failureCount` 与 `lastError`
- 控制面新增 `POST /api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted`；operator 当前可按 `requestKey` 定点把选中的 dead-letter request 回灌到 pending；同一迁移会重置被选中 request 的 `failureCount = 0`，保留 `lastError`，未选中的 dead-letter request 继续隔离
- projection 已覆盖：
  - friendship-derived contacts
  - direct-chat conversation enrich / backfill
  - `message_interaction_summary`
  - `pinned_messages`
  - `timeline_from_auth_context(...)` 当前已接受 `can_read_shared_history()` 的 linked member 读取 shared history
- 当前已具备 `durable snapshot + social outbox + journal-authoritative startup replay` 样板；若 `social-commit-journal` 存在，则启动时从默认空状态 replay 并回写 snapshot；仅在 journal 缺失时才回退到 snapshot。
- social 持久化当前顺序已改为 `append social-commit-journal -> write state/social-transaction-marker.json -> save social-state -> clear marker`；snapshot 被收敛为 derived cache；若 `journal append` 成功但 `snapshot save` 失败，live state 仍推进到已提交 truth，避免后续写入继续基于旧内存放大冲突；写口现在直接返回 committed result，并显式暴露 `persistence.journalAuthority + persistence.snapshotStatus = repair_required`，同时把 `social-transaction-marker.json` 保留下来作为 pending snapshot repair 边界；当前 `same tenant + same eventId` 已可直接回放 committed result、避免重复追加 journal，并会 best-effort 修复 snapshot cache，把 `snapshotStatus` 收敛回 `current` 且清除 marker；runtime-dir 模式新增 `state/social-failpoints.json` 的一次性 `failNextSnapshotSave` 注入点，控制面新增 `POST /api/v1/control/social/runtime/repair-derived-snapshot` 作为 derived snapshot repair 入口，且该入口现在会直接 replay `social-commit-journal.json` 并同步刷新 live state、snapshot 与 pending marker；`SocialRuntimeRepairResponse` 现已显式暴露 `transactionMarkerCleared`，使 HTTP repair 与 `control-plane-api repair-social-runtime-dir --runtime-dir <path> [--json]` 这两个 operator surface 都能直接报告 pending marker 是否在本次修复中被清除；`repair-runtime-local.ps1/.sh` 也已统一为 `generic repair -> conditional social repair` 的 operator 入口；当前已具备 `repair-marker` 形式的 `atomic multi-file tx` 最小边界，这条证据链已足够支撑 `S05 = step_closure`，而更强 staged/manifest 级事务转入 deferred durability hardening。
- `projection-service` 只负责读模型与 rebuild，不反写 truth。

## 5. 当前 device / IoT As-Built 口径
- 契约层：
  - `AgentSubject / DeviceSubject` 已统一到同一 `Sender` 语义。
  - `DeviceSubject::sender(...)` 真实产出 `kind = device`。
- 运行时：
  - `session-gateway` 的 session resume / heartbeat 已走 device access provider。
  - `local-minimal-node` 的 device register 已走 device access provider 与 owner binding。
  - `device.telemetry` 与 `device.command` 已进入统一 stream/runtime/access 主链。
  - `device_twin` 已进入 `local-minimal-node` runtime/mainline。
  - twin read 允许 bound device 或 registered owner side。
  - desired write 只允许 non-device bound owner side。
  - reported write 只允许 bound device actor。
- 存储与恢复：
  - non-runtime-dir 模式使用 `MemoryDeviceTwinStore`。
  - runtime-dir 模式使用 `FileDeviceTwinStore`。
  - `device-twin-state.json` 已进入 managed runtime state 文件清单。
  - twin state 已通过 restart/rebuild 路径恢复验证。
- 协议层：
  - `iot-access-local` 已公开 `registry / credential / binding / twin` capability。
  - `iot-mqtt` 已具备 uplink decode、downlink encode、provider health。
  - known-device preflight、actor preflight、request mismatch、decoded mismatch 均已测试。
- 当前边界：
  - `shared` republish/runtime 不属于当前 twin 闭环。
  - 不宣称 durable repo / tx / outbox / replay 已闭环。

## 6. 当前 policy As-Built 口径
- 已落地：
  - control-plane 词汇发布
  - governance 只发布真实 runtime 支持模式：`joined / world_readable`
  - runtime policy 聚合建模
  - policy event / replay
  - HTTP 建会即绑定 policy
  - capability 约束
  - 最小历史消息 `GET` 读口
  - history query auth-context 入口
  - `joined / world_readable` 历史读权限差异
  - `retention_policy_ref -> retention_class` runtime commit 传播
- `shared_channel_policy` 已具备控制面 future truth write path；`conversation-runtime` 也已具备 system-owned shared-channel linked-member sync seam；`control-plane-api` 当前已具备 ready-pair auto-trigger seam、durable pending backlog、pending inventory read surface、targeted pending republish seam、pending targeted claim seam、claim conflictItems visibility seam、republish ownership guard、pending targeted release seam、pending targeted takeover seam、pending leaseExpiresAt visibility seam、pending/dead-letter inventory leaseStatus visibility seam、pending/dead-letter inventory takeoverEligible visibility seam、legacy untracked takeover explicit override seam、takeover conflict machine-readable details symmetry seam、republish/release owner-conflict machine-readable details symmetry seam、`next healthy ready-pair write` 自动重放 backlog seam、`next-write proactive stale-claim reclaim` seam、repeated-failure dead-letter seam、dead-letter inventory read surface、dead-letter requeue operator seam、targeted dead-letter requeue seam、requeue failure-budget rearm seam 与 operator repair route，且 `local-minimal-node` 默认装配面与 standalone public app 都已装配真实 runtime consumer；当前剩余主缺口已收敛为 `automatic stale detection / timeout reclaim / scheduler` 与 `release-ready exactly-once semantics`
- 未落地：
  - `shared` vocabulary republish
- `control-plane -> conversation-runtime automatic sync trigger / auto-projection`
  - invitation / shared-channel durable persistence 与 outbox / replay
  - `retention_policy_ref` 与 archive / restore / retention owner 联动

## 7. 当前 deploy / operability / perf As-Built 口径
- deploy/profile/operator 当前真实闭环：
  - `local-minimal / local-default`
  - `install / init-config / start / status / restart / stop / deploy`
  - `inspect / repair / list / archive / prune / preview / restore`
  - `local-default` 当前仍复用 `local-minimal` 的 compose / service / smoke 合同
- runtime-dir 当前真实闭环：
  - backup catalog 可区分 `active / archived`
  - archive metadata 已暴露 `storageClass / retentionPolicy / retentionDays / archivedAt / restoreStatus / legalHold`
  - repair 采用 backup-first
  - restore preview 提供 typed diff 与 fingerprint
  - restore 会创建 pre-restore backup，并阻断 fingerprint mismatch
- Step 11 tier 当前真实闭环：
  - `scenario catalog` 只负责冻结 inventory/gate template，不替代 tier collection state
  - `CI Smoke Tier` 已有 2026-04-10 fresh quant / HA-DR / rollback evidence
  - `Pre-Release Tier` 当前真实状态是 `evidence_collected_gate_blocked`
  - `Capacity Tier` 当前真实状态是 `template_only_pending_execution`
- 当前边界：
  - 不宣称真实 `capacity-dedicated` 容量结果
  - 不宣称多 cell / 多 region / object storage DR 已闭环
  - 不宣称完整生产级 SLO / alert / gate automation 已闭环

## 8. 当前仍未物化为 durable truth 或 step 闭环的能力
- `S05` 已达到 `step_closure`：当前已具备 `journal authority + journal-first persistence + repair-marker based atomic multi-file tx minimal boundary + committed ack boundary + response persistence visibility + same-event replay acknowledgment + injected failpoint + journal-authoritative operator repair endpoint + standalone journal-only repair CLI + unified repair-runtime-local operator surface`；更强 staged/manifest 事务证明转为 deferred durability hardening
- `space / space_member / chat_group / group_member / chat_channel / channel_access_rule / membership_request / ban_record`
- `im_thread_subscription` durable model / per-user unread-notification level
- `cross-service shared-channel automatic stale detection / timeout reclaim / scheduler`
- `release-ready exactly-once delivery semantics for shared-channel sync`
- `retention_policy_ref` 与 archive / restore / retention owner 闭环
- control-plane 可写、可版本化的 automation governance durable truth

## 9. 当前最准确的架构表述
- 当前系统最成熟的是 `会话运行时 + 实时接入 + protocol/provider/control-plane` 主骨架。
- `S05` 当前可以诚实写成 `step_closure`：已具备 file-backed `social-state.json + social-commit-journal.json`、journal-authoritative startup replay、journal-first persistence、`journal append -> social-transaction-marker -> snapshot save -> clear marker` 的最小可恢复边界、`journal append -> committed ack` 边界、`snapshot save fail` 的 `persistence.snapshotStatus` 可见性、`same tenant + same eventId` 的 replay/ack contract、一次性 failpoint 注入、journal-authoritative 的 operator 级 derived-snapshot repair 入口、standalone `repair-social-runtime-dir` CLI，以及统一后的 `repair-runtime-local.*` operator 入口；更强 staged/manifest 级事务保留为 durability hardening backlog，而不再阻塞当前 step。
- `S07` 当前只能诚实写成 `not_closed / local_closure`：已具备 `reaction / pin` runtime truth、`direct_chat -> conversation` runtime binding、`thread minimal runtime truth`、`thread root-author subscription / notification runtime truth`、`invited history visibility runtime truth`、`shared-external history runtime truth`、`shared fanout / rebuild`、`shared_channel_policy durable sync seam`、`control-plane ready-pair auto-trigger seam`、`local-minimal embedded real runtime consumer`、`standalone control-plane public HTTP runtime consumer`、`shared-channel sync durable pending backlog / operator repair seam`、`shared-channel sync pending inventory seam`、`shared-channel sync targeted pending republish seam`、`shared-channel sync pending targeted claim seam`、`shared-channel sync claim conflictItems visibility seam`、`shared-channel sync republish ownership guard seam`、`shared-channel sync pending targeted release lifecycle seam`、`shared-channel sync pending targeted takeover seam`、`shared-channel sync pending leaseExpiresAt visibility seam`、`shared-channel sync pending/dead-letter inventory leaseStatus visibility seam`、`shared-channel sync pending/dead-letter inventory takeoverEligible visibility seam`、`shared-channel sync legacy untracked takeover explicit override seam`、`shared-channel sync takeover conflict machine-readable details symmetry seam`、`shared-channel sync republish/release owner-conflict machine-readable details symmetry seam`、`next healthy ready-pair write auto-retry seam`、`next-write proactive stale-claim reclaim seam`、`shared-channel sync dead-letter seam`、`shared-channel sync dead-letter inventory seam`、`shared-channel sync dead-letter requeue seam`、`shared-channel sync targeted dead-letter requeue seam`、`shared-channel sync dead-letter rearm seam`，以及 `retention enforcement runtime truth`；当前剩余主缺口收敛为 `automatic stale detection / timeout reclaim / scheduler` 与 `release-ready exactly-once semantics`，而更强 `im_thread_subscription` durable model / per-user unread-notification level 继续 deferred。
- `conversation-runtime` 当前真实支持的 `history_visibility` 已扩展为 `joined / world_readable / invited / shared`；其中 `shared` 仅在显式 shared/external 锚点存在时开放历史读取。
- `conversation-runtime` 当前已把 `retention_policy_ref` 派生到 `conversation.policy_applied` 与后续 conversation/message mutation commit 的 `retention_class`，但 archive / restore / legal-hold owner 仍不在当前闭环。
- `S10` 当前可以诚实写成 `step_closure`。
- `S11` 当前可以诚实写成 `step_closure`。
- `S12` 当前可以诚实写成 `step_closure`：`device actor + device_twin runtime/mainline + IoT access/protocol + external collaboration truth` 已同时具备代码、测试、review、release 证据。
- `S13` 当前可以诚实写成 `step_closure`：deploy/profile/operator/runtime-dir lifecycle/CI Smoke quant-drill 已同时具备 fresh tests，而 `Pre-Release Tier` 与 `Capacity Tier` 的未完成边界也已 machine-readable 固化。
- 当前仍不能把 `Pre-Release Tier` 写成 full gate sign-off，也不能把 `Capacity Tier` 写成真实容量结论。
- 当前仍不能把 `shared_channel_policy` 解释为“已具备 release-ready cross-service delivery semantics”；当前已实现 control-plane ready-pair auto-trigger seam、durable pending backlog、pending inventory、targeted pending republish、targeted pending claim、claim conflictItems visibility、republish ownership guard、pending targeted release、pending targeted takeover、pending leaseExpiresAt visibility、pending/dead-letter inventory leaseStatus visibility、pending/dead-letter inventory takeoverEligible visibility、legacy untracked takeover explicit override、takeover conflict machine-readable details symmetry、republish/release owner-conflict machine-readable details symmetry、`next healthy ready-pair write` 自动重放 backlog、repeated-failure dead-letter、dead-letter inventory、dead-letter requeue、targeted dead-letter requeue、failure-budget rearm 与 operator repair route，且 `local-minimal-node` 默认装配面与 standalone `control-plane-api` public app 都已注入真实 consumer，但 `automatic stale detection / timeout reclaim / scheduler` 与 `release-ready exactly-once semantics` 仍由 `S07 / S08 / governance` 后续波次承担。
- `conversation-runtime` 仍是最强业务核心，但不应扩张为 social durable truth 中心。

## 10. 文档写作标准
- 每个能力都要同时写清 `Current State / Target State / Deferred / Owner`。
- 只有满足代码、测试、review、release 回写后，才能把能力从 `not_closed / local_closure` 提升到更高闭环级别。
- 所有未闭环能力必须显式归属到后续 `Sxx` 或 backlog owner。

## 11. Loop67 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `pending stale-lease takeover guard seam`。
- `POST /api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted` 当前不再应被解释为“无条件接管 foreign-owned request”；它现在只允许接管 `expired foreign claim`。
- 当 selected pending request 仍被其他 operator active claim 且 `leaseExpiresAt > now` 时，control-plane 当前会返回 `409 shared_channel_sync_owner_conflict`。
- 当 selected pending request 的 `leaseExpiresAt <= now` 时，既有 targeted takeover 路径仍会刷新 `ownerActorId / ownerActorKind / claimedAt / leaseExpiresAt`。
- `leaseExpiresAt` 缺失的 legacy pending claim 当前保持最小兼容，不会被 stale guard 阻断；本轮没有把它扩展成 migration / rewrite pass。
- 这意味着 `S07` 的剩余主缺口已从“takeover 是否尊重 lease”继续收敛为 `operator stale-status visibility / stale-age policy / SLA semantics`，而不是 scheduler 或 exactly-once。

## 12. Loop68 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `pending/dead-letter inventory leaseStatus visibility seam`。
- `PendingSharedChannelSyncRequest` 当前新增读取态 `lease_status(&self, now)`；inventory/operator surface 会显式返回 `leaseStatus = unclaimed / active / stale / untracked`。
- `leaseExpiresAt` 缺失的 legacy claimed item 当前不会再被 operator 侧隐式猜测；它会被显式标记为 `untracked`。
- pending inventory 与 dead-letter inventory 当前通过共享 response helper 返回同一套 `leaseStatus` contract；本轮直接断言集中在 pending path。
- 本轮没有改变 claim / release / takeover 的状态机语义；补的是 operator-visible derived metadata，而不是 stale policy engine。
- 这意味着 `S07` 的剩余主缺口已进一步从“operator stale-status visibility”收敛为 `legacy untracked claim policy / takeover eligibility visibility / stale-age policy / SLA semantics`，而不是 scheduler 或 exactly-once。

## 13. Loop69 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `pending/dead-letter inventory takeoverEligible visibility seam`。
- `PendingSharedChannelSyncRequest` 当前新增 `takeover_eligible_for(actor_id, actor_kind, now)`；inventory/operator surface 会显式返回 `takeoverEligible`。
- `takeoverEligible` 当前不仅取决于 request 自身状态，还取决于当前 actor 与 `control.write` 权限，因此 read-only viewer 会稳定看到 `false`。
- foreign writer 当前会看到：
  - active foreign claim = `false`
  - legacy `untracked` foreign claim = `true`
  - stale foreign claim = `true`
  - takeover 成为新 owner 之后 = `false`
- 本轮没有改变 takeover route；只是把现有 allow/deny 语义显式投影到 inventory surface。
- 这意味着 `S07` 的剩余主缺口已进一步从“takeover eligibility visibility”收敛为 `legacy untracked claim policy / stale-age policy / SLA semantics`，而不是 scheduler 或 exactly-once。

## 14. Loop70 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `legacy untracked takeover explicit override seam`。
- inventory/operator surface 当前新增 `legacyTakeoverRequired`，用于显式标出 foreign `untracked` claim 不再属于普通 takeover 路径。
- `takeoverEligible` 当前只对应普通 stale foreign claim；foreign `untracked` claim 默认会落成 `takeoverEligible = false` 且 `legacyTakeoverRequired = true`。
- targeted takeover request 当前新增 `allowLegacyUntracked`；只有显式 override 时，legacy no-lease compatibility takeover 才会继续执行。
- 默认 takeover 当前会返回 `shared_channel_sync_legacy_takeover_override_required`，而 takeover response 当前会显式返回 `legacyOverrideUsed`。
- 这意味着 `S07` 的剩余主缺口已进一步从“legacy untracked claim policy”收敛为 `dead-letter metadata parity / stale-age policy / SLA semantics`，而不是普通 pending takeover policy。

## 15. Loop71 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `takeover conflict machine-readable details symmetry seam`。
- `ControlPlaneError` 当前支持可选 `details`；active foreign takeover conflict 与 legacy override-required conflict 当前都会显式回写同一组 machine-readable `details`。
- `details` 当前至少覆盖 `requestKey / ownerActorId / leaseExpiresAt / leaseStatus / takeoverEligible / legacyTakeoverRequired`，使 operator 在写路径冲突上也能直接拿到和 inventory 同语义的最小诊断事实。
- 本轮没有改变 takeover allow/deny 语义；补的是 write-path conflict surface symmetry，而不是新的 stale policy / governance route。
- 这意味着 `S07` 的剩余主缺口已进一步从“takeover conflict write-path 信息不对称”收敛为 `dead-letter metadata parity / claim-release conflict details parity / stale-age policy / SLA semantics`，而不是 pending takeover contract 本身。

## 16. Loop72 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `republish/release owner-conflict machine-readable details symmetry seam`。
- targeted republish 与 targeted release 的 `shared_channel_sync_owner_conflict` 当前都会显式回写 `requestKey / ownerActorId / leaseExpiresAt / leaseStatus / takeoverEligible / legacyTakeoverRequired`。
- takeover / republish / release 三条 owner-conflict 写路径当前复用同一 details helper，operator 不再必须回头再读 inventory 才能理解被拒绝时的 owner / lease / legacy takeover 背景。
- 本轮没有改变 owner boundary allow/deny 语义；补的是 owner-conflict write-path surface symmetry，而不是新的 stale policy / governance route。
- 这意味着 `S07` 的剩余主缺口已从“普通 pending 写路径 conflict 信息不对称”重新收敛回 `dead-letter response parity for lease/takeover metadata / stale-age policy / SLA semantics`，而不是 republish/release contract 本身。

## 17. Loop73 补充事实（2026-04-11）
- `shared-channel sync` 当前新增 `claim conflictItems visibility seam`。
- `SocialSharedChannelSyncPendingClaimResponse` 当前新增 `conflictItems`；foreign-owned pending request 在 targeted claim 冲突时，会按逐 item 回写 `requestKey / ownerActorId / leaseExpiresAt / leaseStatus / takeoverEligible / legacyTakeoverRequired`。
- claim / takeover / republish / release 四条冲突相关 operator surface 当前复用同一 owner/lease 语汇，使 operator 不再只能拿到 claim aggregate conflict count。
- 本轮没有改变 claim status / count 语义；补的是 claim aggregate response 的 item-level diagnostics，而不是新的 batch remediation workflow。
- 这意味着 `S07` 的剩余主缺口继续收敛为 `dead-letter response parity for lease/takeover metadata / stale-age policy / SLA semantics`，而不是普通 pending claim contract 本身。
## Loop 76 Addendum - 2026-04-11
- `repair-shared-channel-sync` is now stale-aware at the operator repair seam.
- Before repair dispatches pending work, expired pending claim ownership is reclaimed and counted.
- The repair response now exposes `reclaimed`.
- The repair audit payload now exposes `reclaimed`.
- This is not a scheduler:
  - no background timeout sweep exists
  - no proactive stale reclaim exists outside explicit operator repair
- Remaining S07 gap after Loop76:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
## Loop 77 Addendum - 2026-04-11
- The next-write auto-retry path now respects pending claim ownership.
- `Active` and `Untracked` claimed backlog is blocked from system auto-retry.
- Same-key incoming requests are also blocked while those claims exist.
- `Unclaimed` backlog still auto-retries, so the original next-write recovery seam remains intact.
- Remaining S07 gap after Loop77:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `stale owner metadata cleanup before next-write auto-retry`
  - `release-ready exactly-once semantics`
## Loop 78 Addendum - 2026-04-11
- The next-write retry failure persistence path is now stale-aware.
- When stale claimed backlog is retried and fails again, the failed pending item is rewritten without obsolete owner metadata.
- This aligns stale reclaim behavior across:
  - runtime-level pending backlog persistence
  - operator repair retry failure
  - targeted republish retry failure
- Remaining S07 gap after Loop78:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current lease-status visibility`
## Loop 79 Addendum - 2026-04-11
- The shared-channel sync control surface now includes an explicit stale pending-claim reclaim route.
- Stale ownership can now be cleared without dispatching backlog or waiting for retry-failure persistence.
- This separates stale cleanup from retry and repair execution while preserving existing backlog membership.
- Remaining S07 gap after Loop79:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current manual reclaim surface`
## Loop 80 Addendum - 2026-04-11
- Dead-letter requeue now clears owner metadata before restoring backlog into the pending pool.
- This prevents historical stale owner metadata from being reintroduced by the dead-letter recovery seam.
- Remaining S07 gap after Loop80:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current path-local reclaim seams`
## Loop 81 Addendum - 2026-04-11
- Dead-letter promotion now also clears owner metadata before a threshold-crossing pending request enters the failure bucket.
- This closes the reverse consistency gap left after Loop80:
  - pending -> dead-letter is now ownership-neutral
  - dead-letter -> pending is now ownership-neutral
- Active claim semantics remain authoritative only inside the pending pool.
- Remaining S07 gap after Loop81:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current manual and path-local reclaim seams`
## Loop 82 Addendum - 2026-04-12
- `repair-shared-channel-sync` is now stale-aware even when shared-channel dispatch is unconfigured.
- The repair surface now normalizes the pending pool before deciding whether dispatch can happen.
- This strengthens the operator-triggered reclaim seam without adding a background scheduler.
- Remaining S07 gap after Loop82:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current operator-triggered and path-local reclaim seams`
## Loop 83 Addendum - 2026-04-12
- Same-owner stale pending claim now renews lease metadata instead of returning success with a stale lease.
- Targeted claim success for the current owner is now lease-consistent:
  - active same-owner repeat claim stays idempotent
  - stale same-owner re-claim refreshes `claimedAt / leaseExpiresAt`
- No `deployment_profile`, script entry, or operator route changed in this loop.
- Remaining S07 gap after Loop83:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current operator-triggered and path-local renew / reclaim seams`
## Loop 84 Addendum - 2026-04-12
- Same-owner stale targeted republish now renews lease metadata before dispatch on trigger-enabled paths.
- Failed targeted republish no longer clears owner metadata just because the previous lease had already expired.
- No `deployment_profile`, script entry, or operator route changed in this loop.
- Remaining S07 gap after Loop84:
  - `trigger-unconfigured targeted republish stale normalization`
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
## Loop 85 Addendum - 2026-04-12
- Same-owner stale targeted republish now renews lease metadata before both dispatch-enabled and `trigger_unconfigured` exits.
- Explicit operator republish is now lease-consistent even when no shared-channel trigger is installed.
- No `deployment_profile`, script entry, or operator route changed in this loop.
- Remaining S07 gap after Loop85:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current operator-triggered and path-local normalization seams`
## Loop 86 Addendum - 2026-04-12
- The ordinary next-write path now proactively reclaims stale pending claim metadata before dispatch / queue evaluation.
- This proactive reclaim also runs before the `trigger_unconfigured` branch persists newly failed requests.
- No `deployment_profile`, script entry, or operator route changed in this loop.
- Remaining S07 gap after Loop86:
  - `automatic stale detection / timeout reclaim / scheduler`
  - `release-ready exactly-once semantics`
  - `idle backlog stale-age / SLA policy beyond current operator-triggered and write-triggered normalization seams`
## Loop 87 Addendum - 2026-04-12
- The shared-channel stale pending-claim reclaim scheduler is now enabled by default.
- Runtime operators can still disable it explicitly via `CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED=false`.
- This closes the old "manual-only stale reclaim" path for idle windows where no new writes arrive.
- Remaining S07 gap after Loop87:
  - `release-ready exactly-once semantics`
  - `stale-age / SLA policy beyond current reclaim and lease-normalization seams`
## Loop 88 Addendum - 2026-04-12
- Shared-channel sync now persists a durable delivered-request ledger and carries it across replay/repair merge points.
- Successful dispatch from ordinary path, operator repair, and targeted republish all write the delivered ledger.
- Delivered ledger growth is now bounded by retention + capacity controls:
  - `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS`
  - `CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES`
- Scheduler and write-triggered normalization now prune both:
  - delivered-backlog shadow in pending/dead-letter pools
  - stale / overflow delivered-ledger entries
- Social aggregate counts now expose `deliveredSharedChannelSyncRequests` for operability.
- Remaining S07 gap after Loop88:
  - `release-ready exactly-once semantics`
  - `formal delivery-ack proof contract and downstream idempotency governance`
