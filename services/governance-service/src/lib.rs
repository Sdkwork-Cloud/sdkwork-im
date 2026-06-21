use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::{AuditRuntime, RecordAuditAnchor};
use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::{HeaderMap, Request, StatusCode, header::CONTENT_TYPE};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{AppContext, AppContextError, resolve_app_context};
use im_platform_contracts::{
    ContractError, EffectiveProviderBinding, PROVIDER_REGISTRY_INTERFACE_VERSION, ProviderDomain,
    ProviderPolicyCommit, ProviderPolicyDiff, ProviderPolicyHistory, ProviderPolicyPreview,
    ProviderPolicyResultStatus, ProviderRegistry, ProviderRegistrySnapshot,
    RuntimeProviderRegistry,
};
use ops_service::{
    OpsRuntime, ProviderBindingItemView, ProviderBindingSnapshotView, RouteOwnershipView,
};
use sdkwork_im_ccp_registry::{
    BusinessPolicyVocabulary, CapabilityProfile, CcpRegistry, ClientCompatibilityDescriptor,
    EffectiveProtocolSnapshot, KillSwitchRule, ProtocolGovernanceSnapshot, QuotaProfile,
    ReleaseChannel, RolloutPolicy, SchemaDescriptor,
};
use sdkwork_im_openapi::{OpenApiServiceSpec, render_docs_html};
use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};
use session_gateway::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteMigrationResult,
};
use tokio::sync::Semaphore;

const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "SDKWORK_IM_CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS";
const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "SDKWORK_IM_CONTROL_PLANE_MAX_REQUEST_BODY_BYTES";
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Clone)]
pub struct AppState {
    realtime_cluster: Arc<RealtimeClusterBridge>,
    protocol_registry: Arc<CcpRegistry>,
    provider_registry: Arc<dyn ProviderRegistry>,
    provider_registry_runtime: Option<Arc<RuntimeProviderRegistry>>,
    governance_loop: Option<GovernanceLoop>,
}

#[derive(Clone)]
struct GovernanceLoop {
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
}

const CONTROL_PLANE_MAX_ID_BYTES: usize = 256;
static CONTROL_PLANE_AUDIT_RECORD_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MigrateRoutesRequest {
    target_node_id: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingsQuery {
    tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertProviderBindingPolicyRequest {
    tenant_id: Option<String>,
    domain: ProviderDomain,
    plugin_id: String,
    expected_base_version: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyRollbackRequest {
    target_version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyDiffQuery {
    from_version: u64,
    to_version: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolRegistryResponse {
    protocol_version: String,
    bindings: Vec<String>,
    codecs: Vec<String>,
    schemas: Vec<ProtocolSchemaResponse>,
    compatibility_matrix: Vec<ClientCompatibilityResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolGovernanceResponse {
    capability_profile: CapabilityProfileResponse,
    quota_profile: QuotaProfileResponse,
    rollout_policy: RolloutPolicyResponse,
    kill_switch: KillSwitchResponse,
    effective_snapshot: EffectiveProtocolSnapshotResponse,
    business_policy_vocabulary: BusinessPolicyVocabularyResponse,
    sdk_compatibility_baseline: SdkCompatibilityBaselineResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolSchemaResponse {
    schema: String,
    kind: String,
    stage: String,
    binding_protocols: Vec<String>,
    required_capabilities: Vec<String>,
    supported_consumers: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientCompatibilityResponse {
    client_type: String,
    minimum_protocol_version: String,
    supported_bindings: Vec<String>,
    supported_codecs: Vec<String>,
    supported_capabilities: Vec<String>,
    blocked_experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityProfileResponse {
    profile_id: String,
    release_channel: String,
    enabled_capabilities: Vec<String>,
    experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuotaProfileResponse {
    profile_id: String,
    max_concurrent_sessions_per_tenant: u32,
    max_subscriptions_per_session: u32,
    max_inflight_messages: u32,
    max_payload_bytes: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RolloutPolicyResponse {
    policy_id: String,
    release_channel: String,
    traffic_percent: u8,
    cell_selector: String,
    region_selector: String,
    operator_override: bool,
    tenant_allowlist: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KillSwitchResponse {
    rule_id: String,
    active: bool,
    reason: String,
    disabled_capabilities: Vec<String>,
    disabled_bindings: Vec<String>,
    disabled_codecs: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EffectiveProtocolSnapshotResponse {
    protocol_version: String,
    release_channel: String,
    enabled_capabilities: Vec<String>,
    allowed_bindings: Vec<String>,
    allowed_codecs: Vec<String>,
    quota_profile_id: String,
    kill_switch_active: bool,
    precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BusinessPolicyVocabularyResponse {
    policy_version_field: String,
    capability_flags_field: String,
    history_visibility_field: String,
    history_visibility_modes: Vec<String>,
    retention_policy_ref_field: String,
    retention_policy_scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SdkCompatibilityBaselineResponse {
    im_sdk_family: &'static str,
    app_sdk_family: &'static str,
    backend_sdk_family: &'static str,
    rtc_sdk_family: &'static str,
    matrix_client_types: Vec<String>,
    protocol_registry_path: &'static str,
    protocol_governance_path: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingsResponse {
    status: ProviderSurfaceReadStatus,
    interface_version: String,
    tenant_id: Option<String>,
    effective_bindings: Vec<EffectiveProviderBinding>,
    precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingCommitResponse {
    status: ProviderPolicyResultStatus,
    applied: bool,
    interface_version: String,
    tenant_id: Option<String>,
    current_version: u64,
    committed_binding: EffectiveProviderBinding,
    diff: ProviderPolicyDiff,
    effective_bindings: Vec<EffectiveProviderBinding>,
    precedence: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProviderSurfaceReadStatus {
    Registry,
    Bindings,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderRegistrySnapshotResponse {
    status: ProviderSurfaceReadStatus,
    #[serde(flatten)]
    snapshot: ProviderRegistrySnapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProviderPolicyReadStatus {
    History,
    Diff,
    RolledBack,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyHistoryResponse {
    status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    history: ProviderPolicyHistory,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyDiffResponse {
    status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    diff: ProviderPolicyDiff,
}

fn openapi_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string"
    })
}

fn openapi_nullable_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string",
        "nullable": true
    })
}

fn openapi_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64"
    })
}

fn openapi_nullable_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64",
        "nullable": true
    })
}

fn openapi_boolean_schema() -> JsonValue {
    serde_json::json!({
        "type": "boolean"
    })
}

fn openapi_string_array_schema() -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": openapi_string_schema()
    })
}

fn openapi_array_schema(items: JsonValue) -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": items
    })
}

fn openapi_object_schema(
    required: &[&str],
    properties: Vec<(&str, JsonValue)>,
    additional_properties: bool,
) -> JsonValue {
    let mut property_map = JsonMap::new();
    for (name, schema) in properties {
        property_map.insert(name.to_owned(), schema);
    }

    let mut schema = JsonMap::new();
    schema.insert("type".to_owned(), serde_json::json!("object"));
    schema.insert("properties".to_owned(), JsonValue::Object(property_map));
    schema.insert(
        "additionalProperties".to_owned(),
        JsonValue::Bool(additional_properties),
    );
    if !required.is_empty() {
        schema.insert("required".to_owned(), serde_json::json!(required));
    }

    JsonValue::Object(schema)
}

fn openapi_describe(mut schema: JsonValue, description: &str) -> JsonValue {
    if !description.is_empty()
        && let JsonValue::Object(object) = &mut schema
    {
        object.insert(
            "description".to_owned(),
            JsonValue::String(description.to_owned()),
        );
    }
    schema
}

fn openapi_generic_object_schema(description: &str) -> JsonValue {
    openapi_describe(openapi_object_schema(&[], Vec::new(), true), description)
}

fn openapi_component_ref(name: &str) -> JsonValue {
    serde_json::json!({
        "$ref": format!("#/components/schemas/{name}")
    })
}

fn insert_openapi_schema(schemas: &mut JsonMap<String, JsonValue>, name: &str, schema: JsonValue) {
    schemas.insert(name.to_owned(), schema);
}

fn openapi_json_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_problem_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/problem+json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_standard_responses(success_schema_name: &str) -> JsonValue {
    let mut responses = JsonMap::new();
    responses.insert(
        "200".to_owned(),
        openapi_json_response("Successful response.", success_schema_name),
    );

    for (status, description) in [
        ("400", "Invalid request."),
        ("401", "Authentication required."),
        ("403", "Permission denied."),
        ("404", "Resource not found."),
        ("409", "Request conflicts with current control-plane state."),
        ("503", "Control-plane dependency is unavailable."),
    ] {
        responses.insert(
            status.to_owned(),
            openapi_problem_response(description, "ControlPlaneErrorResponse"),
        );
    }

    JsonValue::Object(responses)
}

fn openapi_request_body(schema_name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "required": true,
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_query_parameter(
    name: &str,
    required: bool,
    schema: JsonValue,
    description: &str,
) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "query",
        "required": required,
        "description": description,
        "schema": schema
    })
}

fn openapi_path_parameter(name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "path",
        "required": true,
        "description": description,
        "schema": openapi_string_schema()
    })
}

fn openapi_operation(
    summary: &str,
    operation_id: &str,
    tag: &str,
    parameters: Vec<JsonValue>,
    request_body: Option<JsonValue>,
    response_schema_name: &str,
    secure: bool,
) -> JsonValue {
    let mut operation = JsonMap::new();
    operation.insert("summary".to_owned(), JsonValue::String(summary.to_owned()));
    operation.insert(
        "operationId".to_owned(),
        JsonValue::String(operation_id.to_owned()),
    );
    operation.insert("tags".to_owned(), serde_json::json!([tag]));
    operation.insert(
        "responses".to_owned(),
        openapi_standard_responses(response_schema_name),
    );
    if secure {
        operation.insert(
            "security".to_owned(),
            serde_json::json!([{ "AuthToken": [], "AccessToken": [] }]),
        );
    } else {
        operation.insert("security".to_owned(), serde_json::json!([]));
    }
    if !parameters.is_empty() {
        operation.insert("parameters".to_owned(), JsonValue::Array(parameters));
    }
    if let Some(request_body) = request_body {
        operation.insert("requestBody".to_owned(), request_body);
    }

    JsonValue::Object(operation)
}

fn control_plane_openapi_components() -> JsonValue {
    let mut schemas = JsonMap::new();

    insert_openapi_schema(
        &mut schemas,
        "HealthResponse",
        openapi_object_schema(
            &["status", "service"],
            vec![
                ("status", openapi_string_schema()),
                ("service", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ControlPlaneErrorResponse",
        openapi_object_schema(
            &["type", "title", "status"],
            vec![
                (
                    "type",
                    serde_json::json!({
                        "type": "string",
                        "format": "uri-reference"
                    }),
                ),
                ("title", openapi_string_schema()),
                (
                    "status",
                    serde_json::json!({
                        "type": "integer",
                        "minimum": 100,
                        "maximum": 599
                    }),
                ),
                ("detail", openapi_string_schema()),
                ("instance", openapi_string_schema()),
                ("code", openapi_string_schema()),
                ("message", openapi_string_schema()),
                ("errorStatus", openapi_string_schema()),
                (
                    "details",
                    serde_json::json!({
                        "type": "object",
                        "nullable": true,
                        "additionalProperties": true
                    }),
                ),
            ],
            true,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolSchemaResponse",
        openapi_object_schema(
            &[
                "schema",
                "kind",
                "stage",
                "bindingProtocols",
                "requiredCapabilities",
                "supportedConsumers",
            ],
            vec![
                ("schema", openapi_string_schema()),
                ("kind", openapi_string_schema()),
                ("stage", openapi_string_schema()),
                ("bindingProtocols", openapi_string_array_schema()),
                ("requiredCapabilities", openapi_string_array_schema()),
                ("supportedConsumers", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ClientCompatibilityResponse",
        openapi_object_schema(
            &[
                "clientType",
                "minimumProtocolVersion",
                "supportedBindings",
                "supportedCodecs",
                "supportedCapabilities",
                "blockedExperimentalCapabilities",
            ],
            vec![
                ("clientType", openapi_string_schema()),
                ("minimumProtocolVersion", openapi_string_schema()),
                ("supportedBindings", openapi_string_array_schema()),
                ("supportedCodecs", openapi_string_array_schema()),
                ("supportedCapabilities", openapi_string_array_schema()),
                (
                    "blockedExperimentalCapabilities",
                    openapi_string_array_schema(),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolRegistryResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "bindings",
                "codecs",
                "schemas",
                "compatibilityMatrix",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("bindings", openapi_string_array_schema()),
                ("codecs", openapi_string_array_schema()),
                (
                    "schemas",
                    openapi_array_schema(openapi_component_ref("ProtocolSchemaResponse")),
                ),
                (
                    "compatibilityMatrix",
                    openapi_array_schema(openapi_component_ref("ClientCompatibilityResponse")),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "CapabilityProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "releaseChannel",
                "enabledCapabilities",
                "experimentalCapabilities",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("experimentalCapabilities", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "QuotaProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "maxConcurrentSessionsPerTenant",
                "maxSubscriptionsPerSession",
                "maxInflightMessages",
                "maxPayloadBytes",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("maxConcurrentSessionsPerTenant", openapi_integer_schema()),
                ("maxSubscriptionsPerSession", openapi_integer_schema()),
                ("maxInflightMessages", openapi_integer_schema()),
                ("maxPayloadBytes", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RolloutPolicyResponse",
        openapi_object_schema(
            &[
                "policyId",
                "releaseChannel",
                "trafficPercent",
                "cellSelector",
                "regionSelector",
                "operatorOverride",
                "tenantAllowlist",
            ],
            vec![
                ("policyId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("trafficPercent", openapi_integer_schema()),
                ("cellSelector", openapi_string_schema()),
                ("regionSelector", openapi_string_schema()),
                ("operatorOverride", openapi_boolean_schema()),
                ("tenantAllowlist", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "KillSwitchResponse",
        openapi_object_schema(
            &[
                "ruleId",
                "active",
                "reason",
                "disabledCapabilities",
                "disabledBindings",
                "disabledCodecs",
            ],
            vec![
                ("ruleId", openapi_string_schema()),
                ("active", openapi_boolean_schema()),
                ("reason", openapi_string_schema()),
                ("disabledCapabilities", openapi_string_array_schema()),
                ("disabledBindings", openapi_string_array_schema()),
                ("disabledCodecs", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "EffectiveProtocolSnapshotResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "releaseChannel",
                "enabledCapabilities",
                "allowedBindings",
                "allowedCodecs",
                "quotaProfileId",
                "killSwitchActive",
                "precedence",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("allowedBindings", openapi_string_array_schema()),
                ("allowedCodecs", openapi_string_array_schema()),
                ("quotaProfileId", openapi_string_schema()),
                ("killSwitchActive", openapi_boolean_schema()),
                ("precedence", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "BusinessPolicyVocabularyResponse",
        openapi_object_schema(
            &[
                "policyVersionField",
                "capabilityFlagsField",
                "historyVisibilityField",
                "historyVisibilityModes",
                "retentionPolicyRefField",
                "retentionPolicyScopes",
            ],
            vec![
                ("policyVersionField", openapi_string_schema()),
                ("capabilityFlagsField", openapi_string_schema()),
                ("historyVisibilityField", openapi_string_schema()),
                ("historyVisibilityModes", openapi_string_array_schema()),
                ("retentionPolicyRefField", openapi_string_schema()),
                ("retentionPolicyScopes", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "SdkCompatibilityBaselineResponse",
        openapi_object_schema(
            &[
                "imSdkFamily",
                "appSdkFamily",
                "backendSdkFamily",
                "rtcSdkFamily",
                "matrixClientTypes",
                "protocolRegistryPath",
                "protocolGovernancePath",
            ],
            vec![
                ("imSdkFamily", openapi_string_schema()),
                ("appSdkFamily", openapi_string_schema()),
                ("backendSdkFamily", openapi_string_schema()),
                ("rtcSdkFamily", openapi_string_schema()),
                ("matrixClientTypes", openapi_string_array_schema()),
                ("protocolRegistryPath", openapi_string_schema()),
                ("protocolGovernancePath", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolGovernanceResponse",
        openapi_object_schema(
            &[
                "capabilityProfile",
                "quotaProfile",
                "rolloutPolicy",
                "killSwitch",
                "effectiveSnapshot",
                "businessPolicyVocabulary",
                "sdkCompatibilityBaseline",
            ],
            vec![
                (
                    "capabilityProfile",
                    openapi_component_ref("CapabilityProfileResponse"),
                ),
                (
                    "quotaProfile",
                    openapi_component_ref("QuotaProfileResponse"),
                ),
                (
                    "rolloutPolicy",
                    openapi_component_ref("RolloutPolicyResponse"),
                ),
                ("killSwitch", openapi_component_ref("KillSwitchResponse")),
                (
                    "effectiveSnapshot",
                    openapi_component_ref("EffectiveProtocolSnapshotResponse"),
                ),
                (
                    "businessPolicyVocabulary",
                    openapi_component_ref("BusinessPolicyVocabularyResponse"),
                ),
                (
                    "sdkCompatibilityBaseline",
                    openapi_component_ref("SdkCompatibilityBaselineResponse"),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteNodeLifecycle",
        openapi_object_schema(
            &["nodeId", "drainStatus", "rebalanceState", "ownedRouteCount"],
            vec![
                ("nodeId", openapi_string_schema()),
                ("drainStatus", openapi_string_schema()),
                ("rebalanceState", openapi_string_schema()),
                ("ownedRouteCount", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteMigrationResult",
        openapi_object_schema(
            &[
                "sourceNodeId",
                "targetNodeId",
                "migratedRouteCount",
                "sourceDrainStatus",
                "sourceRebalanceState",
                "targetDrainStatus",
                "targetRebalanceState",
            ],
            vec![
                ("sourceNodeId", openapi_string_schema()),
                ("targetNodeId", openapi_string_schema()),
                ("migratedRouteCount", openapi_integer_schema()),
                ("sourceDrainStatus", openapi_string_schema()),
                ("sourceRebalanceState", openapi_string_schema()),
                ("targetDrainStatus", openapi_string_schema()),
                ("targetRebalanceState", openapi_string_schema()),
            ],
            false,
        ),
    );

    for (name, description) in [
        (
            "ProviderRegistrySnapshotResponse",
            "Provider registry snapshot for the current control-plane view.",
        ),
        (
            "ProviderBindingsResponse",
            "Effective provider bindings resolved for the current tenant scope.",
        ),
        (
            "ProviderBindingCommitResponse",
            "Provider binding mutation result after applying a control-plane policy change.",
        ),
        (
            "ProviderPolicyHistoryResponse",
            "Provider policy history snapshot for the current tenant scope.",
        ),
        (
            "ProviderPolicyDiffResponse",
            "Provider policy diff between two committed versions.",
        ),
    ] {
        insert_openapi_schema(
            &mut schemas,
            name,
            openapi_generic_object_schema(description),
        );
    }

    for (name, schema) in [
        (
            "UpsertProviderBindingPolicyRequest",
            openapi_object_schema(
                &["domain", "pluginId"],
                vec![
                    ("tenantId", openapi_nullable_string_schema()),
                    ("domain", openapi_string_schema()),
                    ("pluginId", openapi_string_schema()),
                    ("expectedBaseVersion", openapi_nullable_integer_schema()),
                ],
                false,
            ),
        ),
        (
            "ProviderPolicyRollbackRequest",
            openapi_object_schema(
                &["targetVersion"],
                vec![("targetVersion", openapi_integer_schema())],
                false,
            ),
        ),
        (
            "MigrateRoutesRequest",
            openapi_object_schema(
                &["targetNodeId"],
                vec![("targetNodeId", openapi_string_schema())],
                false,
            ),
        ),
    ] {
        insert_openapi_schema(&mut schemas, name, schema);
    }

    serde_json::json!({
        "securitySchemes": {
            "AuthToken": {
                "type": "http",
                "scheme": "bearer",
                "bearerFormat": "JWT"
            },
            "AccessToken": {
                "type": "apiKey",
                "in": "header",
                "name": "Access-Token"
            }
        },
        "schemas": JsonValue::Object(schemas)
    })
}

fn control_plane_openapi_paths() -> JsonValue {
    let mut paths = JsonMap::new();

    paths.insert(
        "/healthz".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Check control-plane process health.",
                "getHealthz",
                "meta",
                Vec::new(),
                None,
                "HealthResponse",
                false
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol registry snapshot.",
                "getProtocolRegistry",
                "protocol",
                Vec::new(),
                None,
                "ProtocolRegistryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_governance".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol governance snapshot.",
                "getProtocolGovernance",
                "protocol",
                Vec::new(),
                None,
                "ProtocolGovernanceResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the provider registry snapshot.",
                "getProviderRegistry",
                "providers",
                Vec::new(),
                None,
                "ProviderRegistrySnapshotResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_bindings".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read effective provider bindings.",
                "getProviderBindings",
                "providers",
                vec![
                    openapi_query_parameter(
                        "tenantId",
                        false,
                        openapi_string_schema(),
                        "Optional tenant scope for effective provider bindings."
                    )
                ],
                None,
                "ProviderBindingsResponse",
                true
            ),
            "post": openapi_operation(
                "Upsert a provider binding policy.",
                "upsertProviderBindingPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding mutation payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy history.",
                "getProviderPolicyHistory",
                "providers",
                Vec::new(),
                None,
                "ProviderPolicyHistoryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/diff".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy diff between two versions.",
                "getProviderPolicyDiff",
                "providers",
                vec![
                    openapi_query_parameter(
                        "fromVersion",
                        true,
                        openapi_integer_schema(),
                        "Base provider policy version."
                    ),
                    openapi_query_parameter(
                        "toVersion",
                        true,
                        openapi_integer_schema(),
                        "Target provider policy version."
                    )
                ],
                None,
                "ProviderPolicyDiffResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/preview".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Preview the effective provider policy result before commit.",
                "previewProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding preview payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/rollback".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Rollback provider policy history to a target version.",
                "rollbackProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "ProviderPolicyRollbackRequest",
                    "Provider policy rollback payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );

    for (path, operation_id, summary) in [
        (
            "/backend/v3/api/control/nodes/{node_id}/drain",
            "drainNode",
            "Mark a realtime node as draining.",
        ),
        (
            "/backend/v3/api/control/nodes/{node_id}/activate",
            "activateNode",
            "Activate a realtime node and clear drain state.",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "nodes",
                    vec![openapi_path_parameter("node_id", "Realtime node identifier.")],
                    None,
                    "RouteNodeLifecycle",
                    true
                )
            }),
        );
    }

    paths.insert(
        "/backend/v3/api/control/nodes/{node_id}/routes/migrate".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Migrate owned routes from the source node to the target node.",
                "migrateNodeRoutes",
                "nodes",
                vec![openapi_path_parameter("node_id", "Source realtime node identifier.")],
                Some(openapi_request_body(
                    "MigrateRoutesRequest",
                    "Route migration target payload."
                )),
                "RouteMigrationResult",
                true
            )
        }),
    );

    JsonValue::Object(paths)
}

fn control_plane_openapi_document() -> JsonValue {
    serde_json::json!({
        "openapi": "3.1.2",
        "info": {
            "title": "Control Plane API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Live OpenAPI contract for the control-plane runtime. This document is emitted by the running service and is intended to be captured into the admin SDK workspace before generation."
        },
        "servers": [
            {
                "url": "/"
            }
        ],
        "tags": [
            {
                "name": "meta",
                "description": "Service metadata and runtime health endpoints."
            },
            {
                "name": "protocol",
                "description": "Protocol registry and protocol governance surfaces."
            },
            {
                "name": "providers",
                "description": "Provider registry, binding, history, diff, preview, and rollback surfaces."
            },
            {
                "name": "nodes",
                "description": "Realtime node lifecycle and route migration control surfaces."
            }
        ],
        "components": control_plane_openapi_components(),
        "paths": control_plane_openapi_paths()
    })
}

pub fn render_openapi_document() -> serde_json::Value {
    control_plane_openapi_document()
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ControlPlaneErrorStatus {
    Unauthorized,
    Forbidden,
    Invalid,
    Conflict,
    NotFound,
    Unavailable,
}

#[derive(Debug)]
struct ControlPlaneError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<serde_json::Value>,
}

impl From<RealtimeClusterError> for ControlPlaneError {
    fn from(value: RealtimeClusterError) -> Self {
        let status = match value.code {
            "node_not_found" | "target_node_not_found" | "node_runtime_missing" => {
                StatusCode::NOT_FOUND
            }
            "same_node_migration"
            | "node_not_draining"
            | "target_node_unavailable"
            | "node_draining" => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
            details: None,
        }
    }
}

impl From<AppContextError> for ControlPlaneError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
            details: None,
        }
    }
}

impl From<ContractError> for ControlPlaneError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_provider_policy",
                message,
                details: None,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "provider_policy_conflict",
                message,
                details: None,
            },
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_policy_unavailable",
                message,
                details: None,
            },
        }
    }
}

impl ControlPlaneError {
    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
            details: None,
        }
    }

    fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
            details: None,
        }
    }

    fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn response_status(status: StatusCode) -> ControlPlaneErrorStatus {
        match status {
            StatusCode::UNAUTHORIZED => ControlPlaneErrorStatus::Unauthorized,
            StatusCode::FORBIDDEN => ControlPlaneErrorStatus::Forbidden,
            StatusCode::CONFLICT => ControlPlaneErrorStatus::Conflict,
            StatusCode::NOT_FOUND => ControlPlaneErrorStatus::NotFound,
            StatusCode::SERVICE_UNAVAILABLE => ControlPlaneErrorStatus::Unavailable,
            _ => ControlPlaneErrorStatus::Invalid,
        }
    }
}

impl axum::response::IntoResponse for ControlPlaneError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let response_status = Self::response_status(status);
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        let mut body = serde_json::json!({
            "type": "about:blank",
            "title": title,
            "status": status.as_u16(),
            "detail": detail,
            "code": self.code,
            "message": message,
            "errorStatus": response_status
        });
        if let Some(details) = self.details {
            body["details"] = details;
        }
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(body),
        )
            .into_response()
    }
}

pub fn build_app() -> Router {
    build_app_with_cluster(Arc::new(RealtimeClusterBridge::default()))
}

pub fn build_public_app() -> Router {
    apply_public_http_guardrails(build_app())
}

pub fn default_control_state() -> AppState {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    AppState {
        realtime_cluster: Arc::new(RealtimeClusterBridge::default()),
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    }
}

pub fn build_domain_api_router(state: AppState) -> Router {
    build_control_surface_with_state(state)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn export_openapi_document() -> Result<serde_json::Value, String> {
    Ok(control_plane_openapi_document())
}

pub fn export_openapi_spec() -> OpenApiServiceSpec<'static> {
    control_plane_openapi_spec()
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_runtime_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
    })
}

pub fn build_app_with_cluster_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_control_surface_with_cluster_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_app_with_cluster_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

pub fn build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
    })
}

fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/openapi.json", get(openapi_document))
        .route(
            "/backend/v3/api/control/openapi.json",
            get(openapi_document),
        )
        .route("/docs", get(docs))
        .merge(build_control_surface_with_state(state))
}

fn build_control_surface_with_state(state: AppState) -> Router {
    Router::new()
        .route(
            "/backend/v3/api/control/protocol_registry",
            get(protocol_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/protocol_governance",
            get(protocol_governance_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_registry",
            get(provider_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_bindings",
            get(provider_bindings_snapshot).post(upsert_provider_binding_policy),
        )
        .route(
            "/backend/v3/api/control/provider_policies",
            get(provider_policy_history),
        )
        .route(
            "/backend/v3/api/control/provider_policies/diff",
            get(provider_policy_diff),
        )
        .route(
            "/backend/v3/api/control/provider_policies/preview",
            post(provider_policy_preview),
        )
        .route(
            "/backend/v3/api/control/provider_policies/rollback",
            post(rollback_provider_policy),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/drain",
            post(drain_node),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/activate",
            post(activate_node),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/routes/migrate",
            post(migrate_node_routes),
        )
        .with_state(state)
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/openapi.json" | "/backend/v3/api/control/openapi.json" | "/docs" => {
            next.run(request).await
        }
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return ControlPlaneError::service_unavailable(
                        "http_overloaded",
                        "server is at maximum in-flight request capacity, please retry later",
                    )
                    .into_response();
                }
            };
            let response = next.run(request).await;
            drop(permit);
            response
        }
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "governance-service",
    })
}

async fn openapi_document() -> Json<JsonValue> {
    Json(control_plane_openapi_document())
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&control_plane_openapi_spec()))
}

fn control_plane_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Control Plane API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Detailed OpenAPI contract for the Sdkwork IM control-plane runtime, including protocol governance, provider policy, and node lifecycle operations.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

async fn protocol_registry_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolRegistryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    Ok(Json(ProtocolRegistryResponse {
        protocol_version: state.protocol_registry.protocol_version().to_owned(),
        bindings: state.protocol_registry.bindings().iter().cloned().collect(),
        codecs: state.protocol_registry.codecs().iter().cloned().collect(),
        schemas: state
            .protocol_registry
            .schemas()
            .values()
            .map(schema_response)
            .collect(),
        compatibility_matrix: state
            .protocol_registry
            .compatibility_matrix()
            .values()
            .map(compatibility_response)
            .collect(),
    }))
}

async fn protocol_governance_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolGovernanceResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let governance = state
        .protocol_registry
        .governance_snapshot()
        .ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "protocol_governance_unavailable",
                "control plane governance snapshot is not initialized",
            )
        })?;

    Ok(Json(governance_response(
        governance,
        state.protocol_registry.as_ref(),
    )))
}

async fn provider_registry_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderRegistrySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    Ok(Json(provider_registry_snapshot_response(
        state.provider_registry.snapshot(),
    )))
}

async fn provider_bindings_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderBindingsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(query.tenant_id)?;

    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
    mirror_provider_bindings_into_ops_runtime(&state, &response);

    Ok(Json(response))
}

async fn upsert_provider_binding_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderBindingCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_write_unavailable",
            "control plane provider policy write is not enabled for this registry",
        )
    })?;

    let (action, aggregate_id, selection_source, commit) =
        if let Some(tenant_id) = tenant_id.as_deref() {
            let commit = provider_registry.commit_upsert(
                Some(tenant_id),
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_tenant_override_updated",
                format!(
                    "tenant:{tenant_id}:{}",
                    provider_domain_name(request.domain)
                ),
                "tenant_override",
                commit,
            )
        } else {
            let commit = provider_registry.commit_upsert(
                None,
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_deployment_profile_updated",
                format!("deployment:{}", provider_domain_name(request.domain)),
                "deployment_profile",
                commit,
            )
        };

    if commit.applied {
        mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    }
    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
    if commit.applied {
        record_control_plane_audit(
            &state,
            &auth,
            action,
            "provider_policy",
            aggregate_id,
            serde_json::json!({
                "tenantId": response.tenant_id,
                "domain": provider_domain_name(request.domain),
                "pluginId": request.plugin_id,
                "expectedBaseVersion": request.expected_base_version,
                "currentVersion": commit.current_version,
                "selectionSource": selection_source
            }),
        );
    }

    Ok(Json(provider_binding_commit_response(response, commit)))
}

async fn provider_policy_history(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_history_unavailable",
            "control plane provider policy history is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::History,
        provider_registry.policy_history(),
    )))
}

async fn provider_policy_diff(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderPolicyDiffQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyDiffResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_diff_unavailable",
            "control plane provider policy diff is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_diff_response(
        ProviderPolicyReadStatus::Diff,
        provider_registry.diff_versions(query.from_version, query.to_version)?,
    )))
}

async fn provider_policy_preview(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderPolicyPreview>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_preview_unavailable",
            "control plane provider policy preview is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_registry.preview_upsert(
        tenant_id.as_deref(),
        request.domain,
        request.plugin_id.as_str(),
    )?))
}

async fn rollback_provider_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ProviderPolicyRollbackRequest>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_rollback_unavailable",
            "control plane provider policy rollback is not enabled for this registry",
        )
    })?;

    let rollback_snapshot = provider_registry.rollback_to(request.target_version)?;
    mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    let history = provider_registry.policy_history();
    record_control_plane_audit(
        &state,
        &auth,
        "control.provider_policy_rolled_back",
        "provider_policy",
        format!("version:{}", rollback_snapshot.version),
        serde_json::json!({
            "targetVersion": request.target_version,
            "currentVersion": history.current_version,
            "rollbackFromVersion": rollback_snapshot.rollback_from_version
        }),
    );

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::RolledBack,
        history,
    )))
}

async fn drain_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state
        .realtime_cluster
        .mark_node_draining(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_draining_marked",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

async fn activate_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state.realtime_cluster.activate_node(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_activated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

async fn migrate_node_routes(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Result<Json<RealtimeRouteMigrationResult>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let migration = state
        .realtime_cluster
        .migrate_node_routes(node_id.as_str(), request.target_node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    mirror_node_into_ops_runtime(&state, request.target_node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_routes_migrated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "sourceNodeId": migration.source_node_id,
            "targetNodeId": migration.target_node_id,
            "migratedRouteCount": migration.migrated_route_count,
            "sourceDrainStatus": migration.source_drain_status,
            "sourceRebalanceState": migration.source_rebalance_state,
            "targetDrainStatus": migration.target_drain_status,
            "targetRebalanceState": migration.target_rebalance_state
        }),
    );
    Ok(Json(migration))
}

fn mirror_node_into_ops_runtime(state: &AppState, node_id: &str) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    if governance_loop.ops_runtime.node_id() != node_id {
        return;
    }

    let Some(lifecycle) = state.realtime_cluster.node_lifecycle(node_id) else {
        return;
    };
    governance_loop.ops_runtime.set_node_lifecycle(
        lifecycle.drain_status.as_str(),
        lifecycle.rebalance_state.as_str(),
    );
    governance_loop.ops_runtime.update_route_ownership(
        state
            .realtime_cluster
            .routes_for_node(node_id)
            .into_iter()
            .map(|route| RouteOwnershipView {
                tenant_id: route.tenant_id,
                principal_id: route.principal_id,
                device_id: route.device_id,
                owner_node_id: route.owner_node_id,
                connection_kind: route.connection_kind,
                bound_at: route.bound_at,
            })
            .collect(),
    );
}

fn mirror_provider_bindings_into_ops_runtime(
    state: &AppState,
    response: &ProviderBindingsResponse,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    governance_loop
        .ops_runtime
        .update_provider_binding_snapshot(provider_binding_snapshot_view(response));
}

fn mirror_all_provider_bindings_into_ops_runtime(
    state: &AppState,
    provider_registry: &RuntimeProviderRegistry,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    let mut snapshots = vec![provider_binding_snapshot_view(&provider_bindings_response(
        provider_registry,
        None,
    ))];
    let mut tenant_ids = provider_registry.tenant_ids_with_overrides();
    tenant_ids.sort();
    snapshots.extend(tenant_ids.into_iter().map(|tenant_id| {
        provider_binding_snapshot_view(&provider_bindings_response(
            provider_registry,
            Some(tenant_id),
        ))
    }));
    governance_loop
        .ops_runtime
        .replace_provider_binding_snapshots(snapshots);
}

fn record_control_plane_audit(
    state: &AppState,
    auth: &AppContext,
    action: &str,
    aggregate_type: &str,
    aggregate_id: String,
    payload: serde_json::Value,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    let record_id = control_plane_audit_record_id();
    let payload =
        serde_json::to_string(&payload).expect("control plane audit payload should serialize");
    if let Err(error) = governance_loop.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id,
            aggregate_type: aggregate_type.into(),
            aggregate_id,
            action: action.into(),
            payload: Some(payload),
        },
    ) {
        tracing::warn!("control-plane audit write failed for {aggregate_type}/{action}: {error:?}");
    }
}

fn control_plane_audit_record_id() -> String {
    let recorded_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let sequence = CONTROL_PLANE_AUDIT_RECORD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("control-audit-{recorded_nanos:x}-{sequence:x}")
}

fn provider_bindings_response(
    provider_registry: &dyn ProviderRegistry,
    tenant_id: Option<String>,
) -> ProviderBindingsResponse {
    let precedence = provider_registry.snapshot().precedence;
    let effective_bindings = ProviderDomain::ALL
        .into_iter()
        .filter_map(|domain| provider_registry.effective_binding(domain, tenant_id.as_deref()))
        .collect();
    ProviderBindingsResponse {
        status: ProviderSurfaceReadStatus::Bindings,
        interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
        tenant_id,
        effective_bindings,
        precedence,
    }
}

fn provider_binding_commit_response(
    response: ProviderBindingsResponse,
    commit: ProviderPolicyCommit,
) -> ProviderBindingCommitResponse {
    ProviderBindingCommitResponse {
        status: commit.status,
        applied: commit.applied,
        interface_version: response.interface_version,
        tenant_id: response.tenant_id,
        current_version: commit.current_version,
        committed_binding: commit.committed_binding,
        diff: commit.diff,
        effective_bindings: response.effective_bindings,
        precedence: response.precedence,
    }
}

fn provider_registry_snapshot_response(
    snapshot: ProviderRegistrySnapshot,
) -> ProviderRegistrySnapshotResponse {
    ProviderRegistrySnapshotResponse {
        status: ProviderSurfaceReadStatus::Registry,
        snapshot,
    }
}

fn provider_policy_history_response(
    status: ProviderPolicyReadStatus,
    history: ProviderPolicyHistory,
) -> ProviderPolicyHistoryResponse {
    ProviderPolicyHistoryResponse { status, history }
}

fn provider_policy_diff_response(
    status: ProviderPolicyReadStatus,
    diff: ProviderPolicyDiff,
) -> ProviderPolicyDiffResponse {
    ProviderPolicyDiffResponse { status, diff }
}

fn provider_binding_snapshot_view(
    response: &ProviderBindingsResponse,
) -> ProviderBindingSnapshotView {
    ProviderBindingSnapshotView {
        interface_version: response.interface_version.clone(),
        tenant_id: response.tenant_id.clone(),
        effective_bindings: response
            .effective_bindings
            .iter()
            .map(|binding| ProviderBindingItemView {
                domain: provider_domain_name(binding.domain).into(),
                default_plugin_id: binding.default_plugin_id.clone(),
                selected_plugin_id: binding.selected_plugin_id.clone(),
                selection_source: binding.selection_source.clone(),
                tenant_override_allowed: binding.tenant_override_allowed,
            })
            .collect(),
        precedence: response.precedence.clone(),
    }
}

fn ensure_control_write_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.write"))
}

fn ensure_control_read_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.read") || auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.read"))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, ControlPlaneError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(ControlPlaneError::from),
    }
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX)
}

fn schema_response(schema: &SchemaDescriptor) -> ProtocolSchemaResponse {
    ProtocolSchemaResponse {
        schema: schema.schema.clone(),
        kind: schema.kind.clone(),
        stage: schema.stage.as_str().to_owned(),
        binding_protocols: schema.binding_protocols.iter().cloned().collect(),
        required_capabilities: schema.required_capabilities.iter().cloned().collect(),
        supported_consumers: schema.supported_consumers.iter().cloned().collect(),
    }
}

fn compatibility_response(
    descriptor: &ClientCompatibilityDescriptor,
) -> ClientCompatibilityResponse {
    ClientCompatibilityResponse {
        client_type: descriptor.client_type.clone(),
        minimum_protocol_version: descriptor.minimum_protocol_version.clone(),
        supported_bindings: descriptor.supported_bindings.iter().cloned().collect(),
        supported_codecs: descriptor.supported_codecs.iter().cloned().collect(),
        supported_capabilities: descriptor.supported_capabilities.iter().cloned().collect(),
        blocked_experimental_capabilities: descriptor
            .blocked_experimental_capabilities
            .iter()
            .cloned()
            .collect(),
    }
}

fn governance_response(
    governance: &ProtocolGovernanceSnapshot,
    registry: &CcpRegistry,
) -> ProtocolGovernanceResponse {
    ProtocolGovernanceResponse {
        capability_profile: capability_profile_response(&governance.capability_profile),
        quota_profile: quota_profile_response(&governance.quota_profile),
        rollout_policy: rollout_policy_response(&governance.rollout_policy),
        kill_switch: kill_switch_response(&governance.kill_switch),
        effective_snapshot: effective_snapshot_response(&governance.effective_snapshot),
        business_policy_vocabulary: business_policy_vocabulary_response(
            &governance.business_policy_vocabulary,
        ),
        sdk_compatibility_baseline: sdk_compatibility_baseline_response(registry),
    }
}

fn capability_profile_response(profile: &CapabilityProfile) -> CapabilityProfileResponse {
    CapabilityProfileResponse {
        profile_id: profile.profile_id.clone(),
        release_channel: release_channel(profile.release_channel.clone()).to_owned(),
        enabled_capabilities: profile.enabled_capabilities.iter().cloned().collect(),
        experimental_capabilities: profile.experimental_capabilities.iter().cloned().collect(),
    }
}

fn quota_profile_response(profile: &QuotaProfile) -> QuotaProfileResponse {
    QuotaProfileResponse {
        profile_id: profile.profile_id.clone(),
        max_concurrent_sessions_per_tenant: profile.max_concurrent_sessions_per_tenant,
        max_subscriptions_per_session: profile.max_subscriptions_per_session,
        max_inflight_messages: profile.max_inflight_messages,
        max_payload_bytes: profile.max_payload_bytes,
    }
}

fn rollout_policy_response(policy: &RolloutPolicy) -> RolloutPolicyResponse {
    RolloutPolicyResponse {
        policy_id: policy.policy_id.clone(),
        release_channel: release_channel(policy.release_channel.clone()).to_owned(),
        traffic_percent: policy.traffic_percent,
        cell_selector: policy.cell_selector.clone(),
        region_selector: policy.region_selector.clone(),
        operator_override: policy.operator_override,
        tenant_allowlist: policy.tenant_allowlist.iter().cloned().collect(),
    }
}

fn kill_switch_response(kill_switch: &KillSwitchRule) -> KillSwitchResponse {
    KillSwitchResponse {
        rule_id: kill_switch.rule_id.clone(),
        active: kill_switch.active,
        reason: kill_switch.reason.clone(),
        disabled_capabilities: kill_switch.disabled_capabilities.iter().cloned().collect(),
        disabled_bindings: kill_switch.disabled_bindings.iter().cloned().collect(),
        disabled_codecs: kill_switch.disabled_codecs.iter().cloned().collect(),
    }
}

fn effective_snapshot_response(
    snapshot: &EffectiveProtocolSnapshot,
) -> EffectiveProtocolSnapshotResponse {
    EffectiveProtocolSnapshotResponse {
        protocol_version: snapshot.protocol_version.clone(),
        release_channel: release_channel(snapshot.release_channel.clone()).to_owned(),
        enabled_capabilities: snapshot.enabled_capabilities.iter().cloned().collect(),
        allowed_bindings: snapshot.allowed_bindings.iter().cloned().collect(),
        allowed_codecs: snapshot.allowed_codecs.iter().cloned().collect(),
        quota_profile_id: snapshot.quota_profile_id.clone(),
        kill_switch_active: snapshot.kill_switch_active,
        precedence: snapshot.precedence.clone(),
    }
}

fn business_policy_vocabulary_response(
    vocabulary: &BusinessPolicyVocabulary,
) -> BusinessPolicyVocabularyResponse {
    BusinessPolicyVocabularyResponse {
        policy_version_field: vocabulary.policy_version_field.clone(),
        capability_flags_field: vocabulary.capability_flags_field.clone(),
        history_visibility_field: vocabulary.history_visibility_field.clone(),
        history_visibility_modes: vocabulary.history_visibility_modes.clone(),
        retention_policy_ref_field: vocabulary.retention_policy_ref_field.clone(),
        retention_policy_scopes: vocabulary.retention_policy_scopes.clone(),
    }
}

fn sdk_compatibility_baseline_response(registry: &CcpRegistry) -> SdkCompatibilityBaselineResponse {
    SdkCompatibilityBaselineResponse {
        im_sdk_family: "sdkwork-im-sdk",
        app_sdk_family: "sdkwork-im-app-sdk",
        backend_sdk_family: "sdkwork-im-backend-sdk",
        rtc_sdk_family: "sdkwork-rtc-sdk",
        matrix_client_types: registry.compatibility_matrix().keys().cloned().collect(),
        protocol_registry_path: "/backend/v3/api/control/protocol_registry",
        protocol_governance_path: "/backend/v3/api/control/protocol_governance",
    }
}

fn release_channel(channel: ReleaseChannel) -> &'static str {
    channel.as_str()
}

fn provider_domain_name(domain: ProviderDomain) -> &'static str {
    domain.as_str()
}

fn validate_optional_tenant_id(
    tenant_id: Option<String>,
) -> Result<Option<String>, ControlPlaneError> {
    if let Some(tenant_id) = tenant_id {
        validate_required_with_code("tenantId", tenant_id.as_str(), "invalid_provider_policy")?;
        validate_payload_size("tenantId", tenant_id.as_str(), CONTROL_PLANE_MAX_ID_BYTES)?;
        return Ok(Some(tenant_id));
    }

    Ok(None)
}

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), ControlPlaneError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(ControlPlaneError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }

    Ok(())
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), ControlPlaneError> {
    if value.trim().is_empty() {
        return Err(ControlPlaneError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::*;

    fn unix_epoch_seconds(at: SystemTime) -> u64 {
        at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
    }

    #[test]
    fn test_unix_epoch_seconds_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(std::time::Duration::from_secs(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_seconds(before_epoch), 0);
    }

    #[test]
    fn test_unix_epoch_seconds_preserves_post_epoch_time() {
        let after_epoch = UNIX_EPOCH + std::time::Duration::from_secs(42);
        assert_eq!(unix_epoch_seconds(after_epoch), 42);
    }
}
