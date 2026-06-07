use super::*;
use axum::body::Bytes;
use axum::http::{Method, StatusCode, Uri};
use sdkwork_aiot_http_api::{AiotApiServer, handle_api_request};
use sdkwork_aiot_transport::{HttpRequest, HttpResponse};

pub(super) async fn handle_app_iot_api(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
    body: Bytes,
) -> axum::response::Response {
    forward_to_aiot_server(
        state.aiot_app_api_server.as_ref(),
        method,
        uri,
        headers,
        body,
    )
}

pub(super) async fn handle_backend_iot_api(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    State(state): State<AppState>,
    body: Bytes,
) -> axum::response::Response {
    forward_to_aiot_server(
        state.aiot_backend_api_server.as_ref(),
        method,
        uri,
        headers,
        body,
    )
}

fn forward_to_aiot_server(
    server: &AiotApiServer,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> axum::response::Response {
    let request = aiot_request_from_axum(method, uri, headers, body);
    aiot_response_into_axum(handle_api_request(server, &request))
}

fn aiot_request_from_axum(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> HttpRequest {
    let mut request = HttpRequest::new(method.as_str(), uri.path());
    for (name, value) in headers.iter() {
        if let Ok(value) = value.to_str() {
            request = request.with_header(name.as_str(), value);
        }
    }
    if let Some(query) = uri.query() {
        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            request = request.with_query_param(name, value);
        }
    }
    request.body = body.to_vec();
    request
}

fn aiot_response_into_axum(response: HttpResponse) -> axum::response::Response {
    let status =
        StatusCode::from_u16(response.status.code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut builder = axum::response::Response::builder().status(status);
    for (name, value) in response.headers() {
        builder = builder.header(name, value);
    }
    builder
        .body(axum::body::Body::from(response.body))
        .unwrap_or_else(|error| {
            ApiError::unavailable(
                "aiot_response_conversion_failed",
                format!("AIoT response conversion failed: {error}"),
            )
            .into_response()
        })
}
