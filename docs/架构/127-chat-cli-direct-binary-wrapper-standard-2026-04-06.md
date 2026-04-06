# chat-cli 直连二进制包装标准 2026-04-06

## 目标

统一 `bin/chat-cli.*` 入口的运行时语义，确保：

- 交互聊天窗口不依赖 `cargo run`
- 运行输出只包含业务会话输出，不包含构建器噪音
- 多窗口并发启动时不进入 Cargo 锁竞争路径
- 本地开发与部署交互体验一致

## 适用范围

- `bin/chat-cli.ps1`
- `bin/chat-cli-local.ps1`
- `bin/chat-cli.cmd`
- `bin/chat-cli-local.cmd`
- `bin/chat-cli.sh`
- `bin/chat-cli-local.sh`
- 所有调用上述入口的脚本：
  - `bin/chat-window.ps1`
  - `bin/chat-window-gui.ps1`
  - `bin/open-chat-test.ps1`

## 标准要求

### 1. 运行入口

本地 CLI 包装脚本必须优先执行已构建的 `craw-chat-cli` 二进制：

- Windows debug: `target\\debug\\craw-chat-cli.exe`
- Windows release: `target\\release\\craw-chat-cli.exe`
- Linux/macOS debug: `target/debug/craw-chat-cli`
- Linux/macOS release: `target/release/craw-chat-cli`

### 2. 构建触发条件

只有在目标二进制不存在时，包装脚本才允许触发构建：

- debug: `cargo build -p craw-chat-cli`
- release: `cargo build -p craw-chat-cli --release`

不得在每次执行命令时无条件使用 `cargo run`。

### 3. 输出纯净性

交互命令尤其是 `chat-session` 的标准输出和标准错误输出中，不得包含以下构建器噪音：

- `Finished 'dev' profile`
- `Finished 'release' profile`
- `Running 'target/.../craw-chat-cli'`
- `Blocking waiting for file lock`

### 4. GUI 兼容性

所有 GUI 聊天窗必须依赖纯净 CLI 输出流：

- 子进程 stdout 只能承载聊天会话文本
- 子进程 stderr 只能承载真实错误
- 不允许把 Cargo 构建日志注入 GUI transcript

### 5. 多窗口并发

双窗或多窗测试场景中，所有窗口应共享同一份已构建二进制，而不是每窗触发一次构建流程。

## 验证标准

至少满足以下验证：

1. `cargo test -p craw-chat-cli --offline` 全绿。
2. Windows PowerShell 入口 `bin/chat-cli.ps1` 可完成真实交互发消息。
3. GUI 同构进程桥接 `ProcessStartInfo + RedirectStandardInput/Output` 可成功发消息并写入 timeline。
4. 双窗口可见标题存在，且消息可写入对应 conversation。

## 当前结论

截至 2026-04-06，本标准已在当前仓库落地，`chat-cli` 的本地包装入口已经从 `cargo run` 模式切换到“优先执行已构建二进制、缺失时再构建”的模式。
