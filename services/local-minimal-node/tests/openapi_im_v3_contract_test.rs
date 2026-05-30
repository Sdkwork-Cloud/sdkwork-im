use std::collections::BTreeSet;
use std::fs;

use serde_json::Value;

const IM_OPENAPI_SCHEMA: &str = "../../sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml";
const APP_OPENAPI_SCHEMA: &str =
    "../../sdks/sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml";
const BACKEND_OPENAPI_SCHEMA: &str =
    "../../sdks/sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml";
const LOCAL_MINIMAL_NODE_BUILD_RS: &str = "src/node/build.rs";
const CONTROL_PLANE_API_LIB_RS: &str = "../control-plane-api/src/lib.rs";
const ADMIN_SANDBOX_RS: &str =
    "../../crates/sdkwork-api-product-runtime/src/admin_sandbox.rs";

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

    assert!(
        !tags.contains("auth"),
        "{label} must not own upstream identity login SDK namespace"
    );
    assert!(
        !tags.contains("session"),
        "{label} generic session SDK namespace is ambiguous; use a domain-qualified session resource"
    );

    let operation_ids = assert_sdkwork_v3_operation_ids(schema, label);
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
    assert!(!tags.contains("notification"), "notification APIs belong to app-api");
    assert!(!tags.contains("automation"), "automation execution APIs belong to app-api");

    let operation_ids = assert_sdkwork_v3_operation_ids(&schema, "im");

    for expected in [
        "device.sessions.resume",
        "device.sessions.disconnect",
        "rtc.sessions.create",
        "conversations.create",
        "conversations.threads.create",
        "conversations.directChats.bind",
        "conversations.messages.create",
        "conversations.messages.interactionSummary.retrieve",
        "contacts.list",
        "social.friendRequests.create",
        "messages.edit",
    ] {
        assert!(
            operation_ids.contains(expected),
            "OpenAPI contract must expose expected SDKWork v3 operationId `{expected}`"
        );
    }
}

#[test]
fn test_app_api_openapi_uses_app_v3_api_paths_for_im_app_development_only() {
    let schema = load_schema(APP_OPENAPI_SCHEMA);
    let paths = path_map(&schema);

    assert!(
        paths.contains_key("/app/v3/api/portal/access"),
        "app-api must expose portal app-business routes"
    );
    assert!(
        paths.contains_key("/app/v3/api/automation/executions"),
        "app-api must expose automation app development routes"
    );
    assert!(
        paths.contains_key("/app/v3/api/notifications/requests"),
        "app-api must expose notification app-business routes"
    );
    assert!(
        paths.contains_key("/app/v3/api/devices/{deviceId}/twin"),
        "app-api must expose device twin app-business routes"
    );
    assert!(
        paths.contains_key("/app/v3/api/iot/protocol/uplink"),
        "app-api must expose IoT protocol app-business routes"
    );
    assert!(
        paths.contains_key("/app/v3/api/rtc/provider_health"),
        "app-api must expose RTC provider app-business routes"
    );
    let forbidden_paths = vec![
        marker(&["/im/v3/api/chat", "/conversations"]),
        marker(&["/app/v3/api/chat", "/conversations"]),
        marker(&["/app/v3/api/social", "/friend_requests"]),
        marker(&["/app/v3/api/device", "/sessions/resume"]),
        marker(&["/app/v3/api/media", "/uploads"]),
        marker(&["/app/v3/api/rtc", "/sessions"]),
        marker(&["/app/v3/api/streams"]),
        marker(&["/backend/v3/api/ops", "/health"]),
        marker(&["/app/v3/api/auth", "/sessions"]),
        marker(&["/app/v3/api/auth", "/sessions/current"]),
        marker(&["/api/app", "/v1", "/user", "-center/session/login"]),
    ];
    for forbidden_path in forbidden_paths {
        assert!(
            !paths.contains_key(forbidden_path.as_str()),
            "craw-chat app-api must not expose forbidden path {forbidden_path}"
        );
    }
}

#[test]
fn test_openapi_paths_match_local_minimal_node_runtime_route_tables() {
    let build_source = load_local_minimal_node_source(LOCAL_MINIMAL_NODE_BUILD_RS);
    let im_routes = extract_runtime_route_paths(extract_source_section(
        &build_source,
        "fn im_standard_api_routes()",
        "fn app_business_api_routes()",
        "IM standard route table",
    ));
    let app_routes = extract_runtime_route_paths(extract_source_section(
        &build_source,
        "fn app_business_api_routes()",
        "fn backend_api_routes()",
        "app route table",
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
        "app",
        &load_schema(APP_OPENAPI_SCHEMA),
        "/app/v3/api",
        &app_routes,
    );
    assert_openapi_paths_match_runtime_routes(
        "backend",
        &load_schema(BACKEND_OPENAPI_SCHEMA),
        "/backend/v3/api",
        &backend_routes,
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
        let forbidden_terms = vec![
            marker(&["I", "AM"]),
            marker(&["i", "am"]),
            marker(&["user", "-center"]),
            String::from("user center"),
        ];
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
        "backend-api must expose ops health"
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
            "craw-chat backend-api must not expose forbidden path {forbidden_path}"
        );
    }
}
