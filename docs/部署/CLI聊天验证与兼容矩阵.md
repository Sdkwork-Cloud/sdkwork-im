# CLI聊天验证与兼容矩�?

## 1. 文档定位

本文件用于冻�?`Step 12 / CP12-1` �?CLI 验证口径，明确：

- `sdkwork-im-cli` �?`bin/chat-cli*`
- `bin/chat-window*`
- `bin/open-chat-test*`
- 当前 `compatibility matrix` �?CLI 消费范围

目标不是再造一套临时脚本，而是�?CLI 作为发布前可重复执行的标准验证工具�?

## 2. 权威字段模型

当前 CLI 必须遵守新的权威字段模型�?

- 业务请求体中不拼�?`tenantId`、`tenant_id`、`userId`、`clientRouteId`、`sessionId`
- 身份与租户信息通过 SDKWork credential �?realtime `auth_bind` 完成绑定
- 当前�?CLI 本地生成�?token claims 至少包括�?
  - `tenant_id`
  - `sub`
  - `actor_kind`
  - `sid`
  - `did`
- `sdkwork-im-cli token` 默认返回�?
  - `source = appContextProjection`：使�?`--tenant-id` / `--user-id` / `--session-id` / `--device-id`
  - `source = providedBearerToken`：使�?`--bearer-token`
  - `authorization = ""` when using AppContext projection：默�?header 形�?
  - `token = ""` when using AppContext projection：裸 token 形�?
- �?`source = appContextProjection` 时，`claims` 固定�?`null`，CLI 不再本地签发 token�?
- �?`source = providedBearerToken` 时，`claims` 必须返回 `null`�?
  - CLI 只回显外�?token 材料，不伪装成已经解码或校验了外�?token
  - 调用方若要读取外�?token claims，必须在自己的验证链路中完成
- `sdkwork-im-cli token --token-only` 必须�?`authorization` 收敛为裸 token，不能继续保�?`Bearer ` 前缀�?
- `--bearer-token "bearer <token>"` �?`--bearer-token "Bearer <token>"` 必须视为同一输入契约�?
  - 默认 `token` 输出统一规范化为 `authorization = ""` when using AppContext projection
  - `token` 字段始终只暴�?`<token>`
  - `token --token-only` 也只能返�?`<token>`，不能把小写 `bearer ` 前缀泄漏到结果字�?

这意味着�?

- HTTP 主路径通过 `x-sdkwork-*` AppContext 投影头传递权威字�?
- WebSocket / CCP 主路径通过 `hello -> auth_bind -> auth_ok` 完成基础握手与权威确�?
- 只有�?`hello_ack.capabilities` 真实协商�?`session.resume` 时，CLI 才继续执�?`session_resume -> session_resumed`
- CLI 只负责提供调用上下文，不负责在业�?payload 内伪造权威字�?

## 3. 当前命令�?

### 3.1 直接 CLI

- `sdkwork-im-cli token [--token-only]`
- `sdkwork-im-cli create-conversation`
- `sdkwork-im-cli add-member`
- `sdkwork-im-cli members`
- `sdkwork-im-cli send-message`
- `sdkwork-im-cli timeline`
- `sdkwork-im-cli watch`
- `sdkwork-im-cli chat-session`

### 3.2 包装脚本

- `bin/chat-cli.ps1`
- `bin/chat-cli.sh`
- `bin/chat-cli.cmd`
- `bin/chat-window.ps1`
- `bin/chat-window.sh`
- `bin/chat-window.cmd`
- `bin/chat-window-gui.cmd`
- `bin/open-chat-test.ps1`
- `bin/open-chat-test.sh`
- `bin/open-chat-test.cmd`
- `chat-cli` 包装脚本必须把原�?CLI 参数透传�?`sdkwork-im-cli`
- �?Windows 而言，`bin/chat-cli.cmd --help` 必须保持�?`sdkwork-im-cli --help` 相同�?usage 合同，不能被 wrapper 改写�?`-Help`
- �?Windows 而言，`bin/open-chat-test.cmd` 必须接受�?`bin/open-chat-test.sh` 一致的 GNU-style named flags；`--base-url`、`--conversation-id`、`--owner-user-id`、`--guest-user-id`、`--skip-start`、`--scripted-validation`、`--validation-message`、`--json` 不能静默回落到默认开窗流�?
- �?Windows 而言，`bin/open-chat-test.cmd` �?`--validation-message` 还必须按字面保真传给 `open-chat-test.ps1`；像 `!` 这类消息内容不能�?wrapper 边界被吞�?
- �?Windows 而言，`bin/open-chat-test.cmd --help` 还必须显式展�?`--owner-user-id`、`--scripted-validation`、`--validation-message`、`--json` 这类 GNU-style named flags，不能只暴露 PowerShell 风格参数�?
- �?Windows 而言，`bin/chat-window.cmd` 必须接受�?`bin/chat-window.sh` 一致的 GNU-style named flags；`--base-url`、`--tenant-id`、`--conversation-id`、`--user-id`、`--session-id`、`--device-id`、`--label`、`--message-prefix` 不能�?wrapper 边界上回落到 PowerShell 专用参数�?
- �?Windows 而言，`bin/chat-window.cmd` �?`--message-prefix` 还必须按字面保真传给 `chat-window.ps1`；像 `!` 这类前缀内容不能�?wrapper 边界被吞�?
- �?Windows 而言，`bin/chat-window.cmd --help` 还必须显式展�?`--conversation-id`、`--user-id`、`--message-prefix` 这类 GNU-style named flags，不能只暴露 PowerShell 风格参数�?
- �?Windows 而言，`bin/chat-window-gui.cmd` 也必须接受可�?GUI 启动所需�?GNU-style named flags；`--base-url`、`--tenant-id`、`--conversation-id`、`--user-id`、`--session-id`、`--device-id`、`--label`、`--message-prefix` 不能在参数绑定失败后直接回落�?usage
- �?Windows 而言，`bin/chat-window-gui.cmd --help` 还必须显式展�?`--conversation-id`、`--user-id`、`--message-prefix` 这类 GNU-style named flags，不能只暴露 PowerShell 风格参数�?
- �?Windows 而言，`bin/chat-window-gui.cmd` �?`-Label` / `--label` 还必须按字面保真传给 `chat-window-gui.ps1`；像 `!` 这类会话标签内容不能�?wrapper 边界被吞�?

## 4. 最小验证路�?

### 4.1 生成/查看 AppContext 投影

PowerShell:

```powershell
./bin/chat-cli.ps1 -- --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner token
./bin/chat-cli.ps1 -- --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner token --token-only
./bin/chat-cli.ps1 -- --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner --bearer-token "bearer demo_token" token --token-only
./bin/chat-cli.ps1 -- --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner --bearer-token "Bearer externally_supplied_token" token
cmd /c .\bin\chat-cli.cmd --help
cmd /c .\bin\chat-window-gui.cmd --help
```

### 4.2 HTTP 会话主路�?

PowerShell:

```powershell
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner create-conversation --conversation-id c_cli_demo --conversation-type group
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner add-member --conversation-id c_cli_demo --principal-id u_guest --principal-kind user --role member
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_owner --session-id s_owner --device-id d_owner send-message --conversation-id c_cli_demo --summary "hello from cli" --text "hello from cli" --client-msg-id cli_msg_1
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_guest --session-id s_guest --device-id d_guest timeline --conversation-id c_cli_demo
```

### 4.3 Realtime / CCP 主路�?

当前 `watch` �?`chat-session` 走同一�?realtime 主链路：

- websocket 连接 `/im/v3/api/realtime/ws`
- CCP `hello`
- CCP `auth_bind`
- 服务端返�?`auth_ok`
- 当前默认 `effective snapshot` 只协�?`payload.json`，因此公共主路径会在 `auth_ok` 后直接进�?`realtime.connected`
- 如果后续控制面把 `session.resume` 放入协商结果，CLI 会补充执�?`session_resume -> session_resumed`
- 之后执行 subscription sync、event window、events ack

PowerShell:

```powershell
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_guest --session-id s_guest --device-id d_guest watch --conversation-id c_cli_demo --event-type message.posted --exit-after-events 1 --idle-timeout-seconds 5
./bin/chat-cli.ps1 -- --base-url http://127.0.0.1:18079 --tenant-id t_demo --user-id u_guest --session-id s_guest --device-id d_guest chat-session --conversation-id c_cli_demo --label guest
```

### 4.4 多窗口验�?

PowerShell:

```powershell
./bin/open-chat-test.ps1 -ConversationId c_cli_demo -OwnerUserId u_owner -GuestUserId u_guest
```

`open-chat-test` 会负责：

- 建立 conversation
- 添加第二成员
- 拉起两个 `chat-window` 终端
- 让双端基于同一 `chat-session` 语义做人工收口验�?

### 4.5 可重复执行的 scripted validation

当需要发布前重复执行，而不是人工盯窗口时，`open-chat-test` 还可以直接跑 scripted mode�?

PowerShell:

```powershell
./bin/open-chat-test.ps1 -ConversationId c_cli_demo -OwnerUserId u_owner -OwnerLogin u_owner -OwnerPassword Owner#2026 -GuestUserId u_guest -GuestLogin u_guest -GuestPassword Guest#2026 -ScriptedValidation -ValidationMessage "hello from scripted validation" -Json
```

Bash:

```bash
./bin/open-chat-test.sh --conversation-id c_cli_demo --owner-user-id u_owner --owner-login u_owner --owner-password Owner#2026 --guest-user-id u_guest --guest-login u_guest --guest-password Guest#2026 --scripted-validation --validation-message "hello from scripted validation" --json
```

Windows CMD:

```powershell
cmd /c .\bin\open-chat-test.cmd --base-url http://127.0.0.1:18079 --conversation-id c_cli_demo --owner-user-id u_owner --owner-login u_owner --owner-password Owner#2026 --guest-user-id u_guest --guest-login u_guest --guest-password Guest#2026 --skip-start --scripted-validation --validation-message "hello from scripted validation" --json
```

scripted mode 当前最小保证：

- 仍由 `open-chat-test` 负责�?conversation 与添加第二成�?
- 使用 guest `watch` 观察首条 `message.posted`
- 验证输出中包含：
  - `realtime.connected`
  - `event.window`
- 再用 guest `timeline` 确认验证消息已落�?
- `-Json` / `--json` 下输出可供自动化消费的摘要，而不是只开窗口
  - `watchFrameTypes`
  - `watchDelivered`
  - `timelineContainsValidationMessage`

## 5. compatibility matrix 当前消费边界

当前 CLI 不是 compatibility matrix 的唯一消费者，但它必须与当前控制面口径一致�?

当前最小可信来源：

- `crates/sdkwork-im-ccp-registry/tests/compatibility_matrix_test.rs`
- `services/control-plane-api/tests/protocol_registry_test.rs`
- `services/control-plane-api/tests/protocol_governance_test.rs`
- `docs/部署/性能与灾备演练场�?md`

### 5.1 当前 client row

| clientType | registry bindings | registry codecs | 当前高风�?/ 降级边界 | 当前消费�?|
| --- | --- | --- | --- | --- |
| `web` | `ccp/http/1` `ccp/ws/1` `ccp/sse/1` | `json` | 默认不放开 `agent.tool_call`、`device.signature` | `sdkwork-im-sdk`、`tools/chat-cli` |
| `desktop` | `ccp/http/1` `ccp/ws/1` `ccp/sse/1` | `json` `cbor` | `payload.cbor` 仍可能被 kill switch 收回 | `sdkwork-im-sdk`、`chat-window` |
| `mobile` | `ccp/http/1` `ccp/ws/1` `ccp/sse/1` | `json` `cbor` | `device.signature` 当前仍不在稳定消费面 | `sdkwork-im-sdk` |
| `backend` | `ccp/http/1` `ccp/ws/1` `ccp/sse/1` `ccp/mqtt/1` | `json` `cbor` | `ccp/mqtt/1` �?`payload.cbor` 当前可能�?control-plane kill switch 暂停 | `sdkwork-im-backend-sdk` |

当前 CLI 明确消费的能力包括：

- `ccp/ws/1`
- `payload.json`
- `hello / auth_bind / auth_ok`
- `session.resume / session_resumed` 的按协商启用
- `message.posted` 的实时观察与 ACK

### 5.2 control-plane 映射

兼容矩阵�?`Step 12 / CP12-3` 需要同时落�?registry、控制面和消费者文档�?

当前控制面映射如下：

| 控制面入�?/ 字段 | 当前作用 |
| --- | --- |
| `GET /backend/v3/api/control/protocol-registry` | 返回原始 `compatibilityMatrix` client row |
| `GET /backend/v3/api/control/protocol-governance` | 返回 runtime 实际消费的治理结�?|
| `effectiveSnapshot.allowedBindings` | �?matrix 中可声明�?binding 收口为当前允�?binding |
| `effectiveSnapshot.allowedCodecs` | �?matrix 中可声明�?codec 收口为当前允�?codec |
| `effectiveSnapshot.enabledCapabilities` | �?matrix 中可声明 capability 收口为当前允�?capability |
| `sdkCompatibilityBaseline` | �?`sdkwork-im-sdk` / `sdkwork-im-app-sdk` / `sdkwork-im-backend-sdk` / `sdkwork-rtc-sdk` 对应到当前稳�?`matrixClientTypes` |

当前稳定控制面结论是�?

- registry 可以声明�?
  - `desktop` / `mobile` / `backend` 支持 `cbor`
  - `backend` 支持 `ccp/mqtt/1`
- governance `effectiveSnapshot` 当前会收回：
  - `ccp/mqtt/1`
  - `cbor`
  - `payload.cbor`
- 因此 SDK / CLI 不能只看 registry row，还必须同时服从�?
  - `/backend/v3/api/control/protocol-registry`
  - `/backend/v3/api/control/protocol-governance`

当前不声称已完成的内容：

- 所�?SDK facade 都已消费 compatibility matrix
- �?region rollout orchestration
- 全量 binding / codec 组合�?CLI 支持

### 5.3 close / error registry 恢复基线

当前 CLI / operator �?realtime close 与控制帧的最小恢复口径必须固定为�?

| 场景 | 当前可观察面 | 当前客户端动�?|
| --- | --- | --- |
| `session.disconnect` | 控制�?链路侧会给出 `goaway`，其 message �?`session.disconnect`，随�?websocket close reason 也是 `session.disconnect` | 当前 session 立即失效；不要继续发�?heartbeat 或复用旧 session，必须走 fresh resume fallback，重新建立新�?session 上下�?|
| `realtime.overload` | backlog 仍在 hard limit 以内时，live push 会退化为 pull-only；客户端仍可通过 `events.pull` 拉取 `event.window` | 不要把“暂时收不到 live push”误判为断线；先切到 pull-only，再根据 `event.window` �?`nextAfterSeq` 继续追平 |
| `realtime.overload` 极限�?| �?backlog 超过 disconnect limit，链路可能直接关闭，close reason �?`realtime.overload` | 视为过载切断；重新建链后按当前协商结果走 resume fallback，不要本地硬�?capability 或跳�?catchup |
| `goaway` 通用语义 | `goaway` 是客户端恢复决策的权威前置信号，而不是可忽略日志 | 先消�?`goaway code/message`，再决定�?fresh resume fallback、pull-only，还是彻底重建连�?|

当前仓库里已经被测试冻结的恢复事实包括：

- `session.disconnect` 会先给出 `cc.control.goaway.v1`，其�?message �?`session.disconnect`
- `session.disconnect` 对应�?websocket close code 固定�?`4001`
- `session.disconnect` 对应�?websocket close reason 固定�?`session.disconnect`
- �?session 在断开后继�?heartbeat 会收�?`reconnect_required`
- fresh resume fallback 使用�?session 后，heartbeat 才重新恢复正�?
- overload backlog 会先出现 pull-only 窗口；极限过载时 close reason �?`realtime.overload`

当前最小细粒度恢复词汇必须继续保持一致：

- `4001`
- `session.disconnect`
- `reconnect_required`
- `pull-only`
- `events.pull`

## 6. 当前验证资产

- `tools/chat-cli/tests/chat_cli_e2e_test.rs`
- `tools/chat-cli/tests/chat_cli_contract_test.rs`
- `tools/chat-cli/src/realtime.rs`
- `services/session-gateway/tests/websocket_smoke_test.rs`
- `bin/open-chat-test.*`
- `bin/chat-window.*`
- `README.md`

## 7. 结论

`Step 12 / CP12-1` 的最小闭环不是“CLI 能打�?help”，而是�?

- CLI 主路径与当前权威字段模型一�?
- CLI 不再对未协商 capability 硬猜，当�?public app 的首�?`realtime.connected` 已真实可�?
- `watch` / `chat-session` 能消费当�?realtime / CCP 主链�?
- 文档、包装脚本、测试三者使用同一套命令语�?
