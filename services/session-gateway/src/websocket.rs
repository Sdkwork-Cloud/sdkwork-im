use std::sync::Arc;

use axum::extract::ws::{CloseFrame, Message, Utf8Bytes, WebSocket};
use futures_util::StreamExt;
use im_auth_context::AuthContext;
use serde::Deserialize;
use serde_json::json;

use crate::{RealtimeDeliveryRuntime, RealtimeRuntimeError, RealtimeSubscriptionItemInput};

pub const SESSION_DISCONNECT_CLOSE_CODE: u16 = 4001;
pub const SESSION_DISCONNECT_CLOSE_REASON: &str = "session.disconnect";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClientFrameEnvelope {
    #[serde(rename = "type")]
    frame_type: String,
    request_id: Option<String>,
    #[serde(default)]
    items: Vec<RealtimeSubscriptionItemInput>,
    after_seq: Option<u64>,
    limit: Option<usize>,
    acked_seq: Option<u64>,
}

pub async fn serve_realtime_websocket(
    socket: WebSocket,
    auth: AuthContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
) {
    let tenant_id = auth.tenant_id.clone();
    let principal_id = auth.actor_id.clone();
    let mut socket = socket;
    if let Err(error) = runtime.ensure_device_state(
        tenant_id.as_str(),
        principal_id.as_str(),
        device_id.as_str(),
    ) {
        let _ = send_runtime_error(&mut socket, None, &error).await;
        return;
    }
    let checkpoint = match runtime.window_checkpoint(
        tenant_id.as_str(),
        principal_id.as_str(),
        device_id.as_str(),
    ) {
        Ok(checkpoint) => checkpoint,
        Err(error) => {
            let _ = send_runtime_error(&mut socket, None, &error).await;
            return;
        }
    };
    let disconnect_generation = match runtime.disconnect_generation(
        tenant_id.as_str(),
        principal_id.as_str(),
        device_id.as_str(),
    ) {
        Ok(disconnect_generation) => disconnect_generation,
        Err(error) => {
            let _ = send_runtime_error(&mut socket, None, &error).await;
            return;
        }
    };
    let mut last_sent_seq = checkpoint
        .acked_through_seq
        .max(checkpoint.trimmed_through_seq);
    let mut receiver = match runtime.subscribe_device(
        tenant_id.as_str(),
        principal_id.as_str(),
        device_id.as_str(),
    ) {
        Ok(receiver) => receiver,
        Err(error) => {
            let _ = send_runtime_error(&mut socket, None, &error).await;
            return;
        }
    };
    let mut disconnect_receiver = match runtime.subscribe_disconnect_signal(
        tenant_id.as_str(),
        principal_id.as_str(),
        device_id.as_str(),
    ) {
        Ok(receiver) => receiver,
        Err(error) => {
            let _ = send_runtime_error(&mut socket, None, &error).await;
            return;
        }
    };

    if send_json(
        &mut socket,
        json!({
            "type": "realtime.connected",
            "tenantId": tenant_id,
            "principalId": principal_id,
            "deviceId": device_id,
            "ackedThroughSeq": checkpoint.acked_through_seq,
            "trimmedThroughSeq": checkpoint.trimmed_through_seq,
            "latestRealtimeSeq": checkpoint.latest_realtime_seq
        }),
    )
    .await
    .is_err()
    {
        return;
    }

    let catchup = match runtime.list_events(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        last_sent_seq,
        100,
    ) {
        Ok(catchup) => catchup,
        Err(error) => {
            let _ = send_runtime_error(&mut socket, None, &error).await;
            return;
        }
    };
    if !catchup.items.is_empty() {
        last_sent_seq = catchup.next_after_seq.unwrap_or(last_sent_seq);
        if send_json(
            &mut socket,
            json!({
                "type": "event.window",
                "requestId": serde_json::Value::Null,
                "reason": "catchup",
                "window": catchup
            }),
        )
        .await
        .is_err()
        {
            return;
        }
    }

    loop {
        tokio::select! {
            changed = receiver.changed() => {
                if changed.is_err() {
                    break;
                }

                let window = match runtime.list_events(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id.as_str(),
                    last_sent_seq,
                    100,
                ) {
                    Ok(window) => window,
                    Err(error) => {
                        let _ = send_runtime_error(&mut socket, None, &error).await;
                        break;
                    }
                };
                if window.items.is_empty() {
                    continue;
                }
                last_sent_seq = window.next_after_seq.unwrap_or(last_sent_seq);
                if send_json(
                    &mut socket,
                    json!({
                        "type": "event.window",
                        "requestId": serde_json::Value::Null,
                        "reason": "push",
                        "window": window
                    }),
                )
                .await
                .is_err()
                {
                    break;
                }
            }
            disconnect_changed = disconnect_receiver.changed() => {
                if disconnect_changed.is_err() {
                    break;
                }
                let current_disconnect_generation = match runtime.disconnect_generation(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id.as_str(),
                ) {
                    Ok(disconnect_generation) => disconnect_generation,
                    Err(error) => {
                        let _ = send_runtime_error(&mut socket, None, &error).await;
                        break;
                    }
                };
                if current_disconnect_generation != disconnect_generation
                {
                    let _ = socket.send(session_disconnect_close_message()).await;
                    break;
                }
            }
            message = socket.next() => {
                let Some(message) = message else {
                    break;
                };
                let Ok(message) = message else {
                    break;
                };
                let current_disconnect_generation = match runtime.disconnect_generation(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id.as_str(),
                ) {
                    Ok(disconnect_generation) => disconnect_generation,
                    Err(error) => {
                        let _ = send_runtime_error(&mut socket, None, &error).await;
                        break;
                    }
                };
                if current_disconnect_generation != disconnect_generation
                {
                    let _ = socket.send(session_disconnect_close_message()).await;
                    break;
                }

                let keep_open = handle_client_message(
                    &mut socket,
                    &runtime,
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    device_id.as_str(),
                    &mut last_sent_seq,
                    message,
                )
                .await;
                if !keep_open {
                    break;
                }
            }
        }
    }
}

fn session_disconnect_close_message() -> Message {
    Message::Close(Some(CloseFrame {
        code: SESSION_DISCONNECT_CLOSE_CODE,
        reason: Utf8Bytes::from_static(SESSION_DISCONNECT_CLOSE_REASON),
    }))
}

async fn handle_client_message(
    socket: &mut WebSocket,
    runtime: &RealtimeDeliveryRuntime,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    last_sent_seq: &mut u64,
    message: Message,
) -> bool {
    match message {
        Message::Text(text) => {
            let frame: ClientFrameEnvelope = match serde_json::from_str(text.as_str()) {
                Ok(frame) => frame,
                Err(_) => {
                    let _ =
                        send_error(socket, None, "invalid_frame", "frame must be valid json").await;
                    return true;
                }
            };

            match frame.frame_type.as_str() {
                "subscriptions.sync" => {
                    let snapshot = match runtime.sync_subscriptions(
                        tenant_id,
                        principal_id,
                        device_id,
                        frame.items,
                    ) {
                        Ok(snapshot) => snapshot,
                        Err(error) => {
                            let _ = send_runtime_error(socket, frame.request_id, &error).await;
                            return true;
                        }
                    };
                    let _ = send_json(
                        socket,
                        json!({
                            "type": "subscriptions.synced",
                            "requestId": frame.request_id,
                            "snapshot": snapshot
                        }),
                    )
                    .await;
                    true
                }
                "events.pull" => {
                    let limit = frame.limit.unwrap_or(100);
                    if limit == 0 {
                        let _ = send_error(
                            socket,
                            frame.request_id,
                            "limit_invalid",
                            "limit must be greater than 0",
                        )
                        .await;
                        return true;
                    }

                    let after_seq = frame.after_seq.unwrap_or(*last_sent_seq);
                    let window = match runtime.list_events(
                        tenant_id,
                        principal_id,
                        device_id,
                        after_seq,
                        limit,
                    ) {
                        Ok(window) => window,
                        Err(error) => {
                            let _ = send_runtime_error(socket, frame.request_id, &error).await;
                            return true;
                        }
                    };
                    *last_sent_seq =
                        (*last_sent_seq).max(window.next_after_seq.unwrap_or(after_seq));
                    let _ = send_json(
                        socket,
                        json!({
                            "type": "event.window",
                            "requestId": frame.request_id,
                            "reason": "pull",
                            "window": window
                        }),
                    )
                    .await;
                    true
                }
                "events.ack" => {
                    let Some(acked_seq) = frame.acked_seq else {
                        let _ = send_error(
                            socket,
                            frame.request_id,
                            "acked_seq_missing",
                            "ackedSeq is required",
                        )
                        .await;
                        return true;
                    };

                    let ack =
                        match runtime.ack_events(tenant_id, principal_id, device_id, acked_seq) {
                            Ok(ack) => ack,
                            Err(error) => {
                                let _ = send_runtime_error(socket, frame.request_id, &error).await;
                                return true;
                            }
                        };
                    *last_sent_seq = (*last_sent_seq).max(ack.acked_through_seq);
                    let _ = send_json(
                        socket,
                        json!({
                            "type": "events.acked",
                            "requestId": frame.request_id,
                            "ack": ack
                        }),
                    )
                    .await;
                    true
                }
                _ => {
                    let _ = send_error(
                        socket,
                        frame.request_id,
                        "frame_type_unsupported",
                        format!("unsupported frame type: {}", frame.frame_type),
                    )
                    .await;
                    true
                }
            }
        }
        Message::Binary(_) => {
            let _ = send_error(
                socket,
                None,
                "binary_unsupported",
                "binary websocket frames are not supported",
            )
            .await;
            true
        }
        Message::Ping(payload) => socket.send(Message::Pong(payload)).await.is_ok(),
        Message::Pong(_) => true,
        Message::Close(frame) => {
            let _ = socket.send(Message::Close(frame)).await;
            false
        }
    }
}

async fn send_error(
    socket: &mut WebSocket,
    request_id: Option<String>,
    code: impl Into<String>,
    message: impl Into<String>,
) -> Result<(), axum::Error> {
    send_json(
        socket,
        json!({
            "type": "error",
            "requestId": request_id,
            "code": code.into(),
            "message": message.into()
        }),
    )
    .await
}

async fn send_runtime_error(
    socket: &mut WebSocket,
    request_id: Option<String>,
    error: &RealtimeRuntimeError,
) -> Result<(), axum::Error> {
    send_error(socket, request_id, error.code, error.message.as_str()).await
}

async fn send_json(socket: &mut WebSocket, value: serde_json::Value) -> Result<(), axum::Error> {
    socket.send(Message::Text(value.to_string().into())).await
}
