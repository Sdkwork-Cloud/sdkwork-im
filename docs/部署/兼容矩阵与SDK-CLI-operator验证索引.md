# 兼容矩阵与SDK-CLI-operator验证索引

## 1. 目的

本页把当前仓库里已经分散落地的几类资产收敛到一个入口：

- `compatibility matrix`
- IM/App/Backend/RTC `SDK facade`
- `craw-chat-cli`
- `open-chat-test` �?operator 验证入口
- 对应�?contract / E2E / control-plane / registry 证据

当前目标不是重复写一套新规则，而是降低从治理基线追到真实消费者与验证入口的查找成本�?

## 2. 权威来源

当前这条链路的权威来源按顺序收敛为：

1. 协议与治理基�?
   - `crates/craw-chat-ccp-registry/tests/compatibility_matrix_test.rs`
   - `services/control-plane-api/tests/protocol_registry_test.rs`
   - `services/control-plane-api/tests/protocol_governance_test.rs`
2. operator / CLI 消费说明
   - [`CLI聊天验证与兼容矩�?md`](./CLI聊天验证与兼容矩�?md)
3. SDK facade 边界
   - [`../../sdks/README.md`](../../sdks/README.md)
   - [`../../sdks/sdkwork-im-sdk/README.md`](../../sdks/sdkwork-im-sdk/README.md)
   - [`../../sdks/sdkwork-im-app-sdk/README.md`](../../sdks/sdkwork-im-app-sdk/README.md)
   - [`../../sdks/sdkwork-im-backend-sdk/README.md`](../../sdks/sdkwork-im-backend-sdk/README.md)
   - [`../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md`](../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/README.md)

## 3. �?compatibility matrix 到消费�?

### 3.1 app-facing

- `sdkwork-im-sdk`
- 负责 conversation / message / timeline / realtime watch / session surface
- 当前消费基线�?
  - `payload.json`
  - `ccp/ws/1`
  - `auth_bind`
  - 按协商启�?`session.resume`

### 3.2 admin-facing

- `sdkwork-im-backend-sdk`
- 负责�?
  - `compatibility matrix`
  - `protocol governance`
  - `control-plane`
  - registry / rollout / kill switch 的管理消费面

### 3.3 CLI / operator

- `craw-chat-cli`
- `bin/chat-cli.*`
- `bin/chat-window.*`
- `bin/open-chat-test.*`

这条 operator/CLI 消费链当前固定使用：

- `compatibility matrix`
- `goaway`
- `session.disconnect`
- `realtime.overload`
- `resume fallback`
- `4001`
- `reconnect_required`
- `pull-only`
- `events.pull`

## 4. 验证入口索引

### 4.1 contract gates

- `tools/chat-cli/tests/chat_cli_contract_test.rs`
  - 冻结�?
    - CLI authority model
    - `token` 默认 header 输出�?`--token-only` �?token 边界
    - `appContextProjection / providedBearerToken` 来源语义
    - `--bearer-token` 大小写无关前缀归一化边界，`bearer <token>` 必须收敛�?`Bearer <token>` / `<token>`
    - `providedBearerToken` �?`claims = null` 边界，禁止把本地 CLI 输入伪装成外�?token 已解�?claims
    - SDK facade 边界
    - `compatibility matrix`
    - scripted validation contract
    - recovery baseline
    - 单一索引页本�?
- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
  - 冻结�?
    - `bin/chat-cli.ps1` 交互会话包装�?
    - `bin/chat-cli.cmd --help` �?raw-arg pass-through 边界
    - Windows 包装器不得把 `--help` 改写�?`-Help`

### 4.2 E2E / operator gates

- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
  - 覆盖 `craw-chat-cli` 主链路与 `open-chat-test` scripted validation
  - 覆盖 `bin/open-chat-test.cmd` �?GNU-style named flag 合同，确�?Windows `.cmd` 路径�?`.sh` scripted validation 参数表保持一�?
  - 覆盖 `bin/open-chat-test.cmd` �?`validation-message` 字面保真边界，确�?Windows wrapper 不会吞掉 `!`
  - 覆盖 `bin/open-chat-test.cmd --help` �?discoverability 合同，确�?Windows help 面显式展�?GNU-style named flags
  - 覆盖 `bin/chat-window.cmd` �?GNU-style named flag 合同，确�?Windows `.cmd` 路径�?`.sh` interactive launch 参数表保持一�?
  - 覆盖 `bin/chat-window.cmd` �?`message-prefix` 字面保真边界，确�?Windows wrapper 不会吞掉 `!`
  - 覆盖 `bin/chat-window.cmd --help` �?discoverability 合同，确�?Windows help 面显式展�?GNU-style named flags
  - 覆盖 `bin/chat-window-gui.cmd` �?GNU-style named flag 合同，确�?Windows GUI `.cmd` 路径能真正绑定到可见 GUI 启动参数�?
  - 覆盖 `bin/chat-window-gui.cmd --help` �?discoverability 合同，确�?Windows GUI help 面显式展�?GNU-style named flags
  - 覆盖 `bin/chat-window-gui.cmd` �?`-Label` / `--label` 字面保真边界，确�?Windows GUI wrapper 不会吞掉 `!`
- `services/session-gateway/tests/websocket_smoke_test.rs`
  - 覆盖 `goaway`、`session.disconnect`、`4001`、`realtime.overload`
- `services/session-gateway/tests/http_smoke_test.rs`
  - 覆盖 `reconnect_required`、`events.pull` / `event.window` 继续追平�?fresh resume fallback

### 4.3 operator 文档入口

- [`CLI聊天验证与兼容矩�?md`](./CLI聊天验证与兼容矩�?md)
- [`性能与灾备演练场�?md`](./性能与灾备演练场�?md)
- [`快速启动脚�?md`](./快速启动脚�?md)

### 4.4 commercial release gate

- `node scripts/release/commercial-readiness.mjs`
  - 作为统一 commercial readiness 入口，串行执�?frontend / backend / contract / performance 基线验证
  - 若依赖命令执行失败、运行环境缺失、或容量证据索引无法读取 / 解析，必须以 `exit code 1` 失败退�?
  - �?`artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json` 仍是模板或待采集状态，必须�?`exit code 2` 阻断发布
  - release 口径、阻断语义与文档要求�?[`../release/README.md`](../release/README.md) 为准

## 5. 当前使用建议

当需要从治理结论一路追到真实消费面时，按下面顺序看�?

1. 先看 `compatibility matrix` �?control-plane 测试�?
   - `compatibility_matrix_test.rs`
   - `protocol_registry_test.rs`
   - `protocol_governance_test.rs`
2. 再看 SDK facade 边界�?
   - `sdkwork-im-sdk`
   - `sdkwork-im-backend-sdk`
3. 再看 CLI / operator 入口�?
   - `craw-chat-cli`
   - `open-chat-test`
4. 再看 commercial release gate�?
   - `node scripts/release/commercial-readiness.mjs`
   - `docs/release/README.md`
5. 最后看 contract / E2E�?
   - `chat_cli_contract_test.rs`
   - `chat_cli_e2e_test.rs`

## 6. 当前边界

- 当前索引页只收敛已存在证据，不宣称多语言 SDK 真实生成链已经完成�?
- 当前索引页只覆盖现阶段已稳定�?CLI / operator / SDK facade 验证链，不替代后�?release bundle 归档物�?
