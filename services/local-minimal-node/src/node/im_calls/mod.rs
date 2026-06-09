use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::rtc::{
    RtcSession, RtcSessionState, RtcSignalEvent, RtcSignalSender, RtcStateStore,
    encode_im_call_key_segments,
};
use sdkwork_rtc_adapter_agora::{AGORA_RTC_PLUGIN_ID, AgoraRtcProvider};
use sdkwork_rtc_adapter_aliyun::{ALIYUN_RTC_PLUGIN_ID, AliyunRtcProvider};
use sdkwork_rtc_adapter_livekit::{LIVEKIT_RTC_PLUGIN_ID, LivekitRtcProvider};
use sdkwork_rtc_adapter_tencent::{TENCENT_RTC_PLUGIN_ID, TencentRtcProvider};
use sdkwork_rtc_adapter_volcengine::{VOLCENGINE_RTC_PLUGIN_ID, VolcengineRtcProvider};
use sdkwork_rtc_app_context::AppContext;
use sdkwork_rtc_core::{
    EffectiveProviderBinding, ProviderDomain, ProviderRegistry, RtcContractError,
    RtcCreateMediaSessionRequest as ProviderRtcCreateSessionRequest, RtcMediaSessionMode,
    RtcParticipantCredential, RtcProviderPort, StaticProviderRegistry, utc_now_rfc3339_millis,
};
use serde::{Deserialize, Serialize};

mod session_store;
mod state_store;

pub(super) use im_domain_core::rtc::RtcStateRecord;
use session_store::ImCallSessionRuntimeStore;
use state_store::MemoryImCallStateStore;
pub(super) use state_store::{FileImCallStateStore, validate_im_call_state_store_file};

const RTC_MAX_SESSION_ID_BYTES: usize = 256;
const RTC_MAX_CONVERSATION_ID_BYTES: usize = 512;
const RTC_MAX_MODE_BYTES: usize = 64;
const RTC_MAX_SIGNALING_STREAM_ID_BYTES: usize = 256;
const RTC_MAX_ARTIFACT_MESSAGE_ID_BYTES: usize = 256;
const RTC_MAX_SIGNAL_TYPE_BYTES: usize = 128;
const RTC_MAX_SIGNAL_SCHEMA_REF_BYTES: usize = 256;
const RTC_MAX_SIGNAL_PAYLOAD_BYTES: usize = 256 * 1024;
const RTC_MAX_PARTICIPANT_ID_BYTES: usize = 256;
const RTC_SESSION_DELIVERY_PROOF_VERSION: &str = "rtc.session.delivery-proof.v1";
const RTC_CREATE_SESSION_LOCK_STRIPES: usize = 256;

fn lock_im_call_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in local-minimal-node IM calls: {label}");
            poisoned.into_inner()
        }
    }
}

pub(super) struct ImCallRuntime {
    sessions: Mutex<ImCallSessionRuntimeStore>,
    signals: Mutex<HashMap<String, BTreeMap<u64, RtcSignalEvent>>>,
    create_session_locks: Arc<Vec<Mutex<()>>>,
    state_store: Arc<dyn RtcStateStore>,
    provider_registry: Arc<dyn ProviderRegistry>,
    rtc_providers: HashMap<String, Arc<dyn RtcProviderPort>>,
}

#[derive(Clone, Debug)]
pub(super) struct RtcSessionMutationOutcome {
    pub(super) session: RtcSession,
    pub(super) applied: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CreateRtcSessionRequest {
    pub(super) rtc_session_id: String,
    pub(super) conversation_id: Option<String>,
    pub(super) rtc_mode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct InviteRtcSessionRequest {
    pub(super) signaling_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateRtcSessionRequest {
    pub(super) artifact_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PostRtcSignalRequest {
    pub(super) signal_type: String,
    pub(super) schema_ref: Option<String>,
    pub(super) payload: String,
    pub(super) signaling_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IssueRtcParticipantCredentialRequest {
    pub(super) participant_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum RtcSessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RtcSessionMutationResponse {
    #[serde(flatten)]
    pub(super) session: RtcSession,
    pub(super) request_key: String,
    pub(super) delivery_status: RtcSessionDeliveryStatus,
    pub(super) proof_version: String,
}

impl RtcSessionMutationResponse {
    pub(super) fn from_outcome(outcome: RtcSessionMutationOutcome, request_key: String) -> Self {
        Self {
            session: outcome.session,
            request_key,
            delivery_status: if outcome.applied {
                RtcSessionDeliveryStatus::Applied
            } else {
                RtcSessionDeliveryStatus::Replayed
            },
            proof_version: RTC_SESSION_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

impl ImCallRuntime {
    pub(super) fn with_store(state_store: Arc<dyn RtcStateStore>) -> Self {
        let volcengine = Arc::new(VolcengineRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let aliyun = Arc::new(AliyunRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let tencent = Arc::new(TencentRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let agora = Arc::new(AgoraRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let livekit = Arc::new(LivekitRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let provider_registry = Arc::new(StaticProviderRegistry::platform_default());
        Self::with_store_and_provider_registry(
            state_store,
            provider_registry,
            [
                (VOLCENGINE_RTC_PLUGIN_ID.into(), volcengine),
                (ALIYUN_RTC_PLUGIN_ID.into(), aliyun),
                (TENCENT_RTC_PLUGIN_ID.into(), tencent),
                (AGORA_RTC_PLUGIN_ID.into(), agora),
                (LIVEKIT_RTC_PLUGIN_ID.into(), livekit),
            ],
        )
    }

    pub(super) fn with_store_and_provider_registry<I>(
        state_store: Arc<dyn RtcStateStore>,
        provider_registry: Arc<dyn ProviderRegistry>,
        rtc_providers: I,
    ) -> Self
    where
        I: IntoIterator<Item = (String, Arc<dyn RtcProviderPort>)>,
    {
        Self {
            sessions: Mutex::new(ImCallSessionRuntimeStore::default()),
            signals: Mutex::new(HashMap::new()),
            create_session_locks: Arc::new(
                (0..RTC_CREATE_SESSION_LOCK_STRIPES)
                    .map(|_| Mutex::new(()))
                    .collect(),
            ),
            state_store,
            provider_registry,
            rtc_providers: rtc_providers.into_iter().collect(),
        }
    }

    fn ensure_session_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<(), ImCallError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let needs_restore =
            !lock_im_call_mutex(&self.sessions, "sessions").has_session(tenant_id, rtc_session_id);
        if !needs_restore {
            lock_im_call_mutex(&self.signals, "signals")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, rtc_session_id)
            .map_err(ImCallError::rtc_store)?;
        if let Some(record) = restored {
            lock_im_call_mutex(&self.sessions, "sessions").insert_session(record.session);
            lock_im_call_mutex(&self.signals, "signals")
                .insert(scope_key, rtc_signal_index(record.signals));
        }

        Ok(())
    }

    pub(super) fn session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
    ) -> Result<RtcSession, ImCallError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        lock_im_call_mutex(&self.sessions, "sessions")
            .session(auth.tenant_id.as_str(), rtc_session_id)
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })
    }

    pub(super) fn create_session_with_outcome(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, ImCallError> {
        validate_create_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        let create_lock_index = hash_scope_lock_index(
            auth.tenant_id.as_str(),
            request.rtc_session_id.as_str(),
            self.create_session_locks.len(),
        );
        let _create_lock = lock_im_call_mutex(
            &self.create_session_locks[create_lock_index],
            "create_session_locks",
        );

        {
            let sessions = lock_im_call_mutex(&self.sessions, "sessions");
            if let Some(existing) =
                sessions.session(auth.tenant_id.as_str(), request.rtc_session_id.as_str())
            {
                if rtc_session_matches_create_request(&existing, auth, &request) {
                    return Ok(RtcSessionMutationOutcome {
                        session: existing,
                        applied: false,
                    });
                }

                return Err(ImCallError::conflict(request.rtc_session_id.as_str()));
            }
        }

        let provider_plugin_id = self.selected_provider_plugin_id(auth.tenant_id.as_str(), None)?;
        let provider = self.rtc_provider(provider_plugin_id.as_str())?;
        let requested_session_id = request.rtc_session_id.clone();
        let requested_conversation_id = request.conversation_id.clone();
        let requested_rtc_mode = request.rtc_mode.clone();
        let provider_media_mode = rtc_media_session_mode_from_im_mode(requested_rtc_mode.as_str());
        let provider_session = provider
            .create_session(ProviderRtcCreateSessionRequest {
                tenant_id: auth.tenant_id.clone(),
                rtc_session_id: requested_session_id.clone(),
                media_mode: provider_media_mode,
                room_id: requested_conversation_id.clone(),
                region: None,
            })
            .map_err(ImCallError::rtc_provider)?;
        let started_at = utc_now_rfc3339_millis();
        let session = RtcSession {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: requested_session_id.clone(),
            conversation_id: requested_conversation_id,
            rtc_mode: requested_rtc_mode,
            initiator_id: auth.actor_id.clone(),
            initiator_kind: auth.actor_kind.clone(),
            provider_plugin_id: Some(provider_plugin_id),
            provider_session_id: Some(provider_session.provider_session_id),
            access_endpoint: provider_session.access_endpoint,
            provider_region: provider_session.region,
            state: RtcSessionState::Started,
            signaling_stream_id: None,
            artifact_message_id: None,
            started_at,
            ended_at: None,
        };

        {
            let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
            if let Some(existing) =
                sessions.session(auth.tenant_id.as_str(), requested_session_id.as_str())
            {
                if rtc_session_matches_create_request(&existing, auth, &request) {
                    return Ok(RtcSessionMutationOutcome {
                        session: existing,
                        applied: false,
                    });
                }
                return Err(ImCallError::conflict(requested_session_id.as_str()));
            }
            sessions.insert_session(session.clone());
        }
        lock_im_call_mutex(&self.signals, "signals")
            .entry(rtc_scope_key(
                auth.tenant_id.as_str(),
                requested_session_id.as_str(),
            ))
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), requested_session_id.as_str())?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub(super) fn invite_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, ImCallError> {
        validate_invite_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
        let outcome = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                if matches!(
                    session.state,
                    RtcSessionState::Rejected | RtcSessionState::Ended
                ) {
                    return Err(ImCallError::state_conflict(
                        rtc_session_id,
                        "invite",
                        &session.state,
                    ));
                }

                if rtc_session_matches_invite_request(session, &request) {
                    return Ok(RtcSessionMutationOutcome {
                        session: session.clone(),
                        applied: false,
                    });
                }

                if matches!(session.state, RtcSessionState::Accepted) {
                    return Err(ImCallError::state_conflict(
                        rtc_session_id,
                        "invite",
                        &session.state,
                    ));
                }

                if let Some(signaling_stream_id) = request.signaling_stream_id {
                    session.signaling_stream_id = Some(signaling_stream_id);
                    return Ok(RtcSessionMutationOutcome {
                        session: session.clone(),
                        applied: true,
                    });
                }

                Err(ImCallError::state_conflict(
                    rtc_session_id,
                    "invite",
                    &session.state,
                ))
            })
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })??;
        drop(sessions);
        if outcome.applied {
            self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        }
        Ok(outcome)
    }

    pub(super) fn accept_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, ImCallError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
        let outcome = sessions
            .update_session(
                auth.tenant_id.as_str(),
                rtc_session_id,
                |session| match session.state {
                    RtcSessionState::Started => {
                        session.state = RtcSessionState::Accepted;
                        session.artifact_message_id = request.artifact_message_id;
                        Ok(RtcSessionMutationOutcome {
                            session: session.clone(),
                            applied: true,
                        })
                    }
                    RtcSessionState::Accepted
                        if rtc_session_matches_update_request(session, &request) =>
                    {
                        Ok(RtcSessionMutationOutcome {
                            session: session.clone(),
                            applied: false,
                        })
                    }
                    _ => Err(ImCallError::state_conflict(
                        rtc_session_id,
                        "accept",
                        &session.state,
                    )),
                },
            )
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })??;
        drop(sessions);
        if outcome.applied {
            self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        }
        Ok(outcome)
    }

    pub(super) fn reject_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, ImCallError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let provider_plugin_id = {
            let sessions = lock_im_call_mutex(&self.sessions, "sessions");
            let session = sessions
                .session(auth.tenant_id.as_str(), rtc_session_id)
                .ok_or_else(|| ImCallError {
                    status: axum::http::StatusCode::NOT_FOUND,
                    code: "rtc_session_not_found",
                    message: format!("rtc session not found: {rtc_session_id}"),
                })?;
            match session.state {
                RtcSessionState::Started => self.selected_provider_plugin_id(
                    auth.tenant_id.as_str(),
                    session.provider_plugin_id.as_deref(),
                )?,
                RtcSessionState::Rejected
                    if rtc_session_matches_update_request(&session, &request) =>
                {
                    return Ok(RtcSessionMutationOutcome {
                        session,
                        applied: false,
                    });
                }
                _ => {
                    return Err(ImCallError::state_conflict(
                        rtc_session_id,
                        "reject",
                        &session.state,
                    ));
                }
            }
        };
        self.rtc_provider(provider_plugin_id.as_str())?
            .close_session(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(ImCallError::rtc_provider)?;

        let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
        let outcome = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                session.state = RtcSessionState::Rejected;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                }
            })
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        Ok(outcome)
    }

    pub(super) fn end_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, ImCallError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let provider_plugin_id = {
            let sessions = lock_im_call_mutex(&self.sessions, "sessions");
            let session = sessions
                .session(auth.tenant_id.as_str(), rtc_session_id)
                .ok_or_else(|| ImCallError {
                    status: axum::http::StatusCode::NOT_FOUND,
                    code: "rtc_session_not_found",
                    message: format!("rtc session not found: {rtc_session_id}"),
                })?;
            match session.state {
                RtcSessionState::Started | RtcSessionState::Accepted => self
                    .selected_provider_plugin_id(
                        auth.tenant_id.as_str(),
                        session.provider_plugin_id.as_deref(),
                    )?,
                RtcSessionState::Ended
                    if rtc_session_matches_update_request(&session, &request) =>
                {
                    return Ok(RtcSessionMutationOutcome {
                        session,
                        applied: false,
                    });
                }
                _ => {
                    return Err(ImCallError::state_conflict(
                        rtc_session_id,
                        "end",
                        &session.state,
                    ));
                }
            }
        };
        self.rtc_provider(provider_plugin_id.as_str())?
            .close_session(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(ImCallError::rtc_provider)?;

        let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
        let outcome = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                session.state = RtcSessionState::Ended;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                }
            })
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        Ok(outcome)
    }

    pub(super) fn post_signal(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<RtcSignalEvent, ImCallError> {
        validate_post_rtc_signal_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_im_call_mutex(&self.sessions, "sessions");
        let signal_session = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                if matches!(
                    session.state,
                    RtcSessionState::Rejected | RtcSessionState::Ended
                ) {
                    return Err(ImCallError {
                        status: axum::http::StatusCode::BAD_REQUEST,
                        code: "rtc_session_closed",
                        message: format!("rtc session is closed: {rtc_session_id}"),
                    });
                }

                if let Some(signaling_stream_id) = request.signaling_stream_id {
                    session.signaling_stream_id = Some(signaling_stream_id);
                }

                Ok(session.clone())
            })
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })??;
        drop(sessions);

        let mut signals = lock_im_call_mutex(&self.signals, "signals");
        let session_signals = signals
            .entry(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id))
            .or_default();
        let next_signal_seq = session_signals
            .last_key_value()
            .map_or(1, |(seq, _)| seq + 1);
        let occurred_at = utc_now_rfc3339_millis();
        let signal = RtcSignalEvent {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: signal_session.rtc_session_id,
            signal_seq: next_signal_seq,
            conversation_id: signal_session.conversation_id,
            rtc_mode: signal_session.rtc_mode,
            signal_type: request.signal_type,
            schema_ref: request.schema_ref,
            payload: request.payload,
            sender: RtcSignalSender {
                id: auth.actor_id.clone(),
                kind: auth.actor_kind.clone(),
                member_id: None,
                device_id: auth.device_id.clone(),
                session_id: auth.session_id.clone(),
                metadata: BTreeMap::new(),
            },
            signaling_stream_id: signal_session.signaling_stream_id,
            occurred_at,
        };

        session_signals.insert(next_signal_seq, signal.clone());
        drop(signals);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(signal)
    }

    pub(super) fn issue_participant_credential(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, ImCallError> {
        validate_payload_size(
            "participantId",
            participant_id,
            RTC_MAX_PARTICIPANT_ID_BYTES,
        )?;
        let session = self.session(auth, rtc_session_id)?;
        let provider = self.rtc_provider_for_session(auth.tenant_id.as_str(), &session)?;
        provider
            .issue_participant_credential(auth.tenant_id.as_str(), rtc_session_id, participant_id)
            .map_err(ImCallError::rtc_provider)
    }

    pub(super) fn provider_binding(
        &self,
        tenant_id: Option<&str>,
    ) -> Result<EffectiveProviderBinding, ImCallError> {
        self.provider_registry
            .effective_binding(ProviderDomain::Rtc, tenant_id)
            .ok_or_else(|| {
                ImCallError::provider_binding_missing(
                    "rtc provider binding is missing for the current scope",
                )
            })
    }

    fn persist_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), ImCallError> {
        let record = self.state_record(tenant_id, rtc_session_id)?;
        self.state_store
            .save_state(record)
            .map_err(ImCallError::rtc_store)
    }

    fn state_record(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<RtcStateRecord, ImCallError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let session = lock_im_call_mutex(&self.sessions, "sessions")
            .session(tenant_id, rtc_session_id)
            .ok_or_else(|| ImCallError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        let signals = lock_im_call_mutex(&self.signals, "signals")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default()
            .into_values()
            .collect();

        Ok(RtcStateRecord {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            session,
            signals,
            updated_at: utc_now_rfc3339_millis(),
        })
    }

    fn selected_provider_plugin_id(
        &self,
        tenant_id: &str,
        frozen_plugin_id: Option<&str>,
    ) -> Result<String, ImCallError> {
        if let Some(plugin_id) = frozen_plugin_id.filter(|value| !value.trim().is_empty()) {
            return Ok(plugin_id.to_string());
        }

        let binding = self
            .provider_registry
            .effective_binding(ProviderDomain::Rtc, Some(tenant_id))
            .ok_or_else(|| {
                ImCallError::provider_binding_missing(
                    "rtc provider binding is missing for the current tenant",
                )
            })?;
        binding
            .selected_plugin_id
            .or(binding.default_plugin_id)
            .ok_or_else(|| {
                ImCallError::provider_binding_missing(
                    "rtc provider selection is missing for the current tenant",
                )
            })
    }

    fn rtc_provider_for_session(
        &self,
        tenant_id: &str,
        session: &RtcSession,
    ) -> Result<Arc<dyn RtcProviderPort>, ImCallError> {
        let plugin_id =
            self.selected_provider_plugin_id(tenant_id, session.provider_plugin_id.as_deref())?;
        self.rtc_provider(plugin_id.as_str())
    }

    fn rtc_provider(&self, plugin_id: &str) -> Result<Arc<dyn RtcProviderPort>, ImCallError> {
        self.rtc_providers.get(plugin_id).cloned().ok_or_else(|| {
            ImCallError::provider_binding_missing(format!(
                "rtc provider is not installed in IM calls runtime: {plugin_id}"
            ))
        })
    }
}

#[derive(Debug)]
pub(super) struct ImCallError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ImCallError {
    fn rtc_store(value: RtcContractError) -> Self {
        match value {
            RtcContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_store_unavailable",
                message,
            },
            RtcContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_store_conflict",
                message,
            },
            RtcContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "rtc_store_unsupported",
                message,
            },
        }
    }

    fn rtc_provider(value: RtcContractError) -> Self {
        match value {
            RtcContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_provider_unavailable",
                message,
            },
            RtcContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_provider_conflict",
                message,
            },
            RtcContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "rtc_provider_unsupported",
                message,
            },
        }
    }

    fn provider_binding_missing(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "rtc_provider_binding_missing",
            message: message.into(),
        }
    }

    fn conflict(rtc_session_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "rtc_session_conflict",
            message: format!(
                "rtc session create request conflicts with existing rtc session idempotency key: {rtc_session_id}"
            ),
        }
    }

    fn state_conflict(
        rtc_session_id: &str,
        transition: &'static str,
        current_state: &RtcSessionState,
    ) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "rtc_session_state_conflict",
            message: format!(
                "rtc session transition {transition} conflicts with current state {}: {rtc_session_id}",
                current_state.as_wire_value()
            ),
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    pub(super) fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub(super) fn code(&self) -> &'static str {
        self.code
    }

    pub(super) fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Default for ImCallRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(MemoryImCallStateStore::default()))
    }
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), ImCallError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(ImCallError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

fn validate_create_rtc_session_request_payload_size(
    request: &CreateRtcSessionRequest,
) -> Result<(), ImCallError> {
    validate_payload_size(
        "rtcSessionId",
        request.rtc_session_id.as_str(),
        RTC_MAX_SESSION_ID_BYTES,
    )?;
    if let Some(conversation_id) = request.conversation_id.as_deref() {
        validate_payload_size(
            "conversationId",
            conversation_id,
            RTC_MAX_CONVERSATION_ID_BYTES,
        )?;
    }
    validate_payload_size("rtcMode", request.rtc_mode.as_str(), RTC_MAX_MODE_BYTES)?;
    Ok(())
}

fn validate_invite_rtc_session_request_payload_size(
    request: &InviteRtcSessionRequest,
) -> Result<(), ImCallError> {
    if let Some(signaling_stream_id) = request.signaling_stream_id.as_deref() {
        validate_payload_size(
            "signalingStreamId",
            signaling_stream_id,
            RTC_MAX_SIGNALING_STREAM_ID_BYTES,
        )?;
    }
    Ok(())
}

fn validate_update_rtc_session_request_payload_size(
    request: &UpdateRtcSessionRequest,
) -> Result<(), ImCallError> {
    if let Some(artifact_message_id) = request.artifact_message_id.as_deref() {
        validate_payload_size(
            "artifactMessageId",
            artifact_message_id,
            RTC_MAX_ARTIFACT_MESSAGE_ID_BYTES,
        )?;
    }
    Ok(())
}

fn validate_post_rtc_signal_request_payload_size(
    request: &PostRtcSignalRequest,
) -> Result<(), ImCallError> {
    validate_payload_size(
        "signalType",
        request.signal_type.as_str(),
        RTC_MAX_SIGNAL_TYPE_BYTES,
    )?;
    if let Some(schema_ref) = request.schema_ref.as_deref() {
        validate_payload_size("schemaRef", schema_ref, RTC_MAX_SIGNAL_SCHEMA_REF_BYTES)?;
    }
    validate_payload_size(
        "payload",
        request.payload.as_str(),
        RTC_MAX_SIGNAL_PAYLOAD_BYTES,
    )?;
    if let Some(signaling_stream_id) = request.signaling_stream_id.as_deref() {
        validate_payload_size(
            "signalingStreamId",
            signaling_stream_id,
            RTC_MAX_SIGNALING_STREAM_ID_BYTES,
        )?;
    }
    Ok(())
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    encode_im_call_key_segments([tenant_id, rtc_session_id])
}

fn rtc_signal_index(signals: Vec<RtcSignalEvent>) -> BTreeMap<u64, RtcSignalEvent> {
    signals
        .into_iter()
        .map(|signal| (signal.signal_seq, signal))
        .collect()
}

fn hash_scope_lock_index(tenant_id: &str, session_id: &str, stripe_count: usize) -> usize {
    if stripe_count == 0 {
        return 0;
    }
    let mut hasher = DefaultHasher::new();
    tenant_id.hash(&mut hasher);
    session_id.hash(&mut hasher);
    (hasher.finish() as usize) % stripe_count
}

pub(super) fn rtc_create_request_key(
    auth: &AppContext,
    request: &CreateRtcSessionRequest,
) -> String {
    encode_legacy_rtc_request_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "create",
        request.rtc_session_id.as_str(),
    ])
}

pub(super) fn rtc_session_action_request_key(
    tenant_id: &str,
    rtc_session_id: &str,
    action: &str,
) -> String {
    encode_legacy_rtc_request_key_segments([tenant_id, action, rtc_session_id])
}

fn encode_legacy_rtc_request_key_segments<const N: usize>(segments: [&str; N]) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

fn rtc_session_matches_create_request(
    session: &RtcSession,
    auth: &AppContext,
    request: &CreateRtcSessionRequest,
) -> bool {
    session.rtc_session_id == request.rtc_session_id.as_str()
        && session.initiator_id == auth.actor_id.as_str()
        && session.initiator_kind == auth.actor_kind
        && session.conversation_id.as_ref() == request.conversation_id.as_ref()
        && session.rtc_mode == request.rtc_mode.as_str()
}

fn rtc_media_session_mode_from_im_mode(rtc_mode: &str) -> RtcMediaSessionMode {
    match rtc_mode.trim().to_ascii_lowercase().as_str() {
        "voice" | "audio" => RtcMediaSessionMode::Audio,
        "live" | "livestream" | "live_stream" => RtcMediaSessionMode::Live,
        _ => RtcMediaSessionMode::Video,
    }
}

fn rtc_session_matches_invite_request(
    session: &RtcSession,
    request: &InviteRtcSessionRequest,
) -> bool {
    session.signaling_stream_id.as_ref() == request.signaling_stream_id.as_ref()
}

fn rtc_session_matches_update_request(
    session: &RtcSession,
    request: &UpdateRtcSessionRequest,
) -> bool {
    session.artifact_message_id.as_ref() == request.artifact_message_id.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtc_media_session_mode_from_im_mode_maps_legacy_im_modes() {
        assert_eq!(
            rtc_media_session_mode_from_im_mode("voice"),
            RtcMediaSessionMode::Audio
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode(" audio "),
            RtcMediaSessionMode::Audio
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode("live"),
            RtcMediaSessionMode::Live
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode("livestream"),
            RtcMediaSessionMode::Live
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode("live_stream"),
            RtcMediaSessionMode::Live
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode("video"),
            RtcMediaSessionMode::Video
        );
        assert_eq!(
            rtc_media_session_mode_from_im_mode("call"),
            RtcMediaSessionMode::Video
        );
    }
}
