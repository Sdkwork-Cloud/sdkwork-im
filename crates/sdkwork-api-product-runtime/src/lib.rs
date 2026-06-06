use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri},
    response::{Redirect, Response},
    routing::{any, get},
    Router,
};
use bytes::Bytes;
use rand::random;
use reqwest::Client;
use sdkwork_api_config::{StandaloneConfig, StandaloneConfigLoader};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Component, Path as StdPath, PathBuf},
};
use tokio::{fs, net::TcpListener, sync::oneshot};
use url::{Host, Url};

mod admin_sandbox;
mod local_iam;

use admin_sandbox::{handle_admin_sandbox_request, SharedAdminSandboxState};
use local_iam::{local_iam_router, LocalIamState};

const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const BACKEND_ADMIN_API_PREFIX: &str = "/backend/v3/api/admin";
const ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE: &str = "Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a backend that serves /backend/v3/api/admin.";
const PC_PRODUCT_API_UPSTREAM_ENV: &str = "CRAW_CHAT_PC_API_UPSTREAM";
const CACHE_CONTROL_HEADER: &str = "cache-control";
const CONTENT_SECURITY_POLICY_HEADER: &str = "content-security-policy";
const CROSS_ORIGIN_RESOURCE_POLICY_HEADER: &str = "cross-origin-resource-policy";
const PERMISSIONS_POLICY_HEADER: &str = "permissions-policy";
const REFERRER_POLICY_HEADER: &str = "referrer-policy";
const X_CONTENT_TYPE_OPTIONS_HEADER: &str = "x-content-type-options";
const X_FRAME_OPTIONS_HEADER: &str = "x-frame-options";
const DEFAULT_PERMISSIONS_POLICY: &str = "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()";
const LOCAL_APP_MODULES: &[&str] = &[
    "chat",
    "workspace",
    "orders",
    "shop",
    "calendar",
    "notary",
    "knowledge",
    "enterprise",
    "devices",
    "community",
    "voice",
    "agent",
    "course",
    "contacts",
    "favorites",
];
const PORTAL_SNAPSHOT_SECTIONS: &[&str] = &[
    "access",
    "automation",
    "conversations",
    "dashboard",
    "governance",
    "home",
    "media",
    "realtime",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductSiteDirs {
    pub admin_site_dir: PathBuf,
    pub portal_site_dir: PathBuf,
}

impl ProductSiteDirs {
    pub fn new(admin_site_dir: impl Into<PathBuf>, portal_site_dir: impl Into<PathBuf>) -> Self {
        Self {
            admin_site_dir: admin_site_dir.into(),
            portal_site_dir: portal_site_dir.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterProductRuntimeOptions {
    pub site_dirs: ProductSiteDirs,
}

impl RouterProductRuntimeOptions {
    pub fn desktop(site_dirs: ProductSiteDirs) -> Self {
        Self { site_dirs }
    }
}

#[derive(Debug)]
pub struct RouterProductRuntime {
    base_url: String,
    shutdown_tx: Option<oneshot::Sender<()>>,
    _site_dirs: ProductSiteDirs,
}

#[derive(Clone)]
struct RuntimeProxyState {
    client: Client,
    admin_proxy_target: String,
    admin_sandbox: Option<SharedAdminSandboxState>,
    pc_product_api_upstream: String,
    portal_api_base_url: String,
    local_iam: LocalIamState,
    site_dirs: ProductSiteDirs,
}

enum ResolvedSiteAsset {
    StaticFile(PathBuf),
    SpaShell(PathBuf),
}

impl RouterProductRuntime {
    pub async fn start(
        _loader: StandaloneConfigLoader,
        config: StandaloneConfig,
        options: RouterProductRuntimeOptions,
    ) -> Result<Self> {
        let listener = TcpListener::bind(resolve_runtime_bind_addr(
            config.runtime_bind_addr.as_str(),
        )?)
        .await
        .context("failed to bind local desktop runtime listener")?;
        let local_addr = listener
            .local_addr()
            .context("failed to resolve local desktop runtime listener address")?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let site_dirs = options.site_dirs.clone();
        let app = build_product_runtime_router(config, options).await?;

        tokio::spawn(async move {
            let _ = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await;
        });

        Ok(Self {
            base_url: format!("http://{local_addr}"),
            shutdown_tx: Some(shutdown_tx),
            _site_dirs: site_dirs,
        })
    }

    pub fn public_base_url(&self) -> Option<&str> {
        Some(self.base_url.as_str())
    }
}

async fn validate_product_site_dirs(site_dirs: ProductSiteDirs) -> Result<()> {
    validate_site_dir(site_dirs.admin_site_dir.as_path(), "admin").await?;
    validate_site_dir(site_dirs.portal_site_dir.as_path(), "portal").await?;
    Ok(())
}

pub async fn build_product_runtime_router(
    config: StandaloneConfig,
    options: RouterProductRuntimeOptions,
) -> Result<Router> {
    validate_product_site_dirs(options.site_dirs.clone()).await?;
    let site_dirs = options.site_dirs;
    let state = build_runtime_proxy_state(config, site_dirs.clone());

    Ok(local_iam_router()
        .route(BACKEND_ADMIN_API_PREFIX, any(proxy_admin_request))
        .route(
            format!("{BACKEND_ADMIN_API_PREFIX}/{{*path}}").as_str(),
            any(proxy_admin_request),
        )
        .route("/api/config/modules", get(get_local_app_modules))
        .route("/api/agent/{*path}", any(proxy_pc_product_api_request))
        .route("/app/v3/api/portal/workspace", get(get_portal_workspace))
        .route("/app/v3/api/portal/{section}", get(get_portal_snapshot))
        .route("/api", any(api_not_found))
        .route("/api/{*path}", any(api_not_found))
        .route("/admin", get(redirect_admin_root))
        .route("/admin/", get(serve_admin_site))
        .route("/admin/{*path}", get(serve_admin_site))
        .route("/", get(serve_portal_site))
        .route("/{*path}", get(serve_portal_site))
        .with_state(state))
}

fn build_runtime_proxy_state(
    config: StandaloneConfig,
    site_dirs: ProductSiteDirs,
) -> RuntimeProxyState {
    let admin_proxy_target = trim_trailing_slash(config.admin_proxy_target);
    let admin_sandbox = if admin_proxy_target.trim().is_empty() && config.admin_sandbox_enabled {
        let state = match config.admin_sandbox_storage_file {
            Some(storage_file) => SharedAdminSandboxState::seeded_with_storage_file(storage_file),
            None => SharedAdminSandboxState::seeded(),
        };
        eprintln!(
            "warning: SDKWORK_ADMIN_SANDBOX is enabled. Admin sandbox consumes sdkwork-appbase bearer tokens and does not provide craw-chat login endpoints."
        );
        Some(state)
    } else {
        None
    };

    RuntimeProxyState {
        client: Client::new(),
        admin_proxy_target,
        admin_sandbox,
        pc_product_api_upstream: resolve_pc_product_api_upstream(),
        portal_api_base_url: config.portal_api_base_url,
        local_iam: LocalIamState::default(),
        site_dirs,
    }
}

async fn validate_site_dir(site_dir: &StdPath, site_name: &str) -> Result<()> {
    let metadata = fs::metadata(site_dir).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site directory is missing: {}",
            site_dir.display()
        )
    })?;
    if !metadata.is_dir() {
        anyhow::bail!(
            "desktop runtime {site_name} site directory is not a directory: {}",
            site_dir.display()
        );
    }

    let index_path = site_dir.join("index.html");
    let index_metadata = fs::metadata(index_path.as_path()).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site is missing index.html: {}",
            index_path.display()
        )
    })?;
    if !index_metadata.is_file() {
        anyhow::bail!(
            "desktop runtime {site_name} site index.html is not a file: {}",
            index_path.display()
        );
    }

    Ok(())
}

impl Drop for RouterProductRuntime {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
    }
}

fn resolve_runtime_bind_addr(value: &str) -> Result<SocketAddr> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0));
    }

    trimmed
        .parse()
        .with_context(|| format!("invalid desktop runtime bind address: {trimmed}"))
}

fn trim_trailing_slash(value: String) -> String {
    value.trim().trim_end_matches('/').to_owned()
}

fn resolve_pc_product_api_upstream() -> String {
    std::env::var(PC_PRODUCT_API_UPSTREAM_ENV)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_default()
}

fn admin_proxy_path_and_query(uri: &Uri) -> String {
    uri.path_and_query()
        .map(|value| value.as_str())
        .unwrap_or(BACKEND_ADMIN_API_PREFIX)
        .to_owned()
}

async fn api_not_found() -> Response {
    json_error_response(StatusCode::NOT_FOUND, "Runtime route not found.")
}

async fn get_local_app_modules() -> Response {
    let modules = LOCAL_APP_MODULES
        .iter()
        .map(|module| format!("\"{}\"", escape_json_string(module)))
        .collect::<Vec<_>>()
        .join(",");
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(format!("{{\"modules\":[{modules}]}}")))
        .expect("local modules response should build")
}

async fn get_portal_snapshot(Path(section): Path<String>) -> Response {
    let section = section.trim();
    if !PORTAL_SNAPSHOT_SECTIONS.contains(&section) {
        return json_error_response(StatusCode::NOT_FOUND, "Portal snapshot route not found.");
    }

    json_response(StatusCode::OK, portal_snapshot_json(section))
}

async fn get_portal_workspace() -> Response {
    json_response(
        StatusCode::OK,
        r#"{"name":"Craw Chat Local","slug":"craw-chat-local","tier":"local","region":"local","supportPlan":"local","seats":1,"activeBrands":1,"uptime":"local"}"#,
    )
}

async fn redirect_admin_root() -> Redirect {
    Redirect::permanent("/admin/")
}

async fn serve_admin_site(State(state): State<RuntimeProxyState>, uri: Uri) -> Response {
    let request_path = uri.path().strip_prefix("/admin").unwrap_or("/");
    serve_site_request(state.site_dirs.admin_site_dir.as_path(), request_path).await
}

async fn serve_portal_site(State(state): State<RuntimeProxyState>, uri: Uri) -> Response {
    match resolve_site_request_asset(state.site_dirs.portal_site_dir.as_path(), uri.path()).await {
        Ok(ResolvedSiteAsset::StaticFile(path)) => serve_site_file(path.as_path()).await,
        Ok(ResolvedSiteAsset::SpaShell(path)) => {
            serve_portal_shell(path.as_path(), state.portal_api_base_url.as_str()).await
        }
        Err(response) => response,
    }
}

async fn serve_site_request(site_dir: &StdPath, request_path: &str) -> Response {
    match resolve_site_request_asset(site_dir, request_path).await {
        Ok(ResolvedSiteAsset::StaticFile(path) | ResolvedSiteAsset::SpaShell(path)) => {
            serve_site_file(path.as_path()).await
        }
        Err(response) => response,
    }
}

async fn resolve_site_request_asset(
    site_dir: &StdPath,
    request_path: &str,
) -> Result<ResolvedSiteAsset, Response> {
    let Some(relative_path) = sanitize_site_relative_path(request_path) else {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    };

    if relative_path.as_os_str().is_empty() {
        return Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")));
    }

    let candidate = site_dir.join(&relative_path);
    let top_level_index = relative_path == StdPath::new("index.html");
    match fs::metadata(&candidate).await {
        Ok(metadata) if metadata.is_file() => {
            return Ok(if top_level_index {
                ResolvedSiteAsset::SpaShell(candidate)
            } else {
                ResolvedSiteAsset::StaticFile(candidate)
            });
        }
        Ok(metadata) if metadata.is_dir() => {
            let nested_index = candidate.join("index.html");
            return Ok(ResolvedSiteAsset::StaticFile(nested_index));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to inspect runtime site asset: {error}"),
            ));
        }
    }

    if request_looks_like_static_asset(relative_path.as_path()) {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    }

    Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")))
}

fn sanitize_site_relative_path(request_path: &str) -> Option<PathBuf> {
    let trimmed = request_path.trim_start_matches('/');
    let mut normalized = PathBuf::new();

    for component in StdPath::new(trimmed).components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::RootDir => {}
            Component::ParentDir | Component::Prefix(_) => return None,
        }
    }

    Some(normalized)
}

fn request_looks_like_static_asset(relative_path: &StdPath) -> bool {
    relative_path.extension().is_some()
}

async fn serve_site_file(path: &StdPath) -> Response {
    match fs::read(path).await {
        Ok(body) => {
            let content_type = mime_guess::from_path(path)
                .first_or_octet_stream()
                .essence_str()
                .to_owned();
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(Body::from(body))
                .expect("runtime site file response should build");
            let is_html = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.starts_with("text/html"))
                .unwrap_or(false);
            if is_html {
                apply_html_shell_headers(response.headers_mut(), None);
            } else {
                apply_site_security_headers(response.headers_mut());
            }
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime site asset: {error}"),
        ),
    }
}

async fn serve_portal_shell(path: &StdPath, portal_api_base_url: &str) -> Response {
    match fs::read_to_string(path).await {
        Ok(html) => {
            let script_nonce = create_script_nonce();
            let injected = inject_portal_api_base_url(
                html.as_str(),
                portal_api_base_url,
                script_nonce.as_str(),
            );
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(injected))
                .expect("portal shell response should build");
            apply_html_shell_headers(
                response.headers_mut(),
                Some(HtmlShellSecurityPolicy::for_portal_shell(
                    portal_api_base_url,
                    script_nonce.as_str(),
                )),
            );
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime portal shell: {error}"),
        ),
    }
}

fn inject_portal_api_base_url(html: &str, portal_api_base_url: &str, script_nonce: &str) -> String {
    let html = apply_nonce_to_inline_portal_scripts(html, script_nonce);
    let serialized_url = serde_json::to_string(portal_api_base_url)
        .expect("portal api base url should serialize into javascript");
    let script = format!(
        "<script nonce=\"{script_nonce}\">window.__CRAW_CHAT_PORTAL_API_BASE_URL__ = {serialized_url};</script>"
    );

    if let Some(head_close_index) = html.find("</head>") {
        let mut injected = String::with_capacity(html.len() + script.len());
        injected.push_str(&html[..head_close_index]);
        injected.push_str(script.as_str());
        injected.push_str(&html[head_close_index..]);
        return injected;
    }

    format!("{script}{html}")
}

fn apply_nonce_to_inline_portal_scripts(html: &str, script_nonce: &str) -> String {
    let mut result = String::with_capacity(html.len() + 64);
    let mut cursor = 0;

    while let Some(relative_start) = html[cursor..].find("<script") {
        let start = cursor + relative_start;
        result.push_str(&html[cursor..start]);

        let Some(relative_end) = html[start..].find('>') else {
            result.push_str(&html[start..]);
            return result;
        };
        let end = start + relative_end + 1;
        let opening_tag = &html[start..end];

        if script_tag_requires_runtime_nonce(opening_tag) {
            let tag_without_close = &opening_tag[..opening_tag.len() - 1];
            result.push_str(tag_without_close);
            result.push_str(format!(" nonce=\"{script_nonce}\">").as_str());
        } else {
            result.push_str(opening_tag);
        }

        cursor = end;
    }

    result.push_str(&html[cursor..]);
    result
}

fn script_tag_requires_runtime_nonce(opening_tag: &str) -> bool {
    let normalized = opening_tag.to_ascii_lowercase();
    let is_importmap = normalized.contains(r#"type="importmap""#)
        || normalized.contains("type='importmap'")
        || normalized.contains("type=importmap");

    is_importmap && !normalized.contains(" src=") && !normalized.contains(" nonce=")
}

struct HtmlShellSecurityPolicy {
    connect_src: String,
    script_nonce: Option<String>,
}

impl HtmlShellSecurityPolicy {
    fn default_shell() -> Self {
        Self {
            connect_src: "'self'".to_owned(),
            script_nonce: None,
        }
    }

    fn for_portal_shell(portal_api_base_url: &str, script_nonce: &str) -> Self {
        Self {
            connect_src: resolve_connect_src(portal_api_base_url),
            script_nonce: Some(script_nonce.to_owned()),
        }
    }
}

fn create_script_nonce() -> String {
    format!("{:032x}", random::<u128>())
}

fn apply_site_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        X_CONTENT_TYPE_OPTIONS_HEADER,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        REFERRER_POLICY_HEADER,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(X_FRAME_OPTIONS_HEADER, HeaderValue::from_static("DENY"));
    headers.insert(
        PERMISSIONS_POLICY_HEADER,
        HeaderValue::from_static(DEFAULT_PERMISSIONS_POLICY),
    );
    headers.insert(
        CROSS_ORIGIN_RESOURCE_POLICY_HEADER,
        HeaderValue::from_static("same-origin"),
    );
}

fn apply_html_shell_headers(headers: &mut HeaderMap, policy: Option<HtmlShellSecurityPolicy>) {
    let policy = policy.unwrap_or_else(HtmlShellSecurityPolicy::default_shell);
    apply_site_security_headers(headers);
    headers.insert(CACHE_CONTROL_HEADER, HeaderValue::from_static("no-store"));
    headers.insert(
        CONTENT_SECURITY_POLICY_HEADER,
        HeaderValue::from_str(create_html_content_security_policy(policy).as_str())
            .expect("html shell content security policy should be valid"),
    );
}

fn create_html_content_security_policy(policy: HtmlShellSecurityPolicy) -> String {
    let script_src = match policy.script_nonce.as_deref() {
        Some(script_nonce) => format!("'self' 'nonce-{script_nonce}'"),
        None => "'self'".to_owned(),
    };

    format!(
        "default-src 'self'; base-uri 'self'; connect-src {}; font-src 'self' data:; form-action 'self'; frame-ancestors 'none'; img-src 'self' data: blob:; object-src 'none'; script-src {}; style-src 'self' 'unsafe-inline'",
        policy.connect_src, script_src
    )
}

fn resolve_connect_src(portal_api_base_url: &str) -> String {
    let mut sources = vec!["'self'".to_owned()];

    if let Ok(url) = Url::parse(portal_api_base_url) {
        if matches!(url.scheme(), "http" | "https") {
            let origin = url.origin().ascii_serialization();
            push_unique_source(&mut sources, origin);
            if let Some(websocket_origin) = websocket_origin_for_url(&url) {
                push_unique_source(&mut sources, websocket_origin);
            }
        }
    }

    sources.join(" ")
}

fn push_unique_source(sources: &mut Vec<String>, value: String) {
    if !sources.iter().any(|existing| existing == &value) {
        sources.push(value);
    }
}

fn websocket_origin_for_url(url: &Url) -> Option<String> {
    let websocket_scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        _ => return None,
    };
    let host = match url.host()? {
        Host::Domain(value) => value.to_owned(),
        Host::Ipv4(value) => value.to_string(),
        Host::Ipv6(value) => format!("[{value}]"),
    };

    match url.port() {
        Some(port) => Some(format!("{websocket_scheme}://{host}:{port}")),
        None => Some(format!("{websocket_scheme}://{host}")),
    }
}

async fn proxy_admin_request(
    State(state): State<RuntimeProxyState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    if let Some(admin_sandbox) = &state.admin_sandbox {
        return handle_admin_sandbox_request(admin_sandbox, method, headers, uri, body).await;
    }

    if state.admin_proxy_target.trim().is_empty() {
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE,
        );
    }

    let upstream_url = format!(
        "{}{}",
        state.admin_proxy_target,
        admin_proxy_path_and_query(&uri),
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("desktop admin proxy request failed: {error}").as_str(),
        ),
    }
}

async fn proxy_pc_product_api_request(
    State(state): State<RuntimeProxyState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    if state.pc_product_api_upstream.trim().is_empty() {
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            format!(
                "PC product API upstream is not configured. Set {PC_PRODUCT_API_UPSTREAM_ENV} to a backend that serves /api/agent/*."
            )
            .as_str(),
        );
    }

    let upstream_url = format!(
        "{}{}",
        state.pc_product_api_upstream,
        uri.path_and_query()
            .map(|value| value.as_str())
            .unwrap_or("/api/agent"),
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("PC product API proxy request failed: {error}").as_str(),
        ),
    }
}

fn portal_snapshot_json(section: &str) -> String {
    let modules = LOCAL_APP_MODULES
        .iter()
        .map(|module| format!("\"{}\"", escape_json_string(module)))
        .collect::<Vec<_>>()
        .join(",");
    let section = escape_json_string(section);
    format!(
        concat!(
            "{{",
            "\"section\":\"{section}\",",
            "\"enabledModules\":[{modules}],",
            "\"sidebarModules\":[{modules}],",
            "\"modules\":{{\"items\":[{modules}]}},",
            "\"organizationDirectory\":{{",
            "\"departments\":[",
            "{{\"id\":\"dept-root\",\"name\":\"Craw Chat\",\"parentId\":null,\"order\":0}},",
            "{{\"id\":\"dept-product\",\"name\":\"Product\",\"parentId\":\"dept-root\",\"order\":10}},",
            "{{\"id\":\"dept-support\",\"name\":\"Support\",\"parentId\":\"dept-root\",\"order\":20}}",
            "]",
            "}},",
            "\"features\":{{",
            "\"chat\":true,",
            "\"contacts\":true,",
            "\"workspace\":true",
            "}}",
            "}}"
        ),
        section = section,
        modules = modules,
    )
}

fn json_response(status: StatusCode, body: impl Into<String>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(body.into()))
        .expect("json runtime response should build")
}

fn text_response(status: StatusCode, message: impl Into<String>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(message.into()))
        .expect("text runtime response should build")
}

fn json_error_response(status: StatusCode, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(format!(
            "{{\"error\":{{\"message\":\"{}\"}},\"status\":{}}}",
            escape_json_string(message),
            status.as_u16()
        )))
        .expect("json proxy error response should build")
}

fn escape_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

async fn build_proxy_response(upstream_response: reqwest::Response) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let body = upstream_response.bytes().await.unwrap_or_default();
    let mut response_builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        if *name == header::TRANSFER_ENCODING || *name == header::CONNECTION {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(Body::from(body))
        .expect("proxied admin response should build")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestSiteDir {
        path: PathBuf,
    }

    impl TestSiteDir {
        fn new(label: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos();
            let path =
                std::env::temp_dir().join(format!("sdkwork-api-product-runtime-{label}-{unique}"));
            fs::create_dir_all(&path).expect("test site dir should be creatable");
            Self { path }
        }

        fn path(&self) -> &Path {
            self.path.as_path()
        }

        fn write(&self, relative_path: &str, body: &str) {
            let file_path = self.path.join(relative_path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("test file parent dir should be creatable");
            }
            fs::write(file_path, body).expect("test site file should be writable");
        }
    }

    impl Drop for TestSiteDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    async fn start_runtime(site_dirs: ProductSiteDirs) -> RouterProductRuntime {
        RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(site_dirs),
        )
        .await
        .expect("desktop product runtime should start")
    }

    async fn fetch_response(base_url: &str, path: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{base_url}{path}"))
            .send()
            .await
            .expect("runtime request should succeed")
    }

    fn response_header(response: &reqwest::Response, name: &str) -> Option<String> {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    }

    async fn parse_json_response(
        response: reqwest::Response,
        description: &str,
    ) -> serde_json::Value {
        let body = response
            .text()
            .await
            .unwrap_or_else(|error| panic!("{description} body should be readable: {error}"));
        serde_json::from_str(body.as_str()).unwrap_or_else(|error| {
            panic!("{description} should be valid JSON: {error}; body: {body}")
        })
    }

    #[tokio::test]
    async fn proxy_admin_request_returns_structured_503_when_backend_target_is_missing() {
        let response = proxy_admin_request(
            State(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target: String::new(),
                admin_sandbox: None,
                pc_product_api_upstream: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                local_iam: LocalIamState::default(),
                site_dirs: ProductSiteDirs::new(".", "."),
            }),
            Method::GET,
            HeaderMap::new(),
            Uri::from_static("/backend/v3/api/admin/storage/config"),
            Bytes::new(),
        )
        .await;

        let status = response.status();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let body_text = String::from_utf8(body.to_vec()).expect("response body should be utf8");

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            content_type.as_deref(),
            Some("application/json; charset=utf-8")
        );
        assert!(body_text.contains("SDKWORK_ADMIN_PROXY_TARGET"));
        assert!(body_text.contains("/backend/v3/api/admin"));
    }

    #[tokio::test]
    async fn router_runtime_serves_portal_root_and_admin_shell_with_spa_fallbacks() {
        let admin_site_dir = TestSiteDir::new("admin-site");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");
        admin_site_dir.write("assets/admin.js", "console.log('admin-asset');");

        let portal_site_dir = TestSiteDir::new("portal-site");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");
        portal_site_dir.write("assets/portal.js", "console.log('portal-asset');");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_index = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_index.status(), StatusCode::OK);
        assert!(admin_index
            .text()
            .await
            .expect("admin index body should be readable")
            .contains("admin-shell"));

        let admin_route = fetch_response(base_url.as_str(), "/admin/operators/shift").await;
        assert_eq!(admin_route.status(), StatusCode::OK);
        assert!(admin_route
            .text()
            .await
            .expect("admin route body should be readable")
            .contains("admin-shell"));

        let admin_asset = fetch_response(base_url.as_str(), "/admin/assets/admin.js").await;
        assert_eq!(admin_asset.status(), StatusCode::OK);
        assert_eq!(
            admin_asset
                .text()
                .await
                .expect("admin asset body should be readable"),
            "console.log('admin-asset');"
        );

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        assert!(portal_index
            .text()
            .await
            .expect("portal index body should be readable")
            .contains("portal-shell"));

        let portal_route = fetch_response(base_url.as_str(), "/workspace/inbox").await;
        assert_eq!(portal_route.status(), StatusCode::OK);
        assert!(portal_route
            .text()
            .await
            .expect("portal route body should be readable")
            .contains("portal-shell"));

        let portal_asset = fetch_response(base_url.as_str(), "/assets/portal.js").await;
        assert_eq!(portal_asset.status(), StatusCode::OK);
        assert_eq!(
            portal_asset
                .text()
                .await
                .expect("portal asset body should be readable"),
            "console.log('portal-asset');"
        );
    }

    #[tokio::test]
    async fn router_runtime_injects_portal_api_base_url_into_portal_shell() {
        let admin_site_dir = TestSiteDir::new("admin-injection");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("portal-injection");
        portal_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>portal-shell</title></head><body>portal</body></html>",
        );

        let runtime = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "https://portal-api.example.com/runtime-edge".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect("desktop product runtime should start");
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        let content_security_policy =
            response_header(&portal_index, CONTENT_SECURITY_POLICY_HEADER)
                .expect("portal shell should include a content security policy");
        assert!(content_security_policy.contains("https://portal-api.example.com"));
        assert!(content_security_policy.contains("wss://portal-api.example.com"));
        assert_eq!(
            response_header(&portal_index, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );
        assert_eq!(
            response_header(&portal_index, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&portal_index, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&portal_index, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&portal_index, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&portal_index, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        let body = portal_index
            .text()
            .await
            .expect("portal index body should be readable");
        assert!(body.contains("__CRAW_CHAT_PORTAL_API_BASE_URL__"));
        assert!(body.contains("https://portal-api.example.com/runtime-edge"));
        let nonce_start = body
            .find("script nonce=\"")
            .expect("portal shell should inject a nonce-backed script")
            + "script nonce=\"".len();
        let nonce_end = body[nonce_start..]
            .find('"')
            .map(|offset| nonce_start + offset)
            .expect("portal shell nonce should terminate");
        let nonce = &body[nonce_start..nonce_end];
        assert!(content_security_policy.contains(format!("'nonce-{nonce}'").as_str()));
        assert!(content_security_policy.contains("script-src 'self' 'nonce-"));
        assert!(!content_security_policy.contains("script-src 'self' 'unsafe-inline'"));
    }

    #[test]
    fn portal_shell_injection_applies_runtime_nonce_to_inline_importmap_scripts() {
        let html = r#"<!doctype html><html><head><script type="importmap">{ "imports": { "@sdkwork/sdk-common": "/__vendor__/sdkwork-sdk-common/index.js" } }</script></head><body>portal</body></html>"#;

        let injected = inject_portal_api_base_url(
            html,
            "https://portal-api.example.com/runtime-edge",
            "nonce123",
        );

        assert!(injected.contains(r#"<script type="importmap" nonce="nonce123">"#));
    }

    #[tokio::test]
    async fn router_runtime_applies_security_headers_to_admin_shells_and_static_assets() {
        let admin_site_dir = TestSiteDir::new("admin-security");
        admin_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>admin-shell</title></head><body>admin</body></html>",
        );

        let portal_site_dir = TestSiteDir::new("portal-security");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");
        portal_site_dir.write("assets/portal.js", "console.log('portal-asset');");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_shell = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_shell.status(), StatusCode::OK);
        let admin_csp = response_header(&admin_shell, CONTENT_SECURITY_POLICY_HEADER)
            .expect("admin shell should emit a content security policy");
        assert!(admin_csp.contains("connect-src 'self'"));
        assert!(!admin_csp.contains("nonce-"));
        assert_eq!(
            response_header(&admin_shell, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );

        let portal_asset = fetch_response(base_url.as_str(), "/assets/portal.js").await;
        assert_eq!(portal_asset.status(), StatusCode::OK);
        assert_eq!(
            response_header(&portal_asset, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&portal_asset, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&portal_asset, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&portal_asset, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&portal_asset, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        assert_eq!(
            response_header(&portal_asset, CONTENT_SECURITY_POLICY_HEADER),
            None
        );
        assert_eq!(response_header(&portal_asset, CACHE_CONTROL_HEADER), None);
    }

    #[tokio::test]
    async fn router_runtime_refuses_to_start_without_admin_index_html() {
        let admin_site_dir = TestSiteDir::new("admin-missing-index");
        let portal_site_dir = TestSiteDir::new("portal-valid-index");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let error = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect_err("runtime should fail fast when admin index is missing");

        assert!(error.to_string().contains("admin"));
        assert!(error.to_string().contains("index.html"));
    }

    #[tokio::test]
    async fn router_runtime_refuses_to_start_without_portal_index_html() {
        let admin_site_dir = TestSiteDir::new("admin-valid-index");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");
        let portal_site_dir = TestSiteDir::new("portal-missing-index");

        let error = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
                admin_sandbox_storage_file: None,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect_err("runtime should fail fast when portal index is missing");

        assert!(error.to_string().contains("portal"));
        assert!(error.to_string().contains("index.html"));
    }

    #[tokio::test]
    async fn router_runtime_keeps_api_and_missing_assets_outside_spa_fallback() {
        let admin_site_dir = TestSiteDir::new("admin-api-guard");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("portal-api-guard");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let missing_admin_asset =
            fetch_response(base_url.as_str(), "/admin/assets/missing.js").await;
        assert_eq!(missing_admin_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_admin_asset
            .text()
            .await
            .expect("missing admin asset body should be readable")
            .contains("admin-shell"));

        let missing_portal_asset = fetch_response(base_url.as_str(), "/assets/missing.js").await;
        assert_eq!(missing_portal_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_portal_asset
            .text()
            .await
            .expect("missing portal asset body should be readable")
            .contains("portal-shell"));

        let unknown_api = fetch_response(base_url.as_str(), "/api/runtime-health").await;
        assert_eq!(unknown_api.status(), StatusCode::NOT_FOUND);
        assert!(!unknown_api
            .text()
            .await
            .expect("unknown api body should be readable")
            .contains("portal-shell"));

        let modules_api = fetch_response(base_url.as_str(), "/api/config/modules").await;
        assert_eq!(modules_api.status(), StatusCode::OK);
        let modules_body = modules_api
            .text()
            .await
            .expect("modules api body should be readable");
        assert!(modules_body.contains("\"chat\""));
        assert!(modules_body.contains("\"knowledge\""));

        let agent_api = reqwest::Client::new()
            .post(format!("{base_url}/api/agent/doc"))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(r#"{"action":"summarize","content":"hello"}"#)
            .send()
            .await
            .expect("agent api request should complete");
        assert_eq!(agent_api.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(agent_api
            .text()
            .await
            .expect("agent api body should be readable")
            .contains("CRAW_CHAT_PC_API_UPSTREAM"));

        let admin_api =
            fetch_response(base_url.as_str(), "/backend/v3/api/admin/storage/config").await;
        assert_eq!(admin_api.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(admin_api
            .text()
            .await
            .expect("admin api body should be readable")
            .contains("SDKWORK_ADMIN_PROXY_TARGET"));
    }

    #[tokio::test]
    async fn router_runtime_serves_sdkwork_app_portal_home_snapshot() {
        let admin_site_dir = TestSiteDir::new("portal-home-admin");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("portal-home-portal");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let response = fetch_response(base_url.as_str(), "/app/v3/api/portal/home").await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response_header(&response, header::CONTENT_TYPE.as_str()).as_deref(),
            Some(JSON_CONTENT_TYPE),
        );
        let value = parse_json_response(response, "portal home snapshot").await;
        assert!(
            value["enabledModules"]
                .as_array()
                .expect("portal home should include enabledModules")
                .contains(&serde_json::json!("chat")),
            "portal home snapshot must expose modules consumable by SettingsService"
        );
        assert!(
            value["organizationDirectory"]["departments"]
                .as_array()
                .expect("portal home should include legacy department records")
                .iter()
                .any(|department| department["id"] == "dept-root"),
            "portal home snapshot may keep legacy department hints, but the organization directory is served by IAM endpoints"
        );
    }

    #[tokio::test]
    async fn router_runtime_serves_local_iam_organization_department_directory() {
        let admin_site_dir = TestSiteDir::new("iam-org-admin");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("iam-org-portal");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let organizations =
            fetch_response(base_url.as_str(), "/app/v3/api/iam/organizations").await;
        assert_eq!(organizations.status(), StatusCode::OK);
        let organizations = parse_json_response(organizations, "organizations").await;
        assert_eq!(organizations["code"], "2000");
        assert_eq!(organizations["msg"], "SUCCESS");
        let organization_items = organizations["data"]["items"]
            .as_array()
            .expect("organizations should include items");
        assert!(
            organization_items.iter().any(|organization| {
                organization["organizationId"] == "sdkwork-local-org"
                    && organization["tenantId"] == "sdkwork-local-tenant"
                    && organization["parentOrganizationId"] == "sdkwork-local-group"
                    && organization["tenantBoundaryKind"] == "sub_tenant"
                    && organization["dataBoundaryKind"] == "organization_isolated"
                    && organization["appBoundaryEnabled"] == true
            }),
            "organizations must model tenant-scoped operating subjects with parent organization and sub-tenant boundaries"
        );
        assert_eq!(organizations["data"]["iam_accounts"], serde_json::Value::Null);

        let organization_tree =
            fetch_response(base_url.as_str(), "/app/v3/api/iam/organizations/tree").await;
        assert_eq!(organization_tree.status(), StatusCode::OK);
        let organization_tree =
            parse_json_response(organization_tree, "organization tree").await;
        assert_eq!(
            organization_tree["data"]["items"][0]["children"][0]["organizationId"],
            "sdkwork-local-org"
        );

        let departments = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/departments?organizationId=sdkwork-local-org",
        )
        .await;
        assert_eq!(departments.status(), StatusCode::OK);
        let departments = parse_json_response(departments, "departments").await;
        let department_items = departments["data"]["items"]
            .as_array()
            .expect("departments should include items");
        assert!(
            department_items.iter().any(|department| {
                department["departmentId"] == "dept-product"
                    && department["organizationId"] == "sdkwork-local-org"
                    && department["parentDepartmentId"] == "dept-root"
                    && department["departmentKind"] == "department"
            }),
            "departments must be independent organization-internal nodes, not iam_organizations rows"
        );
        assert_eq!(departments["data"]["iam_department_members"], serde_json::Value::Null);

        let department_tree = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/departments/tree?organizationId=sdkwork-local-org",
        )
        .await;
        assert_eq!(department_tree.status(), StatusCode::OK);
        let department_tree = parse_json_response(department_tree, "department tree").await;
        assert_eq!(
            department_tree["data"]["items"][0]["children"][0]["departmentId"],
            "dept-product"
        );

        let memberships = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/organization_memberships?organizationId=sdkwork-local-org",
        )
        .await;
        assert_eq!(memberships.status(), StatusCode::OK);
        let memberships = parse_json_response(memberships, "memberships").await;
        assert_eq!(
            memberships["data"]["items"][0]["membershipId"],
            "membership-local-default"
        );
        assert_eq!(memberships["data"]["items"][0]["membershipType"], "employee");

        let assignments = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/department_assignments?departmentId=dept-product",
        )
        .await;
        assert_eq!(assignments.status(), StatusCode::OK);
        let assignments = parse_json_response(assignments, "department assignments").await;
        let assignment = &assignments["data"]["items"][0];
        assert_eq!(assignment["assignmentId"], "assignment-local-default-product");
        assert_eq!(assignment["membershipId"], "membership-local-default");
        assert_eq!(assignment["departmentId"], "dept-product");
        assert_eq!(assignment["positionName"], "Product Owner");
        assert!(
            assignment["roleCodes"]
                .as_array()
                .expect("department assignment should include roleCodes")
                .contains(&serde_json::json!("department.product_owner")),
            "department assignments must expose role summaries for directory UX"
        );

        let positions = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/positions?departmentId=dept-product",
        )
        .await;
        assert_eq!(positions.status(), StatusCode::OK);
        let positions = parse_json_response(positions, "positions").await;
        assert_eq!(positions["data"]["items"][0]["positionId"], "position-product-owner");

        let position_assignments = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/position_assignments?departmentAssignmentId=assignment-local-default-product",
        )
        .await;
        assert_eq!(position_assignments.status(), StatusCode::OK);
        let position_assignments =
            parse_json_response(position_assignments, "position assignments").await;
        assert_eq!(
            position_assignments["data"]["items"][0]["positionAssignmentId"],
            "position-assignment-local-default-product-owner"
        );

        let role_bindings = fetch_response(
            base_url.as_str(),
            "/app/v3/api/iam/role_bindings?scopeKind=department&scopeId=dept-product",
        )
        .await;
        assert_eq!(role_bindings.status(), StatusCode::OK);
        let role_bindings = parse_json_response(role_bindings, "role bindings").await;
        assert_eq!(
            role_bindings["data"]["items"][0]["principalKind"],
            "department_assignment"
        );
        assert_eq!(
            role_bindings["data"]["items"][0]["roleCode"],
            "department.product_owner"
        );
    }

    #[tokio::test]
    async fn router_runtime_serves_local_iam_qr_auth_flow() {
        let admin_site_dir = TestSiteDir::new("iam-qr-admin");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("iam-qr-portal");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();
        let client = reqwest::Client::new();

        let created = client
            .post(format!(
                "{base_url}/app/v3/api/open_platform/qr_auth/sessions"
            ))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(serde_json::json!({}).to_string())
            .send()
            .await
            .expect("create QR auth session request should complete");
        assert_eq!(created.status(), StatusCode::OK);
        let created = parse_json_response(created, "QR auth session").await;
        assert_eq!(created["code"], "2000");
        assert_eq!(created["msg"], "SUCCESS");
        let created_data = &created["data"];
        let session_key = created_data["sessionKey"]
            .as_str()
            .expect("QR auth session should include sessionKey")
            .to_owned();
        assert!(!session_key.is_empty());
        assert!(!session_key.starts_with("local-qr-"));
        assert_eq!(created_data["id"], format!("qr_auth_session_{session_key}"));
        assert_eq!(created_data["purpose"], "login");
        assert_eq!(created_data["defaultAccountId"], serde_json::Value::Null);
        assert_eq!(created_data["defaultEntryId"], serde_json::Value::Null);
        assert_eq!(created_data["defaultProvider"], serde_json::Value::Null);
        assert_eq!(created_data["defaultAccountType"], serde_json::Value::Null);
        assert_eq!(created_data["status"], "pending");
        assert_eq!(created_data["qrContent"]["mode"], "fallback_url");
        let fallback_url = created_data["fallbackUrl"]
            .as_str()
            .expect("QR auth session should include fallbackUrl");
        assert_eq!(created_data["qrContent"]["content"], fallback_url);
        assert!(fallback_url.contains(format!("/auth/qr/{session_key}").as_str()));
        assert!(fallback_url.contains(format!("session_key={session_key}").as_str()));
        assert!(fallback_url.contains("purpose=login"));
        assert!(fallback_url.contains("scan_source=browser"));
        assert_eq!(created_data["scannedAt"], serde_json::Value::Null);
        assert_eq!(created_data["completedAt"], serde_json::Value::Null);
        assert!(created_data["createdAt"].as_str().is_some());
        assert!(created_data["updatedAt"].as_str().is_some());

        let retrieved = fetch_response(
            base_url.as_str(),
            format!("/app/v3/api/open_platform/qr_auth/sessions/{session_key}").as_str(),
        )
        .await;
        assert_eq!(retrieved.status(), StatusCode::OK);
        let retrieved = parse_json_response(retrieved, "retrieved QR auth session").await;
        assert_eq!(retrieved["code"], "2000");
        assert_eq!(retrieved["msg"], "SUCCESS");
        assert_eq!(retrieved["data"]["sessionKey"], session_key);
        assert_eq!(retrieved["data"]["status"], "pending");
        assert_eq!(retrieved["data"]["qrContent"]["mode"], "fallback_url");

        let scanned = client
            .post(format!(
                "{base_url}/app/v3/api/open_platform/qr_auth/sessions/{session_key}/scans"
            ))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(serde_json::json!({"deviceId": "pc-test"}).to_string())
            .send()
            .await
            .expect("scan QR auth session request should complete");
        assert_eq!(scanned.status(), StatusCode::OK);
        let scanned = parse_json_response(scanned, "scanned QR auth session").await;
        assert_eq!(scanned["code"], "2000");
        assert_eq!(scanned["msg"], "SUCCESS");
        assert_eq!(scanned["data"]["status"], "scanned");
        assert_eq!(scanned["data"]["scannedAt"].is_string(), true);
        assert_eq!(scanned["data"]["completedAt"], serde_json::Value::Null);

        let auth_session = client
            .post(format!(
                "{base_url}/app/v3/api/open_platform/qr_auth/sessions/{session_key}/passwords"
            ))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(
                serde_json::json!({
                    "username": "local-default@sdkwork-iam.local",
                    "password": "dev123456",
                    "deviceId": "pc-test",
                })
                .to_string(),
            )
            .send()
            .await
            .expect("QR password auth request should complete");
        assert_eq!(auth_session.status(), StatusCode::OK);
        let auth_session = parse_json_response(auth_session, "QR password auth session").await;
        assert_eq!(auth_session["code"], "2000");
        assert_eq!(auth_session["msg"], "SUCCESS");
        let auth_session_data = &auth_session["data"];
        assert_eq!(auth_session_data["sessionKey"], session_key);
        assert_eq!(auth_session_data["status"], "completed");
        assert_eq!(auth_session_data["completedAt"].is_string(), true);
        let auth_session = &auth_session_data["session"];
        assert!(auth_session["accessToken"]
            .as_str()
            .expect("auth session should include accessToken")
            .starts_with("local-access-"));
        assert!(auth_session["authToken"]
            .as_str()
            .expect("auth session should include authToken")
            .starts_with("local-auth-"));
        assert!(auth_session["refreshToken"].as_str().is_some());
        assert!(auth_session["sessionId"].as_str().is_some());
        assert_eq!(auth_session["context"]["environment"], "dev");
        assert_eq!(auth_session["context"]["deploymentMode"], "local");
        assert_eq!(auth_session["context"]["authLevel"], "password");
        assert_eq!(auth_session["context"]["deviceId"], "pc-test");
        assert_eq!(auth_session["context"]["userId"], "U1000000000");
        assert_eq!(auth_session["user"]["id"], "U1000000000");
        assert_eq!(auth_session["user"]["userId"], "U1000000000");
        assert_eq!(
            auth_session["user"]["username"],
            "local-default@sdkwork-iam.local"
        );

        let access_token = auth_session["accessToken"]
            .as_str()
            .expect("auth session should include accessToken");
        let current_session = client
            .get(format!("{base_url}/app/v3/api/auth/sessions/current"))
            .header("Access-Token", access_token)
            .send()
            .await
            .expect("current session request should complete");
        assert_eq!(current_session.status(), StatusCode::OK);
        let current_session = parse_json_response(current_session, "current session").await;
        assert_eq!(current_session["code"], "2000");
        assert_eq!(current_session["msg"], "SUCCESS");
        assert_eq!(
            current_session["data"]["sessionId"],
            auth_session["sessionId"]
        );

        let current_user = client
            .get(format!("{base_url}/app/v3/api/iam/users/current"))
            .header("Access-Token", access_token)
            .send()
            .await
            .expect("current user request should complete");
        assert_eq!(current_user.status(), StatusCode::OK);
        let current_user = parse_json_response(current_user, "current user").await;
        assert_eq!(current_user["code"], "2000");
        assert_eq!(current_user["msg"], "SUCCESS");
        assert_eq!(
            current_user["data"]["id"],
            auth_session["context"]["userId"]
        );
    }

    #[tokio::test]
    async fn router_runtime_serves_local_iam_password_login_metadata_and_verification() {
        let admin_site_dir = TestSiteDir::new("iam-password-admin");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("iam-password-portal");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();
        let client = reqwest::Client::new();

        let runtime_info =
            fetch_response(base_url.as_str(), "/app/v3/api/system/iam/runtime").await;
        assert_eq!(runtime_info.status(), StatusCode::OK);
        let runtime_info = parse_json_response(runtime_info, "runtime info").await;
        assert_eq!(runtime_info["code"], "2000");
        assert_eq!(runtime_info["msg"], "SUCCESS");
        let runtime_info = &runtime_info["data"];
        assert_eq!(runtime_info["appId"], "sdkwork-chat-pc");
        assert_eq!(runtime_info["environment"], "dev");
        assert_eq!(runtime_info["deploymentMode"], "local");
        assert_eq!(runtime_info["qrAuthEnabled"], true);

        let verification_policy = fetch_response(
            base_url.as_str(),
            "/app/v3/api/system/iam/verification_policy",
        )
        .await;
        assert_eq!(verification_policy.status(), StatusCode::OK);
        let verification_policy =
            parse_json_response(verification_policy, "verification policy").await;
        assert_eq!(verification_policy["code"], "2000");
        assert_eq!(verification_policy["msg"], "SUCCESS");
        let verification_policy = &verification_policy["data"];
        assert_eq!(verification_policy["password"], true);
        assert_eq!(verification_policy["email"], false);
        assert_eq!(verification_policy["captcha"], false);

        let verification_code = client
            .post(format!("{base_url}/app/v3/api/auth/verification_codes"))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(
                serde_json::json!({
                    "target": "local-default@sdkwork-iam.local",
                    "scene": "login",
                    "verifyType": "email",
                })
                .to_string(),
            )
            .send()
            .await
            .expect("verification code request should complete");
        assert_eq!(verification_code.status(), StatusCode::OK);
        let verification_code =
            parse_json_response(verification_code, "verification code response").await;
        assert_eq!(verification_code["code"], "2000");
        assert_eq!(verification_code["msg"], "SUCCESS");
        let verification_code = &verification_code["data"];
        assert!(verification_code["codeId"].as_str().is_some());

        let verification = client
            .post(format!(
                "{base_url}/app/v3/api/auth/verification_codes/verify"
            ))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(
                serde_json::json!({
                    "target": "local-default@sdkwork-iam.local",
                    "scene": "login",
                    "verifyType": "email",
                    "code": "123456",
                })
                .to_string(),
            )
            .send()
            .await
            .expect("verification request should complete");
        assert_eq!(verification.status(), StatusCode::OK);
        let verification = parse_json_response(verification, "verification response").await;
        assert_eq!(verification["code"], "2000");
        assert_eq!(verification["msg"], "SUCCESS");
        let verification = &verification["data"];
        assert_eq!(verification["verified"], true);

        let auth_session = client
            .post(format!("{base_url}/app/v3/api/auth/sessions"))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(
                serde_json::json!({
                    "username": "local-default@sdkwork-iam.local",
                    "password": "dev123456",
                    "remember": true,
                })
                .to_string(),
            )
            .send()
            .await
            .expect("password auth request should complete");
        assert_eq!(auth_session.status(), StatusCode::OK);
        let auth_session = parse_json_response(auth_session, "password auth session").await;
        assert_eq!(auth_session["code"], "2000");
        assert_eq!(auth_session["msg"], "SUCCESS");
        let auth_session = &auth_session["data"];
        assert_eq!(auth_session["context"]["userId"], "U1000000000");
        assert_eq!(auth_session["user"]["id"], "U1000000000");
        assert_eq!(auth_session["user"]["userId"], "U1000000000");
        assert_eq!(auth_session["context"]["tenantId"], "sdkwork-local-tenant");
        assert!(auth_session["context"]["dataScope"]
            .as_array()
            .expect("dataScope should be an array")
            .contains(&serde_json::json!("local")));
        assert!(auth_session["context"]["permissionScope"]
            .as_array()
            .expect("permissionScope should be an array")
            .contains(&serde_json::json!("*")));

        let refreshed = client
            .post(format!("{base_url}/app/v3/api/auth/sessions/refresh"))
            .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
            .body(
                serde_json::json!({
                    "refreshToken": auth_session["refreshToken"],
                })
                .to_string(),
            )
            .send()
            .await
            .expect("refresh auth request should complete");
        assert_eq!(refreshed.status(), StatusCode::OK);
        let refreshed = parse_json_response(refreshed, "refreshed auth session").await;
        assert_eq!(refreshed["code"], "2000");
        assert_eq!(refreshed["msg"], "SUCCESS");
        let refreshed = &refreshed["data"];
        assert_eq!(refreshed["sessionId"], auth_session["sessionId"]);
    }

    #[test]
    fn standalone_config_tracks_admin_sandbox_mode() {
        let config_source = include_str!("../../sdkwork-api-config/src/lib.rs");

        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX"));
        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX_STORAGE_FILE"));
    }
}
