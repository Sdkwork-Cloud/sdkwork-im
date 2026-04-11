use std::collections::BTreeMap;

use craw_chat_contract_core::ContractError;
use im_platform_contracts::{
    IotProtocolAdapter, IotProtocolDecodeRequest, IotProtocolEncodeRequest, IotProtocolEnvelope,
    ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
};
use im_time::utc_now_rfc3339_millis;
use serde_json::{Value, json};

pub const MQTT_IOT_PLUGIN_ID: &str = "iot-mqtt";
const DEFAULT_BROKER_ENDPOINT: &str = "mqtt://broker.local:1883";
const DEFAULT_QOS: &str = "1";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MqttIotProtocolAdapterConfig {
    pub broker_endpoint: String,
    pub default_qos: String,
}

impl Default for MqttIotProtocolAdapterConfig {
    fn default() -> Self {
        Self {
            broker_endpoint: std::env::var("CRAW_CHAT_IOT_MQTT_BROKER_ENDPOINT")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_BROKER_ENDPOINT.into()),
            default_qos: std::env::var("CRAW_CHAT_IOT_MQTT_DEFAULT_QOS")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_QOS.into()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MqttIotProtocolAdapter {
    config: MqttIotProtocolAdapterConfig,
}

impl MqttIotProtocolAdapter {
    pub fn new(config: MqttIotProtocolAdapterConfig) -> Self {
        Self { config }
    }

    fn descriptor_with_defaults(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            MQTT_IOT_PLUGIN_ID,
            ProviderDomain::IotProtocol,
            "mqtt",
            "MQTT",
        )
        .with_default_selected(true)
        .with_required_capabilities(["uplink", "downlink", "telemetry"])
        .with_optional_capabilities(["command", "qos"])
    }

    fn parse_json_payload(&self, payload: &str) -> Result<Value, ContractError> {
        serde_json::from_str(payload).map_err(|error| {
            ContractError::UnsupportedCapability(format!(
                "mqtt payload must be valid json: {error}"
            ))
        })
    }

    fn resolve_device_id(
        &self,
        explicit_device_id: Option<&str>,
        payload: &Value,
    ) -> Result<String, ContractError> {
        if let Some(device_id) = explicit_device_id.filter(|value| !value.trim().is_empty()) {
            return Ok(device_id.to_owned());
        }

        payload
            .get("deviceId")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(str::to_owned)
            .ok_or_else(|| {
                ContractError::Conflict(
                    "mqtt uplink requires device id in request.device_id or payload.deviceId"
                        .into(),
                )
            })
    }

    fn resolve_qos(&self, payload: &Value) -> String {
        payload
            .get("qos")
            .and_then(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .or_else(|| value.as_u64().map(|number| number.to_string()))
            })
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.config.default_qos.clone())
    }

    fn normalize_payload_value(&self, payload: Value) -> Value {
        if let Some(normalized) = payload.get("payload") {
            return normalized.clone();
        }

        if let Some(mut object) = payload.as_object().cloned() {
            object.remove("deviceId");
            object.remove("topic");
            object.remove("qos");
            return Value::Object(object);
        }

        payload
    }
}

impl IotProtocolAdapter for MqttIotProtocolAdapter {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor_with_defaults()
    }

    fn protocol_key(&self) -> &'static str {
        "mqtt"
    }

    fn decode_uplink(
        &self,
        request: IotProtocolDecodeRequest,
    ) -> Result<IotProtocolEnvelope, ContractError> {
        let payload = self.parse_json_payload(request.payload.as_str())?;
        let device_id = self.resolve_device_id(request.device_id.as_deref(), &payload)?;
        let qos = self.resolve_qos(&payload);
        let normalized_payload = self.normalize_payload_value(payload);
        let mut attributes = BTreeMap::new();
        attributes.insert("protocol".into(), "mqtt".into());
        attributes.insert("topic".into(), request.channel.clone());
        attributes.insert("qos".into(), qos);
        attributes.insert("brokerEndpoint".into(), self.config.broker_endpoint.clone());

        Ok(IotProtocolEnvelope {
            tenant_id: request.tenant_id,
            device_id,
            channel: request.channel,
            payload_json: normalized_payload.to_string(),
            attributes,
        })
    }

    fn encode_downlink(&self, request: IotProtocolEncodeRequest) -> Result<String, ContractError> {
        let payload = self.parse_json_payload(request.payload_json.as_str())?;
        Ok(json!({
            "protocol": "mqtt",
            "topic": request.channel,
            "deviceId": request.device_id,
            "qos": self.config.default_qos.clone(),
            "payload": payload,
            "brokerEndpoint": self.config.broker_endpoint.clone(),
        })
        .to_string())
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), "mqtt".into());
        details.insert("brokerEndpoint".into(), self.config.broker_endpoint.clone());
        details.insert("defaultQos".into(), self.config.default_qos.clone());
        details.insert("protocolKey".into(), self.protocol_key().into());
        ProviderHealthSnapshot {
            plugin_id: MQTT_IOT_PLUGIN_ID.into(),
            status: "healthy".into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }
}
