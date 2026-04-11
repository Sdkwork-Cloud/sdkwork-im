# local-default 发布后验证样本

本文补齐 `local-default` 的最小 post-release 验证样本，避免该 profile 只停留在 profile 名称、模板和帮助面合同。

配套执行记录模板：

- [local-default发布后验证执行记录模板](./local-default发布后验证执行记录模板.md)

配套 machine-readable evidence index：

- `artifacts/releases/wave-d-2026-04-08/local-default-post-release-evidence-index.json`
- `artifactRoot = artifacts/releases/wave-d-2026-04-08/evidence/local-default`

## 1. 当前边界

- 当前 `local-default` 仍复用 `local-minimal` 的 compose 服务合同与 smoke 链路。
- 当前目标不是宣称 `local-default` 已经拥有独立拓扑，而是把“发布后怎么核对”固定为可重复执行的样本。
- 如果后续 `local-default` 迁移为独立编排或独立端口入口，应继续沿用这份样本的结构，而不是重新发明一套验证口径。

## 2. PowerShell post-release 样本

### 2.1 Docker 部署与基础健康

```powershell
./bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090
./bin/status-local.ps1 -ProfileName local-default
tools\smoke\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090
```

预期核对点：

- `deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090`
  - 明确走 `local-default` profile 入口
  - 用显式 `SmokeBaseUrl` 固定发布后 smoke 地址
- `status-local.ps1 -ProfileName local-default`
  - 输出 `profile: local-default`
  - `health:` 指向发布后地址
  - 后续 runtime ops 建议命令保留 `-ProfileName local-default`
- `tools\smoke\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090`
  - 对发布后入口执行独立 public smoke

### 2.2 聊天与运维闭环

```powershell
./bin/open-chat-test.ps1
./bin/inspect-runtime-local.ps1 -ProfileName local-default
./bin/repair-runtime-local.ps1 -ProfileName local-default
./bin/list-runtime-backups-local.ps1 -ProfileName local-default
./bin/archive-runtime-backup-local.ps1 -ProfileName local-default -BackupDir <path>
./bin/preview-runtime-restore-local.ps1 -ProfileName local-default -BackupDir <path>
./bin/restore-runtime-local.ps1 -ProfileName local-default -BackupDir <path> -ExpectedPreviewFingerprint <fingerprint>
```

预期核对点：

- `open-chat-test.ps1`
  - 验证聊天测试窗口仍可正常走本地默认入口
- `inspect-runtime-local.ps1 -ProfileName local-default`
  - 确认 `local-default` 的 config-first / runtime-contract fallback 仍成立
- `repair/archive/preview/restore`
  - 确认 post-release 排障、归档和恢复命令都能复用同一 profile 口径

## 3. Bash post-release 样本

```bash
bash bin/deploy-local.sh --profile local-default --smoke-base-url http://127.0.0.1:28090
bash bin/status-local.sh --profile local-default
tools/smoke/local_stack_smoke.sh --base-url http://127.0.0.1:28090
bash bin/open-chat-test.sh
bash bin/inspect-runtime-local.sh --profile local-default
```

说明：

- `deploy-local.sh --profile local-default --smoke-base-url http://127.0.0.1:28090`
  - 与 PowerShell 样本保持同一发布入口与 smoke 地址
- `status-local.sh --profile local-default`
  - 应输出 `profile: local-default`
- `tools/smoke/local_stack_smoke.sh --base-url http://127.0.0.1:28090`
  - 用于有可用 Bash runtime 的节点补充对称证据

## 4. 建议采集的证据

- `deploy-local` 执行日志
- `status-local` 输出
- `local_stack_smoke` 成功输出
- `open-chat-test` 的窗口联通截图或 operator 记录
- `inspect-runtime-local` 的运行结果
- 若执行了 `archive / preview / restore`，保留目标目录和命令回显
- 对每个 evidence slot，补齐：
  - `artifactPath`
  - `suggestedRelativePath`
  - `collectedAt`
  - `sizeBytes`
  - `checksumSha256`
- `artifactPath` 应落在 `artifactRoot = artifacts/releases/wave-d-2026-04-08/evidence/local-default` 之下
- 默认优先沿用 `suggestedRelativePath` 形成稳定归档命名，只有在实际采集受限时才偏离并备注原因
- `sizeBytes` 记录归档文件的字节数；当前模板态证据索引统一保留 `null`，待真实采集后再回填
- `checksumManifestPath = artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt`
- `checksum-manifest.txt` 建议按 `sha256:<digest>  <suggestedRelativePath>` 逐行归档 bundle 级校验和；当前模板态只冻结路径和格式，不伪造真实 digest
- `artifactFileListPath = artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt`
- `artifact-file-list.txt` 建议统一记录本次 bundle 期望归档的 `suggestedRelativePath` 列表；当前模板态只冻结清单路径和默认内容
- `collectionSummary` 用于机器可读地表达当前 evidence bundle 的完成度；当前模板态样例固定为：
  - `totalSlots = 6`
  - `requiredSlots = 5`
  - `optionalSlots = 1`
  - `collectedSlots = 0`
  - `pendingSlots = 6`
  - `skippedOptionalSlots = 0`
- `collectionSummary` 应直接由 `evidenceSlots[*].required` 与 `evidenceSlots[*].status` 推导，更新 slot 明细时同步回写摘要计数
- 如需统一归档，直接复用 `local-default发布后验证执行记录模板.md`
- 如需结构化归档证据槽位，更新 `local-default-post-release-evidence-index.json`

## 5. 当前结论

- `local-default` 当前仍是兼容基线 profile，不是独立拓扑。
- 这份 post-release 样本的意义，是让 `local-default` 至少具备和 `local-minimal` 对称的发布后核对入口。
- 后续若 `local-default` 独立化，应直接扩充这份样本，而不是替换它。
