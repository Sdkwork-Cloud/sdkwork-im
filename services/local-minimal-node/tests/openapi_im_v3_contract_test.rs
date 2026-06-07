use std::collections::BTreeSet;
use std::fs;

use serde_json::Value;

const IM_OPENAPI_SCHEMA: &str = "../../sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml";
const APP_OPENAPI_SCHEMA: &str =
    "../../sdks/sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml";
const RTC_APP_OPENAPI_SCHEMA: &str =
    "../../../sdkwork-rtc/sdks/sdkwork-rtc-app-sdk/openapi/sdkwork-rtc-app-api.openapi.json";
const BACKEND_OPENAPI_SCHEMA: &str =
    "../../sdks/sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml";
const API_STANDARD_SPEC: &str = "../../../../specs/API_SPEC.md";
const CRAW_CHAT_LOCAL_STANDARD_SPEC: &str = "../../specs/im-app-api-sdk-integration.spec.md";
const LOCAL_MINIMAL_NODE_BUILD_RS: &str = "src/node/build.rs";
const CONTROL_PLANE_API_LIB_RS: &str = "../control-plane-api/src/lib.rs";
const ADMIN_SANDBOX_RS: &str = "../../crates/sdkwork-api-product-runtime/src/admin_sandbox.rs";

fn load_schema(relative_path: &str) -> Value {
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    let raw = fs::read_to_string(&schema_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", schema_path.display()));
    serde_yaml::from_str(&raw)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", schema_path.display()))
}

fn load_schema_raw(relative_path: &str) -> String {
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&schema_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", schema_path.display()))
}

fn load_local_minimal_node_source(relative_path: &str) -> String {
    let source_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_path.display()))
}

fn load_workspace_standard_source(relative_path: &str) -> String {
    let source_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", source_path.display()))
}

fn marker(parts: &[&str]) -> String {
    parts.concat()
}

fn path_map(schema: &Value) -> &serde_json::Map<String, Value> {
    schema
        .get("paths")
        .and_then(Value::as_object)
        .expect("OpenAPI schema must contain object paths")
}

fn extract_source_section<'a>(
    source: &'a str,
    start_marker: &str,
    end_marker: &str,
    label: &str,
) -> &'a str {
    let start = source
        .find(start_marker)
        .unwrap_or_else(|| panic!("{label} source must contain start marker {start_marker}"));
    let tail = &source[start..];
    let end = tail
        .find(end_marker)
        .map(|offset| start + offset)
        .unwrap_or_else(|| panic!("{label} source must contain end marker {end_marker}"));
    &source[start..end]
}

fn snake_to_lower_camel(value: &str) -> String {
    let mut output = String::new();
    let mut uppercase_next = false;
    for character in value.chars() {
        if character == '_' {
            uppercase_next = true;
            continue;
        }
        if uppercase_next {
            output.extend(character.to_uppercase());
            uppercase_next = false;
        } else {
            output.push(character);
        }
    }
    output
}

fn normalize_runtime_route_path(route: &str) -> String {
    let path_parameter_pattern = regex::Regex::new(r"\{([a-z0-9_]+)\}").unwrap();
    path_parameter_pattern
        .replace_all(route, |captures: &regex::Captures<'_>| {
            format!("{{{}}}", snake_to_lower_camel(&captures[1]))
        })
        .to_string()
}

fn extract_runtime_route_paths(source_section: &str) -> BTreeSet<String> {
    let route_pattern = regex::Regex::new(r#"\.route\(\s*"([^"]+)""#).unwrap();
    route_pattern
        .captures_iter(source_section)
        .map(|captures| normalize_runtime_route_path(&captures[1]))
        .collect()
}

fn extract_backend_admin_route_registry_paths(source: &str) -> BTreeSet<String> {
    let registry_section = extract_source_section(
        source,
        "pub const BACKEND_ADMIN_API_ROUTES: &[&str] = &[",
        "];",
        "backend admin route registry",
    );
    let route_pattern = regex::Regex::new(r#""(/backend/v3/api/admin[^"]*)""#).unwrap();
    route_pattern
        .captures_iter(registry_section)
        .map(|captures| normalize_runtime_route_path(&captures[1]))
        .collect()
}

fn assert_openapi_paths_match_runtime_routes(
    label: &str,
    schema: &Value,
    api_prefix: &str,
    runtime_routes: &BTreeSet<String>,
) {
    let expected_paths = runtime_routes
        .iter()
        .map(|route| {
            if route.starts_with(api_prefix) {
                route.to_owned()
            } else {
                format!("{api_prefix}{route}")
            }
        })
        .collect::<BTreeSet<_>>();
    let openapi_paths = path_map(schema)
        .keys()
        .filter(|path| path.starts_with(api_prefix))
        .cloned()
        .collect::<BTreeSet<_>>();

    let missing = expected_paths
        .difference(&openapi_paths)
        .cloned()
        .collect::<Vec<_>>();
    let extra = openapi_paths
        .difference(&expected_paths)
        .cloned()
        .collect::<Vec<_>>();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "{label} OpenAPI paths must match runtime route table exactly; missing={missing:?}, extra={extra:?}"
    );
}

fn assert_openapi_paths_include_runtime_routes(
    label: &str,
    schema: &Value,
    api_prefix: &str,
    runtime_routes: &BTreeSet<String>,
) {
    let openapi_paths = path_map(schema)
        .keys()
        .filter(|path| path.starts_with(api_prefix))
        .cloned()
        .collect::<BTreeSet<_>>();
    for route in runtime_routes {
        let expected_path = if route.starts_with(api_prefix) {
            route.to_owned()
        } else {
            format!("{api_prefix}{route}")
        };
        assert!(
            openapi_paths.contains(expected_path.as_str()),
            "{label} OpenAPI schema must include local runtime route {expected_path}"
        );
    }
}

fn assert_sdkwork_v3_security_and_problem_details(schema: &Value, label: &str) {
    let security = schema
        .get("security")
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("{label} OpenAPI schema must define top-level security"));
    assert!(
        security
            .iter()
            .any(|entry| entry.get("AuthToken").is_some() && entry.get("AccessToken").is_some()),
        "{label} OpenAPI schema must use dual AuthToken plus AccessToken security"
    );

    let security_schemes = schema
        .pointer("/components/securitySchemes")
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("{label} OpenAPI schema must define security schemes"));
    let auth_token = security_schemes
        .get("AuthToken")
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("{label} must define AuthToken security scheme"));
    assert_eq!(
        auth_token.get("type").and_then(Value::as_str),
        Some("http"),
        "{label} AuthToken must be an HTTP bearer token"
    );
    assert_eq!(
        auth_token.get("scheme").and_then(Value::as_str),
        Some("bearer"),
        "{label} AuthToken must use bearer scheme"
    );

    let access_token = security_schemes
        .get("AccessToken")
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("{label} must define AccessToken security scheme"));
    assert_eq!(
        access_token.get("type").and_then(Value::as_str),
        Some("apiKey"),
        "{label} AccessToken must be an apiKey header"
    );
    assert_eq!(
        access_token.get("in").and_then(Value::as_str),
        Some("header"),
        "{label} AccessToken must be passed in a header"
    );
    assert_eq!(
        access_token.get("name").and_then(Value::as_str),
        Some("Access-Token"),
        "{label} AccessToken header name must be Access-Token"
    );

    assert!(
        schema
            .pointer("/components/schemas/ProblemDetail")
            .is_some(),
        "{label} OpenAPI schema must define ProblemDetail"
    );

    for (path, path_item) in path_map(schema) {
        for method in ["get", "post", "put", "patch", "delete"] {
            let Some(operation) = path_item.get(method).and_then(Value::as_object) else {
                continue;
            };

            let operation_security = operation
                .get("security")
                .and_then(Value::as_array)
                .unwrap_or_else(|| {
                    panic!("{label} {method} {path} must define operation security")
                });
            let anonymous = operation_security.is_empty();
            let dual_token = operation_security.iter().any(|entry| {
                entry.get("AuthToken").is_some() && entry.get("AccessToken").is_some()
            });
            assert!(
                anonymous || dual_token,
                "{label} {method} {path} must use security: [] or dual AuthToken plus AccessToken security"
            );

            let responses = operation
                .get("responses")
                .and_then(Value::as_object)
                .unwrap_or_else(|| panic!("{label} {method} {path} must define responses"));
            for (status, response) in responses {
                let Ok(status_code) = status.parse::<u16>() else {
                    continue;
                };
                if status_code < 400 {
                    continue;
                }
                let problem_schema_ref = response
                    .pointer("/content/application~1problem+json/schema/$ref")
                    .and_then(Value::as_str);
                assert_eq!(
                    problem_schema_ref,
                    Some("#/components/schemas/ProblemDetail"),
                    "{label} {method} {path} response {status} must directly expose application/problem+json ProblemDetail"
                );
            }
        }
    }
}

fn assert_sdkwork_v3_path_segments(schema: &Value, label: &str, api_prefix: &str) {
    let path_static_segment_pattern = regex::Regex::new(r"^[a-z0-9_]+$").unwrap();
    let paths = path_map(schema);
    assert!(
        !paths.is_empty(),
        "{label} OpenAPI contract must expose paths"
    );
    assert!(
        paths.keys().all(|path| path.starts_with(api_prefix)),
        "all {label} paths must use {api_prefix}; got {:?}",
        paths.keys().collect::<Vec<_>>()
    );
    for path in paths.keys() {
        for segment in path.trim_start_matches('/').split('/') {
            if segment.starts_with('{') && segment.ends_with('}') {
                continue;
            }
            assert!(
                path_static_segment_pattern.is_match(segment),
                "{label} static path segment `{segment}` in `{path}` must use lower_snake_case"
            );
        }
    }
}

fn assert_sdkwork_v3_operation_ids(schema: &Value, label: &str) -> BTreeSet<String> {
    let operation_id_pattern =
        regex::Regex::new(r"^[a-z][A-Za-z0-9]*(\.[a-z][A-Za-z0-9]*)+$").unwrap();
    let mut operation_ids = BTreeSet::new();
    for (path, path_item) in path_map(schema) {
        for method in ["get", "post", "put", "patch", "delete"] {
            let Some(operation) = path_item.get(method).and_then(Value::as_object) else {
                continue;
            };
            let operation_id = operation
                .get("operationId")
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("{label} {method} {path} must define operationId"));
            assert!(
                operation_id_pattern.is_match(operation_id),
                "{label} {method} {path} operationId `{operation_id}` must use dotted resource.action style"
            );
            assert!(
                operation_ids.insert(operation_id.to_owned()),
                "{label} operationId `{operation_id}` must be globally unique"
            );
        }
    }
    operation_ids
}

fn assert_no_ambiguous_identity_tags_or_generic_session_operations(schema: &Value, label: &str) {
    let tags = schema
        .get("tags")
        .and_then(Value::as_array)
        .expect("OpenAPI schema must contain tags")
        .iter()
        .filter_map(|tag| tag.get("name").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();

    if label == "app" {
        assert!(
            tags.contains("auth"),
            "app SDK owns the IAM identity lifecycle and must expose the auth namespace"
        );
    } else {
        assert!(
            !tags.contains("auth"),
            "{label} must not own upstream identity login SDK namespace"
        );
    }
    assert!(
        !tags.contains("session"),
        "{label} generic session SDK namespace is ambiguous; use a domain-qualified session resource"
    );

    let operation_ids = assert_sdkwork_v3_operation_ids(schema, label);
    if label != "app" {
        assert!(
            operation_ids
                .iter()
                .all(|operation_id| !operation_id.starts_with("sessions.")),
            "{label} generic sessions.* operationIds are ambiguous with upstream identity sessions; got {:?}",
            operation_ids
                .iter()
                .filter(|operation_id| operation_id.starts_with("sessions."))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn test_workspace_api_standard_documents_craw_chat_three_surface_authority() {
    let api_standard = load_workspace_standard_source(API_STANDARD_SPEC);
    let local_standard = load_workspace_standard_source(CRAW_CHAT_LOCAL_STANDARD_SPEC);

    assert!(
        api_standard.contains("SDKWork uses three canonical API surfaces."),
        "API standard must describe IM, app, and backend as three first-class API surfaces"
    );
    assert!(
        api_standard.contains(
            "IM API | `/im/v3/api` | Current instant messaging application standard open API system"
        ),
        "API standard must define /im/v3/api as the current IM application's standard open API system"
    );
    assert!(
        api_standard.contains(
            "App API | `/app/v3/api` | Instant messaging application app/client integration capabilities for mobile App, H5, PC applications, and other clients"
        ),
        "API standard must define /app/v3/api as the IM app/client integration surface"
    );
    assert!(
        local_standard
            .contains("| IM Open API | `/im/v3/api` | `im-open-api` | `@sdkwork/im-sdk` |"),
        "Craw Chat local standard must define product-owned im-open-api"
    );
    assert!(
        local_standard
            .contains("| IM App API | `/app/v3/api` | `im-app-api` | `sdkwork-im-app-sdk` |"),
        "Craw Chat local standard must point app API integration at sdkwork-im-app-sdk"
    );
    assert!(
        local_standard.contains(
            "| IM Backend API | `/backend/v3/api` | `im-backend-api` | `sdkwork-im-backend-sdk` |"
        ),
        "Craw Chat local standard must point backend API integration at sdkwork-im-backend-sdk"
    );
    assert!(
        local_standard
            .contains("Craw Chat uses product-scoped SDKs, not generic Spring SDK packages."),
        "Craw Chat local standard must override generic Spring SDK examples"
    );
    assert!(
        local_standard.contains(
            "Craw Chat app code must not import retired generic Spring app/backend SDK packages or authorities."
        ),
        "Craw Chat local standard must forbid generic app/backend SDK imports"
    );
}

#[test]
fn test_openapi_surfaces_use_sdkwork_v3_dual_token_and_problem_detail_standard() {
    for (label, relative_path) in [
        ("im", IM_OPENAPI_SCHEMA),
        ("app", APP_OPENAPI_SCHEMA),
        ("backend", BACKEND_OPENAPI_SCHEMA),
    ] {
        let schema = load_schema(relative_path);
        assert_sdkwork_v3_security_and_problem_details(&schema, label);
    }
}

#[test]
fn test_openapi_surfaces_use_sdkwork_v3_path_and_operation_style() {
    for (label, relative_path, api_prefix) in [
        ("im", IM_OPENAPI_SCHEMA, "/im/v3/api/"),
        ("app", APP_OPENAPI_SCHEMA, "/app/v3/api/"),
        ("backend", BACKEND_OPENAPI_SCHEMA, "/backend/v3/api/"),
    ] {
        let schema = load_schema(relative_path);
        assert_sdkwork_v3_path_segments(&schema, label, api_prefix);
        assert_no_ambiguous_identity_tags_or_generic_session_operations(&schema, label);
    }
}

#[test]
fn test_im_openapi_uses_im_v3_api_paths_without_craw_chat_identity_or_legacy_sessions() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let paths = path_map(&schema);

    assert!(
        paths.contains_key("/im/v3/api/device/sessions/resume"),
        "device runtime session resume must live under /im/v3/api/device/sessions"
    );
    assert!(
        paths.contains_key("/im/v3/api/device/sessions/disconnect"),
        "device runtime session disconnect must live under /im/v3/api/device/sessions"
    );
    let forbidden_paths = vec![
        marker(&["/api", "/v1", "/auth", "/login"]),
        marker(&["/api", "/v1", "/auth", "/me"]),
        marker(&["/api", "/v1", "/portal", "/auth"]),
        marker(&["/im/v3/api/portal", "/auth"]),
        marker(&["/api", "/v1", "/sessions/resume"]),
        marker(&["/api", "/v1", "/sessions/disconnect"]),
        marker(&["/api", "/v1", "/chat", "-runtime/sessions/resume"]),
        marker(&["/im/v3/api/device", "-sessions/resume"]),
        marker(&["/app/v3/api/device", "/sessions/resume"]),
        marker(&["/backend/v3/api/device", "/sessions/resume"]),
        marker(&["/im/v3/api/portal", "/access"]),
        marker(&["/im/v3/api/media", "/uploads"]),
        marker(&["/im/v3/api/media", "/uploads/{mediaAssetId}/complete"]),
        marker(&["/im/v3/api/media", "/{mediaAssetId}"]),
        marker(&["/im/v3/api/media", "/{mediaAssetId}/download_url"]),
        marker(&["/im/v3/api/media", "/{mediaAssetId}/attach"]),
        marker(&["/im/v3/api/notifications"]),
        marker(&["/im/v3/api/automation", "/executions"]),
        marker(&["/im/v3/api/devices/", "d_demo", "/twin"]),
        marker(&["/app/v3/api/auth", "/sessions"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
    ];
    for forbidden_path in forbidden_paths {
        assert!(
            !paths.contains_key(forbidden_path.as_str()),
            "craw-chat OpenAPI must not expose legacy identity/login/session path {forbidden_path}"
        );
    }
}

#[test]
fn test_im_openapi_tags_and_operation_ids_are_sdkwork_v3_resource_style() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let tags = schema
        .get("tags")
        .and_then(Value::as_array)
        .expect("OpenAPI schema must contain tags")
        .iter()
        .filter_map(|tag| tag.get("name").and_then(Value::as_str))
        .collect::<BTreeSet<_>>();

    assert!(
        tags.contains("device"),
        "device SDK namespace must own runtime device sessions"
    );
    assert!(
        tags.contains("chat"),
        "chat SDK namespace must own chat-domain resources"
    );
    assert!(
        tags.contains("social"),
        "social SDK namespace must own IM open-platform friend request and friendship resources"
    );
    assert!(
        !tags.contains("notification"),
        "notification APIs belong to im-app-api"
    );
    assert!(
        !tags.contains("automation"),
        "automation execution APIs belong to im-app-api"
    );

    let operation_ids = assert_sdkwork_v3_operation_ids(&schema, "im");

    for expected in [
        "device.sessions.resume",
        "device.sessions.disconnect",
        "rtc.sessions.create",
        "rtc.sessions.retrieve",
        "conversations.create",
        "conversations.threads.create",
        "conversations.directChats.bind",
        "conversations.messages.create",
        "conversations.messages.interactionSummary.retrieve",
        "conversations.profile.retrieve",
        "conversations.profile.update",
        "conversations.preferences.retrieve",
        "conversations.preferences.update",
        "contacts.list",
        "social.contacts.preferences.retrieve",
        "social.contacts.preferences.update",
        "social.contacts.tags.list",
        "social.contacts.tags.create",
        "social.contacts.tags.update",
        "social.contacts.tags.delete",
        "social.contacts.recommendations.create",
        "social.friendRequests.create",
        "messages.edit",
        "messages.visibility.delete",
    ] {
        assert!(
            operation_ids.contains(expected),
            "OpenAPI contract must expose expected SDKWork v3 operationId `{expected}`"
        );
    }
}

#[test]
fn test_im_openapi_rtc_session_retrieve_supports_state_backfill() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let operation = schema
        .pointer("/paths/~1im~1v3~1api~1rtc~1sessions~1{rtcSessionId}/get")
        .and_then(Value::as_object)
        .expect("RTC session retrieve route must expose a GET operation");

    assert_eq!(
        operation.get("operationId").and_then(Value::as_str),
        Some("rtc.sessions.retrieve"),
        "RTC session retrieve must expose a semantic SDKWork v3 operationId"
    );

    let parameters = operation
        .get("parameters")
        .and_then(Value::as_array)
        .expect("RTC session retrieve route must define path parameters");
    assert!(
        parameters
            .iter()
            .any(|parameter| parameter.get("$ref").and_then(Value::as_str)
                == Some("#/components/parameters/RtcSessionIdPath")),
        "RTC session retrieve route must use the standard RtcSessionIdPath parameter"
    );
    assert_eq!(
        Value::Object(operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/RtcSession"),
        "RTC session retrieve must return the standard RtcSession schema"
    );
    for status in ["401", "403", "404"] {
        assert_eq!(
            Value::Object(operation.clone())
                .pointer(
                    format!("/responses/{status}/content/application~1problem+json/schema/$ref")
                        .as_str()
                )
                .and_then(Value::as_str),
            Some("#/components/schemas/ProblemDetail"),
            "RTC session retrieve response {status} must expose ProblemDetail"
        );
    }
}

#[test]
fn test_im_openapi_message_visibility_delete_is_standardized_for_pc_message_list_delete() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let visibility_path = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1messages~1{messageId}~1visibility")
        .and_then(Value::as_object)
        .expect("message visibility route must be defined");

    let delete_operation = visibility_path
        .get("delete")
        .and_then(Value::as_object)
        .expect("message visibility route must support DELETE");
    assert_eq!(
        delete_operation.get("operationId").and_then(Value::as_str),
        Some("messages.visibility.delete"),
        "message visibility delete must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(delete_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/MessageVisibilityMutationResult"),
        "message visibility delete must return a typed mutation response",
    );

    let visibility_properties = schema
        .pointer("/components/schemas/MessageVisibilityMutationResult/properties")
        .and_then(Value::as_object)
        .expect("MessageVisibilityMutationResult must define properties");
    let visibility_required = schema
        .pointer("/components/schemas/MessageVisibilityMutationResult/required")
        .and_then(Value::as_array)
        .expect("MessageVisibilityMutationResult must define required fields");
    for property in [
        "tenantId",
        "conversationId",
        "messageId",
        "messageSeq",
        "principalKind",
        "principalId",
        "isDeleted",
        "updatedAt",
    ] {
        assert!(
            visibility_properties.contains_key(property),
            "MessageVisibilityMutationResult must expose {property}"
        );
        assert!(
            visibility_required.contains(&Value::String(property.to_owned())),
            "MessageVisibilityMutationResult must require {property}"
        );
    }
}

#[test]
fn test_im_openapi_conversation_preferences_are_standardized_for_pc_chat_state() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let preferences_path = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1conversations~1{conversationId}~1preferences")
        .and_then(Value::as_object)
        .expect("conversation preferences route must be defined");

    let get_operation = preferences_path
        .get("get")
        .and_then(Value::as_object)
        .expect("conversation preferences route must support GET");
    assert_eq!(
        get_operation.get("operationId").and_then(Value::as_str),
        Some("conversations.preferences.retrieve"),
        "conversation preferences GET must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(get_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ConversationPreferencesView"),
        "conversation preferences GET must return a typed view",
    );

    let patch_operation = preferences_path
        .get("patch")
        .and_then(Value::as_object)
        .expect("conversation preferences route must support PATCH");
    assert_eq!(
        patch_operation.get("operationId").and_then(Value::as_str),
        Some("conversations.preferences.update"),
        "conversation preferences PATCH must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/UpdateConversationPreferencesRequest"),
        "conversation preferences PATCH must use a typed partial update request",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ConversationPreferencesView"),
        "conversation preferences PATCH must return the updated typed view",
    );

    let preferences_properties = schema
        .pointer("/components/schemas/ConversationPreferencesView/properties")
        .and_then(Value::as_object)
        .expect("ConversationPreferencesView must define properties");
    let preferences_required = schema
        .pointer("/components/schemas/ConversationPreferencesView/required")
        .and_then(Value::as_array)
        .expect("ConversationPreferencesView must define required fields");
    for property in [
        "tenantId",
        "conversationId",
        "principalKind",
        "principalId",
        "isPinned",
        "isMuted",
        "isMarkedUnread",
        "isHidden",
        "updatedAt",
    ] {
        assert!(
            preferences_properties.contains_key(property),
            "ConversationPreferencesView must expose {property}"
        );
        assert!(
            preferences_required.contains(&Value::String(property.to_owned())),
            "ConversationPreferencesView must require {property}"
        );
    }

    let update_properties = schema
        .pointer("/components/schemas/UpdateConversationPreferencesRequest/properties")
        .and_then(Value::as_object)
        .expect("UpdateConversationPreferencesRequest must define properties");
    for property in ["isPinned", "isMuted", "isMarkedUnread", "isHidden"] {
        assert!(
            update_properties.contains_key(property),
            "conversation preference updates must support PC {property} control"
        );
    }
    assert!(
        schema
            .pointer("/components/schemas/UpdateConversationPreferencesRequest/required")
            .is_none(),
        "conversation preference update request must be partial so PATCH can update one control at a time"
    );
}

#[test]
fn test_im_openapi_contact_tags_and_recommendations_are_standardized_for_pc_contacts() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let tags_path = schema
        .pointer("/paths/~1im~1v3~1api~1social~1contacts~1tags")
        .and_then(Value::as_object)
        .expect("contact tags collection route must be defined");
    let tag_path = schema
        .pointer("/paths/~1im~1v3~1api~1social~1contacts~1tags~1{tagId}")
        .and_then(Value::as_object)
        .expect("contact tag item route must be defined");
    let recommendation_path = schema
        .pointer("/paths/~1im~1v3~1api~1social~1contacts~1{targetUserId}~1recommendations")
        .and_then(Value::as_object)
        .expect("contact recommendation route must be defined");

    let list_operation = tags_path
        .get("get")
        .and_then(Value::as_object)
        .expect("contact tags route must support GET");
    assert_eq!(
        list_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.tags.list"),
        "contact tags list must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(list_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ContactTagsResponse"),
        "contact tags list must return a typed page response",
    );
    let list_parameters = list_operation
        .get("parameters")
        .and_then(Value::as_array)
        .expect("contact tags list must define pagination parameters");
    for expected_ref in [
        "#/components/parameters/LimitQuery",
        "#/components/parameters/CursorQuery",
    ] {
        assert!(
            list_parameters.iter().any(
                |parameter| parameter.get("$ref").and_then(Value::as_str) == Some(expected_ref)
            ),
            "contact tags list must expose {expected_ref} for bounded sync"
        );
    }

    let create_operation = tags_path
        .get("post")
        .and_then(Value::as_object)
        .expect("contact tags route must support POST");
    assert_eq!(
        create_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.tags.create"),
        "contact tag create must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(create_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/CreateContactTagRequest"),
        "contact tag create must use a typed create request",
    );
    assert_eq!(
        Value::Object(create_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ContactTagView"),
        "contact tag create must return the created typed tag",
    );

    let update_operation = tag_path
        .get("patch")
        .and_then(Value::as_object)
        .expect("contact tag item route must support PATCH");
    assert_eq!(
        update_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.tags.update"),
        "contact tag update must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(update_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/UpdateContactTagRequest"),
        "contact tag update must use a typed partial update request",
    );

    let delete_operation = tag_path
        .get("delete")
        .and_then(Value::as_object)
        .expect("contact tag item route must support DELETE");
    assert_eq!(
        delete_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.tags.delete"),
        "contact tag delete must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(delete_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/DeleteContactTagResponse"),
        "contact tag delete must return a typed mutation response",
    );

    let recommend_operation = recommendation_path
        .get("post")
        .and_then(Value::as_object)
        .expect("contact recommendation route must support POST");
    assert_eq!(
        recommend_operation
            .get("operationId")
            .and_then(Value::as_str),
        Some("social.contacts.recommendations.create"),
        "contact recommendation create must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(recommend_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/CreateContactRecommendationRequest"),
        "contact recommendation create must use a typed request",
    );
    assert_eq!(
        Value::Object(recommend_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ContactRecommendationView"),
        "contact recommendation create must return a typed action view",
    );

    let tag_properties = schema
        .pointer("/components/schemas/ContactTagView/properties")
        .and_then(Value::as_object)
        .expect("ContactTagView must define properties");
    let tag_required = schema
        .pointer("/components/schemas/ContactTagView/required")
        .and_then(Value::as_array)
        .expect("ContactTagView must define required fields");
    for property in [
        "tenantId",
        "ownerUserId",
        "tagId",
        "name",
        "color",
        "count",
        "bg",
        "border",
        "createdAt",
        "updatedAt",
    ] {
        assert!(
            tag_properties.contains_key(property),
            "ContactTagView must expose {property}"
        );
        assert!(
            tag_required.contains(&Value::String(property.to_owned())),
            "ContactTagView must require {property}"
        );
    }

    let create_tag_properties = schema
        .pointer("/components/schemas/CreateContactTagRequest/properties")
        .and_then(Value::as_object)
        .expect("CreateContactTagRequest must define properties");
    for property in ["name", "color", "count", "bg", "border"] {
        assert!(
            create_tag_properties.contains_key(property),
            "contact tag create must support PC {property} field"
        );
    }
    let create_tag_required = schema
        .pointer("/components/schemas/CreateContactTagRequest/required")
        .and_then(Value::as_array)
        .expect("CreateContactTagRequest must define required fields");
    for property in ["name", "color"] {
        assert!(
            create_tag_required.contains(&Value::String(property.to_owned())),
            "CreateContactTagRequest must require {property}"
        );
    }

    let update_tag_properties = schema
        .pointer("/components/schemas/UpdateContactTagRequest/properties")
        .and_then(Value::as_object)
        .expect("UpdateContactTagRequest must define properties");
    for property in ["name", "color", "count", "bg", "border"] {
        assert!(
            update_tag_properties.contains_key(property),
            "contact tag update must support PC {property} field"
        );
    }
    assert!(
        schema
            .pointer("/components/schemas/UpdateContactTagRequest/required")
            .is_none(),
        "contact tag update request must be partial"
    );

    let recommendation_properties = schema
        .pointer("/components/schemas/ContactRecommendationView/properties")
        .and_then(Value::as_object)
        .expect("ContactRecommendationView must define properties");
    let recommendation_required = schema
        .pointer("/components/schemas/ContactRecommendationView/required")
        .and_then(Value::as_array)
        .expect("ContactRecommendationView must define required fields");
    for property in [
        "tenantId",
        "ownerUserId",
        "targetUserId",
        "recommendationId",
        "createdAt",
    ] {
        assert!(
            recommendation_properties.contains_key(property),
            "ContactRecommendationView must expose {property}"
        );
        assert!(
            recommendation_required.contains(&Value::String(property.to_owned())),
            "ContactRecommendationView must require {property}"
        );
    }
}

#[test]
fn test_im_openapi_contact_preferences_are_standardized_for_pc_contact_state() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let preferences_path = schema
        .pointer("/paths/~1im~1v3~1api~1social~1contacts~1{targetUserId}~1preferences")
        .and_then(Value::as_object)
        .expect("contact preferences route must be defined");

    let get_operation = preferences_path
        .get("get")
        .and_then(Value::as_object)
        .expect("contact preferences route must support GET");
    assert_eq!(
        get_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.preferences.retrieve"),
        "contact preferences GET must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(get_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ContactPreferencesView"),
        "contact preferences GET must return a typed view",
    );

    let patch_operation = preferences_path
        .get("patch")
        .and_then(Value::as_object)
        .expect("contact preferences route must support PATCH");
    assert_eq!(
        patch_operation.get("operationId").and_then(Value::as_str),
        Some("social.contacts.preferences.update"),
        "contact preferences PATCH must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/UpdateContactPreferencesRequest"),
        "contact preferences PATCH must use a typed partial update request",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ContactPreferencesView"),
        "contact preferences PATCH must return the updated typed view",
    );

    let target_parameter_ref = preferences_path
        .get("parameters")
        .and_then(Value::as_array)
        .and_then(|parameters| {
            parameters
                .iter()
                .find_map(|parameter| parameter.get("$ref").and_then(Value::as_str))
        })
        .expect("contact preferences route must declare a reusable path parameter");
    assert_eq!(
        target_parameter_ref, "#/components/parameters/TargetUserIdPath",
        "contact preferences route must use the standard targetUserId path parameter",
    );
    let target_parameter = schema
        .pointer("/components/parameters/TargetUserIdPath")
        .and_then(Value::as_object)
        .expect("TargetUserIdPath parameter must be defined");
    assert_eq!(
        target_parameter.get("name").and_then(Value::as_str),
        Some("targetUserId"),
        "TargetUserIdPath must use lowerCamelCase path parameter naming",
    );
    assert_eq!(
        target_parameter.get("in").and_then(Value::as_str),
        Some("path"),
        "TargetUserIdPath must be a path parameter",
    );
    assert_eq!(
        Value::Object(target_parameter.clone())
            .pointer("/schema/type")
            .and_then(Value::as_str),
        Some("string"),
        "TargetUserIdPath must be a string",
    );

    let preferences_properties = schema
        .pointer("/components/schemas/ContactPreferencesView/properties")
        .and_then(Value::as_object)
        .expect("ContactPreferencesView must define properties");
    let preferences_required = schema
        .pointer("/components/schemas/ContactPreferencesView/required")
        .and_then(Value::as_array)
        .expect("ContactPreferencesView must define required fields");
    for property in [
        "tenantId",
        "ownerUserId",
        "targetUserId",
        "isStarred",
        "remark",
        "isBlocked",
        "updatedAt",
    ] {
        assert!(
            preferences_properties.contains_key(property),
            "ContactPreferencesView must expose {property}"
        );
        assert!(
            preferences_required.contains(&Value::String(property.to_owned())),
            "ContactPreferencesView must require {property}"
        );
    }

    let update_properties = schema
        .pointer("/components/schemas/UpdateContactPreferencesRequest/properties")
        .and_then(Value::as_object)
        .expect("UpdateContactPreferencesRequest must define properties");
    for property in ["isStarred", "remark", "isBlocked"] {
        assert!(
            update_properties.contains_key(property),
            "contact preference updates must support PC {property} control"
        );
    }
    assert_eq!(
        update_properties
            .get("remark")
            .and_then(|remark| remark.get("maxLength"))
            .and_then(Value::as_u64),
        Some(256),
        "contact remark updates must bound user-controlled display text"
    );
    assert!(
        schema
            .pointer("/components/schemas/UpdateContactPreferencesRequest/required")
            .is_none(),
        "contact preference update request must be partial so PATCH can update one control at a time"
    );
}

#[test]
fn test_im_openapi_conversation_profile_is_standardized_for_pc_group_info() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let profile_path = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1conversations~1{conversationId}~1profile")
        .and_then(Value::as_object)
        .expect("conversation profile route must be defined");

    let get_operation = profile_path
        .get("get")
        .and_then(Value::as_object)
        .expect("conversation profile route must support GET");
    assert_eq!(
        get_operation.get("operationId").and_then(Value::as_str),
        Some("conversations.profile.retrieve"),
        "conversation profile GET must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(get_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ConversationProfileView"),
        "conversation profile GET must return a typed view",
    );

    let patch_operation = profile_path
        .get("patch")
        .and_then(Value::as_object)
        .expect("conversation profile route must support PATCH");
    assert_eq!(
        patch_operation.get("operationId").and_then(Value::as_str),
        Some("conversations.profile.update"),
        "conversation profile PATCH must expose a semantic SDKWork v3 operationId",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/UpdateConversationProfileRequest"),
        "conversation profile PATCH must use a typed partial update request",
    );
    assert_eq!(
        Value::Object(patch_operation.clone())
            .pointer("/responses/200/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/ConversationProfileView"),
        "conversation profile PATCH must return the updated typed view",
    );

    let profile_properties = schema
        .pointer("/components/schemas/ConversationProfileView/properties")
        .and_then(Value::as_object)
        .expect("ConversationProfileView must define properties");
    let profile_required = schema
        .pointer("/components/schemas/ConversationProfileView/required")
        .and_then(Value::as_array)
        .expect("ConversationProfileView must define required fields");
    for property in [
        "tenantId",
        "conversationId",
        "displayName",
        "avatarUrl",
        "notice",
        "updatedAt",
    ] {
        assert!(
            profile_properties.contains_key(property),
            "ConversationProfileView must expose {property}"
        );
        assert!(
            profile_required.contains(&Value::String(property.to_owned())),
            "ConversationProfileView must require {property}"
        );
    }

    let update_properties = schema
        .pointer("/components/schemas/UpdateConversationProfileRequest/properties")
        .and_then(Value::as_object)
        .expect("UpdateConversationProfileRequest must define properties");
    for property in ["displayName", "avatarUrl", "notice"] {
        assert!(
            update_properties.contains_key(property),
            "conversation profile updates must support PC {property} control"
        );
    }
    assert!(
        schema
            .pointer("/components/schemas/UpdateConversationProfileRequest/required")
            .is_none(),
        "conversation profile update request must be partial so PATCH can update one field at a time"
    );
}

#[test]
fn test_im_openapi_device_sync_feed_is_after_seq_and_limit_bounded() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let sync_feed_parameters = schema
        .pointer("/paths/~1im~1v3~1api~1devices~1{deviceId}~1sync_feed/get/parameters")
        .and_then(Value::as_array)
        .expect("device sync feed route must define query/path parameters");

    for expected_ref in [
        "#/components/parameters/DeviceIdPath",
        "#/components/parameters/AfterSeqQuery",
        "#/components/parameters/LimitQuery",
    ] {
        assert!(
            sync_feed_parameters
                .iter()
                .any(
                    |parameter| parameter.get("$ref").and_then(Value::as_str) == Some(expected_ref)
                ),
            "device sync feed must expose {expected_ref} so clients can page bounded sync windows"
        );
    }
}

#[test]
fn test_im_openapi_message_timeline_is_after_seq_and_limit_bounded() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let timeline_parameters = schema
        .pointer(
            "/paths/~1im~1v3~1api~1chat~1conversations~1{conversationId}~1messages/get/parameters",
        )
        .and_then(Value::as_array)
        .expect("message timeline route must define query/path parameters");

    for expected_ref in [
        "#/components/parameters/ConversationIdPath",
        "#/components/parameters/AfterSeqQuery",
        "#/components/parameters/LimitQuery",
    ] {
        assert!(
            timeline_parameters
                .iter()
                .any(
                    |parameter| parameter.get("$ref").and_then(Value::as_str) == Some(expected_ref)
                ),
            "message timeline must expose {expected_ref} so clients can page bounded history windows"
        );
    }
}

#[test]
fn test_im_openapi_message_timeline_entries_preserve_complete_message_projection() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let timeline_entry_properties = schema
        .pointer("/components/schemas/TimelineViewEntry/properties")
        .and_then(Value::as_object)
        .expect("TimelineViewEntry schema must define properties");
    let timeline_entry_required = schema
        .pointer("/components/schemas/TimelineViewEntry/required")
        .and_then(Value::as_array)
        .expect("TimelineViewEntry schema must define required fields");

    for property in [
        "tenantId",
        "conversationId",
        "messageId",
        "messageSeq",
        "sender",
        "body",
        "messageType",
        "deliveryMode",
        "occurredAt",
    ] {
        assert!(
            timeline_entry_properties.contains_key(property),
            "TimelineViewEntry must expose {property} so SDK clients can render historical messages without local fallbacks"
        );
        assert!(
            timeline_entry_required.contains(&Value::String(property.to_owned())),
            "TimelineViewEntry must require {property} for deterministic message history sync"
        );
    }

    assert_eq!(
        timeline_entry_properties
            .get("sender")
            .and_then(|schema| schema.get("$ref"))
            .and_then(Value::as_str),
        Some("#/components/schemas/Sender"),
        "TimelineViewEntry.sender must reuse the standard Sender schema"
    );
    assert_eq!(
        timeline_entry_properties
            .get("body")
            .and_then(|schema| schema.get("$ref"))
            .and_then(Value::as_str),
        Some("#/components/schemas/MessageBody"),
        "TimelineViewEntry.body must reuse the standard MessageBody schema"
    );
    assert_eq!(
        timeline_entry_properties
            .get("messageType")
            .and_then(|schema| schema.get("$ref"))
            .and_then(Value::as_str),
        Some("#/components/schemas/MessageType"),
        "TimelineViewEntry.messageType must be a typed message kind"
    );
    assert!(
        schema.pointer("/components/schemas/MessageBody").is_some(),
        "IM OpenAPI must define MessageBody for timeline history payloads"
    );
    let message_body_properties = schema
        .pointer("/components/schemas/MessageBody/properties")
        .and_then(Value::as_object)
        .expect("MessageBody schema must define properties");
    assert_eq!(
        message_body_properties
            .get("replyTo")
            .and_then(|schema| schema.get("$ref"))
            .and_then(Value::as_str),
        Some("#/components/schemas/MessageReplyReference"),
        "MessageBody.replyTo must use the standard reply reference schema so PC replies survive message sync"
    );
    let reply_reference_properties = schema
        .pointer("/components/schemas/MessageReplyReference/properties")
        .and_then(Value::as_object)
        .expect("MessageReplyReference schema must define properties");
    let reply_reference_required = schema
        .pointer("/components/schemas/MessageReplyReference/required")
        .and_then(Value::as_array)
        .expect("MessageReplyReference schema must define required fields");
    for property in ["messageId", "senderDisplayName", "contentPreview"] {
        assert!(
            reply_reference_properties.contains_key(property),
            "MessageReplyReference must expose {property} for PC reply rendering"
        );
        assert!(
            reply_reference_required.contains(&Value::String(property.to_owned())),
            "MessageReplyReference must require {property} for deterministic reply sync"
        );
    }
    let message_type_values = schema
        .pointer("/components/schemas/MessageType/enum")
        .and_then(Value::as_array)
        .expect("MessageType schema must define enum values");
    for expected in ["standard", "system", "signal"] {
        assert!(
            message_type_values.contains(&Value::String(expected.to_owned())),
            "MessageType must expose {expected}"
        );
    }
}

#[test]
fn test_im_openapi_message_favorites_are_principal_scoped_and_paged() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let paths = path_map(&schema);

    for expected_path in [
        "/im/v3/api/chat/messages/favorites",
        "/im/v3/api/chat/messages/{messageId}/favorites",
        "/im/v3/api/chat/messages/favorites/{favoriteId}",
    ] {
        assert!(
            paths.contains_key(expected_path),
            "IM OpenAPI must expose message favorite path {expected_path} so PC favorites sync through the standard IM API"
        );
    }

    let list_parameters = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1messages~1favorites/get/parameters")
        .and_then(Value::as_array)
        .expect("message favorites list route must define query parameters");
    for expected_ref in [
        "#/components/parameters/LimitQuery",
        "#/components/parameters/CursorQuery",
        "#/components/parameters/FavoriteTypeQuery",
        "#/components/parameters/QQuery",
    ] {
        assert!(
            list_parameters.iter().any(
                |parameter| parameter.get("$ref").and_then(Value::as_str) == Some(expected_ref)
            ),
            "message favorites list API must expose {expected_ref} for bounded current-principal favorite sync"
        );
    }

    let create_operation = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1messages~1{messageId}~1favorites/post")
        .expect("message favorite create route must define post operation");
    assert_eq!(
        create_operation.get("operationId").and_then(Value::as_str),
        Some("messages.favorites.create"),
        "message favorite create operationId must generate chat.messages.favorites.create"
    );
    assert_eq!(
        create_operation
            .pointer("/requestBody/content/application~1json/schema/$ref")
            .and_then(Value::as_str),
        Some("#/components/schemas/FavoriteMessageRequest"),
        "message favorite create must use the standard FavoriteMessageRequest schema"
    );

    let delete_operation = schema
        .pointer("/paths/~1im~1v3~1api~1chat~1messages~1favorites~1{favoriteId}/delete")
        .expect("message favorite delete route must define delete operation");
    assert_eq!(
        delete_operation.get("operationId").and_then(Value::as_str),
        Some("messages.favorites.delete"),
        "message favorite delete operationId must generate chat.messages.favorites.delete"
    );

    let favorite_view_properties = schema
        .pointer("/components/schemas/MessageFavoriteView/properties")
        .and_then(Value::as_object)
        .expect("MessageFavoriteView schema must define properties");
    let favorite_view_required = schema
        .pointer("/components/schemas/MessageFavoriteView/required")
        .and_then(Value::as_array)
        .expect("MessageFavoriteView schema must define required fields");
    for property in [
        "tenantId",
        "principalKind",
        "principalId",
        "favoriteId",
        "favoriteType",
        "conversationId",
        "messageId",
        "messageSeq",
        "title",
        "contentPreview",
        "sourceDisplayName",
        "favoritedAt",
    ] {
        assert!(
            favorite_view_properties.contains_key(property),
            "MessageFavoriteView must expose {property} for PC favorite sync"
        );
        assert!(
            favorite_view_required.contains(&Value::String(property.to_owned())),
            "MessageFavoriteView must require {property} for deterministic favorite sync"
        );
    }

    let favorite_response_properties = schema
        .pointer("/components/schemas/FavoriteMessagesResponse/properties")
        .and_then(Value::as_object)
        .expect("FavoriteMessagesResponse schema must define properties");
    let favorite_response_required = schema
        .pointer("/components/schemas/FavoriteMessagesResponse/required")
        .and_then(Value::as_array)
        .expect("FavoriteMessagesResponse schema must define required fields");
    for property in ["items", "nextCursor", "hasMore"] {
        assert!(
            favorite_response_properties.contains_key(property),
            "FavoriteMessagesResponse must expose {property} for paged favorite sync"
        );
    }
    for property in ["items", "hasMore"] {
        assert!(
            favorite_response_required.contains(&Value::String(property.to_owned())),
            "FavoriteMessagesResponse must require {property} for paged favorite sync"
        );
    }

    let request_properties = schema
        .pointer("/components/schemas/FavoriteMessageRequest/properties")
        .and_then(Value::as_object)
        .expect("FavoriteMessageRequest schema must define properties");
    let request_required = schema
        .pointer("/components/schemas/FavoriteMessageRequest/required")
        .and_then(Value::as_array)
        .expect("FavoriteMessageRequest schema must define required fields");
    for property in [
        "conversationId",
        "favoriteType",
        "title",
        "contentPreview",
        "sourceDisplayName",
    ] {
        assert!(
            request_properties.contains_key(property),
            "FavoriteMessageRequest must expose {property} for PC message favorite persistence"
        );
        assert!(
            request_required.contains(&Value::String(property.to_owned())),
            "FavoriteMessageRequest must require {property} for deterministic favorite persistence"
        );
    }
}

#[test]
fn test_im_openapi_sync_window_responses_expose_cursor_metadata() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    for (label, schema_pointer, expected_properties, required_properties) in [
        (
            "timeline",
            "/components/schemas/TimelineResponse",
            ["items", "nextAfterSeq", "hasMore", ""],
            ["items", "hasMore", "", ""],
        ),
        (
            "device sync feed",
            "/components/schemas/DeviceSyncFeedResponse",
            ["items", "nextAfterSeq", "hasMore", "trimmedThroughSeq"],
            ["items", "hasMore", "trimmedThroughSeq", ""],
        ),
    ] {
        let response_schema = schema
            .pointer(schema_pointer)
            .unwrap_or_else(|| panic!("{label} response schema must exist"));
        let properties = response_schema
            .get("properties")
            .and_then(Value::as_object)
            .unwrap_or_else(|| panic!("{label} response schema must define properties"));
        let required = response_schema
            .get("required")
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("{label} response schema must define required fields"));

        for property in expected_properties
            .into_iter()
            .filter(|property| !property.is_empty())
        {
            assert!(
                properties.contains_key(property),
                "{label} response must expose {property} for SDK window synchronization"
            );
        }
        for property in required_properties
            .into_iter()
            .filter(|property| !property.is_empty())
        {
            assert!(
                required.contains(&Value::String(property.to_owned())),
                "{label} response must require {property} for deterministic client sync"
            );
        }
    }
}

#[test]
fn test_im_openapi_contacts_and_inbox_are_limit_and_cursor_bounded() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    for (label, pointer) in [
        (
            "contacts",
            "/paths/~1im~1v3~1api~1chat~1contacts/get/parameters",
        ),
        ("inbox", "/paths/~1im~1v3~1api~1chat~1inbox/get/parameters"),
    ] {
        let parameters = schema
            .pointer(pointer)
            .and_then(Value::as_array)
            .unwrap_or_else(|| panic!("{label} route must define query parameters"));

        for expected_ref in [
            "#/components/parameters/LimitQuery",
            "#/components/parameters/CursorQuery",
        ] {
            assert!(
                parameters
                    .iter()
                    .any(|parameter| parameter.get("$ref").and_then(Value::as_str)
                        == Some(expected_ref)),
                "{label} list API must expose {expected_ref} so clients can page bounded windows"
            );
        }
    }
}

#[test]
fn test_im_openapi_conversation_members_are_limit_and_cursor_bounded() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let parameters = schema
        .pointer(
            "/paths/~1im~1v3~1api~1chat~1conversations~1{conversationId}~1members/get/parameters",
        )
        .and_then(Value::as_array)
        .expect("conversation members route must define path/query parameters");

    for expected_ref in [
        "#/components/parameters/ConversationIdPath",
        "#/components/parameters/LimitQuery",
        "#/components/parameters/CursorQuery",
    ] {
        assert!(
            parameters.iter().any(
                |parameter| parameter.get("$ref").and_then(Value::as_str) == Some(expected_ref)
            ),
            "conversation members list API must expose {expected_ref} so clients can page bounded group member windows"
        );
    }

    let response_schema = schema
        .pointer("/components/schemas/ListMembersResponse")
        .expect("ListMembersResponse schema must exist");
    let properties = response_schema
        .get("properties")
        .and_then(Value::as_object)
        .expect("ListMembersResponse schema must define properties");
    let required = response_schema
        .get("required")
        .and_then(Value::as_array)
        .expect("ListMembersResponse schema must define required fields");

    for property in ["items", "nextCursor", "hasMore"] {
        assert!(
            properties.contains_key(property),
            "ListMembersResponse must expose {property} for bounded group member sync"
        );
    }
    for property in ["items", "hasMore"] {
        assert!(
            required.contains(&Value::String(property.to_owned())),
            "ListMembersResponse must require {property} for deterministic group member sync"
        );
    }
}

#[test]
fn test_im_openapi_membership_state_includes_shared_history_linked_reader() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let membership_state_values = schema
        .pointer("/components/schemas/MembershipState/enum")
        .and_then(Value::as_array)
        .expect("MembershipState schema must define enum values");

    for expected_state in ["joined", "invited", "linked", "left", "removed"] {
        assert!(
            membership_state_values.contains(&Value::String(expected_state.to_owned())),
            "MembershipState must expose {expected_state} so SDK clients can type shared-history linked readers"
        );
    }
}

#[test]
fn test_im_openapi_media_resource_and_message_parts_are_drive_backed() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let raw = load_schema_raw(IM_OPENAPI_SCHEMA);
    let media_resource_spec = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../specs/MEDIA_RESOURCE_SPEC.md"),
    )
    .expect("MediaResource standard spec should be readable");

    for forbidden in [
        "mediaAssetId:",
        "bucketId:",
        "objectKey:",
        "objectVersion:",
        "CreateUploadRequest:",
        "CompleteUploadRequest:",
        "MediaAsset:",
        "MediaUploadMutationResponse:",
        "MediaUploadSession:",
        "MediaDownloadUrlResponse:",
        "object_storage",
    ] {
        assert!(
            !raw.contains(forbidden),
            "IM OpenAPI must not expose app-local media storage lifecycle field/schema `{forbidden}`"
        );
    }

    let media_content_part_properties = schema
        .pointer("/components/schemas/MediaContentPart/properties")
        .and_then(Value::as_object)
        .expect("MediaContentPart schema must define properties");
    assert!(
        media_content_part_properties.get("drive").is_some(),
        "media ContentPart must expose a Drive reference"
    );
    assert!(
        media_content_part_properties.get("mediaAssetId").is_none(),
        "media ContentPart must not expose legacy mediaAssetId"
    );
    let content_part_one_of = schema
        .pointer("/components/schemas/ContentPart/oneOf")
        .and_then(Value::as_array)
        .expect("ContentPart must be a kind-discriminated oneOf schema");
    assert!(
        !content_part_one_of.is_empty(),
        "ContentPart oneOf must define typed content part variants"
    );
    let media_content_part_required = schema
        .pointer("/components/schemas/MediaContentPart/required")
        .and_then(Value::as_array)
        .expect("MediaContentPart schema must define required fields");
    for required_field in ["kind", "drive", "resource"] {
        assert!(
            media_content_part_required.contains(&Value::String(required_field.into())),
            "MediaContentPart must require {required_field}"
        );
    }

    let media_resource_properties = schema
        .pointer("/components/schemas/MediaResource/properties")
        .and_then(Value::as_object)
        .expect("MediaResource schema must define properties");
    assert!(
        media_resource_properties.get("uri").is_some(),
        "MediaResource must expose stable Drive uri"
    );
    let media_source_values = schema
        .pointer("/components/schemas/MediaSource/enum")
        .and_then(Value::as_array)
        .expect("MediaSource schema must define enum values");
    assert!(
        media_source_values.contains(&Value::String("drive".into())),
        "MediaSource must expose Drive as the SDKWork-owned file source"
    );
    assert!(
        !media_source_values.contains(&Value::String("object_storage".into())),
        "MediaSource must not expose storage implementation names"
    );
    assert!(
        media_resource_spec.contains("| 'drive'")
            && !media_resource_spec.contains("| 'object_storage'"),
        "canonical MediaResource standard must use drive, not object_storage"
    );
    for forbidden in ["bucketId", "objectKey", "objectVersion"] {
        assert!(
            media_resource_properties.get(forbidden).is_none(),
            "MediaResource must not expose Drive-owned storage field {forbidden}"
        );
    }
}

#[test]
fn test_im_openapi_social_user_search_contract_is_sdk_backed() {
    let schema = load_schema(IM_OPENAPI_SCHEMA);
    let paths = path_map(&schema);
    let users_path = paths
        .get("/im/v3/api/social/users")
        .expect("IM OpenAPI must expose SDK-backed social user search for add-friend lookup");
    let get_operation = users_path
        .get("get")
        .expect("social user search must use safe GET semantics");

    assert_eq!(
        get_operation
            .get("operationId")
            .and_then(Value::as_str)
            .expect("social user search operationId must be present"),
        "social.users.list"
    );
    assert_eq!(
        get_operation
            .get("tags")
            .and_then(Value::as_array)
            .and_then(|tags| tags.first())
            .and_then(Value::as_str),
        Some("social")
    );

    let parameters = get_operation
        .get("parameters")
        .and_then(Value::as_array)
        .expect("social user search must define query parameters");
    assert!(
        parameters.iter().any(|parameter| {
            parameter.get("$ref").and_then(Value::as_str) == Some("#/components/parameters/QQuery")
        }),
        "social user search must use the standard q query parameter"
    );
    assert!(
        parameters.iter().any(|parameter| {
            parameter.get("$ref").and_then(Value::as_str)
                == Some("#/components/parameters/LimitQuery")
        }),
        "social user search must be bounded with the standard limit query parameter"
    );

    assert_eq!(
        get_operation.pointer("/responses/200/content/application~1json/schema/$ref"),
        Some(&Value::String(
            "#/components/schemas/SocialUserSearchResponse".into()
        )),
        "social user search must return a typed paginated response"
    );
    for status in ["400", "401", "403", "503"] {
        assert_eq!(
            get_operation.pointer(&format!(
                "/responses/{status}/content/application~1problem+json/schema/$ref"
            )),
            Some(&Value::String("#/components/schemas/ProblemDetail".into())),
            "social user search {status} response must use ProblemDetail"
        );
    }

    let response_required = schema
        .pointer("/components/schemas/SocialUserSearchResponse/required")
        .and_then(Value::as_array)
        .expect("SocialUserSearchResponse must define required fields");
    for required_field in ["items", "hasMore"] {
        assert!(
            response_required.contains(&Value::String(required_field.into())),
            "SocialUserSearchResponse must require {required_field}"
        );
    }
    let result_properties = schema
        .pointer("/components/schemas/SocialUserSearchResult/properties")
        .and_then(Value::as_object)
        .expect("SocialUserSearchResult schema must define properties");
    for property in ["userId", "displayName", "relationshipState"] {
        assert!(
            result_properties.contains_key(property),
            "SocialUserSearchResult must expose {property}"
        );
    }
}

#[test]
fn test_app_api_openapi_uses_sdkwork_im_app_sdk_contract_with_appbase_iam_paths() {
    let schema = load_schema(APP_OPENAPI_SCHEMA);
    let paths = path_map(&schema);

    assert!(
        paths.len() > 10,
        "im-app-api must use the product sdkwork-im-app-sdk contract and include appbase IAM integration paths"
    );
    for expected in [
        "/app/v3/api/auth/sessions",
        "/app/v3/api/auth/sessions/current",
        "/app/v3/api/auth/sessions/refresh",
        "/app/v3/api/auth/registrations",
        "/app/v3/api/auth/verification_codes",
        "/app/v3/api/auth/verification_codes/verify",
        "/app/v3/api/system/iam/runtime",
        "/app/v3/api/system/iam/verification_policy",
        "/app/v3/api/open_platform/qr_auth/sessions",
    ] {
        assert!(
            paths.contains_key(expected),
            "im-app-api must expose product app SDK path {expected}"
        );
    }
    let forbidden_paths = vec![
        marker(&["/im/v3/api/chat", "/conversations"]),
        marker(&["/app/v3/api/chat", "/conversations"]),
        marker(&["/app/v3/api/device", "/sessions/resume"]),
        marker(&["/app/v3/api/streams"]),
        marker(&["/backend/v3/api/ops", "/health"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
    ];
    for forbidden_path in forbidden_paths {
        assert!(
            !paths.contains_key(forbidden_path.as_str()),
            "craw-chat im-app-api must not expose private/conflicting path {forbidden_path}"
        );
    }
}

#[test]
fn test_openapi_paths_match_local_minimal_node_runtime_route_tables() {
    let build_source = load_local_minimal_node_source(LOCAL_MINIMAL_NODE_BUILD_RS);
    assert!(
        !build_source.contains(".nest(\"/app/v3/api\","),
        "local-minimal-node must not mount the generic /app/v3/api route table; RTC is mounted as a dedicated /app/v3/api/rtc authority"
    );
    assert!(
        build_source.contains(".nest(\"/app/v3/api/rtc\", rtc_app_api_routes())"),
        "local-minimal-node RTC routes must move out of /im/v3/api and mount under /app/v3/api/rtc"
    );
    assert!(
        !build_source.contains("fn app_business_api_routes()"),
        "local-minimal-node must not keep a private Craw Chat im-app-api route table"
    );
    let im_routes = extract_runtime_route_paths(extract_source_section(
        &build_source,
        "fn im_standard_api_routes()",
        "fn rtc_app_api_routes()",
        "IM standard route table",
    ));
    let rtc_routes = extract_runtime_route_paths(extract_source_section(
        &build_source,
        "fn rtc_app_api_routes()",
        "fn backend_api_routes()",
        "RTC app route table",
    ));
    let backend_routes = extract_runtime_route_paths(extract_source_section(
        &build_source,
        "fn backend_api_routes()",
        "#[cfg(test)]",
        "backend route table",
    ));
    let control_plane_source = load_local_minimal_node_source(CONTROL_PLANE_API_LIB_RS);
    let control_plane_routes = extract_runtime_route_paths(extract_source_section(
        &control_plane_source,
        "fn build_control_surface_with_state_and_scheduler_config(",
        "async fn require_app_context(",
        "control-plane route table",
    ));
    let admin_sandbox_source = load_local_minimal_node_source(ADMIN_SANDBOX_RS);
    let admin_routes = extract_backend_admin_route_registry_paths(&admin_sandbox_source);
    let backend_routes = backend_routes
        .into_iter()
        .chain(control_plane_routes)
        .chain(admin_routes)
        .collect::<BTreeSet<_>>();

    assert_openapi_paths_match_runtime_routes(
        "im",
        &load_schema(IM_OPENAPI_SCHEMA),
        "/im/v3/api",
        &im_routes,
    );
    assert_openapi_paths_match_runtime_routes(
        "backend",
        &load_schema(BACKEND_OPENAPI_SCHEMA),
        "/backend/v3/api",
        &backend_routes,
    );
    assert_openapi_paths_include_runtime_routes(
        "rtc app",
        &load_schema(RTC_APP_OPENAPI_SCHEMA),
        "/app/v3/api/rtc",
        &rtc_routes,
    );
}

#[test]
fn test_openapi_sources_do_not_keep_legacy_authority_files_or_identity_wording_debt() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative_path in [
        "../../sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml",
        "../../sdks/sdkwork-im-sdk/openapi/craw-chat-app.sdkgen.yaml",
        "../../sdks/sdkwork-im-sdk/openapi/craw-chat-app.flutter.sdkgen.yaml",
    ] {
        let absolute_path = manifest_dir.join(relative_path);
        assert!(
            !absolute_path.exists(),
            "legacy single-surface OpenAPI source must be removed: {}",
            absolute_path.display()
        );
    }

    for relative_path in [
        IM_OPENAPI_SCHEMA,
        APP_OPENAPI_SCHEMA,
        BACKEND_OPENAPI_SCHEMA,
    ] {
        let raw = load_schema_raw(relative_path);
        let forbidden_terms = vec![marker(&["user", "-center"]), String::from("user center")];
        for forbidden in forbidden_terms {
            assert!(
                !raw.contains(forbidden.as_str()),
                "{relative_path} must not keep local identity implementation wording `{forbidden}`"
            );
        }
    }
}

#[test]
fn test_backend_api_openapi_uses_backend_v3_api_paths_without_login_or_client_sessions() {
    let schema = load_schema(BACKEND_OPENAPI_SCHEMA);
    let paths = path_map(&schema);

    assert!(
        paths.contains_key("/backend/v3/api/ops/health"),
        "im-backend-api must expose ops health"
    );
    let forbidden_paths = vec![
        marker(&["/im/v3/api/chat", "/conversations"]),
        marker(&["/app/v3/api/chat", "/conversations"]),
        marker(&["/backend/v3/api/auth", "/sessions"]),
        marker(&["/backend/v3/api/auth", "/login"]),
        marker(&["/backend/v3/api/device", "/sessions/resume"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
    ];
    for forbidden_path in forbidden_paths {
        assert!(
            !paths.contains_key(forbidden_path.as_str()),
            "craw-chat im-backend-api must not expose forbidden path {forbidden_path}"
        );
    }
}
