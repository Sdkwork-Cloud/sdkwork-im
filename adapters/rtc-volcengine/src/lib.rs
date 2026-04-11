use std::collections::BTreeMap;

use craw_chat_contract_core::ContractError;
use im_platform_contracts::{
    ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor, RtcCallbackEvent,
    RtcCallbackRequest, RtcCreateSessionRequest, RtcParticipantCredential, RtcProviderPort,
    RtcRecordingArtifact, RtcSessionHandle,
};
use im_time::utc_now_rfc3339_millis;

pub const VOLCENGINE_RTC_PLUGIN_ID: &str = "rtc-volcengine";
const DEFAULT_ACCESS_ENDPOINT: &str = "wss://rtc.volcengine.local/session";
const DEFAULT_REGION: &str = "cn-beijing";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolcengineRtcProviderConfig {
    pub access_endpoint: String,
    pub region: String,
}

impl Default for VolcengineRtcProviderConfig {
    fn default() -> Self {
        Self {
            access_endpoint: std::env::var("CRAW_CHAT_RTC_VOLCENGINE_ACCESS_ENDPOINT")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_ACCESS_ENDPOINT.into()),
            region: std::env::var("CRAW_CHAT_RTC_VOLCENGINE_REGION")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_REGION.into()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct VolcengineRtcProvider {
    config: VolcengineRtcProviderConfig,
}

impl VolcengineRtcProvider {
    pub fn new(config: VolcengineRtcProviderConfig) -> Self {
        Self { config }
    }

    fn descriptor_with_defaults(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            VOLCENGINE_RTC_PLUGIN_ID,
            ProviderDomain::Rtc,
            "volcengine",
            "Volcengine RTC",
        )
        .with_default_selected(true)
        .with_required_capabilities(["session", "credential", "callback", "health"])
        .with_optional_capabilities(["recording", "artifact"])
    }
}

impl RtcProviderPort for VolcengineRtcProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor_with_defaults()
    }

    fn create_session(
        &self,
        request: RtcCreateSessionRequest,
    ) -> Result<RtcSessionHandle, ContractError> {
        Ok(RtcSessionHandle {
            tenant_id: request.tenant_id,
            rtc_session_id: request.rtc_session_id.clone(),
            provider_session_id: format!("volcengine:{}", request.rtc_session_id),
            access_endpoint: Some(self.config.access_endpoint.clone()),
            region: Some(self.config.region.clone()),
        })
    }

    fn close_session(
        &self,
        _tenant_id: &str,
        _rtc_session_id: &str,
    ) -> Result<bool, ContractError> {
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
            credential: format!("volcengine-token:{tenant_id}:{rtc_session_id}:{participant_id}"),
            expires_at: utc_now_rfc3339_millis(),
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
        let object_key = format!("recordings/{tenant_id}/{rtc_session_id}.mp4");
        Ok(Some(RtcRecordingArtifact {
            tenant_id: tenant_id.into(),
            rtc_session_id: rtc_session_id.into(),
            bucket: "rtc-artifacts".into(),
            object_key,
            storage_provider: None,
            playback_url: None,
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), "volcengine".into());
        details.insert("accessEndpoint".into(), self.config.access_endpoint.clone());
        details.insert("region".into(), self.config.region.clone());
        ProviderHealthSnapshot {
            plugin_id: VOLCENGINE_RTC_PLUGIN_ID.into(),
            status: "healthy".into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }
}
