use im_adapter_iot_access_local::{
    LOCAL_IOT_ACCESS_PLUGIN_ID, LocalDeviceAccessProvider, LocalDeviceAccessProviderConfig,
};
use im_platform_contracts::{
    DeviceAccessOwnerBindingRequest, DeviceAccessProvider, DeviceAccessRegistrationRequest,
    ProviderDomain,
};

#[test]
fn test_local_iot_access_provider_exposes_expected_contract_shape() {
    let provider = LocalDeviceAccessProvider::new(LocalDeviceAccessProviderConfig {
        assigned_protocols: vec!["mqtt".into(), "xiaozhi".into()],
        credential_secret_prefix: "demo-secret".into(),
    });

    let descriptor = provider.descriptor();
    assert_eq!(descriptor.plugin_id, LOCAL_IOT_ACCESS_PLUGIN_ID);
    assert_eq!(descriptor.domain, ProviderDomain::IotAccess);
    assert_eq!(descriptor.provider_kind, "local");
    assert_eq!(
        descriptor.required_capabilities,
        vec!["registry", "credential", "binding", "twin"]
    );
    assert_eq!(
        descriptor.optional_capabilities,
        vec!["session", "owner-binding", "protocol-assignment"]
    );

    let registration = provider
        .register_device(DeviceAccessRegistrationRequest {
            tenant_id: "t_demo".into(),
            device_id: "d_demo".into(),
            product_id: "p_demo".into(),
            credential_kind: "token".into(),
            owner_principal_id: Some("u_demo".into()),
        })
        .expect("register_device should succeed");
    assert_eq!(registration.tenant_id, "t_demo");
    assert_eq!(registration.device_id, "d_demo");
    assert_eq!(registration.product_id, "p_demo");
    assert_eq!(registration.owner_principal_id.as_deref(), Some("u_demo"));
    assert_eq!(registration.assigned_protocols, vec!["mqtt", "xiaozhi"]);
    assert!(
        registration
            .credential_secret
            .as_deref()
            .is_some_and(|value| value.contains("demo-secret:t_demo:d_demo:token"))
    );

    assert!(
        provider
            .bind_owner(DeviceAccessOwnerBindingRequest {
                tenant_id: "t_demo".into(),
                device_id: "d_demo".into(),
                owner_principal_id: "u_demo".into(),
                session_id: Some("s_demo".into()),
            })
            .expect("bind_owner should succeed")
    );
    assert!(
        provider
            .disable_device("t_demo", "d_demo")
            .expect("disable_device should succeed")
    );

    let health = provider.provider_health_snapshot();
    assert_eq!(health.plugin_id, LOCAL_IOT_ACCESS_PLUGIN_ID);
    assert_eq!(health.status, "healthy");
    assert_eq!(health.details["providerKind"], "local");
    assert_eq!(health.details["assignedProtocols"], "mqtt,xiaozhi");
    assert_eq!(health.details["credentialSecretPrefix"], "demo-secret");
}

#[test]
fn test_local_iot_access_provider_can_override_default_protocol_assignment() {
    let provider = LocalDeviceAccessProvider::new(LocalDeviceAccessProviderConfig {
        assigned_protocols: vec!["xiaozhi".into()],
        credential_secret_prefix: "demo-secret".into(),
    });

    let registration = provider
        .register_device(DeviceAccessRegistrationRequest {
            tenant_id: "t_demo".into(),
            device_id: "d_xiaozhi".into(),
            product_id: "p_xiaozhi".into(),
            credential_kind: "token".into(),
            owner_principal_id: None,
        })
        .expect("register_device should respect configured protocols");

    assert_eq!(registration.assigned_protocols, vec!["xiaozhi"]);
}
