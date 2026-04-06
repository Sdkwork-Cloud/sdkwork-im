# GUI 聊天窗 UTF-8 CLI JSON 读取标准 2026-04-06

## 目标

保证 Windows GUI 聊天验证窗读取 IM CLI 输出时：

- 中文 JSON 不乱码
- `ConvertFrom-Json` 可稳定解析
- 脱离式 PowerShell 宿主下行为一致

## 适用范围

- `bin/chat-window-gui.ps1`
- 所有在 WinForms / 脱离式 PowerShell 宿主中读取 `craw-chat-cli` JSON 的脚本

## 标准要求

### 1. JSON 读取入口

GUI 聊天窗必须直接调用已构建的 `craw-chat-cli.exe`，不得再通过 PowerShell 管道抓取 `chat-cli.ps1` 文本输出后解析 JSON。

### 2. 编码要求

读取 CLI 输出时必须显式设置：

- `StandardOutputEncoding = [System.Text.Encoding]::UTF8`
- `StandardErrorEncoding = [System.Text.Encoding]::UTF8`

### 3. 错误实现示例

以下模式不再允许作为 GUI 标准实现：

- `& "$PSScriptRoot\\chat-cli.ps1" ... 2>&1`
- `($output | ForEach-Object { $_.ToString() }) -join ...`
- 直接把上述文本送给 `ConvertFrom-Json`

### 4. 正确实现方向

使用：

- `System.Diagnostics.ProcessStartInfo`
- `UseShellExecute = $false`
- `RedirectStandardOutput = $true`
- `RedirectStandardError = $true`
- UTF-8 明确解码

### 5. 回归约束

至少保留以下约束：

- `chat-window-gui.ps1` 必须包含 `StandardOutputEncoding = [System.Text.Encoding]::UTF8`
- `chat-window-gui.ps1` 必须包含 `StandardErrorEncoding = [System.Text.Encoding]::UTF8`
- `chat-window-gui.ps1` 必须直接调用 `craw-chat-cli.exe`

## 结论

GUI 聊天验证窗本质上是一个 Windows PowerShell 图形宿主，不应依赖 PowerShell 对外部进程标准输出的默认编码推断。凡是读取 IM CLI JSON，必须走 UTF-8 明确读取标准。
