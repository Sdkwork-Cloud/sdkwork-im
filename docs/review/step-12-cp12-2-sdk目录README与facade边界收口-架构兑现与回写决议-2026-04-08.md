# Step 12 / CP12-2 SDK 目录 README 与 facade 边界收口 架构兑现与回写决议 - 2026-04-08

## 对应架构文档
- `docs/架构/09-实施计划.md`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计-2026-04-06.md`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`

## 本轮已兑现能力
- `09`
  - `Wave D / Step 12` 已从“CLI 主链路可信”推进到“SDK workspace 入口与 facade 边界可信”。
  - 根 README、`sdks/README.md` 与两套 SDK README 现在形成了一套真实、可回归的目录入口。
- `146`
  - `sdkwork-craw-chat-sdk` 与 `sdkwork-craw-chat-sdk-admin` 已冻结为两套不同的 facade 容器，而不是共享模糊职责的同类目录。
  - `TypeScript` / `Flutter` 子目录已被定义为稳定消费路径，后续多语言工作必须遵守这一边界。
  - `compatibility matrix` 已被明确为 capability 决策边界，而不是 README 各自猜测。
- `148`
  - admin 侧 README 已把 `control-plane`、`protocol governance`、`compatibility matrix` 固定为管理侧 SDK 的权威来源。
  - 这意味着管理侧 facade 的职责已与 control-plane snapshot 口径对齐，而不是在各语言目录内本地拼装协议能力决策。

## 本轮未兑现能力
- `146`
  - 多语言 SDK 的真实生成链、样例代码与发布流程尚未完成
  - close / error registry 的客户端恢复语义尚未纳入 SDK README 范围
- `148`
  - tenant / client segment 级治理
  - release bundle 与 rollout 编排
  - 发布后观测与审计闭环
- `149`
  - 多 cell / 多 region 升级兼容、回滚与灾备演练未被本轮新增能力触达

## 是否偏离架构
- 本轮修正的是一个边界缺口，而不是 runtime 偏差：
  - 偏离现象：
    - `sdks/` 已存在目录，但缺少 workspace 入口与明确 facade 合同
    - 多语言子目录容易沦为“空目录占位”，无法承接后续 compatibility matrix 治理
  - 修正结果：
    - `README.md -> sdks/README.md -> app/admin SDK -> TypeScript/Flutter` 现在构成清晰入口链
    - app-facing 与 admin-facing SDK 的职责被显式分离
- 因此当前实现回到了 `146 / 148` 共同定义的“统一治理、分层 facade、控制面权威”主路径。

## 回写决议
- `docs/架构/09-实施计划.md`
  - 追加 `As-Built 107`
- `docs/架构/146-CCP协议注册表与多端SDK兼容治理设计-2026-04-06.md`
  - 追加 `As-Built 5`
- `docs/架构/148-CCP控制面注册表与协议发布治理设计-2026-04-06.md`
  - 追加 `As-Built 6`
- `docs/架构/149-多Cell多Region协议升级与灾备兼容设计-2026-04-06.md`
  - 本轮不追加回写，因为没有新增多 cell / 多 region 兼容声明

## 证据
- 测试：
  - `tools/chat-cli/tests/chat_cli_contract_test.rs`
- 文档：
  - `README.md`
  - `sdks/README.md`
  - `sdks/sdkwork-craw-chat-sdk/README.md`
  - `sdks/sdkwork-craw-chat-sdk-admin/README.md`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/README.md`
  - `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter/README.md`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript/README.md`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/README.md`
- 验证：
  - `cargo test -p craw-chat-cli --offline --test chat_cli_contract_test -- --nocapture`
  - `cargo test -p craw-chat-cli --offline --test chat_cli_e2e_test -- --nocapture`
  - `cargo fmt --all --check`

## 当前判断
- `CP12-2`：通过
- `Step 12`：继续进行中
- 下一步：进入 `CP12-3`
