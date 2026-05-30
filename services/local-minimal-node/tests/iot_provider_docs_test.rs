use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn read_repo_file(relative_path: &str) -> String {
    let path = workspace_root().join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing repository file: {}", path.display()))
}

fn read_repo_file_by_fragment(fragment: &str) -> String {
    let workspace_root = workspace_root();
    let mut pending = vec![workspace_root.join("docs")];

    while let Some(dir) = pending.pop() {
        let entries = fs::read_dir(&dir)
            .unwrap_or_else(|_| panic!("failed to read repository directory: {}", dir.display()));
        for entry in entries {
            let entry = entry.expect("directory entry should be readable");
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }

            let relative = path
                .strip_prefix(&workspace_root)
                .expect("repo file should live under workspace root");
            if relative.to_string_lossy().contains(fragment) {
                return fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("missing repository file: {}", path.display()));
            }
        }
    }

    panic!("missing repository file fragment: {fragment}");
}

#[test]
fn test_iot_mqtt_runtime_baseline_docs_freeze_adapter_and_iteration_entry() {
    let workspace_manifest = read_repo_file("Cargo.toml");
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc = read_repo_file("docs/step/08-B-IoT-MQTT协议插件运行时基线-2026-04-08.md");
    let plan_doc =
        read_repo_file("docs/架构/09AE-实施计划-iot-mqtt协议插件运行时基线-2026-04-08.md");
    let architecture_doc =
        read_repo_file("docs/架构/150AE-iot-mqtt-protocol-adapter-baseline设计-2026-04-08.md");
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-iot-mqtt-protocol-adapter-baseline-2026-04-08.md",
    );
    let adapter_readme = read_repo_file("adapters/iot-mqtt/README.md");

    assert!(
        workspace_manifest.contains("\"adapters/iot-mqtt\""),
        "workspace manifest must include adapters/iot-mqtt"
    );

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
    ] {
        for required in [
            "iot-mqtt",
            "IotProtocolAdapter",
            "device.telemetry",
            "device.command",
            "decode_uplink",
            "encode_downlink",
        ] {
            assert!(
                doc.contains(required),
                "IoT MQTT runtime baseline docs must cover {required}"
            );
        }
    }

    for required in [
        "iot-mqtt",
        "MQTT",
        "IotProtocolAdapter",
        "ProviderRegistry",
        "decode_uplink",
        "encode_downlink",
        "provider_health_snapshot",
    ] {
        assert!(
            adapter_readme.contains(required),
            "iot-mqtt adapter README must contain {required}"
        );
    }
}

#[test]
fn test_xiaozhi_external_source_docs_freeze_submodule_path_and_alignment_loop() {
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc =
        read_repo_file("docs/step/08-C-小智源码对齐与external-submodule标准-2026-04-08.md");
    let plan_doc = read_repo_file(
        "docs/架构/09AF-实施计划-xiaozhi源码external-submodule与协议对齐-2026-04-08.md",
    );
    let architecture_doc =
        read_repo_file("docs/架构/150AF-xiaozhi-external-source-alignment设计-2026-04-08.md");
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-xiaozhi-external-source-alignment-2026-04-08.md",
    );
    let external_readme = read_repo_file("external/README.md");

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
        &external_readme,
    ] {
        for required in [
            "https://github.com/78/xiaozhi-esp32.git",
            "external/xiaozhi-esp32",
            "git submodule add",
            "xiaozhi",
            "IotProtocolAdapter",
            "DeviceAccessProvider",
        ] {
            assert!(
                doc.contains(required),
                "xiaozhi source-alignment docs must cover {required}"
            );
        }
    }

    assert!(
        review_doc.contains("git ls-remote")
            && review_doc.contains("unexpected eof")
            && review_doc.contains("未完成实际 submodule 拉取"),
        "xiaozhi review doc must record the current environment block instead of faking a submodule fetch"
    );
}

#[test]
fn test_iot_access_local_runtime_baseline_docs_freeze_device_management_entry() {
    let workspace_manifest = read_repo_file("Cargo.toml");
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc =
        read_repo_file("docs/step/08-D-IoT-DeviceAccessProvider本地运行时基线-2026-04-08.md");
    let plan_doc =
        read_repo_file("docs/架构/09AG-实施计划-iot-access-local设备接入运行时基线-2026-04-08.md");
    let architecture_doc = read_repo_file(
        "docs/架构/150AG-iot-access-local-device-access-provider-baseline设计-2026-04-08.md",
    );
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-iot-access-local-provider-baseline-2026-04-08.md",
    );
    let adapter_readme = read_repo_file("adapters/iot-access-local/README.md");

    assert!(
        workspace_manifest.contains("\"adapters/iot-access-local\""),
        "workspace manifest must include adapters/iot-access-local"
    );

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
    ] {
        for required in [
            "iot-access-local",
            "DeviceAccessProvider",
            "register_device",
            "bind_owner",
            "disable_device",
            "provider_health_snapshot",
        ] {
            assert!(
                doc.contains(required),
                "IoT access local baseline docs must cover {required}"
            );
        }
    }

    for required in [
        "iot-access-local",
        "DeviceAccessProvider",
        "register_device",
        "bind_owner",
        "disable_device",
        "provider_health_snapshot",
        "ProviderRegistry",
        "mqtt",
        "xiaozhi",
    ] {
        assert!(
            adapter_readme.contains(required),
            "iot-access-local adapter README must contain {required}"
        );
    }
}

#[test]
fn test_local_minimal_node_device_access_provider_injection_docs_freeze_runtime_closure() {
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc = read_repo_file(
        "docs/step/08-E-IoT-DeviceAccessProvider接入local-minimal-node-2026-04-08.md",
    );
    let plan_doc = read_repo_file(
        "docs/架构/09AH-实施计划-local-minimal-node设备接入提供商注入-2026-04-08.md",
    );
    let architecture_doc = read_repo_file(
        "docs/架构/150AH-local-minimal-node-device-access-provider-injection设计-2026-04-08.md",
    );
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-local-minimal-node-device-access-provider-injection-2026-04-08.md",
    );

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
    ] {
        for required in [
            "local-minimal-node",
            "DeviceAccessProvider",
            "iot-access-local",
            "build_default_app_with_runtime_dir_and_device_access_provider",
            "register_device",
            "bind_owner",
            "local-minimal-device",
            "session",
        ] {
            assert!(
                doc.contains(required),
                "local-minimal-node device access injection docs must cover {required}"
            );
        }
    }

    assert!(
        step_doc.contains("session-gateway") && step_doc.contains("不闭环"),
        "step 08-E must explicitly keep session-gateway outside the delivered closure"
    );
    assert!(
        architecture_doc.contains("projection")
            && architecture_doc.contains("首次注册")
            && architecture_doc.contains("route / resume preflight"),
        "150AH must freeze the first-registration-only provider call order"
    );
    assert!(
        review_doc.contains("TDD")
            && review_doc.contains("build_default_app_with_runtime_dir_and_device_access_provider")
            && review_doc.contains("/im/v3/api/devices/register"),
        "review doc must capture the failing seam, the delivered seam and the runtime path"
    );
}

#[test]
fn test_session_gateway_device_access_provider_injection_docs_freeze_runtime_closure() {
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc =
        read_repo_file("docs/step/08-F-IoT-DeviceAccessProvider接入session-gateway-2026-04-09.md");
    let plan_doc =
        read_repo_file("docs/架构/09AI-实施计划-session-gateway设备接入提供商注入-2026-04-09.md");
    let architecture_doc = read_repo_file(
        "docs/架构/150AI-session-gateway-device-access-provider-injection设计-2026-04-09.md",
    );
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-session-gateway-device-access-provider-injection-2026-04-09.md",
    );

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
    ] {
        for required in [
            "session-gateway",
            "DeviceAccessProvider",
            "iot-access-local",
            "build_app_with_device_access_provider",
            "/im/v3/api/device/sessions/resume",
            "register_device",
            "bind_owner",
        ] {
            assert!(
                doc.contains(required),
                "session-gateway device access injection docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("DeviceSyncState::has_registered_device")
            && architecture_doc.contains("session-gateway-device")
            && architecture_doc.contains("credential_kind = device_route"),
        "150AI must freeze the first-registration guard and request constants"
    );
    assert!(
        review_doc.contains("TDD")
            && review_doc.contains("build_app_with_device_access_provider")
            && review_doc.contains("/im/v3/api/presence/heartbeat"),
        "review doc must capture the TDD seam and the no-repeat heartbeat verification"
    );
}

#[test]
fn test_iot_access_provider_health_http_surface_docs_freeze_first_external_visibility() {
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_doc =
        read_repo_file("docs/step/08-G-IoT-access-provider-health-http-surface-2026-04-09.md");
    let plan_doc = read_repo_file(
        "docs/架构/09AJ-实施计划-IoT-access-provider-health-http-surface-2026-04-09.md",
    );
    let architecture_doc =
        read_repo_file("docs/架构/150AJ-iot-access-provider-health-http-surface设计-2026-04-09.md");
    let review_doc = read_repo_file(
        "docs/review/continuous-optimization-iot-access-provider-http-surface-2026-04-09.md",
    );

    for doc in [
        &plugin_doc,
        &step_eight,
        &step_doc,
        &plan_doc,
        &architecture_doc,
        &review_doc,
    ] {
        for required in [
            "local-minimal-node",
            "iot-access-local",
            "/backend/v3/api/iot/access/provider_health",
            "provider_health_snapshot",
            "DeviceAccessProvider",
            "mqtt,xiaozhi",
        ] {
            assert!(
                doc.contains(required),
                "IoT access provider HTTP surface docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("不是 protocol surface")
            && architecture_doc.contains("不是完整 access 管理 API"),
        "150AJ must explicitly freeze the external visibility boundary"
    );
    assert!(
        review_doc.contains("404") && review_doc.contains("TDD"),
        "review doc must record the red-green route introduction"
    );
}

#[test]
fn test_iot_protocol_provider_health_http_surface_docs_freeze_first_external_visibility() {
    let step_doc = read_repo_file_by_fragment("08-H-IoT-protocol-provider-health-http-surface");
    let plan_doc = read_repo_file_by_fragment("09AK-");
    let architecture_doc =
        read_repo_file_by_fragment("150AK-iot-protocol-provider-health-http-surface");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-provider-http-surface-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "iot-mqtt",
            "/backend/v3/api/iot/protocol/provider_health",
            "IotProtocolAdapter",
            "provider_health_snapshot",
            "build_default_app_with_iot_protocol_adapter",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol provider HTTP surface docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("decode_uplink / encode_downlink")
            && architecture_doc.contains("HTTP API")
            && architecture_doc.contains("设备管理 API"),
        "150AK must explicitly freeze the external visibility boundary"
    );
    assert!(
        review_doc.contains("404") && review_doc.contains("TDD"),
        "review doc must record the red-green protocol route introduction"
    );
}

#[test]
fn test_iot_protocol_uplink_device_telemetry_mainline_docs_freeze_runtime_consumption() {
    let step_doc = read_repo_file_by_fragment("08-I-IoT-protocol-uplink");
    let plan_doc = read_repo_file_by_fragment("09AL-");
    let architecture_doc =
        read_repo_file_by_fragment("150AL-iot-protocol-uplink-device-telemetry-mainline");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-uplink-device-telemetry-mainline-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/uplink",
            "IotProtocolAdapter",
            "decode_uplink",
            "build_default_app_with_iot_protocol_adapter",
            "device.telemetry",
            "cc.device.telemetry.v1",
            "st_device_telemetry_{device_id}",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol uplink mainline docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("request.device_id.or_else(|| auth.device_id.clone())")
            && architecture_doc.contains("encode_downlink")
            && architecture_doc.contains("device.command"),
        "150AL must freeze the auth device fallback and the non-goals"
    );
    assert!(
        review_doc.contains("404") && review_doc.contains("409") && review_doc.contains("TDD"),
        "review doc must capture the red-green path and the intermediate auth/device seam fix"
    );
}

#[test]
fn test_iot_protocol_downlink_device_command_mainline_docs_freeze_runtime_consumption() {
    let step_doc = read_repo_file_by_fragment("08-J-IoT-protocol-downlink");
    let plan_doc = read_repo_file_by_fragment("09AM-");
    let architecture_doc =
        read_repo_file_by_fragment("150AM-iot-protocol-downlink-device-command-mainline");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-downlink-device-command-mainline-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/downlink",
            "IotProtocolAdapter",
            "encode_downlink",
            "build_default_app_with_iot_protocol_adapter",
            "device.command",
            "cc.device.command.v1",
            "st_device_command_{device_id}",
            "device.command.send",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol downlink mainline docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("protocolPayload")
            && architecture_doc.contains("ACK / retry / timeout")
            && architecture_doc.contains("不是设备已真实收到命令"),
        "150AM must freeze the response shape and the non-goals"
    );
    assert!(
        review_doc.contains("404") && review_doc.contains("TDD"),
        "review doc must capture the red-green downlink route introduction"
    );
}

#[test]
fn test_wave_c_93_review_refreshes_step08_continuous_optimization_closure() {
    let review_doc = read_repo_file_by_fragment("wave-c-93-持续优化复核-2026-04-09");

    for required in [
        "Wave C / 93",
        "持续通过",
        "Step 08",
        "08-H",
        "08-I",
        "08-J",
        "08-K",
        "08-L",
        "08-M",
        "08-N",
        "/backend/v3/api/iot/protocol/provider_health",
        "/backend/v3/api/iot/protocol/uplink",
        "/backend/v3/api/iot/protocol/downlink",
        "09AK",
        "09AL",
        "09AM",
        "09AN",
        "09AO",
        "09AP",
        "09AQ",
        "150AK",
        "150AL",
        "150AM",
        "150AN",
        "150AO",
        "150AP",
        "150AQ",
    ] {
        assert!(
            review_doc.contains(required),
            "Wave C / 93 refresh review must cover {required}"
        );
    }

    assert!(
        review_doc.contains("Step 09")
            && review_doc.contains("不重新打开 Wave C 门禁")
            && review_doc.contains("local-minimal-node"),
        "Wave C / 93 refresh review must clarify the gate remains passed and why"
    );
}

#[test]
fn test_iot_protocol_uplink_known_device_preflight_docs_freeze_auth_before_decode() {
    let step_doc = read_repo_file_by_fragment("08-K-IoT-protocol-uplink-known-device");
    let plan_doc = read_repo_file_by_fragment("09AN-");
    let architecture_doc =
        read_repo_file_by_fragment("150AN-iot-protocol-uplink-known-device-preflight");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-uplink-known-device-preflight-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/uplink",
            "IotProtocolAdapter",
            "decode_uplink",
            "request.device_id.clone().or_else(|| auth.device_id.clone())",
            "preflight",
            "device_permission_denied",
            "build_default_app_with_iot_protocol_adapter",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol uplink preflight docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("已知 deviceId")
            && architecture_doc.contains("先鉴权")
            && architecture_doc.contains("后 decode")
            && architecture_doc.contains("payload.deviceId"),
        "150AN must freeze the known-device preflight boundary and the remaining decode-first edge case"
    );
    assert!(
        review_doc.contains("403") && review_doc.contains("TDD"),
        "review doc must capture the red-green authorization preflight closure"
    );
}

#[test]
fn test_iot_protocol_uplink_actor_preflight_docs_freeze_auth_before_decode() {
    let step_doc = read_repo_file_by_fragment("08-L-IoT-protocol-uplink-actor-preflight");
    let plan_doc = read_repo_file_by_fragment("09AO-");
    let architecture_doc = read_repo_file_by_fragment("150AO-iot-protocol-uplink-actor-preflight");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-uplink-actor-preflight-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/uplink",
            "IotProtocolAdapter",
            "decode_uplink",
            "ensure_iot_protocol_uplink_actor_preflight",
            "device_permission_denied",
            "device_id_missing",
            "build_default_app_with_iot_protocol_adapter",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol uplink actor preflight docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("auth.actor_kind")
            && architecture_doc.contains("auth.device_id")
            && architecture_doc.contains("payload.deviceId")
            && architecture_doc.contains("decode 后"),
        "150AO must freeze the actor-only preflight boundary and the retained decode-after checks"
    );
    assert!(
        review_doc.contains("403") && review_doc.contains("TDD"),
        "review doc must capture the red-green actor preflight closure"
    );
}

#[test]
fn test_iot_protocol_uplink_request_device_mismatch_docs_freeze_error_before_decode() {
    let step_doc = read_repo_file_by_fragment("08-M-IoT-protocol-uplink-request-device-mismatch");
    let plan_doc = read_repo_file_by_fragment("09AP-");
    let architecture_doc =
        read_repo_file_by_fragment("150AP-iot-protocol-uplink-request-device-mismatch");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-uplink-request-device-mismatch-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/uplink",
            "IotProtocolAdapter",
            "decode_uplink",
            "resolve_requested_device_id",
            "device_id_mismatch",
            "build_default_app_with_iot_protocol_adapter",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol uplink request-device mismatch docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("request.device_id")
            && architecture_doc.contains("auth.device_id")
            && architecture_doc.contains("400")
            && architecture_doc.contains("后 decode"),
        "150AP must freeze request-device mismatch as a pre-decode bad-request boundary"
    );
    assert!(
        review_doc.contains("400") && review_doc.contains("TDD"),
        "review doc must capture the red-green mismatch closure"
    );
}

#[test]
fn test_iot_protocol_uplink_decoded_device_mismatch_docs_freeze_error_after_decode() {
    let step_doc = read_repo_file_by_fragment("08-N-IoT-protocol-uplink-decoded-device-mismatch");
    let plan_doc = read_repo_file_by_fragment("09AQ-");
    let architecture_doc =
        read_repo_file_by_fragment("150AQ-iot-protocol-uplink-decoded-device-mismatch");
    let review_doc = read_repo_file_by_fragment(
        "continuous-optimization-iot-protocol-uplink-decoded-device-mismatch-2026-04-09",
    );

    for doc in [&step_doc, &plan_doc, &architecture_doc, &review_doc] {
        for required in [
            "local-minimal-node",
            "/backend/v3/api/iot/protocol/uplink",
            "IotProtocolAdapter",
            "decode_uplink",
            "ensure_iot_protocol_uplink_decoded_device_matches_preflight",
            "device_id_mismatch",
            "build_default_app_with_iot_protocol_adapter",
        ] {
            assert!(
                doc.contains(required),
                "IoT protocol uplink decoded-device mismatch docs must cover {required}"
            );
        }
    }

    assert!(
        architecture_doc.contains("envelope.device_id")
            && architecture_doc.contains("preflight_device_id")
            && architecture_doc.contains("400")
            && architecture_doc.contains("decode 后"),
        "150AQ must freeze decoded-device mismatch as a post-decode bad-request boundary"
    );
    assert!(
        review_doc.contains("400") && review_doc.contains("TDD"),
        "review doc must capture the red-green decoded mismatch closure"
    );
}
