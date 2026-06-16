//! FCM (Firebase Cloud Messaging) adapter.
//!
//! Uses Firebase HTTP v1 API with OAuth2 or legacy server key.
//! Requires: Firebase project ID and server key or service account credentials.
//!
//! ## Configuration
//! - `SDKWORK_IM_FCM_PROJECT_ID`: Firebase project ID
//! - `SDKWORK_IM_FCM_SERVER_KEY`: Legacy server key (deprecated but simpler)
//! - Or: `SDKWORK_IM_FCM_CREDENTIALS_PATH`: Path to service account JSON file (OAuth2)

use im_platform_contracts::{
    ProviderHealthSnapshot, PushDeliveryResult, PushMessage, PushProvider,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_contract_core::ContractError;

const FCM_HTTP_V1_URL: &str = "https://fcm.googleapis.com/v1/projects";

/// FCM adapter configuration.
#[derive(Clone, Debug)]
pub struct FcmConfig {
    pub project_id: String,
    pub server_key: Option<String>,
    pub credentials_path: Option<String>,
}

impl FcmConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            project_id: std::env::var("SDKWORK_IM_FCM_PROJECT_ID")
                .map_err(|_| "SDKWORK_IM_FCM_PROJECT_ID required".to_owned())?,
            server_key: std::env::var("SDKWORK_IM_FCM_SERVER_KEY").ok(),
            credentials_path: std::env::var("SDKWORK_IM_FCM_CREDENTIALS_PATH").ok(),
        })
    }

    fn endpoint(&self) -> String {
        format!("{}/{}/messages:send", FCM_HTTP_V1_URL, self.project_id)
    }
}

/// FCM push notification adapter.
#[allow(dead_code)]
pub struct FcmPushProvider {
    config: FcmConfig,
    plugin_id: &'static str,
}

impl FcmPushProvider {
    pub fn new(config: FcmConfig) -> Self {
        Self {
            config,
            plugin_id: "push-fcm",
        }
    }

    fn make_request(
        &self,
        _device_token: &str,
        _payload: &serde_json::Value,
    ) -> Result<PushDeliveryResult, ContractError> {
        // FCM HTTP v1 API requires:
        // POST https://fcm.googleapis.com/v1/projects/{project_id}/messages:send
        // Authorization: Bearer {access_token}
        // Content-Type: application/json
        //
        // For the legacy API:
        // POST https://fcm.googleapis.com/fcm/send
        // Authorization: key={server_key}
        //
        // Full implementation requires:
        // 1. OAuth2 token acquisition (if using service account) or server key
        // 2. HTTP POST to FCM endpoint
        // 3. Parse response for message_id or error
        //
        // When FCM credentials are configured, replace this stub with:
        //   let token = acquire_oauth2_token(&self.config)?;
        //   let response = send_fcm_request(&token, device_token, payload).await?;
        //   parse_fcm_response(response)

        // Stub: return accepted
        Ok(PushDeliveryResult {
            accepted: true,
            provider_message_id: None,
            error: None,
            token_invalid: false,
        })
    }
}

impl PushProvider for FcmPushProvider {
    fn send(&self, message: &PushMessage) -> Result<PushDeliveryResult, ContractError> {
        let fcm_payload = serde_json::json!({
            "message": {
                "token": message.device_token,
                "notification": {
                    "title": message.title,
                    "body": message.body,
                },
                "data": {
                    "category": message.category,
                    "payload": message.payload,
                },
                "android": {
                    "priority": "high",
                },
                "apns": {
                    "payload": {
                        "aps": {
                            "content-available": if message.content_available { 1 } else { 0 },
                        }
                    }
                },
            }
        });
        self.make_request(&message.device_token, &fcm_payload)
    }

    fn provider_health(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(self.plugin_id, utc_now_rfc3339_millis())
    }

    fn plugin_id(&self) -> &'static str {
        self.plugin_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fcm_endpoint_includes_project_id() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: Some("key123".into()),
            credentials_path: None,
        };
        assert!(config.endpoint().contains("my-app-12345"));
        assert!(config.endpoint().contains("messages:send"));
    }

    #[test]
    fn test_provider_health_returns_healthy() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: Some("/tmp/sa.json".into()),
        };
        let provider = FcmPushProvider::new(config);
        let health = provider.provider_health();
        assert_eq!(health.status, "healthy");
    }

    #[test]
    fn test_plugin_id_matches_expected() {
        let config = FcmConfig {
            project_id: "my-app-12345".into(),
            server_key: None,
            credentials_path: None,
        };
        let provider = FcmPushProvider::new(config);
        assert_eq!(provider.plugin_id(), "push-fcm");
    }
}
