# local-default 发布后验证执行记录模板

本文作为 `local-default` post-release 样本的 operator 填写模板使用。

配套 machine-readable evidence index：

- `artifacts/releases/wave-d-2026-04-08/local-default-post-release-evidence-index.json`
- `artifactRoot = artifacts/releases/wave-d-2026-04-08/evidence/local-default`

## 1. 基本信息

- 验证窗口：
- 执行人：
- 环境：
- profile：`local-default`
- 目标地址：
  - smoke / health base URL：
- 当前结论：`Go / No-Go`

## 2. 执行命令

### 2.1 部署与健康

```powershell
./bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090
./bin/status-local.ps1 -ProfileName local-default
tools\smoke\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090
```

记录：

- deploy-local：
- status-local：
- local_stack_smoke：

### 2.2 聊天与运维

```powershell
./bin/open-chat-test.ps1
./bin/inspect-runtime-local.ps1 -ProfileName local-default
./bin/repair-runtime-local.ps1 -ProfileName local-default
./bin/list-runtime-backups-local.ps1 -ProfileName local-default
```

记录：

- open-chat-test：
- inspect-runtime-local：
- repair-runtime-local：
- list-runtime-backups-local：

## 3. 证据链接

- 截图：
- 日志：
- smoke 输出：
- status 输出：
- 其他证据链接：
- evidence index 更新状态：
- evidence slot metadata：
  - `artifactRoot`：`artifacts/releases/wave-d-2026-04-08/evidence/local-default`
  - `checksumManifestPath`：`artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt`
  - `artifactFileListPath`：`artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt`
  - `collectionSummary`：
    - `totalSlots`：
    - `requiredSlots`：
    - `optionalSlots`：
    - `collectedSlots`：
    - `pendingSlots`：
    - `skippedOptionalSlots`：
  - `artifactPath`：
  - `suggestedRelativePath`：
  - `collectedAt`：
  - `sizeBytes`：
  - `checksumSha256`：
- 记录规则：
  - `checksum-manifest.txt` 建议统一记录 bundle 级校验和清单，单行格式：`sha256:<digest>  <suggestedRelativePath>`
  - `artifact-file-list.txt` 建议统一记录本次 bundle 期望归档的相对文件列表，默认沿用 `suggestedRelativePath`
  - `collectionSummary` 建议与 evidence index 中的 `evidenceSlots[*].required` / `evidenceSlots[*].status` 保持一致，避免人工记录和结构化索引出现计数漂移
  - `artifactPath` 默认应等于 `artifactRoot + "/" + suggestedRelativePath`
  - `sizeBytes` 记录对应归档文件的实际字节数；模板态或未落盘证据保持 `null`
  - 若实际采集路径偏离默认命名，请在备注中写明原因

## 4. 核对清单

- [ ] `deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090` 已执行
- [ ] `status-local.ps1 -ProfileName local-default` 已执行并记录输出
- [ ] `tools\smoke\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090` 已执行
- [ ] `open-chat-test.ps1` 已执行并记录 operator 观察结果
- [ ] `local-default` 当前仍复用 `local-minimal` 服务合同的边界已被确认
- [ ] 关键截图和日志已归档

## 5. 结论

- Go / No-Go：
- 风险说明：
- 后续动作：
