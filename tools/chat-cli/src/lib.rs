use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Method, Request};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use im_auth_context::encode_hs256_bearer_token;
use serde_json::{Value, json};
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:18090";
const DEFAULT_TENANT_ID: &str = "t_demo";
const DEFAULT_USER_ID: &str = "u_demo";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    message: String,
    exit_code: i32,
}

impl CliError {
    fn help(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 0,
        }
    }

    fn usage(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    fn runtime(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandOutput {
    Json(Value),
    Frames(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliCommand {
    context: CommandContext,
    operation: CommandOperation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CommandContext {
    base_url: String,
    auth: AuthInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthInput {
    tenant_id: String,
    user_id: String,
    actor_kind: String,
    session_id: String,
    device_id: String,
    permissions: Vec<String>,
    bearer_token: Option<String>,
    public_bearer_secret: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandOperation {
    Health,
    Token {
        authorization_header: bool,
    },
    CreateConversation {
        conversation_id: String,
        conversation_type: String,
    },
    AddMember {
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: String,
    },
    Members {
        conversation_id: String,
    },
    SendMessage {
        conversation_id: String,
        summary: Option<String>,
        text: Option<String>,
        client_msg_id: Option<String>,
    },
    Timeline {
        conversation_id: String,
    },
    Watch {
        conversation_id: String,
        event_types: Vec<String>,
        exit_after_events: Option<usize>,
        idle_timeout: Option<Duration>,
    },
    ChatSession {
        conversation_id: String,
        label: Option<String>,
        message_prefix: Option<String>,
        event_types: Vec<String>,
        idle_timeout: Option<Duration>,
    },
}

#[derive(Debug, Default, Clone)]
struct GlobalOptions {
    base_url: Option<String>,
    tenant_id: Option<String>,
    user_id: Option<String>,
    actor_kind: Option<String>,
    session_id: Option<String>,
    device_id: Option<String>,
    permissions: Option<Vec<String>>,
    bearer_token: Option<String>,
    public_bearer_secret: Option<String>,
}

pub fn parse_cli_args<I, S>(args: I) -> Result<CliCommand, CliError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let values = args.into_iter().map(Into::into).collect::<Vec<_>>();
    if values.is_empty() {
        return Err(CliError::help(cli_usage()));
    }

    let mut cursor = ArgCursor::new(values.into_iter().skip(1).collect::<Vec<_>>());
    if cursor.is_empty() {
        return Err(CliError::help(cli_usage()));
    }

    let mut global = GlobalOptions::default();

    while let Some(next) = cursor.peek() {
        if is_command_name(next) {
            break;
        }

        match next {
            "-h" | "--help" => return Err(CliError::help(cli_usage())),
            "--base-url" => {
                cursor.next();
                global.base_url = Some(cursor.required_value("--base-url")?);
            }
            "--tenant-id" => {
                cursor.next();
                global.tenant_id = Some(cursor.required_value("--tenant-id")?);
            }
            "--user-id" => {
                cursor.next();
                global.user_id = Some(cursor.required_value("--user-id")?);
            }
            "--actor-kind" => {
                cursor.next();
                global.actor_kind = Some(cursor.required_value("--actor-kind")?);
            }
            "--session-id" => {
                cursor.next();
                global.session_id = Some(cursor.required_value("--session-id")?);
            }
            "--device-id" => {
                cursor.next();
                global.device_id = Some(cursor.required_value("--device-id")?);
            }
            "--permissions" => {
                cursor.next();
                global.permissions =
                    Some(parse_permissions(cursor.required_value("--permissions")?));
            }
            "--bearer-token" => {
                cursor.next();
                global.bearer_token = Some(cursor.required_value("--bearer-token")?);
            }
            "--public-bearer-secret" => {
                cursor.next();
                global.public_bearer_secret =
                    Some(cursor.required_value("--public-bearer-secret")?);
            }
            other if other.starts_with('-') => {
                return Err(CliError::usage(format!(
                    "unknown global flag: {other}\n\n{}",
                    cli_usage()
                )));
            }
            other => {
                return Err(CliError::usage(format!(
                    "unknown command: {other}\n\n{}",
                    cli_usage()
                )));
            }
        }
    }

    let Some(command_name) = cursor.next() else {
        return Err(CliError::help(cli_usage()));
    };

    let context = build_command_context(global);
    let operation = parse_command_operation(command_name.as_str(), &mut cursor)?;

    if !cursor.is_empty() {
        return Err(CliError::usage(format!(
            "unexpected trailing arguments: {}\n\n{}",
            cursor.remaining().join(" "),
            cli_usage()
        )));
    }

    Ok(CliCommand { context, operation })
}

fn parse_command_operation(
    command_name: &str,
    cursor: &mut ArgCursor,
) -> Result<CommandOperation, CliError> {
    match command_name {
        "health" => Ok(CommandOperation::Health),
        "token" => {
            let mut authorization_header = true;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(token_usage())),
                    "--token-only" => {
                        cursor.next();
                        authorization_header = false;
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown token flag: {other}\n\n{}",
                            token_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Token {
                authorization_header,
            })
        }
        "create-conversation" => {
            let mut conversation_id = None;
            let mut conversation_type = Some("group".to_owned());
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(create_conversation_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--conversation-type" => {
                        cursor.next();
                        conversation_type = Some(cursor.required_value("--conversation-type")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown create-conversation flag: {other}\n\n{}",
                            create_conversation_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::CreateConversation {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for create-conversation",
                )?,
                conversation_type: conversation_type.unwrap_or_else(|| "group".to_owned()),
            })
        }
        "add-member" => {
            let mut conversation_id = None;
            let mut principal_id = None;
            let mut principal_kind = Some("user".to_owned());
            let mut role = Some("member".to_owned());
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(add_member_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--principal-id" => {
                        cursor.next();
                        principal_id = Some(cursor.required_value("--principal-id")?);
                    }
                    "--principal-kind" => {
                        cursor.next();
                        principal_kind = Some(cursor.required_value("--principal-kind")?);
                    }
                    "--role" => {
                        cursor.next();
                        role = Some(cursor.required_value("--role")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown add-member flag: {other}\n\n{}",
                            add_member_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::AddMember {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for add-member",
                )?,
                principal_id: required_field(
                    principal_id,
                    "--principal-id is required for add-member",
                )?,
                principal_kind: principal_kind.unwrap_or_else(|| "user".to_owned()),
                role: role.unwrap_or_else(|| "member".to_owned()),
            })
        }
        "members" => {
            let mut conversation_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(members_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown members flag: {other}\n\n{}",
                            members_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Members {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for members",
                )?,
            })
        }
        "send-message" => {
            let mut conversation_id = None;
            let mut summary = None;
            let mut text = None;
            let mut client_msg_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(send_message_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--summary" => {
                        cursor.next();
                        summary = Some(cursor.required_value("--summary")?);
                    }
                    "--text" => {
                        cursor.next();
                        text = Some(cursor.required_value("--text")?);
                    }
                    "--client-msg-id" => {
                        cursor.next();
                        client_msg_id = Some(cursor.required_value("--client-msg-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown send-message flag: {other}\n\n{}",
                            send_message_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if summary.is_none() && text.is_none() {
                return Err(CliError::usage(format!(
                    "send-message requires --summary, --text, or both\n\n{}",
                    send_message_usage()
                )));
            }

            Ok(CommandOperation::SendMessage {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for send-message",
                )?,
                summary,
                text,
                client_msg_id,
            })
        }
        "timeline" => {
            let mut conversation_id = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(timeline_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown timeline flag: {other}\n\n{}",
                            timeline_usage()
                        )));
                    }
                    _ => break,
                }
            }
            Ok(CommandOperation::Timeline {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for timeline",
                )?,
            })
        }
        "watch" => {
            let mut conversation_id = None;
            let mut event_types = Vec::new();
            let mut exit_after_events = None;
            let mut idle_timeout = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(watch_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--event-type" => {
                        cursor.next();
                        event_types.push(cursor.required_value("--event-type")?);
                    }
                    "--exit-after-events" => {
                        cursor.next();
                        exit_after_events = Some(parse_usize(
                            cursor.required_value("--exit-after-events")?.as_str(),
                            "--exit-after-events",
                        )?);
                    }
                    "--idle-timeout-seconds" => {
                        cursor.next();
                        let seconds = parse_u64(
                            cursor.required_value("--idle-timeout-seconds")?.as_str(),
                            "--idle-timeout-seconds",
                        )?;
                        idle_timeout = Some(Duration::from_secs(seconds));
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown watch flag: {other}\n\n{}",
                            watch_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if event_types.is_empty() {
                event_types.push("message.posted".to_owned());
            }

            Ok(CommandOperation::Watch {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for watch",
                )?,
                event_types,
                exit_after_events,
                idle_timeout,
            })
        }
        "chat-session" => {
            let mut conversation_id = None;
            let mut label = None;
            let mut message_prefix = None;
            let mut event_types = Vec::new();
            let mut idle_timeout = None;
            while let Some(next) = cursor.peek() {
                match next {
                    "-h" | "--help" => return Err(CliError::help(chat_session_usage())),
                    "--conversation-id" => {
                        cursor.next();
                        conversation_id = Some(cursor.required_value("--conversation-id")?);
                    }
                    "--label" => {
                        cursor.next();
                        label = Some(cursor.required_value("--label")?);
                    }
                    "--message-prefix" => {
                        cursor.next();
                        message_prefix = Some(cursor.required_value("--message-prefix")?);
                    }
                    "--event-type" => {
                        cursor.next();
                        event_types.push(cursor.required_value("--event-type")?);
                    }
                    "--idle-timeout-seconds" => {
                        cursor.next();
                        let seconds = parse_u64(
                            cursor.required_value("--idle-timeout-seconds")?.as_str(),
                            "--idle-timeout-seconds",
                        )?;
                        idle_timeout = Some(Duration::from_secs(seconds));
                    }
                    other if other.starts_with('-') => {
                        return Err(CliError::usage(format!(
                            "unknown chat-session flag: {other}\n\n{}",
                            chat_session_usage()
                        )));
                    }
                    _ => break,
                }
            }

            if event_types.is_empty() {
                event_types.push("message.posted".to_owned());
            }

            Ok(CommandOperation::ChatSession {
                conversation_id: required_field(
                    conversation_id,
                    "--conversation-id is required for chat-session",
                )?,
                label,
                message_prefix,
                event_types,
                idle_timeout,
            })
        }
        other => Err(CliError::usage(format!(
            "unknown command: {other}\n\n{}",
            cli_usage()
        ))),
    }
}

pub fn is_interactive_command(command: &CliCommand) -> bool {
    matches!(command.operation, CommandOperation::ChatSession { .. })
}

pub async fn execute_command(command: CliCommand) -> Result<CommandOutput, CliError> {
    match command.operation {
        CommandOperation::Health => {
            let value =
                http_request_json(&command.context, Method::GET, "/healthz", None, false).await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Token {
            authorization_header,
        } => {
            let claims = auth_claims(&command.context.auth);
            let authorization = resolve_authorization_header(&command.context.auth)?;
            let token = authorization
                .strip_prefix("Bearer ")
                .unwrap_or(authorization.as_str())
                .to_owned();
            Ok(CommandOutput::Json(json!({
                "source": if command.context.auth.bearer_token.is_some() { "providedBearerToken" } else { "generatedBearerToken" },
                "authorization": if authorization_header { authorization.clone() } else { format!("Bearer {token}") },
                "token": token,
                "claims": claims
            })))
        }
        CommandOperation::CreateConversation {
            conversation_id,
            conversation_type,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                "/api/v1/conversations",
                Some(json!({
                    "conversationId": conversation_id,
                    "conversationType": conversation_type,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::AddMember {
            conversation_id,
            principal_id,
            principal_kind,
            role,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                format!("/api/v1/conversations/{conversation_id}/members/add").as_str(),
                Some(json!({
                    "principalId": principal_id,
                    "principalKind": principal_kind,
                    "role": role,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Members { conversation_id } => {
            let value = http_request_json(
                &command.context,
                Method::GET,
                format!("/api/v1/conversations/{conversation_id}/members").as_str(),
                None,
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::SendMessage {
            conversation_id,
            summary,
            text,
            client_msg_id,
        } => {
            let value = http_request_json(
                &command.context,
                Method::POST,
                format!("/api/v1/conversations/{conversation_id}/messages").as_str(),
                Some(json!({
                    "clientMsgId": client_msg_id,
                    "summary": summary,
                    "text": text,
                })),
                true,
            )
            .await?;
            Ok(CommandOutput::Json(json!({
                "conversationId": conversation_id,
                "clientMsgId": client_msg_id,
                "summary": summary,
                "text": text,
                "result": value,
            })))
        }
        CommandOperation::Timeline { conversation_id } => {
            let value = http_request_json(
                &command.context,
                Method::GET,
                format!("/api/v1/conversations/{conversation_id}/messages").as_str(),
                None,
                true,
            )
            .await?;
            Ok(CommandOutput::Json(value))
        }
        CommandOperation::Watch {
            conversation_id,
            event_types,
            exit_after_events,
            idle_timeout,
        } => {
            let frames = watch_realtime_conversation(
                &command.context,
                conversation_id.as_str(),
                event_types.as_slice(),
                exit_after_events,
                idle_timeout,
            )
            .await?;
            Ok(CommandOutput::Frames(frames))
        }
        CommandOperation::ChatSession { .. } => Err(CliError::usage(
            "chat-session is interactive; execute it through the CLI binary or execute_interactive_command",
        )),
    }
}

pub async fn execute_interactive_command(command: CliCommand) -> Result<(), CliError> {
    execute_interactive_command_with_io(command, io::stdin(), io::stdout()).await
}

pub async fn execute_interactive_command_with_io<R, W>(
    command: CliCommand,
    input: R,
    output: W,
) -> Result<(), CliError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    match command.operation {
        CommandOperation::ChatSession {
            conversation_id,
            label,
            message_prefix,
            event_types,
            idle_timeout,
        } => {
            run_chat_session(
                &command.context,
                conversation_id.as_str(),
                label.as_deref(),
                message_prefix.as_deref(),
                event_types.as_slice(),
                idle_timeout,
                input,
                output,
            )
            .await
        }
        operation => {
            let output_value = execute_command(CliCommand {
                context: command.context,
                operation,
            })
            .await?;
            let rendered = render_output(&output_value)?;
            let mut writer = output;
            if !rendered.is_empty() {
                writer
                    .write_all(rendered.as_bytes())
                    .await
                    .map_err(|error| {
                        CliError::runtime(format!("failed to write rendered output: {error}"))
                    })?;
                writer.write_all(b"\n").await.map_err(|error| {
                    CliError::runtime(format!("failed to finalize rendered output: {error}"))
                })?;
                writer.flush().await.map_err(|error| {
                    CliError::runtime(format!("failed to flush rendered output: {error}"))
                })?;
            }
            Ok(())
        }
    }
}

pub fn render_output(output: &CommandOutput) -> Result<String, CliError> {
    match output {
        CommandOutput::Json(value) => serde_json::to_string_pretty(value)
            .map_err(|error| CliError::runtime(format!("failed to render json output: {error}"))),
        CommandOutput::Frames(values) => {
            let mut lines = Vec::with_capacity(values.len());
            for value in values {
                lines.push(serde_json::to_string(value).map_err(|error| {
                    CliError::runtime(format!("failed to render websocket frame: {error}"))
                })?);
            }
            Ok(lines.join("\n"))
        }
    }
}

fn build_command_context(global: GlobalOptions) -> CommandContext {
    let user_id = global
        .user_id
        .or_else(|| std::env::var("CRAW_CHAT_CLI_USER_ID").ok())
        .unwrap_or_else(|| DEFAULT_USER_ID.to_owned());
    let sanitized_user_id = sanitize_identifier(user_id.as_str());
    let tenant_id = global
        .tenant_id
        .or_else(|| std::env::var("CRAW_CHAT_CLI_TENANT_ID").ok())
        .unwrap_or_else(|| DEFAULT_TENANT_ID.to_owned());
    let actor_kind = global
        .actor_kind
        .or_else(|| std::env::var("CRAW_CHAT_CLI_ACTOR_KIND").ok())
        .unwrap_or_else(|| "user".to_owned());
    let session_id = global
        .session_id
        .or_else(|| std::env::var("CRAW_CHAT_CLI_SESSION_ID").ok())
        .unwrap_or_else(|| format!("s_{sanitized_user_id}"));
    let device_id = global
        .device_id
        .or_else(|| std::env::var("CRAW_CHAT_CLI_DEVICE_ID").ok())
        .unwrap_or_else(|| format!("d_{sanitized_user_id}"));
    let permissions = global
        .permissions
        .or_else(|| {
            std::env::var("CRAW_CHAT_CLI_PERMISSIONS")
                .ok()
                .map(parse_permissions)
        })
        .unwrap_or_default();
    let public_bearer_secret = global
        .public_bearer_secret
        .or_else(|| std::env::var(im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV).ok())
        .or_else(resolve_public_bearer_secret_from_config);
    let base_url = global
        .base_url
        .or_else(|| std::env::var("CRAW_CHAT_BASE_URL").ok())
        .or_else(resolve_base_url_from_config)
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned());

    CommandContext {
        base_url,
        auth: AuthInput {
            tenant_id,
            user_id,
            actor_kind,
            session_id,
            device_id,
            permissions,
            bearer_token: global.bearer_token,
            public_bearer_secret,
        },
    }
}

async fn http_request_json(
    context: &CommandContext,
    method: Method,
    path: &str,
    body: Option<Value>,
    require_auth: bool,
) -> Result<Value, CliError> {
    let client = http_client();
    let uri = build_http_url(context.base_url.as_str(), path);
    let payload = if let Some(value) = body {
        serde_json::to_vec(&value)
            .map(Bytes::from)
            .map_err(|error| CliError::runtime(format!("failed to encode json body: {error}")))?
    } else {
        Bytes::new()
    };

    let mut request_builder = Request::builder()
        .method(method.clone())
        .uri(uri.as_str())
        .header(CONTENT_TYPE, "application/json");
    if require_auth {
        let authorization = resolve_authorization_header(&context.auth)?;
        request_builder = request_builder.header(AUTHORIZATION, authorization.as_str());
    }

    let request = request_builder.body(Full::new(payload)).map_err(|error| {
        CliError::runtime(format!("failed to build request for {uri}: {error}"))
    })?;
    let response = client
        .request(request)
        .await
        .map_err(|error| CliError::runtime(format!("request to {uri} failed: {error}")))?;
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .map_err(|error| CliError::runtime(format!("failed to read response body: {error}")))?
        .to_bytes();

    if !status.is_success() {
        let body_text = String::from_utf8_lossy(bytes.as_ref()).into_owned();
        return Err(CliError::runtime(format!(
            "{} {} failed with status {}: {}",
            method.as_str(),
            path,
            status.as_u16(),
            body_text
        )));
    }

    if bytes.is_empty() {
        return Ok(json!({}));
    }

    serde_json::from_slice(bytes.as_ref()).map_err(|error| {
        CliError::runtime(format!("response from {path} was not valid json: {error}"))
    })
}

async fn watch_realtime_conversation(
    context: &CommandContext,
    conversation_id: &str,
    event_types: &[String],
    exit_after_events: Option<usize>,
    idle_timeout: Option<Duration>,
) -> Result<Vec<Value>, CliError> {
    let mut socket = connect_realtime_socket(context).await?;

    let mut frames = Vec::new();
    frames.push(read_next_json_frame(&mut socket, idle_timeout).await?);

    send_subscription_sync(&mut socket, conversation_id, event_types, "chat_cli_sync_1").await?;

    frames.push(read_next_json_frame(&mut socket, idle_timeout).await?);

    let mut observed_event_windows = 0usize;
    loop {
        let frame = read_next_json_frame(&mut socket, idle_timeout).await?;
        let acked_seq = acked_seq_for_window(&frame);
        let is_event_window = frame["type"] == "event.window";
        frames.push(frame);

        if let Some(acked_seq) = acked_seq {
            send_events_ack(&mut socket, acked_seq).await?;
            frames.push(read_next_json_frame(&mut socket, idle_timeout).await?);
        }

        if is_event_window {
            observed_event_windows += 1;
            if let Some(limit) = exit_after_events {
                if observed_event_windows >= limit {
                    break;
                }
            }
        }
    }

    let _ = socket.close(None).await;
    Ok(frames)
}

async fn run_chat_session<R, W>(
    context: &CommandContext,
    conversation_id: &str,
    label: Option<&str>,
    message_prefix: Option<&str>,
    event_types: &[String],
    idle_timeout: Option<Duration>,
    input: R,
    mut output: W,
) -> Result<(), CliError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let socket = connect_realtime_socket(context).await?;
    let (mut ws_write, mut ws_read) = socket.split();
    let _ = read_next_json_frame_from_read(&mut ws_read, idle_timeout).await?;
    send_subscription_sync_write(
        &mut ws_write,
        conversation_id,
        event_types,
        "chat_cli_session_sync_1",
    )
    .await?;
    let _ = read_next_json_frame_from_read(&mut ws_read, idle_timeout).await?;

    let prompt_label = label.unwrap_or(context.auth.user_id.as_str()).to_owned();
    let mut lines = BufReader::new(input).lines();
    let mut pending_outgoing = Vec::<String>::new();

    write_chat_banner(&mut output, prompt_label.as_str(), conversation_id).await?;
    write_chat_prompt(&mut output, prompt_label.as_str()).await?;

    loop {
        let websocket_frame = read_next_json_frame_from_read(&mut ws_read, idle_timeout);
        tokio::pin!(websocket_frame);

        tokio::select! {
            line = lines.next_line() => {
                let Some(line) = line.map_err(|error| {
                    CliError::runtime(format!("failed to read chat-session input: {error}"))
                })? else {
                    break;
                };

                let trimmed = line.trim();
                if trimmed.is_empty() {
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                    continue;
                }

                if trimmed.eq_ignore_ascii_case("/quit") || trimmed.eq_ignore_ascii_case("/exit") {
                    write_chat_status_line(&mut output, "closing chat session").await?;
                    break;
                }

                if trimmed.eq_ignore_ascii_case("/help") {
                    write_chat_status_line(
                        &mut output,
                        "commands: /help, /quit, /exit",
                    ).await?;
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                    continue;
                }

                let sent_summary = apply_message_prefix(trimmed, message_prefix);
                let client_msg_id = next_chat_client_message_id(context.auth.user_id.as_str());
                send_message_request(
                    context,
                    conversation_id,
                    sent_summary.as_str(),
                    sent_summary.as_str(),
                    client_msg_id.as_str(),
                )
                .await?;
                pending_outgoing.push(sent_summary);
                write_chat_message_line(
                    &mut output,
                    prompt_label.as_str(),
                    trimmed,
                )
                .await?;
                write_chat_prompt(&mut output, prompt_label.as_str()).await?;
            }
            frame = &mut websocket_frame => {
                let frame = frame?;
                if let Some(acked_seq) = acked_seq_for_window(&frame) {
                    send_events_ack_write(&mut ws_write, acked_seq).await?;
                }

                let mut printed_any = false;
                for summary in extract_window_summaries(&frame) {
                    if remove_pending_summary(&mut pending_outgoing, summary.as_str()) {
                        continue;
                    }

                    write_chat_status_line(&mut output, summary.as_str()).await?;
                    printed_any = true;
                }

                if printed_any {
                    write_chat_prompt(&mut output, prompt_label.as_str()).await?;
                }
            }
        }
    }

    let _ = ws_write.send(Message::Close(None)).await;
    Ok(())
}

fn http_client() -> Client<HttpConnector, Full<Bytes>> {
    let connector = HttpConnector::new();
    Client::builder(TokioExecutor::new()).build(connector)
}

fn build_http_url(base_url: &str, path: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), path)
}

fn build_websocket_url(base_url: &str, path: &str) -> Result<String, CliError> {
    let base = base_url.trim_end_matches('/');
    if let Some(rest) = base.strip_prefix("http://") {
        return Ok(format!("ws://{rest}{path}"));
    }
    if let Some(rest) = base.strip_prefix("https://") {
        return Ok(format!("wss://{rest}{path}"));
    }
    Err(CliError::usage(format!(
        "base url must start with http:// or https://: {base_url}"
    )))
}

fn auth_claims(auth: &AuthInput) -> Value {
    let mut claims = json!({
        "tenant_id": auth.tenant_id,
        "sub": auth.user_id,
        "actor_kind": auth.actor_kind,
        "sid": auth.session_id,
        "did": auth.device_id,
    });
    if !auth.permissions.is_empty() {
        claims["permissions"] = json!(auth.permissions);
    }
    claims
}

fn resolve_authorization_header(auth: &AuthInput) -> Result<String, CliError> {
    if let Some(token) = auth.bearer_token.as_deref() {
        return Ok(normalize_bearer_header(token));
    }

    let Some(secret) = auth.public_bearer_secret.as_deref() else {
        return Err(CliError::runtime(format!(
            "missing public bearer secret; provide --public-bearer-secret, set {}, or configure .runtime/local-minimal/config/local-minimal.env",
            im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV
        )));
    };

    let token = encode_hs256_bearer_token(&auth_claims(auth), secret).map_err(|error| {
        CliError::runtime(format!(
            "failed to encode bearer token for {}: {error}",
            auth.user_id
        ))
    })?;
    Ok(format!("Bearer {token}"))
}

fn normalize_bearer_header(value: &str) -> String {
    if value.starts_with("Bearer ") || value.starts_with("bearer ") {
        value.to_owned()
    } else {
        format!("Bearer {value}")
    }
}

async fn connect_realtime_socket(context: &CommandContext) -> Result<WsStream, CliError> {
    let ws_url = build_websocket_url(context.base_url.as_str(), "/api/v1/realtime/ws")?;
    let authorization = resolve_authorization_header(&context.auth)?;

    let mut request = ws_url.into_client_request().map_err(|error| {
        CliError::runtime(format!("failed to build websocket request: {error}"))
    })?;
    request.headers_mut().insert(
        AUTHORIZATION,
        authorization.parse().map_err(|error| {
            CliError::runtime(format!("failed to encode authorization header: {error}"))
        })?,
    );

    let (socket, _) = connect_async(request)
        .await
        .map_err(|error| CliError::runtime(format!("failed to connect websocket: {error}")))?;
    Ok(socket)
}

async fn send_subscription_sync(
    socket: &mut WsStream,
    conversation_id: &str,
    event_types: &[String],
    request_id: &str,
) -> Result<(), CliError> {
    socket
        .send(Message::Text(
            json!({
                "type": "subscriptions.sync",
                "requestId": request_id,
                "items": [
                    {
                        "scopeType": "conversation",
                        "scopeId": conversation_id,
                        "eventTypes": event_types,
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .map_err(|error| CliError::runtime(format!("failed to send subscription sync: {error}")))
}

async fn send_events_ack(socket: &mut WsStream, acked_seq: u64) -> Result<(), CliError> {
    socket
        .send(Message::Text(
            json!({
                "type": "events.ack",
                "requestId": format!("chat_cli_ack_{acked_seq}"),
                "ackedSeq": acked_seq,
            })
            .to_string()
            .into(),
        ))
        .await
        .map_err(|error| CliError::runtime(format!("failed to send events ack: {error}")))
}

async fn send_subscription_sync_write(
    writer: &mut WsWrite,
    conversation_id: &str,
    event_types: &[String],
    request_id: &str,
) -> Result<(), CliError> {
    writer
        .send(Message::Text(
            json!({
                "type": "subscriptions.sync",
                "requestId": request_id,
                "items": [
                    {
                        "scopeType": "conversation",
                        "scopeId": conversation_id,
                        "eventTypes": event_types,
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .map_err(|error| CliError::runtime(format!("failed to send subscription sync: {error}")))
}

async fn send_events_ack_write(writer: &mut WsWrite, acked_seq: u64) -> Result<(), CliError> {
    writer
        .send(Message::Text(
            json!({
                "type": "events.ack",
                "requestId": format!("chat_cli_ack_{acked_seq}"),
                "ackedSeq": acked_seq,
            })
            .to_string()
            .into(),
        ))
        .await
        .map_err(|error| CliError::runtime(format!("failed to send events ack: {error}")))
}

async fn send_message_request(
    context: &CommandContext,
    conversation_id: &str,
    summary: &str,
    text: &str,
    client_msg_id: &str,
) -> Result<Value, CliError> {
    http_request_json(
        context,
        Method::POST,
        format!("/api/v1/conversations/{conversation_id}/messages").as_str(),
        Some(json!({
            "clientMsgId": client_msg_id,
            "summary": summary,
            "text": text,
        })),
        true,
    )
    .await
}

async fn read_next_json_frame(
    socket: &mut WsStream,
    idle_timeout: Option<Duration>,
) -> Result<Value, CliError> {
    loop {
        let next = if let Some(duration) = idle_timeout {
            timeout(duration, socket.next())
                .await
                .map_err(|_| CliError::runtime("timed out waiting for realtime frame"))?
        } else {
            socket.next().await
        };

        let message = next
            .ok_or_else(|| CliError::runtime("websocket closed before expected frame arrived"))?
            .map_err(|error| CliError::runtime(format!("websocket receive failed: {error}")))?;

        match message {
            Message::Text(text) => {
                return serde_json::from_str(text.as_str()).map_err(|error| {
                    CliError::runtime(format!("websocket text frame was not valid json: {error}"))
                });
            }
            Message::Ping(payload) => {
                socket.send(Message::Pong(payload)).await.map_err(|error| {
                    CliError::runtime(format!("failed to reply to websocket ping: {error}"))
                })?;
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                let reason = frame
                    .map(|frame| format!("code={} reason={}", u16::from(frame.code), frame.reason))
                    .unwrap_or_else(|| "without close frame".to_owned());
                return Err(CliError::runtime(format!(
                    "websocket closed before expected frame arrived: {reason}"
                )));
            }
            Message::Binary(_) => {
                return Err(CliError::runtime(
                    "websocket returned unsupported binary frame",
                ));
            }
            Message::Frame(_) => {}
        }
    }
}

async fn read_next_json_frame_from_read(
    reader: &mut WsRead,
    idle_timeout: Option<Duration>,
) -> Result<Value, CliError> {
    loop {
        let next = if let Some(duration) = idle_timeout {
            timeout(duration, reader.next())
                .await
                .map_err(|_| CliError::runtime("timed out waiting for realtime frame"))?
        } else {
            reader.next().await
        };

        let message = next
            .ok_or_else(|| CliError::runtime("websocket closed before expected frame arrived"))?
            .map_err(|error| CliError::runtime(format!("websocket receive failed: {error}")))?;

        match message {
            Message::Text(text) => {
                return serde_json::from_str(text.as_str()).map_err(|error| {
                    CliError::runtime(format!("websocket text frame was not valid json: {error}"))
                });
            }
            Message::Ping(_) | Message::Pong(_) => {}
            Message::Close(frame) => {
                let reason = frame
                    .map(|frame| format!("code={} reason={}", u16::from(frame.code), frame.reason))
                    .unwrap_or_else(|| "without close frame".to_owned());
                return Err(CliError::runtime(format!(
                    "websocket closed before expected frame arrived: {reason}"
                )));
            }
            Message::Binary(_) => {
                return Err(CliError::runtime(
                    "websocket returned unsupported binary frame",
                ));
            }
            Message::Frame(_) => {}
        }
    }
}

fn acked_seq_for_window(frame: &Value) -> Option<u64> {
    if frame.get("type") != Some(&Value::String("event.window".to_owned())) {
        return None;
    }

    if frame["window"]["items"]
        .as_array()
        .is_some_and(|items| items.is_empty())
    {
        return None;
    }

    frame["window"]["nextAfterSeq"]
        .as_u64()
        .and_then(|value| value.checked_sub(1))
}

fn extract_window_summaries(frame: &Value) -> Vec<String> {
    let mut summaries = Vec::new();

    let Some(items) = frame["window"]["items"].as_array() else {
        return summaries;
    };

    for item in items {
        let Some(payload_text) = item["payload"].as_str() else {
            continue;
        };

        let Ok(payload) = serde_json::from_str::<Value>(payload_text) else {
            continue;
        };

        if let Some(summary) = payload["summary"].as_str() {
            summaries.push(summary.to_owned());
        }
    }

    summaries
}

fn apply_message_prefix(message: &str, message_prefix: Option<&str>) -> String {
    match message_prefix {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}{message}"),
        _ => message.to_owned(),
    }
}

fn next_chat_client_message_id(user_id: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_millis();
    format!("chat_cli_{}_{}", sanitize_identifier(user_id), millis)
}

fn remove_pending_summary(pending: &mut Vec<String>, summary: &str) -> bool {
    if let Some(index) = pending.iter().position(|candidate| candidate == summary) {
        pending.remove(index);
        true
    } else {
        false
    }
}

async fn write_chat_banner<W>(
    output: &mut W,
    prompt_label: &str,
    conversation_id: &str,
) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    write_chat_status_line(
        output,
        format!("chat-session ready for {prompt_label} in {conversation_id}; type /quit to exit")
            .as_str(),
    )
    .await
}

async fn write_chat_prompt<W>(output: &mut W, prompt_label: &str) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    output
        .write_all(format!("{prompt_label}> ").as_bytes())
        .await
        .map_err(|error| CliError::runtime(format!("failed to write chat prompt: {error}")))?;
    output
        .flush()
        .await
        .map_err(|error| CliError::runtime(format!("failed to flush chat prompt: {error}")))
}

async fn write_chat_status_line<W>(output: &mut W, line: &str) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    output
        .write_all(format!("{line}\n").as_bytes())
        .await
        .map_err(|error| CliError::runtime(format!("failed to write chat line: {error}")))?;
    output
        .flush()
        .await
        .map_err(|error| CliError::runtime(format!("failed to flush chat line: {error}")))
}

async fn write_chat_message_line<W>(
    output: &mut W,
    prompt_label: &str,
    message: &str,
) -> Result<(), CliError>
where
    W: AsyncWrite + Unpin,
{
    write_chat_status_line(output, format!("[{prompt_label}] {message}").as_str()).await
}

fn resolve_public_bearer_secret_from_config() -> Option<String> {
    let config_path = find_local_env_file()?;
    read_env_file_value(
        config_path.as_path(),
        im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV,
    )
}

fn resolve_base_url_from_config() -> Option<String> {
    let config_path = find_local_env_file()?;
    let bind_address = read_env_file_value(config_path.as_path(), "CRAW_CHAT_BIND_ADDR")?;
    Some(bind_address_to_base_url(bind_address.as_str()))
}

fn find_local_env_file() -> Option<PathBuf> {
    for candidate in env_file_candidates() {
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn env_file_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_dir) = std::env::current_dir() {
        for ancestor in current_dir.ancestors() {
            candidates.push(
                ancestor
                    .join(".runtime")
                    .join("local-minimal")
                    .join("config")
                    .join("local-minimal.env"),
            );
        }
    }

    let manifest_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(repo_root) = manifest_root.ancestors().nth(2) {
        candidates.push(
            repo_root
                .join(".runtime")
                .join("local-minimal")
                .join("config")
                .join("local-minimal.env"),
        );
    }

    candidates.sort();
    candidates.dedup();
    candidates
}

fn read_env_file_value(path: &Path, key: &str) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (current_key, current_value) = trimmed.split_once('=')?;
        if current_key.trim() == key {
            let value = current_value.trim();
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

fn sanitize_identifier(raw: &str) -> String {
    let mut sanitized = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    if sanitized.is_empty() {
        "user".to_owned()
    } else {
        sanitized
    }
}

fn bind_address_to_base_url(bind_address: &str) -> String {
    let mut parts = bind_address.rsplitn(2, ':');
    let port = parts.next().unwrap_or("18090").trim();
    let host = parts.next().unwrap_or("127.0.0.1").trim();
    let normalized_host = match host {
        "" | "0.0.0.0" | "::" | "[::]" => "127.0.0.1",
        other => other,
    };
    format!("http://{normalized_host}:{port}")
}

fn parse_permissions(raw: String) -> Vec<String> {
    raw.split(|ch: char| ch == ',' || ch.is_whitespace())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
}

fn parse_usize(raw: &str, flag: &str) -> Result<usize, CliError> {
    raw.parse::<usize>()
        .map_err(|_| CliError::usage(format!("{flag} must be a positive integer")))
}

fn parse_u64(raw: &str, flag: &str) -> Result<u64, CliError> {
    raw.parse::<u64>()
        .map_err(|_| CliError::usage(format!("{flag} must be a positive integer")))
}

fn required_field<T>(value: Option<T>, message: &str) -> Result<T, CliError> {
    value.ok_or_else(|| CliError::usage(message))
}

fn is_command_name(value: &str) -> bool {
    matches!(
        value,
        "health"
            | "token"
            | "create-conversation"
            | "add-member"
            | "members"
            | "send-message"
            | "timeline"
            | "watch"
            | "chat-session"
    )
}

fn cli_usage() -> String {
    format!(
        concat!(
            "Usage: craw-chat-cli [global options] <command> [command options]\n\n",
            "Global options:\n",
            "  --base-url <url>                 Service base url. Default: {base}\n",
            "  --tenant-id <id>                 Tenant id. Default: {tenant}\n",
            "  --user-id <id>                   User id. Default: {user}\n",
            "  --actor-kind <kind>              Actor kind. Default: user\n",
            "  --session-id <id>                Session id. Default: s_<user-id>\n",
            "  --device-id <id>                 Device id. Default: d_<user-id>\n",
            "  --permissions <csv>              Optional permissions claims\n",
            "  --bearer-token <token>           Use an existing bearer token\n",
            "  --public-bearer-secret <secret>  Generate a signed local bearer token\n",
            "  -h, --help                       Show help\n\n",
            "Commands:\n",
            "  health\n",
            "  token [--token-only]\n",
            "  create-conversation --conversation-id <id> [--conversation-type <type>]\n",
            "  add-member --conversation-id <id> --principal-id <id> [--principal-kind user] [--role member]\n",
            "  members --conversation-id <id>\n",
            "  send-message --conversation-id <id> [--summary <text>] [--text <text>] [--client-msg-id <id>]\n",
            "  timeline --conversation-id <id>\n",
            "  watch --conversation-id <id> [--event-type <type>]... [--exit-after-events <n>] [--idle-timeout-seconds <n>]\n\n",
            "  chat-session --conversation-id <id> [--label <name>] [--message-prefix <prefix>] [--event-type <type>]... [--idle-timeout-seconds <n>]\n\n",
            "Example:\n",
            "  craw-chat-cli --user-id u_owner --device-id d_owner --session-id s_owner create-conversation --conversation-id c_demo --conversation-type group\n"
        ),
        base = DEFAULT_BASE_URL,
        tenant = DEFAULT_TENANT_ID,
        user = DEFAULT_USER_ID
    )
}

fn token_usage() -> String {
    "Usage: craw-chat-cli [global options] token [--token-only]".to_owned()
}

fn create_conversation_usage() -> String {
    "Usage: craw-chat-cli [global options] create-conversation --conversation-id <id> [--conversation-type <type>]".to_owned()
}

fn add_member_usage() -> String {
    "Usage: craw-chat-cli [global options] add-member --conversation-id <id> --principal-id <id> [--principal-kind <kind>] [--role <role>]".to_owned()
}

fn members_usage() -> String {
    "Usage: craw-chat-cli [global options] members --conversation-id <id>".to_owned()
}

fn send_message_usage() -> String {
    "Usage: craw-chat-cli [global options] send-message --conversation-id <id> [--summary <text>] [--text <text>] [--client-msg-id <id>]".to_owned()
}

fn timeline_usage() -> String {
    "Usage: craw-chat-cli [global options] timeline --conversation-id <id>".to_owned()
}

fn watch_usage() -> String {
    "Usage: craw-chat-cli [global options] watch --conversation-id <id> [--event-type <type>]... [--exit-after-events <n>] [--idle-timeout-seconds <n>]".to_owned()
}

fn chat_session_usage() -> String {
    "Usage: craw-chat-cli [global options] chat-session --conversation-id <id> [--label <name>] [--message-prefix <prefix>] [--event-type <type>]... [--idle-timeout-seconds <n>]".to_owned()
}

#[derive(Debug, Clone)]
struct ArgCursor {
    items: Vec<String>,
    index: usize,
}

impl ArgCursor {
    fn new(items: Vec<String>) -> Self {
        Self { items, index: 0 }
    }

    fn is_empty(&self) -> bool {
        self.index >= self.items.len()
    }

    fn peek(&self) -> Option<&str> {
        self.items.get(self.index).map(String::as_str)
    }

    fn next(&mut self) -> Option<String> {
        if self.index >= self.items.len() {
            None
        } else {
            let value = self.items[self.index].clone();
            self.index += 1;
            Some(value)
        }
    }

    fn required_value(&mut self, flag: &str) -> Result<String, CliError> {
        match self.next() {
            Some(value) => Ok(value),
            None => Err(CliError::usage(format!("{flag} requires a value"))),
        }
    }

    fn remaining(&self) -> &[String] {
        &self.items[self.index..]
    }
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsRead = futures_util::stream::SplitStream<WsStream>;
type WsWrite = futures_util::stream::SplitSink<WsStream, Message>;
