use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{Path, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_contract_core::ContractError;
use craw_chat_contract_rtc::{RtcStateRecord, RtcStateStore};
use im_adapter_object_storage_s3::{
    ALIYUN_OBJECT_STORAGE_PLUGIN_ID, AWS_OBJECT_STORAGE_PLUGIN_ID, GOOGLE_OBJECT_STORAGE_PLUGIN_ID,
    MICROSOFT_OBJECT_STORAGE_PLUGIN_ID, S3CompatibleObjectStorageProvider,
    TENCENT_OBJECT_STORAGE_PLUGIN_ID, VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
};
use im_adapter_rtc_aliyun::{ALIYUN_RTC_PLUGIN_ID, AliyunRtcProvider};
use im_adapter_rtc_tencent::{TENCENT_RTC_PLUGIN_ID, TencentRtcProvider};
use im_adapter_rtc_volcengine::{VOLCENGINE_RTC_PLUGIN_ID, VolcengineRtcProvider};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
use im_domain_core::message::Sender;
use im_domain_core::rtc::{RtcSession, RtcSessionState, RtcSignalEvent};
use im_platform_contracts::{
    EffectiveProviderBinding, ObjectStorageDownloadUrlRequest, ObjectStorageProvider,
    ProviderDomain, ProviderHealthSnapshot, ProviderRegistry, RtcCallbackEvent, RtcCallbackRequest,
    RtcCreateSessionRequest as ProviderRtcCreateSessionRequest, RtcParticipantCredential,
    RtcProviderPort, RtcRecordingArtifact, StaticProviderRegistry,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

const DEFAULT_RTC_RECORDING_PLAYBACK_URL_TTL_SECONDS: u32 = 3600;
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

fn lock_rtc_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warning: recovering poisoned mutex in rtc-signaling-service: {label}");
            poisoned.into_inner()
        }
    }
}

#[derive(Clone)]
struct AppState {
    runtime: Arc<RtcRuntime>,
}

pub struct RtcRuntime {
    sessions: Mutex<HashMap<String, RtcSession>>,
    signals: Mutex<HashMap<String, Vec<RtcSignalEvent>>>,
    state_store: Arc<dyn RtcStateStore>,
    provider_registry: Arc<dyn ProviderRegistry>,
    rtc_providers: HashMap<String, Arc<dyn RtcProviderPort>>,
    object_storage_providers: HashMap<String, Arc<dyn ObjectStorageProvider>>,
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
        let provider_registry = Arc::new(
            StaticProviderRegistry::platform_default().with_deployment_profile(
                ProviderDomain::ObjectStorage,
                VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID,
            ),
        );
        Self::with_store_and_provider_registry_and_object_storage_providers(
            state_store,
            provider_registry,
            [
                (VOLCENGINE_RTC_PLUGIN_ID.into(), volcengine),
                (ALIYUN_RTC_PLUGIN_ID.into(), aliyun),
                (TENCENT_RTC_PLUGIN_ID.into(), tencent),
            ],
            built_in_object_storage_providers(),
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
        Self::with_store_and_provider_registry_and_object_storage_providers(
            state_store,
            provider_registry,
            rtc_providers,
            built_in_object_storage_providers(),
        )
    }

    pub fn with_store_and_provider_registry_and_object_storage_providers<I, J>(
        state_store: Arc<dyn RtcStateStore>,
        provider_registry: Arc<dyn ProviderRegistry>,
        rtc_providers: I,
        object_storage_providers: J,
    ) -> Self
    where
        I: IntoIterator<Item = (String, Arc<dyn RtcProviderPort>)>,
        J: IntoIterator<Item = (String, Arc<dyn ObjectStorageProvider>)>,
    {
        Self {
            sessions: Mutex::new(HashMap::new()),
            signals: Mutex::new(HashMap::new()),
            state_store,
            provider_registry,
            rtc_providers: rtc_providers.into_iter().collect(),
            object_storage_providers: object_storage_providers.into_iter().collect(),
        }
    }

    fn ensure_session_state(&self, tenant_id: &str, rtc_session_id: &str) -> Result<(), RtcError> {
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        let needs_restore =
            !lock_rtc_mutex(&self.sessions, "sessions").contains_key(scope_key.as_str());
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
            lock_rtc_mutex(&self.sessions, "sessions").insert(scope_key.clone(), record.session);
            lock_rtc_mutex(&self.signals, "signals").insert(scope_key, record.signals);
        }

        Ok(())
    }

    pub fn session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
    ) -> Result<RtcSession, RtcError> {
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        lock_rtc_mutex(&self.sessions, "sessions")
            .get(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .cloned()
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })
    }

    pub fn create_session(
        &self,
        auth: &AuthContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self.create_session_with_outcome(auth, request)?.session)
    }

    pub fn create_session_with_outcome(
        &self,
        auth: &AuthContext,
        request: CreateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_create_rtc_session_request_payload_size(&request)?;
        let scope_key = rtc_scope_key(auth.tenant_id.as_str(), request.rtc_session_id.as_str());
        self.ensure_session_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        if let Some(existing) = sessions.get(scope_key.as_str()).cloned() {
            if rtc_session_matches_create_request(&existing, auth, &request) {
                return Ok(RtcSessionMutationOutcome {
                    session: existing,
                    applied: false,
                });
            }

            return Err(RtcError::conflict(request.rtc_session_id.as_str()));
        }

        let provider_plugin_id = self.selected_provider_plugin_id(auth.tenant_id.as_str(), None)?;
        let provider = self.rtc_provider(provider_plugin_id.as_str())?;
        let provider_session = provider
            .create_session(ProviderRtcCreateSessionRequest {
                tenant_id: auth.tenant_id.clone(),
                rtc_session_id: request.rtc_session_id.clone(),
                conversation_id: request.conversation_id.clone(),
                rtc_mode: request.rtc_mode.clone(),
                initiator_id: auth.actor_id.clone(),
            })
            .map_err(RtcError::rtc_provider)?;
        let started_at = utc_now_rfc3339_millis();
        let session = RtcSession {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: request.rtc_session_id.clone(),
            conversation_id: request.conversation_id,
            rtc_mode: request.rtc_mode,
            initiator_id: auth.actor_id.clone(),
            initiator_kind: Some(auth.actor_kind.clone()),
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

        sessions.insert(scope_key, session.clone());
        drop(sessions);
        lock_rtc_mutex(&self.signals, "signals")
            .entry(rtc_scope_key(
                auth.tenant_id.as_str(),
                request.rtc_session_id.as_str(),
            ))
            .or_default();
        self.persist_state(auth.tenant_id.as_str(), request.rtc_session_id.as_str())?;

        Ok(RtcSessionMutationOutcome {
            session,
            applied: true,
        })
    }

    pub fn issue_participant_credential(
        &self,
        auth: &AuthContext,
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
        auth: &AuthContext,
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
        auth: &AuthContext,
        rtc_session_id: &str,
    ) -> Result<RtcRecordingArtifact, RtcError> {
        let session = self.session(auth, rtc_session_id)?;
        let provider = self.rtc_provider_for_session(auth.tenant_id.as_str(), &session)?;
        let mut artifact = provider
            .export_recording_artifact(auth.tenant_id.as_str(), rtc_session_id)
            .map_err(RtcError::rtc_provider)?
            .ok_or_else(|| RtcError::recording_artifact_not_found(rtc_session_id))?;
        let object_storage_plugin_id = self.selected_object_storage_provider_plugin_id(
            auth.tenant_id.as_str(),
            artifact.storage_provider.as_deref(),
        )?;
        let object_storage_provider =
            self.object_storage_provider(object_storage_plugin_id.as_str())?;
        let playback_url = object_storage_provider
            .signed_download_url(ObjectStorageDownloadUrlRequest {
                bucket: artifact.bucket.clone(),
                object_key: artifact.object_key.clone(),
                expires_in_seconds: DEFAULT_RTC_RECORDING_PLAYBACK_URL_TTL_SECONDS,
            })
            .map_err(RtcError::object_storage_provider)?;
        artifact.storage_provider = Some(object_storage_plugin_id);
        artifact.playback_url = Some(playback_url);
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

    pub fn invite_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .invite_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn invite_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: InviteRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_invite_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

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
            let outcome = RtcSessionMutationOutcome {
                session: session.clone(),
                applied: true,
            };
            drop(sessions);
            self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
            return Ok(outcome);
        }

        Err(RtcError::state_conflict(
            rtc_session_id,
            "invite",
            &session.state,
        ))
    }

    pub fn accept_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .accept_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn accept_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started => {
                session.state = RtcSessionState::Accepted;
                session.artifact_message_id = request.artifact_message_id;
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Accepted if rtc_session_matches_update_request(session, &request) => {
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
        }
    }

    pub fn reject_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .reject_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn reject_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started => {
                let provider_plugin_id = self.selected_provider_plugin_id(
                    auth.tenant_id.as_str(),
                    session.provider_plugin_id.as_deref(),
                )?;
                self.rtc_provider(provider_plugin_id.as_str())?
                    .close_session(auth.tenant_id.as_str(), rtc_session_id)
                    .map_err(RtcError::rtc_provider)?;
                session.state = RtcSessionState::Rejected;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Rejected if rtc_session_matches_update_request(session, &request) => {
                Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                })
            }
            _ => Err(RtcError::state_conflict(
                rtc_session_id,
                "reject",
                &session.state,
            )),
        }
    }

    pub fn end_session(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSession, RtcError> {
        Ok(self
            .end_session_with_outcome(auth, rtc_session_id, request)?
            .session)
    }

    pub fn end_session_with_outcome(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: UpdateRtcSessionRequest,
    ) -> Result<RtcSessionMutationOutcome, RtcError> {
        validate_update_rtc_session_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

        match session.state {
            RtcSessionState::Started | RtcSessionState::Accepted => {
                let provider_plugin_id = self.selected_provider_plugin_id(
                    auth.tenant_id.as_str(),
                    session.provider_plugin_id.as_deref(),
                )?;
                self.rtc_provider(provider_plugin_id.as_str())?
                    .close_session(auth.tenant_id.as_str(), rtc_session_id)
                    .map_err(RtcError::rtc_provider)?;
                session.state = RtcSessionState::Ended;
                session.artifact_message_id = request.artifact_message_id;
                session.ended_at = Some(utc_now_rfc3339_millis());
                let outcome = RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: true,
                };
                drop(sessions);
                self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;
                Ok(outcome)
            }
            RtcSessionState::Ended if rtc_session_matches_update_request(session, &request) => {
                Ok(RtcSessionMutationOutcome {
                    session: session.clone(),
                    applied: false,
                })
            }
            _ => Err(RtcError::state_conflict(
                rtc_session_id,
                "end",
                &session.state,
            )),
        }
    }

    pub fn post_signal(
        &self,
        auth: &AuthContext,
        rtc_session_id: &str,
        request: PostRtcSignalRequest,
    ) -> Result<RtcSignalEvent, RtcError> {
        validate_post_rtc_signal_request_payload_size(&request)?;
        self.ensure_session_state(auth.tenant_id.as_str(), rtc_session_id)?;
        let mut sessions = lock_rtc_mutex(&self.sessions, "sessions");
        let session = sessions
            .get_mut(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id).as_str())
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;

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

        let occurred_at = utc_now_rfc3339_millis();
        let signal = RtcSignalEvent {
            tenant_id: auth.tenant_id.clone(),
            rtc_session_id: session.rtc_session_id.clone(),
            conversation_id: session.conversation_id.clone(),
            rtc_mode: session.rtc_mode.clone(),
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
            signaling_stream_id: session.signaling_stream_id.clone(),
            occurred_at,
        };

        drop(sessions);

        lock_rtc_mutex(&self.signals, "signals")
            .entry(rtc_scope_key(auth.tenant_id.as_str(), rtc_session_id))
            .or_default()
            .push(signal.clone());
        self.persist_state(auth.tenant_id.as_str(), rtc_session_id)?;

        Ok(signal)
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
            .get(scope_key.as_str())
            .cloned()
            .ok_or_else(|| RtcError {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "rtc_session_not_found",
                message: format!("rtc session not found: {rtc_session_id}"),
            })?;
        let signals = lock_rtc_mutex(&self.signals, "signals")
            .get(scope_key.as_str())
            .cloned()
            .unwrap_or_default();

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
        let scope_key = rtc_scope_key(tenant_id, rtc_session_id);
        if let Some(session) = lock_rtc_mutex(&self.sessions, "sessions")
            .get(scope_key.as_str())
            .cloned()
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

    fn selected_object_storage_provider_plugin_id(
        &self,
        tenant_id: &str,
        frozen_plugin_id: Option<&str>,
    ) -> Result<String, RtcError> {
        if let Some(plugin_id) = frozen_plugin_id.filter(|value| !value.trim().is_empty()) {
            return Ok(plugin_id.to_string());
        }

        let binding = self
            .provider_registry
            .effective_binding(ProviderDomain::ObjectStorage, Some(tenant_id))
            .ok_or_else(|| {
                RtcError::recording_storage_binding_missing(
                    "object storage provider binding is missing for the current tenant",
                )
            })?;
        binding
            .selected_plugin_id
            .or(binding.default_plugin_id)
            .ok_or_else(|| {
                RtcError::recording_storage_binding_missing(
                    "object storage provider selection is missing for the current tenant",
                )
            })
    }

    fn object_storage_provider(
        &self,
        plugin_id: &str,
    ) -> Result<Arc<dyn ObjectStorageProvider>, RtcError> {
        self.object_storage_providers
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| {
                RtcError::recording_storage_binding_missing(format!(
                    "object storage provider is not installed in runtime: {plugin_id}"
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

    fn object_storage_provider(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "rtc_artifact_storage_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "rtc_artifact_storage_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "rtc_artifact_storage_unsupported",
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

    fn recording_storage_binding_missing(message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "rtc_artifact_storage_binding_missing",
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
        lock_rtc_mutex(&self.states, "state_store").insert(
            rtc_scope_key(record.tenant_id.as_str(), record.rtc_session_id.as_str()),
            record,
        );
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

impl From<AuthContextError> for RtcError {
    fn from(value: AuthContextError) -> Self {
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
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<RtcRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/rtc/sessions", post(create_session))
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/invite",
            post(invite_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/accept",
            post(accept_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/reject",
            post(reject_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/end",
            post(end_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/signals",
            post(post_signal),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/credentials",
            post(issue_participant_credential),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/artifacts/recording",
            get(get_recording_artifact),
        )
        .route(
            "/api/v1/rtc/provider-callbacks",
            post(map_provider_callback),
        )
        .route("/api/v1/rtc/provider-health", get(get_provider_health))
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => RtcError::from(error).into_response(),
        },
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

async fn create_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<RtcSignalEvent>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.post_signal(
        &auth,
        rtc_session_id.as_str(),
        request,
    )?))
}

async fn issue_participant_credential(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Result<Json<RtcParticipantCredential>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(state.runtime.issue_participant_credential(
        &auth,
        rtc_session_id.as_str(),
        request.participant_id.as_str(),
    )?))
}

async fn get_recording_artifact(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RtcRecordingArtifact>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_standalone_rtc_session_allowed(&state.runtime, &auth, rtc_session_id.as_str())?;
    Ok(Json(
        state
            .runtime
            .recording_artifact(&auth, rtc_session_id.as_str())?,
    ))
}

async fn map_provider_callback(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RtcCallbackRequest>,
) -> Result<Json<RtcCallbackEvent>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.map_provider_callback(&auth, request)?))
}

async fn get_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, RtcError> {
    let auth = resolve_auth_context(&headers)?;
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

fn built_in_object_storage_providers() -> Vec<(String, Arc<dyn ObjectStorageProvider>)> {
    vec![
        (
            ALIYUN_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::aliyun_default()),
        ),
        (
            TENCENT_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::tencent_default()),
        ),
        (
            VOLCENGINE_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::volcengine_default()),
        ),
        (
            AWS_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::aws_default()),
        ),
        (
            GOOGLE_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::google_default()),
        ),
        (
            MICROSOFT_OBJECT_STORAGE_PLUGIN_ID.into(),
            Arc::new(S3CompatibleObjectStorageProvider::microsoft_default()),
        ),
    ]
}

fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    format!("{tenant_id}:{rtc_session_id}")
}

pub fn rtc_create_request_key(auth: &AuthContext, request: &CreateRtcSessionRequest) -> String {
    format!(
        "{}:{}:{}:create:{}",
        auth.tenant_id, auth.actor_kind, auth.actor_id, request.rtc_session_id
    )
}

pub fn rtc_session_action_request_key(
    tenant_id: &str,
    rtc_session_id: &str,
    action: &str,
) -> String {
    format!("{tenant_id}:{action}:{rtc_session_id}")
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
    auth: &AuthContext,
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
    auth: &AuthContext,
    request: &CreateRtcSessionRequest,
) -> bool {
    session.rtc_session_id == request.rtc_session_id.as_str()
        && session.initiator_id == auth.actor_id.as_str()
        && session
            .initiator_kind
            .as_deref()
            .is_none_or(|initiator_kind| initiator_kind == auth.actor_kind.as_str())
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
    use std::panic::{self, AssertUnwindSafe};

    fn demo_auth_context() -> AuthContext {
        AuthContext {
            tenant_id: "t_demo".into(),
            actor_id: "u_demo".into(),
            actor_kind: "user".into(),
            session_id: Some("s_demo".into()),
            device_id: Some("d_demo".into()),
            permissions: Default::default(),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

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
}
