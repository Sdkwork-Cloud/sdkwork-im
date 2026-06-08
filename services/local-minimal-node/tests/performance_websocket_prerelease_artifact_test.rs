use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

#[test]
fn test_cp11_6_materializes_websocket_e2e_prerelease_supplement_artifact() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("im-websocket-e2e")
        .join("metrics.json");
    let baseline_path = root
        .join("tools")
        .join("perf")
        .join("step-11-cp11-5-websocket-e2e-baseline.json");
    let artifact_raw = fs::read_to_string(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing CP11-6 websocket E2E pre-release supplement artifact: {}",
            artifact_path.display()
        )
    });
    let baseline_raw = fs::read_to_string(&baseline_path)
        .unwrap_or_else(|_| panic!("missing CP11-5 baseline: {}", baseline_path.display()));
    let artifact: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid CP11-6 websocket E2E artifact JSON: {}",
            artifact_path.display()
        )
    });
    let baseline: Value = serde_json::from_str(&baseline_raw)
        .unwrap_or_else(|_| panic!("invalid CP11-5 baseline JSON: {}", baseline_path.display()));

    assert_eq!(artifact["step"], "11");
    assert_eq!(artifact["checkpoint"], "CP11-6");
    assert_eq!(artifact["tierId"], "pre-release");
    assert_eq!(artifact["tier"], "Pre-Release Tier");
    assert_eq!(artifact["profile"], "session-gateway");
    assert_eq!(artifact["scenarioFamily"], "im-websocket-e2e");
    assert_eq!(artifact["artifactKind"], "metrics_json");
    assert_eq!(
        artifact["state"],
        "supplement_collected_gate_blocked_pending_real_pre_release_run"
    );
    assert_eq!(artifact["sourceCheckpoint"], "CP11-5");
    assert_eq!(artifact["sourceMarker"], "STEP11_WEBSOCKET_E2E");
    assert_eq!(
        artifact["sourceBaselinePath"],
        "tools/perf/step-11-cp11-5-websocket-e2e-baseline.json"
    );
    assert_eq!(
        artifact["sourceTestPath"],
        "services/session-gateway/tests/performance_websocket_e2e_baseline_test.rs"
    );
    assert_eq!(
        artifact["commercialReadinessBoundary"],
        "supplemental artifact only; not full Pre-Release Tier sign-off"
    );

    for metric_name in [
        "connectedDeviceCount",
        "liveMessageCount",
        "backlogMessageCount",
        "expectedFanoutPerLiveMessage",
        "expectedCapacityTrimmedEventCount",
        "ackCheckpointSeq",
        "minConnectSuccessPermille",
        "minLiveFanoutSuccessPermille",
        "maxConnectP95Ms",
        "maxSubscribeP95Ms",
        "maxLivePushP95Ms",
        "maxAckP95Ms",
        "maxDisconnectRecoveryMs",
        "maxBacklogRestoreMs",
        "maxClusterHandoffMs",
    ] {
        assert_eq!(
            artifact["websocketE2e"][metric_name], baseline["websocketE2e"][metric_name],
            "CP11-6 artifact metric {metric_name} must be copied from the CP11-5 source baseline"
        );
    }

    let required_vectors = artifact["coveredCommercialVectors"]
        .as_array()
        .expect("coveredCommercialVectors must be an array");
    for vector in [
        "real TCP WebSocket long connection",
        "subscription fanout",
        "disconnect recovery",
        "message window trimming",
        "cross-device checkpoint",
        "consistency compensation",
        "cluster route handoff",
    ] {
        assert!(
            required_vectors.iter().any(|item| item == vector),
            "CP11-6 websocket artifact must cover {vector}"
        );
    }
}

#[test]
fn test_cp11_6_indexes_websocket_e2e_artifact_without_relabeling_full_pre_release_gate() {
    let root = workspace_root();
    let catalog_path = root
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json");
    let pre_release_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let step11_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("README.md");
    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let operator_doc_path = root
        .join("docs")
        .join("\u{90E8}\u{7F72}")
        .join("\u{6027}\u{80FD}\u{4E0E}\u{707E}\u{5907}\u{6F14}\u{7EC3}\u{573A}\u{666F}.md");

    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    let catalog_json: Value = serde_json::from_str(&catalog)
        .unwrap_or_else(|_| panic!("invalid Step 11 catalog JSON: {}", catalog_path.display()));
    let pre_release_readme = fs::read_to_string(&pre_release_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 pre-release README: {}",
            pre_release_readme_path.display()
        )
    });
    let step11_readme = fs::read_to_string(&step11_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 artifact root README: {}",
            step11_readme_path.display()
        )
    });
    let artifact_file_list = fs::read_to_string(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 pre-release artifact file list: {}",
            artifact_file_list_path.display()
        )
    });
    let checksum_manifest = fs::read_to_string(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 pre-release checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    let operator_doc = fs::read_to_string(&operator_doc_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 operator doc: {}",
            operator_doc_path.display()
        )
    });

    for required_text in [
        "CP11-6",
        "artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json",
        "supplement_collected_gate_blocked_pending_real_pre_release_run",
    ] {
        assert!(
            catalog.contains(required_text),
            "Step 11 catalog must index {required_text}"
        );
    }

    let websocket_family = catalog_json["scenarioFamilies"]
        .as_array()
        .and_then(|families| {
            families.iter().find(|family| {
                family["family"]
                    .as_str()
                    .is_some_and(|family_name| family_name == "im-websocket-e2e")
            })
        })
        .unwrap_or_else(|| panic!("Step 11 catalog must define im-websocket-e2e scenario family"));
    let websocket_tier_ids = websocket_family["tierIds"]
        .as_array()
        .expect("im-websocket-e2e tierIds must be an array");
    assert!(
        websocket_tier_ids
            .iter()
            .any(|tier_id| tier_id == "ci-smoke")
            && websocket_tier_ids
                .iter()
                .any(|tier_id| tier_id == "pre-release"),
        "Step 11 catalog must promote im-websocket-e2e to pre-release supplemental tracking"
    );

    for (path, content) in [
        (&pre_release_readme_path, pre_release_readme.as_str()),
        (&step11_readme_path, step11_readme.as_str()),
        (&operator_doc_path, operator_doc.as_str()),
    ] {
        for required_text in [
            "CP11-6",
            "im-websocket-e2e/metrics.json",
            "not full Pre-Release Tier sign-off",
        ] {
            assert!(
                content.contains(required_text),
                "{} must contain {}",
                path.display(),
                required_text
            );
        }
    }

    assert!(
        artifact_file_list.contains("im-websocket-e2e/metrics.json"),
        "{} must list the CP11-6 websocket E2E artifact",
        artifact_file_list_path.display()
    );
    assert!(
        checksum_manifest.contains("im-websocket-e2e/metrics.json")
            && checksum_manifest
                .contains("supplement_collected_gate_blocked_pending_real_pre_release_run"),
        "{} must list the CP11-6 artifact and its supplemental state",
        checksum_manifest_path.display()
    );
}
