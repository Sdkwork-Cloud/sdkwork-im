use std::sync::Arc;

use im_app_context::AppContext;
use im_domain_core::realtime::RealtimeEventWindow;
use sdkwork_im_ccp_control::{ControlFrame, ErrorFrame};
use sdkwork_im_ccp_core::{CcpEnvelope, CcpRoute, TransportBinding};
use sdkwork_im_runtime_link::{
    session_disconnect_goaway, LinkBufferedPushDrainDriver, LinkBufferedPushDrainStatus,
    LinkBufferedPushFetchedWindow, LinkBufferedPushPlan, LinkGoAwayDirective,
    LinkOutboundQueueState, LinkSession, OutboundQueuePolicy, ResumeWindow,
};
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::client_route_registration::ClientRouteRegistration;
use crate::link_business_contract::{validate_link_client_business_envelope, LinkClientBusinessFrame};
use crate::link_framing::{
    read_framed_envelope, write_framed_bytes, FramedStreamCcpCodec,
};
use crate::realtime::{RealtimeDeliveryRuntime, RealtimeRuntimeError, RealtimeSubscriptionItemInput, RealtimeWindowCheckpoint};
use crate::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamClientFrameEnvelope {
    #[serde(rename = "type")]
    frame_type: String,
    request_id: Option<String>,
    #[serde(default)]
    items: Vec<RealtimeSubscriptionItemInput>,
    after_seq: Option<u64>,
    limit: Option<usize>,
    acked_seq: Option<u64>,
}

fn build_link_session(auth: &AppContext, device_id: &str) -> LinkSession {
    LinkSession::new(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id,
        auth.session_id.as_deref(),
        OutboundQueuePolicy::realtime_default(),
    )
}

fn activate_link_session(session: &mut LinkSession, checkpoint: &RealtimeWindowCheckpoint) {
    session.activate(ResumeWindow::new(
        checkpoint.latest_realtime_seq,
        checkpoint.acked_through_seq,
    ));
}

fn map_api_error(error: ApiError) -> String {
    format!("{}: {}", error.code, error.message)
}

pub(crate) async fn serve_realtime_framed_session<R, W>(
    mut reader: R,
    mut writer: W,
    transport: TransportBinding,
    auth: AppContext,
    device_id: String,
    resume_after_seq: Option<u64>,
    runtime: Arc<RealtimeDeliveryRuntime>,
    route_owner: ClientRouteRegistration,
) -> Result<(), String>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let tenant_id = auth.tenant_id.clone();
    let organization_id = auth.organization_id.clone();
    let principal_id = auth.actor_id.clone();
    let principal_kind = auth.actor_kind.clone();
    let authority = auth.ccp_authority();
    let route = CcpRoute::for_principal(
        tenant_id.clone(),
        principal_id.clone(),
        Some(device_id.clone()),
    );
    let ccp = FramedStreamCcpCodec::new(transport.clone());
    let expected_binding = transport.clone();

    if !ensure_framed_route_session(
        &route_owner,
        &auth,
        device_id.as_str(),
        &mut writer,
        &ccp,
        &route,
    )
    .await
    {
        return Ok(());
    }

    let mut route_epoch_receiver = route_owner
        .subscribe_active_client_route_epoch(&auth, device_id.as_str())
        .map_err(map_api_error)?;

    runtime
        .ensure_client_route_state_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    let checkpoint = runtime
        .window_checkpoint_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    let disconnect_generation = runtime
        .disconnect_generation_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    let mut link_session = build_link_session(&auth, device_id.as_str());
    link_session.mark_authenticated();
    activate_link_session(&mut link_session, &checkpoint);
    let resume_after_seq = resume_after_seq.unwrap_or_else(|| {
        checkpoint
            .acked_through_seq
            .max(checkpoint.trimmed_through_seq)
    });
    let mut outbound_queue =
        link_session.start_outbound_queue(resume_after_seq, checkpoint.latest_realtime_seq);

    let mut seq_receiver = runtime
        .subscribe_client_route_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    let mut disconnect_receiver = runtime
        .subscribe_disconnect_signal_for_principal_kind(
            tenant_id.as_str(),
            organization_id.as_str(),
            principal_id.as_str(),
            principal_kind.as_str(),
            device_id.as_str(),
        )
        .map_err(|error| error.message)?;

    send_framed_realtime_connected(
        &mut writer,
        &ccp,
        &route,
        &auth,
        device_id.as_str(),
        &checkpoint,
        &authority.sender.sender_id(),
    )
    .await?;

    if let Some(catchup_plan) = outbound_queue.plan_catchup() {
        if !ensure_framed_route_session(
            &route_owner,
            &auth,
            device_id.as_str(),
            &mut writer,
            &ccp,
            &route,
        )
        .await
        {
            return Ok(());
        }
        let catchup = runtime
            .list_events_for_principal_kind(
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_id.as_str(),
                principal_kind.as_str(),
                device_id.as_str(),
                catchup_plan.after_seq,
                catchup_plan.batch.limit,
            )
            .map_err(|error| error.message)?;
        if !catchup.items.is_empty() {
            let next_after_seq = catchup.next_after_seq;
            send_framed_event_window(
                &mut writer,
                &ccp,
                &route,
                "catchup",
                catchup,
            )
            .await?;
            let _ = outbound_queue.record_window_sent(catchup_plan.after_seq, next_after_seq);
        }
    }

    loop {
        tokio::select! {
            route_epoch_changed = route_epoch_receiver.changed() => {
                if route_epoch_changed.is_err() {
                    break;
                }
                if !ensure_framed_route_session(
                    &route_owner,
                    &auth,
                    device_id.as_str(),
                    &mut writer,
                    &ccp,
                    &route,
                )
                .await
                {
                    break;
                }
            }
            changed = seq_receiver.changed() => {
                if changed.is_err() {
                    break;
                }
                let latest_realtime_seq = *seq_receiver.borrow_and_update();
                let push_plan = outbound_queue.observe_latest_realtime_seq(latest_realtime_seq);
                if !drain_framed_buffered_push(
                    &mut writer,
                    runtime.as_ref(),
                    &route_owner,
                    &auth,
                    tenant_id.as_str(),
                    organization_id.as_str(),
                    principal_id.as_str(),
                    principal_kind.as_str(),
                    device_id.as_str(),
                    &mut outbound_queue,
                    push_plan,
                    &ccp,
                    &route,
                )
                .await
                {
                    break;
                }
            }
            disconnect_changed = disconnect_receiver.changed() => {
                if disconnect_changed.is_err() {
                    break;
                }
                let current = runtime
                    .disconnect_generation_for_principal_kind(
                        tenant_id.as_str(),
                        organization_id.as_str(),
                        principal_id.as_str(),
                        principal_kind.as_str(),
                        device_id.as_str(),
                    )
                    .map_err(|error| error.message)?;
                if current != disconnect_generation {
                    send_framed_session_disconnect(&mut writer, &ccp, &route).await?;
                    break;
                }
            }
            read_result = read_framed_envelope(&mut reader, transport.clone()) => {
                match read_result {
                    Ok(envelope) => {
                        if envelope.binding != expected_binding {
                            return Err("stream link received unexpected binding envelope".into());
                        }
                        if !handle_framed_client_envelope(
                            &mut writer,
                            runtime.as_ref(),
                            &route_owner,
                            &auth,
                            tenant_id.as_str(),
                            organization_id.as_str(),
                            principal_id.as_str(),
                            principal_kind.as_str(),
                            device_id.as_str(),
                            &mut outbound_queue,
                            &envelope,
                            &ccp,
                            &route,
                        )
                        .await
                        {
                            break;
                        }
                    }
                    Err(error) => {
                        if error.contains("early eof") || error.contains("connection") {
                            break;
                        }
                        return Err(error);
                    }
                }
            }
        }
    }

    route_owner.release_active_client_route_if_current_session(&auth, device_id.as_str());
    Ok(())
}

async fn ensure_framed_route_session<W>(
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    device_id: &str,
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    match route_owner.ensure_active_client_route_current_session(auth, device_id) {
        Ok(()) => true,
        Err(error) => {
            let frame = ControlFrame::Error(ErrorFrame {
                code: error.code.into(),
                message: error.message,
                retryable: false,
            });
            let _ = write_framed_bytes(
                writer,
                ccp.encode_control(route, &frame).unwrap_or_default().as_slice(),
            )
            .await;
            false
        }
    }
}

async fn send_framed_realtime_connected<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    auth: &AppContext,
    device_id: &str,
    checkpoint: &RealtimeWindowCheckpoint,
    sender_id: &str,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let authority = auth.ccp_authority();
    let bytes = ccp.encode_business(
        route,
        "evt",
        "cc.realtime.connected.v1",
        json!({
            "type": "realtime.connected",
            "tenantId": auth.tenant_id,
            "principalId": auth.actor_id,
            "deviceId": device_id,
            "actor": {
                "id": authority.actor.actor_id,
                "kind": authority.actor.actor_kind
            },
            "sender": {
                "principalId": authority.sender.principal_id,
                "deviceId": authority.sender.device_id,
                "sessionId": authority.sender.session_id,
                "senderId": sender_id
            },
            "ackedThroughSeq": checkpoint.acked_through_seq,
            "trimmedThroughSeq": checkpoint.trimmed_through_seq,
            "latestRealtimeSeq": checkpoint.latest_realtime_seq
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_event_window<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    reason: &str,
    window: RealtimeEventWindow,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "evt",
        "cc.realtime.event.window.v1",
        json!({
            "type": "event.window",
            "requestId": Value::Null,
            "reason": reason,
            "window": window
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_session_disconnect<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    send_framed_goaway_and_close(writer, ccp, route, &session_disconnect_goaway()).await
}

async fn send_framed_goaway_and_close<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    directive: &LinkGoAwayDirective,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    send_framed_goaway(writer, ccp, route, directive).await?;
    writer
        .shutdown()
        .await
        .map_err(|error| format!("stream shutdown failed: {error}"))
}

enum FramedPushDrainError {
    Runtime(RealtimeRuntimeError),
    Fence,
    Send,
}

struct FramedPushDrainDriver<'a, W> {
    writer: &'a mut W,
    runtime: &'a RealtimeDeliveryRuntime,
    route_owner: &'a ClientRouteRegistration,
    auth: &'a AppContext,
    tenant_id: &'a str,
    organization_id: &'a str,
    principal_id: &'a str,
    principal_kind: &'a str,
    device_id: &'a str,
    ccp: &'a FramedStreamCcpCodec,
    route: &'a CcpRoute,
}

impl<W> LinkBufferedPushDrainDriver for FramedPushDrainDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    type Window = RealtimeEventWindow;
    type Error = FramedPushDrainError;

    async fn fetch_window(
        &mut self,
        after_seq: u64,
        limit: usize,
    ) -> Result<LinkBufferedPushFetchedWindow<Self::Window>, Self::Error> {
        self.ensure_current_route_session()?;
        let window = self
            .runtime
            .list_events_for_principal_kind(
                self.tenant_id,
                self.organization_id,
                self.principal_id,
                self.principal_kind,
                self.device_id,
                after_seq,
                limit,
            )
            .map_err(FramedPushDrainError::Runtime)?;
        Ok(LinkBufferedPushFetchedWindow {
            next_after_seq: window.next_after_seq,
            is_empty: window.items.is_empty(),
            window,
        })
    }

    async fn send_window(&mut self, window: Self::Window) -> Result<(), Self::Error> {
        self.ensure_current_route_session()?;
        send_framed_event_window(self.writer, self.ccp, self.route, "push", window)
            .await
            .map_err(|_| FramedPushDrainError::Send)
    }
}

impl<W> FramedPushDrainDriver<'_, W>
where
    W: AsyncWrite + Unpin,
{
    fn ensure_current_route_session(&self) -> Result<(), FramedPushDrainError> {
        self.route_owner
            .ensure_active_client_route_current_session(self.auth, self.device_id)
            .map_err(|_| FramedPushDrainError::Fence)
    }
}

async fn drain_framed_buffered_push<W>(
    writer: &mut W,
    runtime: &RealtimeDeliveryRuntime,
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    push_plan: Option<LinkBufferedPushPlan>,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    let mut driver = FramedPushDrainDriver {
        writer,
        runtime,
        route_owner,
        auth,
        tenant_id,
        organization_id,
        principal_id,
        principal_kind,
        device_id,
        ccp,
        route,
    };
    match outbound_queue
        .drain_buffered_push_windows(push_plan, &mut driver)
        .await
    {
        Ok(LinkBufferedPushDrainStatus::Drained) | Ok(LinkBufferedPushDrainStatus::PullOnly) => {
            true
        }
        Ok(LinkBufferedPushDrainStatus::Disconnect(directive)) => {
            let _ = send_framed_goaway_and_close(writer, ccp, route, &directive).await;
            false
        }
        Err(FramedPushDrainError::Runtime(error)) => {
            let _ = send_framed_runtime_error(writer, ccp, route, None, &error).await;
            false
        }
        Err(FramedPushDrainError::Fence) => false,
        Err(FramedPushDrainError::Send) => false,
    }
}

async fn send_framed_goaway<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    directive: &LinkGoAwayDirective,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let frame = ControlFrame::GoAway(directive.frame.clone());
    write_framed_bytes(
        writer,
        ccp.encode_control(route, &frame)?.as_slice(),
    )
    .await
}

async fn send_framed_runtime_error<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    request_id: Option<String>,
    error: &RealtimeRuntimeError,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "error",
        "cc.realtime.error.v1",
        json!({
            "type": "error",
            "requestId": request_id,
            "code": error.code,
            "message": error.message
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn send_framed_business_error<W>(
    writer: &mut W,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
    request_id: Option<String>,
    code: &str,
    message: impl Into<String>,
) -> Result<(), String>
where
    W: AsyncWrite + Unpin,
{
    let bytes = ccp.encode_business(
        route,
        "error",
        "cc.realtime.error.v1",
        json!({
            "type": "error",
            "requestId": request_id,
            "code": code,
            "message": message.into()
        }),
    )?;
    write_framed_bytes(writer, bytes.as_slice()).await
}

async fn handle_framed_client_envelope<W>(
    writer: &mut W,
    runtime: &RealtimeDeliveryRuntime,
    route_owner: &ClientRouteRegistration,
    auth: &AppContext,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
    outbound_queue: &mut LinkOutboundQueueState,
    envelope: &CcpEnvelope,
    ccp: &FramedStreamCcpCodec,
    route: &CcpRoute,
) -> bool
where
    W: AsyncWrite + Unpin,
{
    if envelope.kind == "heartbeat" {
        return true;
    }
    if envelope.kind == "goaway" {
        return false;
    }
    if matches!(
        envelope.kind.as_str(),
        "hello" | "hello_ack" | "auth_bind" | "auth_ok" | "session_resume" | "session_resumed"
    ) {
        let _ = send_framed_business_error(
            writer,
            ccp,
            route,
            None,
            "frame_type_unsupported",
            format!("unexpected post-auth control frame: {}", envelope.kind),
        )
        .await;
        return true;
    }

    let frame = match serde_json::from_str::<StreamClientFrameEnvelope>(envelope.payload.as_str()) {
        Ok(frame) => frame,
        Err(_) => {
            let _ = send_framed_business_error(
                writer,
                ccp,
                route,
                None,
                "invalid_frame",
                "frame must be valid json",
            )
            .await;
            return true;
        }
    };

    if let Err(message) = validate_link_client_business_envelope(
        envelope,
        &LinkClientBusinessFrame {
            frame_type: frame.frame_type.clone(),
            request_id: frame.request_id.clone(),
        },
    ) {
        let _ = send_framed_business_error(
            writer,
            ccp,
            route,
            frame.request_id.clone(),
            "invalid_frame",
            message,
        )
        .await;
        return true;
    }

    if !ensure_framed_route_session(route_owner, auth, device_id, writer, ccp, route).await {
        return false;
    }

    match frame.frame_type.as_str() {
        "subscriptions.sync" => {
            match runtime.sync_subscriptions_for_principal_kind(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
                frame.items,
            ) {
                Ok(snapshot) => {
                    let bytes = ccp
                        .encode_business(
                            route,
                            "evt",
                            "cc.realtime.subscriptions.synced.v1",
                            json!({
                                "type": "subscriptions.synced",
                                "requestId": frame.request_id,
                                "snapshot": snapshot
                            }),
                        )
                        .unwrap_or_default();
                    write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                }
                Err(error) => {
                    let _ = send_framed_runtime_error(writer, ccp, route, frame.request_id, &error)
                        .await;
                    true
                }
            }
        }
        "events.pull" => {
            let limit = frame.limit.unwrap_or(100).clamp(1, 500);
            let plan = outbound_queue.plan_pull(frame.after_seq, limit, outbound_queue.latest_realtime_seq());
            match runtime.list_events_for_principal_kind(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
                plan.after_seq,
                plan.batch.limit,
            ) {
                Ok(window) => {
                    let next_after_seq = window.next_after_seq;
                    let _ = send_framed_event_window(writer, ccp, route, "pull", window).await;
                    let _ = outbound_queue.record_window_sent(plan.after_seq, next_after_seq);
                    true
                }
                Err(error) => {
                    let _ = send_framed_runtime_error(writer, ccp, route, frame.request_id, &error)
                        .await;
                    true
                }
            }
        }
        "events.ack" => {
            match frame.acked_seq {
                Some(acked_seq) => match runtime.ack_events_for_principal_kind(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    acked_seq,
                ) {
                    Ok(ack) => {
                        let bytes = ccp
                            .encode_business(
                                route,
                                "evt",
                                "cc.realtime.events.acked.v1",
                                json!({
                                    "type": "events.acked",
                                    "requestId": frame.request_id,
                                    "ack": ack
                                }),
                            )
                            .unwrap_or_default();
                        write_framed_bytes(writer, bytes.as_slice()).await.is_ok()
                    }
                    Err(error) => {
                        let _ =
                            send_framed_runtime_error(writer, ccp, route, frame.request_id, &error)
                                .await;
                        true
                    }
                },
                None => {
                    let _ = send_framed_business_error(
                        writer,
                        ccp,
                        route,
                        frame.request_id,
                        "invalid_frame",
                        "events.ack requires ackedSeq",
                    )
                    .await;
                    true
                }
            }
        }
        _ => {
            let _ = send_framed_business_error(
                writer,
                ccp,
                route,
                frame.request_id,
                "frame_type_unsupported",
                format!("unsupported frame type: {}", frame.frame_type),
            )
            .await;
            true
        }
    }
}
