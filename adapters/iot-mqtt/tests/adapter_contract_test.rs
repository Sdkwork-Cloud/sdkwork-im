use im_adapter_iot_mqtt::{
    MQTT_IOT_PLUGIN_ID, MqttIotProtocolAdapter, MqttIotProtocolAdapterConfig,
};
use im_platform_contracts::{
    IotProtocolAdapter, IotProtocolDecodeRequest, IotProtocolEncodeRequest, ProviderDomain,
};
use serde_json::{Value, json};

#[test]
fn test_mqtt_iot_adapter_exposes_expected_contract_shape() {
    let adapter = MqttIotProtocolAdapter::new(MqttIotProtocolAdapterConfig {
        broker_endpoint: "mqtt://broker.demo:1883".into(),
        default_qos: "1".into(),
    });

    let descriptor = adapter.descriptor();
    assert_eq!(descriptor.plugin_id, MQTT_IOT_PLUGIN_ID);
    assert_eq!(descriptor.domain, ProviderDomain::IotProtocol);
    assert_eq!(descriptor.provider_kind, "mqtt");
    assert_eq!(
        descriptor.required_capabilities,
        vec!["uplink", "downlink", "telemetry"]
    );
    assert_eq!(descriptor.optional_capabilities, vec!["command", "qos"]);
    assert_eq!(adapter.protocol_key(), "mqtt");

    let envelope = adapter
        .decode_uplink(IotProtocolDecodeRequest {
            tenant_id: "t_demo".into(),
            device_id: Some("d_sensor".into()),
            channel: "devices/d_sensor/telemetry".into(),
            payload: json!({
                "temperature": 23,
                "humidity": 48
            })
            .to_string(),
        })
        .expect("mqtt uplink should decode");
    assert_eq!(envelope.tenant_id, "t_demo");
    assert_eq!(envelope.device_id, "d_sensor");
    assert_eq!(envelope.channel, "devices/d_sensor/telemetry");
    assert_eq!(
        serde_json::from_str::<Value>(&envelope.payload_json)
            .expect("payload must stay valid json"),
        json!({
            "temperature": 23,
            "humidity": 48
        })
    );
    assert_eq!(envelope.attributes["protocol"], "mqtt");
    assert_eq!(envelope.attributes["topic"], "devices/d_sensor/telemetry");
    assert_eq!(envelope.attributes["qos"], "1");
    assert_eq!(
        envelope.attributes["brokerEndpoint"],
        "mqtt://broker.demo:1883"
    );

    let downlink = adapter
        .encode_downlink(IotProtocolEncodeRequest {
            tenant_id: "t_demo".into(),
            device_id: "d_sensor".into(),
            channel: "devices/d_sensor/command".into(),
            payload_json: json!({
                "setPoint": 26
            })
            .to_string(),
        })
        .expect("mqtt downlink should encode");
    let downlink_json: Value = serde_json::from_str(&downlink).expect("downlink must be json");
    assert_eq!(downlink_json["protocol"], "mqtt");
    assert_eq!(downlink_json["topic"], "devices/d_sensor/command");
    assert_eq!(downlink_json["deviceId"], "d_sensor");
    assert_eq!(downlink_json["qos"], "1");
    assert_eq!(downlink_json["payload"]["setPoint"], 26);

    let health = adapter.provider_health_snapshot();
    assert_eq!(health.plugin_id, MQTT_IOT_PLUGIN_ID);
    assert_eq!(health.status, "healthy");
    assert_eq!(health.details["providerKind"], "mqtt");
    assert_eq!(health.details["brokerEndpoint"], "mqtt://broker.demo:1883");
}

#[test]
fn test_mqtt_iot_adapter_can_extract_device_id_and_qos_from_payload() {
    let adapter = MqttIotProtocolAdapter::new(MqttIotProtocolAdapterConfig {
        broker_endpoint: "mqtt://broker.demo:1883".into(),
        default_qos: "1".into(),
    });

    let envelope = adapter
        .decode_uplink(IotProtocolDecodeRequest {
            tenant_id: "t_demo".into(),
            device_id: None,
            channel: "devices/d_sensor/telemetry".into(),
            payload: json!({
                "deviceId": "d_sensor",
                "qos": "0",
                "payload": {
                    "battery": 88
                }
            })
            .to_string(),
        })
        .expect("mqtt uplink should extract device id from payload");

    assert_eq!(envelope.device_id, "d_sensor");
    assert_eq!(envelope.attributes["qos"], "0");
    assert_eq!(
        serde_json::from_str::<Value>(&envelope.payload_json)
            .expect("payload must stay valid json"),
        json!({
            "battery": 88
        })
    );
}
