# 用户模块默认运行时提供商选择设计

## 1. 问题

- `PrincipalProfileProvider` 的主链路接入已存在。
- 但默认启动入口此前始终返回 `UpstreamContextPrincipalProfileProvider`。
- 这导致 `principal-profile-external-catalog` 虽有契约、stub 与主链路测试，却没有真实默认装配路径。

## 2. 设计目标

- 默认仍走统一 `PrincipalProfileProvider` port
- 默认值安全：未配置时仍为 `local`
- external 可真实启用，不依赖测试注入
- 主链路输出与现有 external stub 语义一致
- 配置错误可显式暴露

## 3. 方案

### 3.1 Provider 选择

- `CRAW_CHAT_PRINCIPAL_PROFILE_PROVIDER=local|external`
- 缺省为 `local`
- 非法值直接拒绝，避免静默回退掩盖配置错误

### 3.2 External 目录源

- `CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH=<json>`
- `CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM=<name>`
- catalog 最小结构：

```json
{
  "externalSystem": "corp-idp",
  "users": [
    {
      "tenantId": "t_demo",
      "userId": "u_demo",
      "displayName": "Demo User",
      "externalPrincipalId": "ext::u_demo",
      "attributes": {
        "source": "external"
      }
    }
  ]
}
```

### 3.3 行为

- `get_user / batch_get_users / search_users` 先查 override，再查 external catalog
- `map_external_principal` 支持 external principal 到平台用户映射
- `create_or_bind / update / disable` 提供最小 override 能力，保证运行时闭环
- `sender / member / bootstrap member / mutation actor` 继续只消费 `PrincipalProfileProvider`

## 4. 结果

- external 用户目录首次进入默认运行时入口
- 架构声明从“测试注入成立”推进到“默认入口可配置成立”
- 为后续 RTC / Object Storage / IoT 的真实 provider adapter 选择提供一致模板

## 5. 后续

- 把 external catalog 配置补到 operator 文档
- 为 external provider 增加健康检查与配置错误用例
- 继续推进 `rtc-volcengine` 最小真实 adapter
