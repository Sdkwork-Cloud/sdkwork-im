# 外部认证�?Trusted Identity 边界标准�?026-04-05�?

## 1. 目标

冻结 `sdkwork-im` 在外部入口与内部可信链路之间的认证边界，避免以下高风险混淆：

- 把测�?内部装配用的 trusted headers 误暴露给公网入口
- �?Bearer/JWT 入口和内�?service-to-service 身份透传混用
- 在部署脚本、README、主程序入口与代码实现之间出现不一�?

## 2. 冻结标准

### 2.1 外部入口

- 所有对�?app-facing 入口默认必须�?`Bearer-only`�?
- 外部入口不得依赖 `x-tenant-id`、`x-user-id`、`x-actor-id` 作为认证来源�?
- 请求中的 `tenant / actor / session / device` 只能来自已校验的 Bearer token�?

### 2.2 内部可信链路

- `trusted identity headers` 仅允许用于内部服务调用、测试装配、或显式声明的可信边界�?
- 内部可信链路仍可继续使用�?
  - `x-tenant-id`
  - `x-user-id` / `x-actor-id`
  - `x-session-id`
  - `x-device-id`
- 但这类模式不得成为公网部署的默认入口�?

### 2.3 代码级约�?

- 需要同时支持两种模式时，必须显式拆�?builder / profile�?
  - `public app`：Bearer-only
  - `default/test app`：可用于测试装配或内部集�?
- `main` 函数必须绑定 `public app`，而不是测试装�?builder�?

## 3. 本轮最小落�?

当前已在 `sdkwork-im-server` 落地以下边界�?

- 新增 `im_auth_context::resolve_bearer_auth_context(...)`
- 新增 `local_minimal_node::build_public_app()`
- `services/sdkwork-im-gateway/src/main.rs` 已改为启�?`build_public_app()`
- `build_public_app()` �?health 以外的入口统一要求 Bearer
- `build_default_app()` 暂保留为测试/内部装配 builder，避免一次性打断现有大量模块级离线测试

## 4. 当前明确不做的事

本轮不强行改动所有服务的默认 builder，原因是�?

- 工作区内大量模块测试仍直接依�?trusted headers
- 这些测试本身并不代表公网部署入口
- 先把真实主入口收敛正确，再逐步推进其它独立服务/二进制的 strict 化，更可�?

## 5. 剩余风险

- 其它单独服务 crate 若被直接作为公网入口部署，当前仍可能保留 trusted headers fallback
- `build_default_app()` 仍是宽松模式，后续必须避免被外部部署误用
- WebSocket 独立部署、control-plane、session-gateway 等二进制入口仍需逐个 review 是否也应切换 public strict builder

## 6. 下一�?

1. 继续 review 其它可独立部署服务的 `main.rs` �?builder，逐个收敛�?strict public builder�?
2. 评估是否�?trusted identity 模式显式命名�?`build_internal_app()`，进一步减少误用�?
3. �?WebSocket 握手、RTC、streaming 外部入口补充 strict auth 回归测试�?
