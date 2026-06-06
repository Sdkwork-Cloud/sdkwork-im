use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_api_registry::HttpMethod;
use craw_chat_contract_core::ContractError;
use craw_chat_contract_rtc::{RtcStateRecord, RtcStateStore};
use craw_chat_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use im_adapter_rtc_aliyun::{ALIYUN_RTC_PLUGIN_ID, AliyunRtcProvider};
use im_adapter_rtc_tencent::{TENCENT_RTC_PLUGIN_ID, TencentRtcProvider};
use im_adapter_rtc_volcengine::{VOLCENGINE_RTC_PLUGIN_ID, VolcengineRtcProvider};
use im_app_context::{AppContext, AppContextError, resolve_app_context};
use im_domain_core::message::Sender;
use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use im_platform_contracts::{
    EffectiveProviderBinding, ProviderDomain, ProviderHealthSnapshot, ProviderRegistry,
    RtcCallbackEvent, RtcCallbackRequest,
    RtcCreateSessionRequest as ProviderRtcCreateSessionRequest, RtcParticipantCredential,
    RtcProviderPort, RtcRecordingArtifact, StaticProviderRegistry,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

mod session_store;

use session_store::RtcSessionRuntimeStore;

const RTC_MAX_SESSION_ID_BYTES: usize = 256;
const RTC_MAX_CONVERSATION_ID_BYTES: usize = 512;
const RTC_MAX_MODE_BYTES: usize = 64;
const RTC_MAX_SIGNALING_STREAM_ID_BYTES: usize = 256;
const RTC_MAX_ARTIFACT_MESSAGE_ID_BYTES: usize = 256;
const RTC_MAX_SIGNAL_TYPE_BYTES: usize = 128;
const RTC_MAX_SIGNAL_SCHEMA_REF_BYTES: usize = 256;
const RTC_MAX_SIGNAL_PAYLOAD_BYTES: usize = 256 * 1024;
const RTC_MAX_PARTICIPANT_ID_BYTES: usize = 256;
const RTC_SIGNAL_LIST_MAX_LIMIT: usize = 1000;
const RTC_SESSION_DELIVERY_PROOF_VERSION: &str = "rtc.session.delivery-proof.v1";
const RTC_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "CRAW_CHAT_RTC_MAX_IN_FLIGHT_REQUESTS";
const RTC_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const RTC_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const RTC_MAX_REQUEST_BODY_BYTES_ENV: &str = "CRAW_CHAT_RTC_MAX_REQUEST_BODY_BYTES";
const RTC_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const RTC_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const RTC_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str = "CRAW_CHAT_RTC_REQUIRE_DUAL_TOKEN_HEADERS";
const RTC_CREATE_SESSION_LOCK_STRIPES: usize = 256;

fn lock_rtc_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned mutex in rtc-signaling-service: {label}");
            poisoned.into_inner()
        }
    }
}

#[derive(Clone)]
struct AppState {
    runtime: Arc<RtcRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}

pub struct RtcRuntime {
    sessions: Mutex<RtcSessionRuntimeStore>,
    signals: Mutex<HashMap<String, BTreeMap<u64, RtcSignalEvent>>>,
    create_session_locks: Arc<Vec<Mutex<()>>>,
    state_store: Arc<dyn RtcStateStore>,
    provider_registry: Arc<dyn ProviderRegistry>,
    rtc_providers: HashMap<String, Arc<dyn RtcProviderPort>>,
}

#[derive(Clone, Debug)]
pub struct RtcSessionMutationOutcome {
    pub session: RtcSession,
    pub applied: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRtcSessionRequest {
    pub rtc_session_id: String,
    pub conversation_id: Option<String>,
    pub rtc_mode: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRtcSessionRequest {
    pub signaling_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRtcSessionRequest {
    pub artifact_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRtcSignalRequest {
    pub signal_type: String,
    pub schema_ref: Option<String>,
    pub payload: String,
    pub signaling_stream_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRtcSignalsQuery {
    pub after_signal_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSignalWindow {
    pub items: Vec<RtcSignalEvent>,
    pub next_after_signal_seq: Option<u64>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueRtcParticipantCredentialRequest {
    pub participant_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RtcSessionDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RtcSessionMutationResponse {
    #[serde(flatten)]
    pub session: RtcSession,
    pub request_key: String,
    pub delivery_status: RtcSessionDeliveryStatus,
    pub proof_version: String,
}

impl RtcSessionMutationResponse {
    pub fn from_outcome(outcome: RtcSessionMutationOutcome, request_key: String) -> Self {
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

impl RtcRuntime {
    pub fn with_store(state_store: Arc<dyn RtcStateStore>) -> Self {
        let volcengine = Arc::new(VolcengineRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let aliyun = Arc::new(AliyunRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let tencent = Arc::new(TencentRtcProvider::default()) as Arc<dyn RtcProviderPort>;
        let provider_registry = Arc::new(StaticProviderRegistry::platform_default());
        Self::with_store_and_provider_registry(
            state_store,
            provider_registry,
            [
                (VOLCENGINE_RTC_PLUGIN_ID.into(), volcengine),
                (ALIYUN_RTC_PLUGIN_ID.into(), aliyun),
                (TENCENT_RTC_PLUGIN_ID.into(), tencent),
            ],
        )
    }

    pub fn with_store_and_provider_registry<I>(
        state_store: Arc<dyn RtcStateStore>,
        provider_registry: Arc<dyn ProviderRegistry>,
        rtc_providers: I,
    ) -> Self
    where
        I: IntoIterator<Item = (String, Arc<dyn RtcProviderPort>)>,
    {
        Self {
            sessions: Mutex::new(RtcSessionRuntimeStore::default()),
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

    fn ensure_session_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), RtcError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let needs_restore =
            !lock_rtc_mutex(&self.sessions, "sessions").has_session(tenant_id, rtc_session_id);
        if !needs_restore {
            lock_rtc_mutex(&self.signals, "signals")
                .entry(scope_key)
                .or_default();
            return Ok(());
        }

        let restored = self
            .state_store
            .load_state(tenant_id, rtc_session_id)
            .map_err(RtcError::rtc_store)?;
        if let Some(record) = restored {
            lock_rtc_mutex(&self.sessions, "sessions").insert_session(record.session);
            lock_rtc_mutex(&self.signals, "signals")
                .insert(scope_key, rtc_signal_index(record.signals));
        }

        Ok(())
    }

    pub fn session(&self, auth: &AppContext, rtc_session_id: &str) -> Result<RtcSession, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        lock_rtc_mutex(&self.sessions, "sessions")
            .session(auth.tenant_id.as_str(), rtc_session_id)
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })
    }

    pub fn create_session(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self.create_session_with_outcome(auth, request)?.session)
    }

    pub fn create_session_with_outcome(
        &self,
        auth: &AppContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_create_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        let create_lock_index = hash_scope_lock_index(
            auth.tenant_id.as_str(),
            request.rtc_session_id.as_str(),
            self.create_session_locks.len(),
        );
        let _create_lock = lock_rtc_mutex(
            &self.create_session_locks[create_lock_index],
            "create_session_locks",
        );

        {
            let sessions = lock_rtc_mutex(&self.sessions, "sessions");
            if let Some(existing) =
                sessions.session(auth.tenant_id.as_str(), request.rtc_session_id.as_str())
            {
                if rtc_session_matches_create_request(&existing, auth, &request) {
                    return Ok(RtcSessionMutationOutcome {
                        session: existing,
                        applied: false,
                    });
                }

                return Err(RtcError::conflict(request.rtc_session_id.as_str()));
            }
        }

        let provider_plugin_id = self.selected_provider_plugin_id(auth.tenant_id.as_str(), None)?;
        let provider = self.rtc_provider(provider_plugin_id.as_str())?;
        let requested_session_id = request.rtc_session_id.clone();
        let requested_conversation_id = request.conversation_id.clone();
        let requested_rtc_mode = request.rtc_mode.clone();
        let provider_session = provider
            .create_session(ProviderRtcCreateSessionRequest {
                tenant_id: auth.tenant_id.clone(),
                rtc_session_id: requested_session_id.clone(),
                conversation_id: requested_conversation_id.clone(),
                rtc_mode: requested_rtc_mode.clone(),
                initiator_id: auth.actor_id.clone(),
            })
            .map_err(RtcError::rtc_provider)?;
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
            let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
            if let Some(existing) =
                sessions.session(auth.tenant_id.as_str(), requested_session_id.as_str())
            {
                if rtc_session_matches_create_request(&existing, auth, &request) {
                    return Ok(RtcSessionMutationOutcome {
                        session: existing,
                        applied: false,
                    });
                }
                return Err(RtcError::conflict(requested_session_id.as_str()));
            }
            sessions.insert_session(session.clone());
        }
        lock_rtc_mutex(&self.signals, "signals")
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

    pub fn issue_participant_credential(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        participant_id: &str,
    ) -> Result<RtcParticipantCredential, RtcError> {
        validate_payload_size(
            "participantId",
            participant_id,
            RTC_MAX_PARTICIPANT_ID_BYTES,
        )?;
        let session = self.session(auth, rtc_session_id)?;
        let provider = self.rtc_provider_for_session(auth.tenant_id.as_str(), &session)?;
        provider
            .issue_participant_credential(auth.tenant_id.as_str(), rtc_session_id, participant_id)
            .map_err(RtcError::rtc_provider)
    }

    pub fn map_provider_callback(
        &self,
        auth: &AppContext,
        request: RtcCallbackRequest,
    ) -> Result<RtcCallbackEvent, RtcError> {
        validate_rtc_callback_request_payload_size(&request)?;
        let provider = self
            .rtc_provider_for_callback(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        provider
            .map_provider_callback(request)
            .map_err(RtcError::rtc_provider)
    }

    pub fn recording_artifact(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
    ) -> Result<RtcRecordingArtifact, RtcError> {
        let session = self.session(auth, rtc_session_id)?;
        let provider = self.rtc_provider_for_session(auth.tenant_id.as_str(), &session)?;
        let artifact = provider
            .export_recording_artifact(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(RtcError::rtc_provider)?
            .ok_or_else(|| RtcError::recording_artifact_not_found(rtc_session_id))?;
        Ok(artifact)
    }

    pub fn provider_health_snapshot(
        &self,
        tenant_id: &str,
    ) -> Result<ProviderHealthSnapshot, RtcError> {
        let provider =
            self.rtc_provider(self.selected_provider_plugin_id(tenant_id, None)?.as_str())?;
        Ok(provider.provider_health_snapshot())
    }

    pub fn provider_binding(
        &self,
        tenant_id: Option<&str>,
    ) -> Result<EffectiveProviderBinding, RtcError> {
        self.provider_registry
            .effective_binding(ProviderDomain::Rtc, tenant_id)
            .ok_or_else(|| {
                RtcError::provider_binding_missing(
                    "rtc provider binding is missing for the current scope",
                )
            })
    }

    pub fn sessions_for_conversation(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Vec<RtcSession> {
        lock_rtc_mutex(&self.sessions, "sessions")
            .sessions_for_conversation(tenant_id, conversation_id)
    }

    pub fn sessions_for_state(&self, tenant_id: &str, state: RtcSessionState) -> Vec<RtcSession> {
        lock_rtc_mutex(&self.sessions, "sessions").sessions_for_state(tenant_id, &state)
    }

    pub fn invite_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .invite_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn invite_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_invite_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let outcome = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                if matches!(
                    session.state,
                    RtcSessionState::Rejected | RtcSessionState::Ended
                ) {
                    return Err(RtcError::state_conflict(
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
                    return Err(RtcError::state_conflict(
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

                Err(RtcError::state_conflict(
                    rtc_session_id,
                    "invite",
                    &session.state,
                ))
            })
            .ok_or_else(|| RtcError {
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

    pub fn accept_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .accept_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn accept_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
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
                    _ => Err(RtcError::state_conflict(
                        rtc_session_id,
                        "accept",
                        &session.state,
                    )),
                },
            )
            .ok_or_else(|| RtcError {
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

    pub fn reject_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .reject_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn reject_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let provider_plugin_id = {
            let sessions = lock_rtc_mutex(&self.sessions, "sessions");
            let session = sessions
                .session(auth.tenant_id.as_str(), rtc_session_id)
                .ok_or_else(|| RtcError {
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
                    return Err(RtcError::state_conflict(
                        rtc_session_id,
                        "reject",
                        &session.state,
                    ));
                }
            }
        };
        self.rtc_provider(provider_plugin_id.as_str())?
            .close_session(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(RtcError::rtc_provider)?;

        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
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
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        Ok(outcome)
    }

    pub fn end_session(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .end_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn end_session_with_outcome(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let provider_plugin_id = {
            let sessions = lock_rtc_mutex(&self.sessions, "sessions");
            let session = sessions
                .session(auth.tenant_id.as_str(), rtc_session_id)
                .ok_or_else(|| RtcError {
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
                    return Err(RtcError::state_conflict(
                        rtc_session_id,
                        "end",
                        &session.state,
                    ));
                }
            }
        };
        self.rtc_provider(provider_plugin_id.as_str())?
            .close_session(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(RtcError::rtc_provider)?;

        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
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
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        drop(sessions);
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
        Ok(outcome)
    }

    pub fn post_signal(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<RtcSignalEvent, RtcError> {
        validate_post_rtc_signal_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let signal_session = sessions
            .update_session(auth.tenant_id.as_str(), rtc_session_id, |session| {
                if matches!(
                    session.state,
                    RtcSessionState::Rejected | RtcSessionState::Ended
                ) {
                    return Err(RtcError {
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
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })??;
        drop(sessions);

        let mut signals = lock_rtc_mutex(&self.signals, "signals");
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
            sender: Sender {
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

    pub fn list_signals(
        &self,
        auth: &AppContext,
        rtc_session_id: &str,
        query: ListRtcSignalsQuery,
    ) -> Result<RtcSignalWindow, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let after_signal_seq = query.after_signal_seq.unwrap_or(0);
        let limit = query.limit.unwrap_or(100);
        if limit == 0 {
            return Err(RtcError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }
        if limit > RTC_SIGNAL_LIST_MAX_LIMIT {
            return Err(RtcError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!("limit must be less than or equal to {RTC_SIGNAL_LIST_MAX_LIMIT}"),
            });
        }

        let scope_key = rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id);
        let sessions = lock_rtc_mutex(&self.sessions, "sessions");
        sessions
            .session(auth.tenant_id.as_str(), rtc_session_id)
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        drop(sessions);

        let signals = lock_rtc_mutex(&self.signals, "signals");
        let mut has_more = false;
        let mut items = Vec::new();
        if let Some(session_signals) = signals.get(scope_key.as_str()) {
            for (_, signal) in session_signals.range((Excluded(after_signal_seq), Unbounded)) {
                if items.len() == limit {
                    has_more = true;
                    break;
                }
                items.push(signal.clone());
            }
        }
        let next_after_signal_seq = items.last().map(|signal| signal.signal_seq);

        Ok(RtcSignalWindow {
            items,
            next_after_signal_seq,
            has_more,
        })
    }

    fn persist_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), RtcError> {
        let record = self.state_record(tenant_id, rtc_session_id)?;
        self.state_store
            .save_state(record)
            .map_err(RtcError::rtc_store)
    }

    fn state_record(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<RtcStateRecord, RtcError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let session = lock_rtc_mutex(&self.sessions, "sessions")
            .session(tenant_id, rtc_session_id)
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        let signals = lock_rtc_mutex(&self.signals, "signals")
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
    ) -> Result<String, RtcError> {
        if let Some(plugin_id) = frozen_plugin_id.filter(|value| !value.trim().is_empty()) {
            return Ok(plugin_id.to_string());
        }

        let binding = self
            .provider_registry
            .effective_binding(ProviderDomain::Rtc, Some(tenant_id))
            .ok_or_else(|| {
                RtcError::provider_binding_missing(
                    "rtc provider binding is missing for the current tenant",
                )
            })?;
        binding
            .selected_plugin_id
            .or(binding.default_plugin_id)
            .ok_or_else(|| {
                RtcError::provider_binding_missing(
                    "rtc provider selection is missing for the current tenant",
                )
            })
    }

    fn rtc_provider_for_session(
        &self,
        tenant_id: &str,
        session: &RtcSession,
    ) -> Result<Arc<dyn RtcProviderPort>, RtcError> {
        let plugin_id =
            self.selected_provider_plugin_id(tenant_id, session.provider_plugin_id.as_deref())?;
        self.rtc_provider(plugin_id.as_str())
    }

    fn rtc_provider_for_callback(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Arc<dyn RtcProviderPort>, RtcError> {
        self.ensure_session_state(tenant_id, rtc_session_id)?;
        if let Some(session) =
            lock_rtc_mutex(&self.sessions, "sessions").session(tenant_id, rtc_session_id)
        {
            return self.rtc_provider_for_session(tenant_id, &session);
        }

        let plugin_id = self.selected_provider_plugin_id(tenant_id, None)?;
        self.rtc_provider(plugin_id.as_str())
    }

    fn rtc_provider(&self, plugin_id: &str) -> Result<Arc<dyn RtcProviderPort>, RtcError> {
        self.rtc_providers.get(plugin_id).cloned().ok_or_else(|| {
            RtcError::provider_binding_missing(format!(
                "rtc provider is not installed in runtime: {plugin_id}"
            ))
        })
    }
}

#[derive(Debug)]
pub struct RtcError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl RtcError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn rtc_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "rtc_store_unsupported",
                message,
            },
        }
    }

    fn rtc_provider(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_provider_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_provider_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
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

    fn recording_artifact_not_found(rtc_session_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "rtc_recording_artifact_not_found",
            message: format!("rtc recording artifact not found: {rtc_session_id}"),
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

    pub fn status(&self) -> axum::http::StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryRtcStateStore {
    states: Arc<Mutex<HashMap<String, RtcStateRecord>>>,
}

impl RtcStateStore for RuntimeMemoryRtcStateStore {
    fn load_state(
        &self,
        tenant_id: &str,
        rtc_session_id: &str,
    ) -> Result<Option<RtcStateRecord>, ContractError> {
        Ok(lock_rtc_mutex(&self.states, "state_store")
            .get(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .cloned())
    }

    fn save_state(&self, record: RtcStateRecord) -> Result<(), ContractError> {
        let key = rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str());
        let mut states = lock_rtc_mutex(&self.states, "state_store");
        let next = states
            .remove(key.as_str())
            .map(|previous| previous.merge_monotonic(record.clone()))
            .unwrap_or(record);
        states.insert(key, next);
        Ok(())
    }

    fn clear_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<bool, ContractError> {
        Ok(lock_rtc_mutex(&self.states, "state_store")
            .remove(rtc_scope_key(tenant_id, rtc_session_id).as_str())
            .is_some())
    }
}

impl Default for RtcRuntime {
    fn default() -> Self {
        Self::with_store(Arc::new(RuntimeMemoryRtcStateStore::default()))
    }
}

impl From<AppContextError> for RtcError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl axum::response::IntoResponse for RtcError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(RtcRuntime::default()))
}

pub fn build_public_app() -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
    };
    build_default_app()
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
}

pub fn build_app(runtime: Arc<RtcRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route("/im/v3/api/rtc/sessions", post(create_session))
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/invite",
            post(invite_session),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/accept",
            post(accept_session),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/reject",
            post(reject_session),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/end",
            post(end_session),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/signals",
            post(post_signal).get(list_signals),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/credentials",
            post(issue_participant_credential),
        )
        .route(
            "/im/v3/api/rtc/sessions/{rtc_session_id}/artifacts/recording",
            get(get_recording_artifact),
        )
        .route(
            "/app/v3/api/rtc/provider_callbacks",
            post(map_provider_callback),
        )
        .route("/app/v3/api/rtc/provider_health", get(get_provider_health))
        .with_state(AppState { runtime })
}

async fn require_app_context(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" | "/openapi.json" | "/docs" => next.run(request).await,
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return RtcError {
                        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        code: "http_overloaded",
                        message:
                            "server is at maximum in-flight request capacity, please retry later"
                                .to_owned(),
                    }
                    .into_response();
                }
            };
            if guardrails.require_dual_token_headers
                && let Err(error) = require_dual_token_headers(request.headers())
            {
                return error.into_response();
            }
            let auth = match resolve_app_context(request.headers()) {
                Ok(auth) => auth,
                Err(error) => return RtcError::from(error).into_response(),
            };
            request.extensions_mut().insert(auth);
            let response = next.run(request).await;
            drop(permit);
            response
        }
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "rtc-signaling-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "rtc-signaling-service",
    })
}

async fn openapi_json() -> Result<Json<serde_json::Value>, RtcError> {
    Ok(Json(
        build_rtc_signaling_service_openapi_document()
            .map_err(|message| RtcError::internal("openapi_export_failed", message))?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&rtc_signaling_service_openapi_spec()))
}

fn build_rtc_signaling_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &rtc_signaling_service_openapi_spec(),
        &routes,
        rtc_signaling_service_tag,
        rtc_signaling_service_requires_app_context,
        rtc_signaling_service_summary,
    ))
}

fn rtc_signaling_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat RTC Signaling Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the rtc-signaling-service router for RTC session lifecycle, signaling, credential issue, callback mapping, recording artifacts, and provider health flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn rtc_signaling_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        path if path.contains("provider") => "providers".to_owned(),
        path if path.contains("credentials") => "credentials".to_owned(),
        path if path.contains("signals") => "signals".to_owned(),
        _ => "rtc".to_owned(),
    }
}

fn rtc_signaling_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn rtc_signaling_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check rtc signaling service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check rtc signaling service readiness".to_owned(),
        _ => format!(
            "{} {}",
            rtc_signaling_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn rtc_signaling_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

async fn create_session(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_create_allowed(&request)?;
    let request_key = rtc_create_request_key(&auth, &request);
    let outcome = state.runtime.create_session_with_outcome(&auth, request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

async fn invite_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "invite");
    let outcome =
        state
            .runtime
            .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

async fn accept_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "accept");
    let outcome =
        state
            .runtime
            .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

async fn reject_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "reject");
    let outcome =
        state
            .runtime
            .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

async fn end_session(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "end");
    let outcome =
        state
            .runtime
            .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

async fn post_signal(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<RtcSignalEvent>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.post_signal(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn list_signals(
    Path(rtc_session_id): Path<String>,
    Query(query): Query<ListRtcSignalsQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RtcSignalWindow>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.list_signals(
        &auth,
        rtc_session_id.as_str(),
        query,
    )?))
}

async fn issue_participant_credential(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Result<Json<RtcParticipantCredential>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.issue_participant_credential(
        &auth,
        rtc_session_id.as_str(),
        request.participant_id.as_str(),
    )?))
}

async fn get_recording_artifact(
    Path(rtc_session_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RtcRecordingArtifact>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(
        state
            .runtime
            .recording_artifact(&auth, rtc_session_id.as_str())?,
    ))
}

async fn map_provider_callback(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RtcCallbackRequest>,
) -> Result<Json<RtcCallbackEvent>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(state.runtime.map_provider_callback(&auth, request)?))
}

async fn get_provider_health(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, RtcError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state
            .runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?,
    ))
}

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), RtcError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(RtcError::payload_too_large(field, max_bytes, payload_len));
    }
    Ok(())
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, RtcError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(RtcError::from),
    }
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), RtcError> {
    if !has_bearer_auth_token(headers) {
        return Err(RtcError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(RtcError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
        });
    }
    Ok(())
}

fn has_bearer_auth_token(headers: &HeaderMap) -> bool {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(());
            }
            None
        })
        .is_some()
}

fn has_access_token_header(headers: &HeaderMap) -> bool {
    headers
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(RTC_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(RTC_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(RTC_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(RTC_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
        .ok()
        .map(|value| parse_truthy_env_flag(Some(value)))
        .unwrap_or(true)
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(RTC_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(RTC_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(RTC_MAX_REQUEST_BODY_BYTES_MAX)
}

fn validate_create_rtc_session_request_payload_size(
    request: &CreateRtcSessionRequest,
) -> Result<(), RtcError> {
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
) -> Result<(), RtcError> {
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
) -> Result<(), RtcError> {
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
) -> Result<(), RtcError> {
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

fn validate_rtc_callback_request_payload_size(
    request: &RtcCallbackRequest,
) -> Result<(), RtcError> {
    validate_payload_size(
        "rtcSessionId",
        request.rtc_session_id.as_str(),
        RTC_MAX_SESSION_ID_BYTES,
    )?;
    validate_payload_size(
        "callbackType",
        request.callback_type.as_str(),
        RTC_MAX_SIGNAL_TYPE_BYTES,
    )?;
    validate_payload_size(
        "payloadJson",
        request.payload_json.as_str(),
        RTC_MAX_SIGNAL_PAYLOAD_BYTES,
    )?;
    Ok(())
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    encode_rtc_key_segments([tenant_id, rtc_session_id])
}

fn encode_rtc_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
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

pub fn rtc_create_request_key(auth: &AppContext, request: &CreateRtcSessionRequest) -> String {
    encode_rtc_key_segments([
        auth.tenant_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
        "create",
        request.rtc_session_id.as_str(),
    ])
}

pub fn rtc_session_action_request_key(
    tenant_id: &str,
    rtc_session_id: &str,
    action: &str,
) -> String {
    encode_rtc_key_segments([tenant_id, action, rtc_session_id])
}

fn ensure_standalone_rtc_create_allowed(request: &CreateRtcSessionRequest) -> Result<(), RtcError> {
    if request.conversation_id.is_none() {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound RTC sessions must be created through an authorizing IM gateway",
    ))
}

fn ensure_standalone_rtc_session_allowed(
    runtime: &RtcRuntime,
    auth: &AppContext,
    rtc_session_id: &str,
) -> Result<(), RtcError> {
    let session = runtime.session(auth, rtc_session_id)?;
    if session.conversation_id.is_none() {
        return Ok(());
    }

    Err(conversation_gateway_required(
        "conversation-bound RTC sessions must be accessed through an authorizing IM gateway",
    ))
}

fn conversation_gateway_required(message: &str) -> RtcError {
    RtcError {
        status: axum::http::StatusCode::FORBIDDEN,
        code: "conversation_gateway_required",
        message: message.into(),
    }
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
    use axum::http::{HeaderMap, HeaderValue};
    use im_platform_contracts::{ProviderPluginDescriptor, RtcSessionHandle};
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn parse_truthy_env_flag_accepts_common_truthy_values() {
        for value in ["1", "true", "TRUE", " yes ", "On"] {
            assert!(parse_truthy_env_flag(Some(value.to_owned())));
        }
        for value in ["0", "false", "off", "no", "", "  "] {
            assert!(!parse_truthy_env_flag(Some(value.to_owned())));
        }
        assert!(!parse_truthy_env_flag(None));
    }

    #[test]
    fn dual_token_header_helpers_validate_auth_and_access_headers() {
        let mut headers = HeaderMap::new();
        assert!(!has_bearer_auth_token(&headers));
        assert!(!has_access_token_header(&headers));

        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer auth_token"),
        );
        headers.insert("access-token", HeaderValue::from_static("access_token"));
        assert!(has_bearer_auth_token(&headers));
        assert!(has_access_token_header(&headers));
    }

    fn demo_auth_context() -> AppContext {
        AppContext {
            tenant_id: "t_demo".into(),
            organization_id: None,
            user_id: "u_demo".into(),
            actor_id: "u_demo".into(),
            actor_kind: "user".into(),
            session_id: Some("s_demo".into()),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            device_id: Some("d_demo".into()),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    fn test_rtc_state_record(
        state: RtcSessionState,
        updated_at: &str,
        signals: Vec<RtcSignalEvent>,
    ) -> RtcStateRecord {
        RtcStateRecord {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            session: RtcSession {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                initiator_id: "u_demo".into(),
                initiator_kind: "user".into(),
                provider_plugin_id: Some("webrtc".into()),
                provider_session_id: Some("ps_demo".into()),
                access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
                provider_region: Some("cn-shanghai".into()),
                state,
                signaling_stream_id: Some("st_demo".into()),
                artifact_message_id: None,
                started_at: "2026-05-06T00:00:00.000Z".into(),
                ended_at: None,
            },
            signals,
            updated_at: updated_at.into(),
        }
    }

    fn test_rtc_signal_event(signal_seq: u64) -> RtcSignalEvent {
        RtcSignalEvent {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            signal_seq,
            conversation_id: Some("c_demo".into()),
            rtc_mode: "voice".into(),
            signal_type: format!("rtc.signal.{signal_seq}"),
            schema_ref: Some("webrtc.signal.v1".into()),
            payload: format!("{{\"seq\":{signal_seq}}}"),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: BTreeMap::new(),
            },
            signaling_stream_id: Some("st_demo".into()),
            occurred_at: format!("2026-05-06T00:00:0{signal_seq}.000Z"),
        }
    }

    #[derive(Clone)]
    struct SessionLockProbeRtcProvider {
        lock_was_free_during_create: Arc<AtomicBool>,
        lock_was_free_during_close: Arc<AtomicBool>,
    }

    impl RtcProviderPort for SessionLockProbeRtcProvider {
        fn descriptor(&self) -> ProviderPluginDescriptor {
            ProviderPluginDescriptor::new(
                "rtc-lock-probe",
                ProviderDomain::Rtc,
                "test",
                "RTC Lock Probe",
            )
            .with_default_selected(true)
        }

        fn create_session(
            &self,
            request: ProviderRtcCreateSessionRequest,
        ) -> Result<RtcSessionHandle, ContractError> {
            let key = rtc_scope_key(request.tenant_id.as_str(), request.rtc_session_id.as_str());
            let runtime = ACTIVE_LOCK_PROBE_RUNTIME
                .lock()
                .expect("lock probe runtime pointer should lock")
                .clone()
                .expect("lock probe runtime should be installed before create");
            let sessions_available = runtime
                .sessions
                .try_lock()
                .map(|sessions| {
                    !sessions
                        .has_session(request.tenant_id.as_str(), request.rtc_session_id.as_str())
                })
                .unwrap_or(false);
            let signals_available = runtime
                .signals
                .try_lock()
                .map(|signals| !signals.contains_key(key.as_str()))
                .unwrap_or(false);
            self.lock_was_free_during_create
                .store(sessions_available && signals_available, Ordering::SeqCst);
            Ok(RtcSessionHandle {
                tenant_id: request.tenant_id,
                rtc_session_id: request.rtc_session_id.clone(),
                provider_session_id: format!("probe:{}", request.rtc_session_id),
                access_endpoint: None,
                region: None,
            })
        }

        fn close_session(
            &self,
            tenant_id: &str,
            rtc_session_id: &str,
        ) -> Result<bool, ContractError> {
            let key = rtc_scope_key(tenant_id, rtc_session_id);
            let runtime = ACTIVE_LOCK_PROBE_RUNTIME
                .lock()
                .expect("lock probe runtime pointer should lock")
                .clone()
                .expect("lock probe runtime should be installed before close");
            let sessions_available = runtime
                .sessions
                .try_lock()
                .map(|sessions| sessions.has_session(tenant_id, rtc_session_id))
                .unwrap_or(false);
            let signals_available = runtime
                .signals
                .try_lock()
                .map(|signals| signals.contains_key(key.as_str()))
                .unwrap_or(false);
            self.lock_was_free_during_close
                .store(sessions_available && signals_available, Ordering::SeqCst);
            Ok(true)
        }

        fn issue_participant_credential(
            &self,
            tenant_id: &str,
            rtc_session_id: &str,
            participant_id: &str,
        ) -> Result<RtcParticipantCredential, ContractError> {
            Ok(RtcParticipantCredential {
                tenant_id: tenant_id.into(),
                rtc_session_id: rtc_session_id.into(),
                participant_id: participant_id.into(),
                credential: "probe-credential".into(),
                expires_at: "2026-05-06T00:10:00.000Z".into(),
            })
        }

        fn refresh_participant_credential(
            &self,
            tenant_id: &str,
            rtc_session_id: &str,
            participant_id: &str,
        ) -> Result<RtcParticipantCredential, ContractError> {
            self.issue_participant_credential(tenant_id, rtc_session_id, participant_id)
        }

        fn map_provider_callback(
            &self,
            request: RtcCallbackRequest,
        ) -> Result<RtcCallbackEvent, ContractError> {
            Ok(RtcCallbackEvent {
                rtc_session_id: request.rtc_session_id,
                event_type: request.callback_type,
                participant_id: None,
                payload_json: request.payload_json,
            })
        }

        fn export_recording_artifact(
            &self,
            tenant_id: &str,
            rtc_session_id: &str,
        ) -> Result<Option<RtcRecordingArtifact>, ContractError> {
            Ok(Some(RtcRecordingArtifact::drive_backed_recording(
                tenant_id,
                rtc_session_id,
                "space_rtc_recordings",
                format!("node_{rtc_session_id}"),
                None,
            )))
        }

        fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
            ProviderHealthSnapshot::healthy("rtc-lock-probe", "2026-05-06T00:00:00.000Z")
        }
    }

    static ACTIVE_LOCK_PROBE_RUNTIME: Mutex<Option<Arc<RtcRuntime>>> = Mutex::new(None);

    #[test]
    fn test_session_lookup_recovers_from_poisoned_sessions_lock() {
        let runtime = RtcRuntime::default();
        let auth = demo_auth_context();
        runtime
            .create_session(
                &auth,
                CreateRtcSessionRequest {
                    rtc_session_id: "rtc_poison_session_lookup".into(),
                    conversation_id: None,
                    rtc_mode: "voice".into(),
                },
            )
            .expect("session should be created");

        poison_mutex(&runtime.sessions);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.session(&auth, "rtc_poison_session_lookup")
        }));
        assert!(
            result.is_ok(),
            "session lookup should not panic when sessions mutex is poisoned"
        );
        let session_result = result.expect("panic status should be captured");
        assert!(
            session_result.is_ok(),
            "session lookup should still succeed after lock poison recovery"
        );
    }

    #[test]
    fn test_end_session_calls_provider_without_holding_runtime_locks() {
        let lock_was_free_during_create = Arc::new(AtomicBool::new(false));
        let lock_was_free_during_close = Arc::new(AtomicBool::new(false));
        let provider = Arc::new(SessionLockProbeRtcProvider {
            lock_was_free_during_create: lock_was_free_during_create.clone(),
            lock_was_free_during_close: lock_was_free_during_close.clone(),
        });
        let descriptor = provider.descriptor();
        let runtime = Arc::new(RtcRuntime::with_store_and_provider_registry(
            Arc::new(RuntimeMemoryRtcStateStore::default()),
            Arc::new(StaticProviderRegistry::new([descriptor.clone()])),
            [(
                descriptor.plugin_id.clone(),
                provider as Arc<dyn RtcProviderPort>,
            )],
        ));
        *ACTIVE_LOCK_PROBE_RUNTIME
            .lock()
            .expect("lock probe runtime pointer should lock") = Some(runtime.clone());

        let auth = demo_auth_context();
        runtime
            .create_session(
                &auth,
                CreateRtcSessionRequest {
                    rtc_session_id: "rtc_lock_probe".into(),
                    conversation_id: None,
                    rtc_mode: "voice".into(),
                },
            )
            .expect("session should be created before end");
        assert!(
            lock_was_free_during_create.load(Ordering::SeqCst),
            "provider create_session must not run while rtc runtime session/signal locks are held"
        );
        runtime
            .end_session(
                &auth,
                "rtc_lock_probe",
                UpdateRtcSessionRequest {
                    artifact_message_id: Some("msg_lock_probe_end".into()),
                },
            )
            .expect("end should succeed");

        *ACTIVE_LOCK_PROBE_RUNTIME
            .lock()
            .expect("lock probe runtime pointer should lock") = None;

        assert!(
            lock_was_free_during_close.load(Ordering::SeqCst),
            "provider close_session must not run while rtc runtime session/signal locks are held"
        );
    }

    #[test]
    fn test_post_signal_recovers_from_poisoned_signals_lock() {
        let runtime = RtcRuntime::default();
        let auth = demo_auth_context();
        runtime
            .create_session(
                &auth,
                CreateRtcSessionRequest {
                    rtc_session_id: "rtc_poison_post_signal".into(),
                    conversation_id: None,
                    rtc_mode: "voice".into(),
                },
            )
            .expect("session should be created");

        poison_mutex(&runtime.signals);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.post_signal(
                &auth,
                "rtc_poison_post_signal",
                PostRtcSignalRequest {
                    signal_type: "rtc.offer".into(),
                    schema_ref: Some("webrtc.offer.v1".into()),
                    payload: "{}".into(),
                    signaling_stream_id: Some("stream_poison".into()),
                },
            )
        }));
        assert!(
            result.is_ok(),
            "post signal should not panic when signals mutex is poisoned"
        );
        let post_result = result.expect("panic status should be captured");
        assert!(
            post_result.is_ok(),
            "post signal should still succeed after lock poison recovery"
        );
    }

    #[test]
    fn test_runtime_memory_state_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryRtcStateStore::default();
        poison_mutex(&store.states);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_state("t_demo", "rtc_poison_store")
        }));
        assert!(
            result.is_ok(),
            "state-store load should not panic when internal mutex is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(
            load_result.is_ok(),
            "state-store load should recover from poisoned lock"
        );
    }

    #[test]
    fn test_persist_state_returns_error_when_session_missing() {
        let runtime = RtcRuntime::default();

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.persist_state("t_demo", "rtc_missing_for_persist")
        }));
        assert!(
            result.is_ok(),
            "persist_state should not panic when session is missing"
        );
        let persist_result = result.expect("panic status should be captured");
        let error = persist_result.expect_err("missing session should return structured error");
        assert_eq!(error.code(), "rtc_session_not_found");
    }

    #[test]
    fn test_runtime_rtc_state_store_merges_signals_and_rejects_stale_session_regression() {
        let store = RuntimeMemoryRtcStateStore::default();
        store
            .save_state(test_rtc_state_record(
                RtcSessionState::Accepted,
                "2026-05-06T00:00:02.000Z",
                vec![test_rtc_signal_event(1), test_rtc_signal_event(2)],
            ))
            .expect("new rtc state save should succeed");
        store
            .save_state(test_rtc_state_record(
                RtcSessionState::Started,
                "2026-05-06T00:00:01.000Z",
                vec![test_rtc_signal_event(1)],
            ))
            .expect("stale rtc state save should not fail the caller");

        let state = store
            .load_state("t_demo", "rtc_demo")
            .expect("rtc state load should succeed")
            .expect("rtc state should be present");
        assert_eq!(state.session.state, RtcSessionState::Accepted);
        assert_eq!(state.updated_at, "2026-05-06T00:00:02.000Z");
        assert_eq!(
            state
                .signals
                .iter()
                .map(|signal| signal.signal_seq)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }
}
