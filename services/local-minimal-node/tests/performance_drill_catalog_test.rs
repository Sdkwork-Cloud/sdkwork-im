use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn read_utf8_file(path: impl AsRef<std::path::Path>) -> std::io::Result<String> {
    let path = path.as_ref();
    let bytes = fs::read(path)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn assert_current_pre_release_tier_summary(index_json: &Value) {
    assert_eq!(index_json["state"], "evidence_collected_gate_blocked");
    assert_eq!(index_json["collectionSummary"]["totalSlots"], 7);
    assert_eq!(index_json["collectionSummary"]["requiredSlots"], 7);
    assert_eq!(index_json["collectionSummary"]["optionalSlots"], 0);
    assert_eq!(index_json["collectionSummary"]["collectedSlots"], 7);
    assert_eq!(index_json["collectionSummary"]["pendingSlots"], 0);
    assert_eq!(index_json["collectionSummary"]["skippedOptionalSlots"], 0);
}

fn assert_no_pending_pre_release_slots(evidence_slots: &[Value]) {
    let pending_slot_count = evidence_slots
        .iter()
        .filter(|slot| slot["status"] == "pending_collection")
        .count();
    assert_eq!(pending_slot_count, 0);
}

#[test]
fn test_step11_catalog_freezes_tiers_scenarios_and_repo_assets() {
    let root = workspace_root();
    let catalog_path = root
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json");
    let catalog_schema_path = root
        .join("tools")
        .join("perf")
        .join("schemas")
        .join("step-11-scenario-catalog.schema.json");
    let catalog_raw = read_utf8_file(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    let catalog: Value = serde_json::from_str(&catalog_raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 catalog JSON: {}", catalog_path.display()));
    let catalog_schema_raw = read_utf8_file(&catalog_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 catalog schema: {}",
            catalog_schema_path.display()
        )
    });

    assert_eq!(
        catalog["$schema"],
        "tools/perf/schemas/step-11-scenario-catalog.schema.json"
    );
    assert_eq!(catalog["step"], "11");
    assert_eq!(catalog["checkpoint"], "CP11-1");
    assert_eq!(
        catalog["operatorDocPath"],
        "docs/部署/性能与灾备演练场景.md"
    );
    for required_text in [
        "\"operatorDocPath\"",
        "\"reviewBackwrite\"",
        "\"tiers\"",
        "\"scenarioFamilies\"",
    ] {
        assert!(
            catalog_schema_raw.contains(required_text),
            "Step 11 catalog schema must contain {required_text}"
        );
    }

    for required_text in [
        "docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md",
        "docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md",
    ] {
        assert!(
            catalog["reviewBackwrite"]
                .as_array()
                .expect("Step 11 catalog reviewBackwrite must be an array")
                .iter()
                .any(|item| item == required_text),
            "Step 11 catalog must reference backwrite doc {required_text}"
        );
    }

    let tiers = catalog["tiers"]
        .as_array()
        .expect("Step 11 catalog tiers must be an array");
    for (id, name) in [
        ("ci-smoke", "CI Smoke Tier"),
        ("pre-release", "Pre-Release Tier"),
        ("capacity", "Capacity Tier"),
    ] {
        assert!(
            tiers
                .iter()
                .any(|tier| tier["id"] == id && tier["name"] == name),
            "Step 11 catalog must freeze tier {id} / {name}"
        );
    }
    assert!(
        tiers.iter().any(|tier| {
            tier["id"] == "pre-release"
                && tier["state"] == "template_only_pending_execution"
                && tier["profile"] == "local-default"
                && tier["operatorDocPath"] == "docs/部署/性能与灾备演练场景.md"
        }),
        "Step 11 catalog must freeze Pre-Release Tier state/profile/operatorDocPath"
    );
    assert!(
        tiers.iter().any(|tier| {
            tier["id"] == "capacity"
                && tier["state"] == "template_only_pending_execution"
                && tier["profile"] == "capacity-dedicated"
                && tier["operatorDocPath"] == "docs/部署/性能与灾备演练场景.md"
        }),
        "Step 11 catalog must freeze Capacity Tier state/profile/operatorDocPath"
    );

    let scenario_families = catalog["scenarioFamilies"]
        .as_array()
        .expect("Step 11 catalog scenarioFamilies must be an array");
    for family in [
        "connection",
        "message",
        "stream",
        "drain-rebalance",
        "restore-recovery",
        "failover",
        "upgrade-rollback",
    ] {
        assert!(
            scenario_families
                .iter()
                .any(|scenario| scenario["family"] == family),
            "Step 11 catalog must define the {family} scenario family"
        );
    }

    let failover = scenario_families
        .iter()
        .find(|scenario| scenario["family"] == "failover")
        .expect("Step 11 catalog must include the failover scenario family");
    let failover_metrics = failover["metrics"]
        .as_array()
        .expect("failover metrics must be an array");
    for metric in [
        "takeover_duration_ms",
        "owner_switch_accuracy",
        "stale_session_rejection_rate",
        "resume_takeover_success_rate",
    ] {
        assert!(
            failover_metrics.iter().any(|item| item == metric),
            "Step 11 failover catalog must define metric {metric}"
        );
    }
    let failover_assets = failover["repoAssets"]
        .as_array()
        .expect("failover repoAssets must be an array");
    for asset in [
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs",
        "tools/perf/step-11-cp11-3-local-drill-baseline.json",
    ] {
        assert!(
            failover_assets.iter().any(|item| item == asset),
            "Step 11 failover catalog must reference repo asset {asset}"
        );
    }

    for asset in [
        "services/local-minimal-node/tests/websocket_e2e_test.rs",
        "services/local-minimal-node/tests/cluster_realtime_routing_e2e_test.rs",
        "services/local-minimal-node/tests/cluster_drain_rebalance_e2e_test.rs",
        "services/local-minimal-node/tests/stream_runtime_persistence_test.rs",
        "services/local-minimal-node/tests/runtime_dir_restore_test.rs",
        "services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs",
        "tools/chat-cli/tests/chat_cli_e2e_test.rs",
        "tools/smoke/local_stack_smoke.ps1",
        "tools/smoke/local_stack_smoke.sh",
    ] {
        assert!(
            catalog_raw.contains(asset),
            "Step 11 catalog must reference repo asset {asset}"
        );
    }
}

#[test]
fn test_step11_operator_doc_links_to_catalog_and_execution_tiers() {
    let root = workspace_root();
    let doc_path = root.join("docs").join("部署").join("性能与灾备演练场景.md");
    let doc = read_utf8_file(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 11 operator doc: {}", doc_path.display()));

    for required_text in [
        "tools/perf/step-11-scenario-catalog.json",
        "tools/perf/schemas/step-11-scenario-catalog.schema.json",
        "tools/perf/schemas/step-11-tier-gate.schema.json",
        "CI Smoke Tier",
        "Pre-Release Tier",
        "Capacity Tier",
        "`connection`",
        "`message`",
        "`stream`",
        "`drain-rebalance`",
        "`restore-recovery`",
        "`failover`",
        "`upgrade-rollback`",
        "resume takeover",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 11 operator doc must contain {required_text}"
        );
    }

    for asset in [
        "services/local-minimal-node/tests/websocket_e2e_test.rs",
        "services/local-minimal-node/tests/cluster_drain_rebalance_e2e_test.rs",
        "services/local-minimal-node/tests/runtime_dir_restore_test.rs",
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs",
        "tools/chat-cli/tests/chat_cli_e2e_test.rs",
        "tools/smoke/local_stack_smoke.ps1",
    ] {
        assert!(
            doc.contains(asset),
            "Step 11 operator doc must point operators to asset {asset}"
        );
    }
}

#[test]
fn test_continuous_optimization_freezes_pre_release_and_capacity_tier_gate_templates() {
    let root = workspace_root();
    let pre_release_gate_path = root
        .join("tools")
        .join("perf")
        .join("step-11-pre-release-tier-gate.json");
    let capacity_gate_path = root
        .join("tools")
        .join("perf")
        .join("step-11-capacity-tier-gate.json");
    let gate_schema_path = root
        .join("tools")
        .join("perf")
        .join("schemas")
        .join("step-11-tier-gate.schema.json");
    let catalog_path = root
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json");
    let step_doc_path = root
        .join("docs")
        .join("step")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let review_doc_path = root
        .join("docs")
        .join("review")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let architecture_plan_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md");
    let architecture_design_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("150AR-pre-release-capacity-tier-gates-design-2026-04-09.md");

    let pre_release_gate_raw = read_utf8_file(&pre_release_gate_path).unwrap_or_else(|_| {
        panic!(
            "missing Pre-Release Tier gate template: {}",
            pre_release_gate_path.display()
        )
    });
    let pre_release_gate: Value =
        serde_json::from_str(&pre_release_gate_raw).unwrap_or_else(|_| {
            panic!(
                "invalid Pre-Release Tier gate template: {}",
                pre_release_gate_path.display()
            )
        });
    let capacity_gate_raw = read_utf8_file(&capacity_gate_path).unwrap_or_else(|_| {
        panic!(
            "missing Capacity Tier gate template: {}",
            capacity_gate_path.display()
        )
    });
    let capacity_gate: Value = serde_json::from_str(&capacity_gate_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Capacity Tier gate template: {}",
            capacity_gate_path.display()
        )
    });
    let gate_schema_raw = read_utf8_file(&gate_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 tier gate schema: {}",
            gate_schema_path.display()
        )
    });
    let catalog_raw = read_utf8_file(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    let catalog: Value = serde_json::from_str(&catalog_raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 catalog JSON: {}", catalog_path.display()));

    assert_eq!(
        pre_release_gate["$schema"],
        "tools/perf/schemas/step-11-tier-gate.schema.json"
    );
    assert_eq!(pre_release_gate["step"], "11");
    assert_eq!(pre_release_gate["tierId"], "pre-release");
    assert_eq!(pre_release_gate["tier"], "Pre-Release Tier");
    assert_eq!(pre_release_gate["profile"], "local-default");
    assert_eq!(pre_release_gate["state"], "template_only_pending_execution");
    assert_eq!(
        pre_release_gate["operatorDocPath"],
        "docs/部署/性能与灾备演练场景.md"
    );
    assert_eq!(
        pre_release_gate["scenarioCatalogPath"],
        "tools/perf/step-11-scenario-catalog.json"
    );
    for required_text in [
        "\"tierId\"",
        "\"operatorDocPath\"",
        "\"scenarioCatalogPath\"",
        "\"reviewBackwrite\"",
    ] {
        assert!(
            gate_schema_raw.contains(required_text),
            "Step 11 tier gate schema must contain {required_text}"
        );
    }
    for family in [
        "connection",
        "message",
        "stream",
        "drain-rebalance",
        "restore-recovery",
        "failover",
        "upgrade-rollback",
    ] {
        assert!(
            pre_release_gate["requiredScenarioFamilies"]
                .as_array()
                .expect("Pre-Release Tier requiredScenarioFamilies must be an array")
                .iter()
                .any(|item| item == family),
            "Pre-Release Tier gate must require scenario family {family}"
        );
    }

    assert_eq!(
        capacity_gate["$schema"],
        "tools/perf/schemas/step-11-tier-gate.schema.json"
    );
    assert_eq!(capacity_gate["step"], "11");
    assert_eq!(capacity_gate["tierId"], "capacity");
    assert_eq!(capacity_gate["tier"], "Capacity Tier");
    assert_eq!(capacity_gate["profile"], "capacity-dedicated");
    assert_eq!(capacity_gate["state"], "template_only_pending_execution");
    assert_eq!(
        capacity_gate["operatorDocPath"],
        "docs/部署/性能与灾备演练场景.md"
    );
    assert_eq!(
        capacity_gate["scenarioCatalogPath"],
        "tools/perf/step-11-scenario-catalog.json"
    );
    for family in [
        "connection",
        "message",
        "stream",
        "restore-recovery",
        "failover",
    ] {
        assert!(
            capacity_gate["requiredScenarioFamilies"]
                .as_array()
                .expect("Capacity Tier requiredScenarioFamilies must be an array")
                .iter()
                .any(|item| item == family),
            "Capacity Tier gate must require scenario family {family}"
        );
    }
    for report in ["capacity_report", "recovery_report"] {
        assert!(
            capacity_gate["requiredReports"]
                .as_array()
                .expect("Capacity Tier requiredReports must be an array")
                .iter()
                .any(|item| item == report),
            "Capacity Tier gate must require report {report}"
        );
    }

    let tiers = catalog["tiers"]
        .as_array()
        .expect("Step 11 catalog tiers must be an array");
    assert!(
        tiers.iter().any(|tier| {
            tier["id"] == "pre-release"
                && tier["gateTemplate"] == "tools/perf/step-11-pre-release-tier-gate.json"
        }),
        "Step 11 catalog must link the Pre-Release Tier gate template"
    );
    assert!(
        tiers.iter().any(|tier| {
            tier["id"] == "capacity"
                && tier["gateTemplate"] == "tools/perf/step-11-capacity-tier-gate.json"
        }),
        "Step 11 catalog must link the Capacity Tier gate template"
    );

    for doc_path in [
        step_doc_path,
        review_doc_path,
        architecture_plan_path,
        architecture_design_path,
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing backwrite doc: {}", doc_path.display()));
        for required_text in [
            "step-11-pre-release-tier-gate.json",
            "step-11-capacity-tier-gate.json",
            "template_only_pending_execution",
            "local-default",
            "capacity-dedicated",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }
}

#[test]
fn test_continuous_optimization_freezes_tier_gate_evidence_slot_placeholders() {
    let root = workspace_root();
    let pre_release_gate_path = root
        .join("tools")
        .join("perf")
        .join("step-11-pre-release-tier-gate.json");
    let capacity_gate_path = root
        .join("tools")
        .join("perf")
        .join("step-11-capacity-tier-gate.json");
    let gate_schema_path = root
        .join("tools")
        .join("perf")
        .join("schemas")
        .join("step-11-tier-gate.schema.json");
    let operator_doc_path = root
        .join("docs")
        .join("\u{90E8}\u{7F72}")
        .join("\u{6027}\u{80FD}\u{4E0E}\u{707E}\u{5907}\u{6F14}\u{7EC3}\u{573A}\u{666F}.md");
    let step_doc_path = root
        .join("docs")
        .join("step")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let review_doc_path = root
        .join("docs")
        .join("review")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let architecture_plan_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md");
    let architecture_design_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("150AR-pre-release-capacity-tier-gates-design-2026-04-09.md");

    let pre_release_gate_raw = read_utf8_file(&pre_release_gate_path).unwrap_or_else(|_| {
        panic!(
            "missing Pre-Release Tier gate template: {}",
            pre_release_gate_path.display()
        )
    });
    let pre_release_gate: Value =
        serde_json::from_str(&pre_release_gate_raw).unwrap_or_else(|_| {
            panic!(
                "invalid Pre-Release Tier gate template: {}",
                pre_release_gate_path.display()
            )
        });
    let capacity_gate_raw = read_utf8_file(&capacity_gate_path).unwrap_or_else(|_| {
        panic!(
            "missing Capacity Tier gate template: {}",
            capacity_gate_path.display()
        )
    });
    let capacity_gate: Value = serde_json::from_str(&capacity_gate_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Capacity Tier gate template: {}",
            capacity_gate_path.display()
        )
    });
    let gate_schema_raw = read_utf8_file(&gate_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 tier gate schema: {}",
            gate_schema_path.display()
        )
    });

    for required_text in [
        "\"artifactRoot\"",
        "\"collectionSummary\"",
        "\"evidenceSlots\"",
        "\"suggestedRelativePath\"",
        "\"collectedAt\"",
        "\"sizeBytes\"",
        "\"checksumSha256\"",
    ] {
        assert!(
            gate_schema_raw.contains(required_text),
            "Step 11 tier gate schema must contain {required_text}"
        );
    }

    assert_eq!(
        pre_release_gate["artifactRoot"],
        "artifacts/perf/step-11/pre-release"
    );
    assert_eq!(pre_release_gate["collectionSummary"]["totalSlots"], 7);
    assert_eq!(pre_release_gate["collectionSummary"]["requiredSlots"], 7);
    assert_eq!(pre_release_gate["collectionSummary"]["optionalSlots"], 0);
    assert_eq!(pre_release_gate["collectionSummary"]["collectedSlots"], 0);
    assert_eq!(pre_release_gate["collectionSummary"]["pendingSlots"], 7);
    assert_eq!(
        pre_release_gate["collectionSummary"]["skippedOptionalSlots"],
        0
    );
    let pre_release_slots = pre_release_gate["evidenceSlots"]
        .as_array()
        .expect("Pre-Release Tier evidenceSlots must be an array");
    assert_eq!(
        pre_release_slots.len(),
        7,
        "Pre-Release Tier must freeze one evidence slot per required scenario family"
    );
    for (slot_id, family, suggested_relative_path) in [
        (
            "connection_metrics",
            "connection",
            "connection/metrics.json",
        ),
        ("message_metrics", "message", "message/metrics.json"),
        ("stream_metrics", "stream", "stream/metrics.json"),
        (
            "drain_rebalance_drill",
            "drain-rebalance",
            "drain-rebalance/drill.json",
        ),
        (
            "restore_recovery_drill",
            "restore-recovery",
            "restore-recovery/drill.json",
        ),
        ("failover_drill", "failover", "failover/drill.json"),
        (
            "upgrade_rollback_drill",
            "upgrade-rollback",
            "upgrade-rollback/drill.json",
        ),
    ] {
        assert!(
            pre_release_slots.iter().any(|slot| {
                slot["id"] == slot_id
                    && slot["scenarioFamily"] == family
                    && slot["required"] == true
                    && slot["status"] == "pending_collection"
                    && slot["artifactPath"].is_null()
                    && slot["suggestedRelativePath"] == suggested_relative_path
                    && slot["collectedAt"].is_null()
                    && slot["sizeBytes"].is_null()
                    && slot["checksumSha256"].is_null()
            }),
            "Pre-Release Tier must freeze the {slot_id} evidence slot placeholder"
        );
    }

    assert_eq!(
        capacity_gate["artifactRoot"],
        "artifacts/perf/step-11/capacity"
    );
    assert_eq!(capacity_gate["collectionSummary"]["totalSlots"], 7);
    assert_eq!(capacity_gate["collectionSummary"]["requiredSlots"], 7);
    assert_eq!(capacity_gate["collectionSummary"]["optionalSlots"], 0);
    assert_eq!(capacity_gate["collectionSummary"]["collectedSlots"], 0);
    assert_eq!(capacity_gate["collectionSummary"]["pendingSlots"], 7);
    assert_eq!(
        capacity_gate["collectionSummary"]["skippedOptionalSlots"],
        0
    );
    let capacity_slots = capacity_gate["evidenceSlots"]
        .as_array()
        .expect("Capacity Tier evidenceSlots must be an array");
    assert_eq!(
        capacity_slots.len(),
        7,
        "Capacity Tier must freeze scenario outputs and report slots"
    );
    for (slot_id, family, suggested_relative_path) in [
        (
            "connection_capacity",
            "connection",
            "connection/capacity.json",
        ),
        ("message_capacity", "message", "message/capacity.json"),
        ("stream_capacity", "stream", "stream/capacity.json"),
        (
            "restore_recovery_recovery",
            "restore-recovery",
            "restore-recovery/recovery.json",
        ),
        ("failover_recovery", "failover", "failover/recovery.json"),
    ] {
        assert!(
            capacity_slots.iter().any(|slot| {
                slot["id"] == slot_id
                    && slot["scenarioFamily"] == family
                    && slot["required"] == true
                    && slot["status"] == "pending_collection"
                    && slot["artifactPath"].is_null()
                    && slot["suggestedRelativePath"] == suggested_relative_path
                    && slot["collectedAt"].is_null()
                    && slot["sizeBytes"].is_null()
                    && slot["checksumSha256"].is_null()
            }),
            "Capacity Tier must freeze the {slot_id} evidence slot placeholder"
        );
    }
    for (slot_id, report_id, suggested_relative_path) in [
        (
            "capacity_report",
            "capacity_report",
            "reports/capacity-report.md",
        ),
        (
            "recovery_report",
            "recovery_report",
            "reports/recovery-report.md",
        ),
    ] {
        assert!(
            capacity_slots.iter().any(|slot| {
                slot["id"] == slot_id
                    && slot["reportId"] == report_id
                    && slot["required"] == true
                    && slot["status"] == "pending_collection"
                    && slot["artifactPath"].is_null()
                    && slot["suggestedRelativePath"] == suggested_relative_path
                    && slot["collectedAt"].is_null()
                    && slot["sizeBytes"].is_null()
                    && slot["checksumSha256"].is_null()
            }),
            "Capacity Tier must freeze the {slot_id} report slot placeholder"
        );
    }

    for doc_path in [
        operator_doc_path,
        step_doc_path,
        review_doc_path,
        architecture_plan_path,
        architecture_design_path,
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing backwrite doc: {}", doc_path.display()));
        for required_text in [
            "artifactRoot",
            "collectionSummary",
            "evidenceSlots",
            "pending_collection",
            "checksumSha256",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_tier_gate_artifact_roots_in_catalog_schema_and_backwrites()
{
    let root = workspace_root();
    let catalog_path = root
        .join("tools")
        .join("perf")
        .join("step-11-scenario-catalog.json");
    let catalog_schema_path = root
        .join("tools")
        .join("perf")
        .join("schemas")
        .join("step-11-scenario-catalog.schema.json");
    let operator_doc_path = root
        .join("docs")
        .join("\u{90E8}\u{7F72}")
        .join("\u{6027}\u{80FD}\u{4E0E}\u{707E}\u{5907}\u{6F14}\u{7EC3}\u{573A}\u{666F}.md");
    let step_doc_path = root
        .join("docs")
        .join("step")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let review_doc_path = root
        .join("docs")
        .join("review")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let architecture_plan_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md");
    let architecture_design_path = root
        .join("docs")
        .join("\u{67B6}\u{6784}")
        .join("150AR-pre-release-capacity-tier-gates-design-2026-04-09.md");

    let catalog_raw = read_utf8_file(&catalog_path)
        .unwrap_or_else(|_| panic!("missing Step 11 catalog: {}", catalog_path.display()));
    let catalog: Value = serde_json::from_str(&catalog_raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 catalog JSON: {}", catalog_path.display()));
    let catalog_schema_raw = read_utf8_file(&catalog_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 catalog schema: {}",
            catalog_schema_path.display()
        )
    });

    for required_text in ["\"gateTemplate\"", "\"artifactRoot\""] {
        assert!(
            catalog_schema_raw.contains(required_text),
            "Step 11 catalog schema must contain {required_text}"
        );
    }

    let tiers = catalog["tiers"]
        .as_array()
        .expect("Step 11 catalog tiers must be an array");
    for (tier_id, gate_template, artifact_root) in [
        (
            "pre-release",
            "tools/perf/step-11-pre-release-tier-gate.json",
            "artifacts/perf/step-11/pre-release",
        ),
        (
            "capacity",
            "tools/perf/step-11-capacity-tier-gate.json",
            "artifacts/perf/step-11/capacity",
        ),
    ] {
        assert!(
            tiers.iter().any(|tier| {
                tier["id"] == tier_id
                    && tier["gateTemplate"] == gate_template
                    && tier["artifactRoot"] == artifact_root
            }),
            "Step 11 catalog must expose {tier_id} gate template and artifactRoot"
        );
    }

    for doc_path in [
        operator_doc_path,
        step_doc_path,
        review_doc_path,
        architecture_plan_path,
        architecture_design_path,
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing backwrite doc: {}", doc_path.display()));
        for required_text in [
            "step-11-scenario-catalog.json",
            "artifacts/perf/step-11/pre-release",
            "artifacts/perf/step-11/capacity",
            "artifactRoot",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }
}

#[test]
fn test_continuous_optimization_step11_step_doc_marks_artifact_root_gap_closed() {
    let root = workspace_root();
    let step_doc_path = root
        .join("docs")
        .join("step")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let step_doc = read_utf8_file(&step_doc_path)
        .unwrap_or_else(|_| panic!("missing step doc: {}", step_doc_path.display()));

    for required_text in [
        "2026-04-09 Addendum",
        "This gap is now closed.",
        "step-11-scenario-catalog.json already exposes",
        "artifactRoot",
        "Pre-Release Tier",
        "Capacity Tier",
    ] {
        assert!(
            step_doc.contains(required_text),
            "Step 11 step doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_step11_step_doc_supersedes_stale_catalog_gap_wording() {
    let root = workspace_root();
    let step_doc_path = root
        .join("docs")
        .join("step")
        .join("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md");
    let step_doc = read_utf8_file(&step_doc_path)
        .unwrap_or_else(|_| panic!("missing step doc: {}", step_doc_path.display()));

    let required_text = "Any earlier note in this doc that says the catalog still lacks tier-level artifactRoot is stale and superseded by this addendum.";
    assert!(
        step_doc.contains(required_text),
        "Step 11 step doc must explicitly supersede stale earlier catalog-gap wording"
    );
}

#[test]
fn test_continuous_optimization_materializes_step11_tier_artifact_roots_in_repo() {
    let root = workspace_root();

    for (tier_id, gate_template, artifact_root) in [
        (
            "pre-release",
            "tools/perf/step-11-pre-release-tier-gate.json",
            "artifacts/perf/step-11/pre-release",
        ),
        (
            "capacity",
            "tools/perf/step-11-capacity-tier-gate.json",
            "artifacts/perf/step-11/capacity",
        ),
    ] {
        let artifact_root_path = root.join(artifact_root.replace('/', "\\"));
        assert!(
            artifact_root_path.is_dir(),
            "Step 11 artifactRoot must exist in repo for {tier_id}: {}",
            artifact_root_path.display()
        );

        let readme_path = artifact_root_path.join("README.md");
        let readme = read_utf8_file(&readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 artifactRoot README: {}",
                readme_path.display()
            )
        });

        for required_text in [
            artifact_root,
            gate_template,
            "artifactPath = artifactRoot + \"/\" + suggestedRelativePath",
        ] {
            assert!(
                readme.contains(required_text),
                "{} must contain {}",
                readme_path.display(),
                required_text
            );
        }

        if tier_id == "pre-release" {
            for required_text in [
                "evidence_collected_gate_blocked",
                "connection_metrics",
                "message_metrics",
                "stream_metrics",
                "failover/drill.json",
                "drain-rebalance/drill.json",
                "restore-recovery/drill.json",
                "upgrade-rollback/drill.json",
                "all seven truthful local artifacts",
            ] {
                assert!(
                    readme.contains(required_text),
                    "{} must contain {}",
                    readme_path.display(),
                    required_text
                );
            }
            assert!(
                !readme.contains("pending_collection"),
                "{} must not retain stale pending_collection placeholders",
                readme_path.display()
            );
        } else if tier_id == "capacity" {
            for required_text in [
                "evidence_collected_gate_passed",
                "connection_capacity",
                "message_capacity",
                "stream_capacity",
                "restore-recovery/recovery.json",
                "failover/recovery.json",
                "reports/capacity-report.md",
                "reports/recovery-report.md",
                "all seven truthful local Capacity Tier artifacts",
            ] {
                assert!(
                    readme.contains(required_text),
                    "{} must contain {}",
                    readme_path.display(),
                    required_text
                );
            }
            assert!(
                !readme.contains("pending_collection"),
                "{} must not retain stale pending_collection placeholders",
                readme_path.display()
            );
        } else {
            panic!("unexpected Step 11 tier id: {tier_id}");
        }
    }
}

#[test]
fn test_continuous_optimization_co_locates_step11_tier_evidence_indexes_with_artifact_roots() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("schemas")
        .join("step-11-tier-evidence-index.schema.json");
    let schema_raw = read_utf8_file(&schema_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 tier evidence index schema: {}",
            schema_path.display()
        )
    });

    for required_text in [
        "\"artifact\"",
        "\"gateTemplate\"",
        "\"artifactRoot\"",
        "\"checksumManifestPath\"",
        "\"artifactFileListPath\"",
        "\"collectionSummary\"",
        "\"evidenceSlots\"",
    ] {
        assert!(
            schema_raw.contains(required_text),
            "Step 11 tier evidence index schema must contain {required_text}"
        );
    }

    for (
        tier_id,
        gate_template_path,
        index_relative_path,
        artifact_root,
        checksum_manifest_path,
        artifact_file_list_path,
    ) in [
        (
            "pre-release",
            "tools/perf/step-11-pre-release-tier-gate.json",
            "artifacts/perf/step-11/pre-release/pre-release-tier-evidence-index.json",
            "artifacts/perf/step-11/pre-release",
            "artifacts/perf/step-11/pre-release/checksum-manifest.txt",
            "artifacts/perf/step-11/pre-release/artifact-file-list.txt",
        ),
        (
            "capacity",
            "tools/perf/step-11-capacity-tier-gate.json",
            "artifacts/perf/step-11/capacity/capacity-tier-evidence-index.json",
            "artifacts/perf/step-11/capacity",
            "artifacts/perf/step-11/capacity/checksum-manifest.txt",
            "artifacts/perf/step-11/capacity/artifact-file-list.txt",
        ),
    ] {
        let gate_template_file = root.join(gate_template_path.replace('/', "\\"));
        let gate_template_raw = read_utf8_file(&gate_template_file).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 tier gate template: {}",
                gate_template_file.display()
            )
        });
        let gate_template_json: Value =
            serde_json::from_str(&gate_template_raw).unwrap_or_else(|_| {
                panic!(
                    "invalid Step 11 tier gate template JSON: {}",
                    gate_template_file.display()
                )
            });

        let index_path = root.join(index_relative_path.replace('/', "\\"));
        let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 tier evidence index for {tier_id}: {}",
                index_path.display()
            )
        });
        let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
            panic!(
                "invalid Step 11 tier evidence index JSON: {}",
                index_path.display()
            )
        });

        assert_eq!(
            index_json["$schema"],
            "../schemas/step-11-tier-evidence-index.schema.json"
        );
        assert_eq!(index_json["artifact"], "step-11-tier-evidence-index");
        assert_eq!(index_json["gateTemplate"], gate_template_path);
        assert_eq!(index_json["step"], gate_template_json["step"]);
        assert_eq!(index_json["tierId"], gate_template_json["tierId"]);
        assert_eq!(index_json["tier"], gate_template_json["tier"]);
        assert_eq!(index_json["profile"], gate_template_json["profile"]);
        assert_eq!(
            index_json["operatorDocPath"],
            gate_template_json["operatorDocPath"]
        );
        assert_eq!(
            index_json["scenarioCatalogPath"],
            gate_template_json["scenarioCatalogPath"]
        );
        assert_eq!(index_json["artifactRoot"], artifact_root);
        assert_eq!(index_json["checksumManifestPath"], checksum_manifest_path);
        assert_eq!(index_json["artifactFileListPath"], artifact_file_list_path);

        let checksum_manifest_file = root.join(checksum_manifest_path.replace('/', "\\"));
        let checksum_manifest = read_utf8_file(&checksum_manifest_file).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 checksum manifest placeholder: {}",
                checksum_manifest_file.display()
            )
        });

        let artifact_file_list_file = root.join(artifact_file_list_path.replace('/', "\\"));
        let artifact_file_list =
            read_utf8_file(&artifact_file_list_file).unwrap_or_else(|_| {
                panic!(
                    "missing Step 11 artifact-file-list placeholder: {}",
                    artifact_file_list_file.display()
                )
            });

        if tier_id == "pre-release" {
            assert_current_pre_release_tier_summary(&index_json);
            assert!(
                index_json["boundary"]
                    .as_str()
                    .expect("Pre-Release Tier boundary must be a string")
                    .contains("all seven truthful Pre-Release Tier artifacts"),
                "Pre-Release Tier boundary must explain the fully-collected gate-blocked state"
            );
            let evidence_slots = index_json["evidenceSlots"]
                .as_array()
                .expect("Pre-Release Tier evidenceSlots must be an array");
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "connection_metrics"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/connection/metrics.json"
                }),
                "Pre-Release Tier must materialize the collected connection_metrics slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "message_metrics"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/message/metrics.json"
                }),
                "Pre-Release Tier must materialize the collected message_metrics slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "stream_metrics"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/stream/metrics.json"
                }),
                "Pre-Release Tier must materialize the collected stream_metrics slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "failover_drill"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/failover/drill.json"
                }),
                "Pre-Release Tier must materialize the collected failover_drill slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "drain_rebalance_drill"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/drain-rebalance/drill.json"
                }),
                "Pre-Release Tier must materialize the collected drain_rebalance_drill slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "restore_recovery_drill"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/restore-recovery/drill.json"
                }),
                "Pre-Release Tier must materialize the collected restore_recovery_drill slot"
            );
            assert!(
                evidence_slots.iter().any(|slot| {
                    slot["id"] == "upgrade_rollback_drill"
                        && slot["status"] == "collected"
                        && slot["artifactPath"]
                            == "artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json"
                }),
                "Pre-Release Tier must materialize the collected upgrade_rollback_drill slot"
            );
            assert!(
                checksum_manifest.contains("evidence_collected_gate_blocked"),
                "{} must surface the fully-collected gate-blocked state",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("connection/metrics.json"),
                "{} must list the collected connection artifact",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("failover/drill.json"),
                "{} must list the collected failover artifact",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("drain-rebalance/drill.json"),
                "{} must list the collected drain artifact",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("restore-recovery/drill.json"),
                "{} must list the collected restore artifact",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("upgrade-rollback/drill.json"),
                "{} must list the collected upgrade-rollback artifact",
                checksum_manifest_file.display()
            );
            assert!(
                checksum_manifest.contains("stream/metrics.json"),
                "{} must list the collected stream artifact",
                checksum_manifest_file.display()
            );
            assert!(
                artifact_file_list.contains("connection/metrics.json"),
                "{} must list the collected connection artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("failover/drill.json"),
                "{} must list the collected failover artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("drain-rebalance/drill.json"),
                "{} must list the collected drain artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("restore-recovery/drill.json"),
                "{} must list the collected restore artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("upgrade-rollback/drill.json"),
                "{} must list the collected upgrade-rollback artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("message/metrics.json"),
                "{} must list the collected message artifact",
                artifact_file_list_file.display()
            );
            assert!(
                artifact_file_list.contains("stream/metrics.json"),
                "{} must list the collected stream artifact",
                artifact_file_list_file.display()
            );
        } else if tier_id == "capacity" {
            assert_eq!(index_json["state"], "evidence_collected_gate_passed");
            assert_eq!(index_json["collectionSummary"]["totalSlots"], 7);
            assert_eq!(index_json["collectionSummary"]["requiredSlots"], 7);
            assert_eq!(index_json["collectionSummary"]["collectedSlots"], 7);
            assert_eq!(index_json["collectionSummary"]["pendingSlots"], 0);
            let evidence_slots = index_json["evidenceSlots"]
                .as_array()
                .expect("Capacity Tier evidenceSlots must be an array");
            for (slot_id, artifact_suffix) in [
                ("connection_capacity", "connection/capacity.json"),
                ("message_capacity", "message/capacity.json"),
                ("stream_capacity", "stream/capacity.json"),
                (
                    "restore_recovery_recovery",
                    "restore-recovery/recovery.json",
                ),
                ("failover_recovery", "failover/recovery.json"),
                ("capacity_report", "reports/capacity-report.md"),
                ("recovery_report", "reports/recovery-report.md"),
            ] {
                assert!(
                    evidence_slots.iter().any(|slot| {
                        slot["id"] == slot_id
                            && slot["status"] == "collected"
                            && slot["artifactPath"]
                                == format!(
                                    "artifacts/perf/step-11/capacity/{artifact_suffix}"
                                )
                    }),
                    "Capacity Tier must materialize the collected {slot_id} slot"
                );
                assert!(
                    checksum_manifest.contains(artifact_suffix),
                    "{} must list the collected {slot_id} artifact",
                    checksum_manifest_file.display()
                );
                assert!(
                    artifact_file_list.contains(artifact_suffix),
                    "{} must list the collected {slot_id} artifact",
                    artifact_file_list_file.display()
                );
            }
            assert!(
                checksum_manifest.contains("evidence_collected_gate_passed"),
                "{} must surface the fully-collected gate-passed state",
                checksum_manifest_file.display()
            );
        } else {
            panic!("unexpected Step 11 tier id: {tier_id}");
        }

        let artifact_root_readme_path = root
            .join(artifact_root.replace('/', "\\"))
            .join("README.md");
        let artifact_root_readme =
            read_utf8_file(&artifact_root_readme_path).unwrap_or_else(|_| {
                panic!(
                    "missing Step 11 artifactRoot README: {}",
                    artifact_root_readme_path.display()
                )
            });
        for required_text in [
            index_relative_path
                .rsplit('/')
                .next()
                .expect("index filename should exist"),
            "../schemas/step-11-tier-evidence-index.schema.json",
            "checksum-manifest.txt",
            "artifact-file-list.txt",
        ] {
            assert!(
                artifact_root_readme.contains(required_text),
                "{} must contain {}",
                artifact_root_readme_path.display(),
                required_text
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_tier_gate_templates_in_public_indexes() {
    let root = workspace_root();
    let deployment_index_path = root.join("docs").join("部署").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let architecture_index_path = root.join("docs").join("架构").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    let implementation_plan_path = root.join("docs").join("架构").join("09-实施计划.md");
    let implementation_plan = read_utf8_file(&implementation_plan_path).unwrap_or_else(|_| {
        panic!(
            "missing implementation plan doc: {}",
            implementation_plan_path.display()
        )
    });

    for required_text in [
        "性能与灾备演练场景.md",
        "step-11-pre-release-tier-gate.json",
        "step-11-capacity-tier-gate.json",
    ] {
        assert!(
            deployment_index.contains(required_text),
            "deployment index doc must contain {required_text}"
        );
    }

    for required_text in [
        "性能与灾备演练场景.md",
        "step-11-pre-release-tier-gate.json",
        "step-11-capacity-tier-gate.json",
    ] {
        assert!(
            repo_readme.contains(required_text),
            "repository README must contain {required_text}"
        );
    }

    for required_text in [
        "Pre-Release Tier",
        "Capacity Tier",
        "step-11-pre-release-tier-gate.json",
        "step-11-capacity-tier-gate.json",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "architecture index doc must contain {required_text}"
        );
    }

    for required_text in [
        "Pre-Release Tier",
        "Capacity Tier",
        "step-11-pre-release-tier-gate.json",
        "step-11-capacity-tier-gate.json",
        "template_only_pending_execution",
    ] {
        assert!(
            implementation_plan.contains(required_text),
            "implementation plan doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_tier_gate_backwrite_docs_in_indexes() {
    let root = workspace_root();
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let architecture_index_path = root.join("docs").join("架构").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    let implementation_plan_path = root.join("docs").join("架构").join("09-实施计划.md");
    let implementation_plan = read_utf8_file(&implementation_plan_path).unwrap_or_else(|_| {
        panic!(
            "missing implementation plan doc: {}",
            implementation_plan_path.display()
        )
    });

    assert!(
        step_index
            .contains("continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md"),
        "step index doc must contain the tier gate step backwrite doc"
    );

    for required_text in [
        "09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md",
        "150AR-pre-release-capacity-tier-gates-design-2026-04-09.md",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "architecture index doc must contain {required_text}"
        );
    }

    for required_text in [
        "docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md",
        "docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md",
    ] {
        assert!(
            implementation_plan.contains(required_text),
            "implementation plan doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_tier_gate_review_doc_in_review_index() {
    let root = workspace_root();
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));

    for required_text in [
        "continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "Step 11",
        "Pre-Release Tier",
        "Capacity Tier",
    ] {
        assert!(
            review_index.contains(required_text),
            "review index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_step_and_review_indexes_in_repo_readme() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));

    for required_text in ["docs/step/README.md", "docs/review/README.md"] {
        assert!(
            repo_readme.contains(required_text),
            "repository README must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_related_step_and_architecture_backwrites_in_review_index()
{
    let root = workspace_root();
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));

    for required_text in [
        "docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
        "docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md",
        "docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md",
    ] {
        assert!(
            review_index.contains(required_text),
            "review index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_review_index_in_step_index() {
    let root = workspace_root();
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));

    assert!(
        step_index.contains("docs/review/README.md"),
        "step index doc must contain docs/review/README.md"
    );
}

#[test]
fn test_continuous_optimization_keeps_template_only_state_visible_in_review_index() {
    let root = workspace_root();
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));

    for required_text in [
        "template_only_pending_execution",
        "local-default",
        "capacity-dedicated",
    ] {
        assert!(
            review_index.contains(required_text),
            "review index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_operational_assets_in_review_index() {
    let root = workspace_root();
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));

    for required_text in [
        "docs/部署/性能与灾备演练场景.md",
        "tools/perf/step-11-pre-release-tier-gate.json",
        "tools/perf/step-11-capacity-tier-gate.json",
    ] {
        assert!(
            review_index.contains(required_text),
            "review index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_keeps_template_only_state_visible_in_step_index() {
    let root = workspace_root();
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));

    for required_text in [
        "template_only_pending_execution",
        "local-default",
        "capacity-dedicated",
    ] {
        assert!(
            step_index.contains(required_text),
            "step index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_operational_assets_in_step_index() {
    let root = workspace_root();
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));

    for required_text in [
        "docs/部署/性能与灾备演练场景.md",
        "tools/perf/step-11-pre-release-tier-gate.json",
        "tools/perf/step-11-capacity-tier-gate.json",
    ] {
        assert!(
            step_index.contains(required_text),
            "step index doc must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_catalog_and_artifact_roots_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "tools/perf/step-11-scenario-catalog.json",
            "artifacts/perf/step-11/pre-release",
            "artifacts/perf/step-11/capacity",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_step11_schemas_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "tools/perf/schemas/step-11-scenario-catalog.schema.json",
            "tools/perf/schemas/step-11-tier-gate.schema.json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_root_field_name_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        assert!(
            doc.contains("artifactRoot"),
            "{doc_label} must contain artifactRoot"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_evidence_slot_contract_fields_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "collectionSummary",
            "evidenceSlots",
            "pending_collection",
            "checksumSha256",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_template_only_state_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "template_only_pending_execution",
            "local-default",
            "capacity-dedicated",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_evidence_slot_metadata_fields_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "artifactPath",
            "suggestedRelativePath",
            "collectedAt",
            "sizeBytes",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_null_placeholder_state_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "artifactPath",
            "collectedAt",
            "sizeBytes",
            "checksumSha256",
            "`null`",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_path_naming_rule_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "artifactPath",
            "artifactRoot",
            "suggestedRelativePath",
            "artifactRoot + \"/\" + suggestedRelativePath",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_collection_summary_counters_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "totalSlots",
            "requiredSlots",
            "optionalSlots",
            "collectedSlots",
            "pendingSlots",
            "skippedOptionalSlots",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_collection_summary_frozen_values_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "totalSlots = 7",
            "requiredSlots = 7",
            "optionalSlots = 0",
            "collectedSlots = 0",
            "pendingSlots = 7",
            "skippedOptionalSlots = 0",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_evidence_slot_semantics_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in ["scenarioFamily", "required", "reportId"] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_evidence_slot_semantic_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection",
            "failover",
            "required = true",
            "capacity_report",
            "recovery_report",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_suggested_relative_path_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection/metrics.json",
            "failover/drill.json",
            "reports/capacity-report.md",
            "reports/recovery-report.md",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_evidence_slot_ids_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "id",
            "connection_metrics",
            "connection_capacity",
            "failover_recovery",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_capacity_tier_path_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection/capacity.json",
            "restore-recovery/recovery.json",
            "failover/recovery.json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_remaining_capacity_slot_ids_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "message_capacity",
            "stream_capacity",
            "restore_recovery_recovery",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_remaining_capacity_path_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in ["message/capacity.json", "stream/capacity.json"] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_remaining_pre_release_slot_ids_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in ["message_metrics", "stream_metrics"] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_remaining_pre_release_path_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in ["message/metrics.json", "stream/metrics.json"] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_separates_pre_release_and_capacity_current_states_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "Pre-Release Tier current state is now `evidence_collected_gate_blocked`",
            "Capacity Tier current state is now `evidence_collected_gate_passed`",
            "Both Step 11 tier artifact roots now carry truthful local evidence; dedicated topology runs still gate full commercial sign-off.",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_kind_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "artifactKind",
            "metrics_json",
            "drill_json",
            "capacity_json",
            "recovery_json",
            "report_markdown",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_required_field_contract_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "requiredFields",
            "requiredSections",
            "runId",
            "connectP95Ms",
            "input_scale",
            "operator_follow_up",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_additional_required_field_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "messageTps",
            "frameP95Ms",
            "recovery_window",
            "rto_rpo_summary",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_additional_report_section_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "throughput_summary",
            "tail_latency_summary",
            "recovery_window",
            "operator_follow_up",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_advanced_required_field_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "fanoutP95Ms",
            "streamFramesPerSecond",
            "previewDiffAccuracy",
            "rollbackActivationSeconds",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_drill_required_field_examples_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "drainCompletionSeconds",
            "restoreRtoSeconds",
            "compatibilityMatrixPassRate",
            "postRollbackProtocolErrorRate",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_kind_to_field_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "metrics_json -> connectP95Ms / messageTps / frameP95Ms",
            "drill_json -> drainCompletionSeconds / rollbackActivationSeconds",
            "capacity_json -> fanoutP95Ms / streamFramesPerSecond",
            "recovery_json -> restoreRtoSeconds / previewDiffAccuracy",
            "report_markdown -> throughput_summary / rto_rpo_summary",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_scenario_family_to_slot_id_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection -> connection_metrics / connection_capacity",
            "failover -> failover_drill / failover_recovery",
            "restore-recovery -> restore_recovery_drill / restore_recovery_recovery",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_scenario_family_to_suggested_relative_path_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection -> connection/metrics.json / connection/capacity.json",
            "failover -> failover/drill.json / failover/recovery.json",
            "restore-recovery -> restore-recovery/drill.json / restore-recovery/recovery.json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_report_id_to_suggested_relative_path_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "capacity_report -> reports/capacity-report.md",
            "recovery_report -> reports/recovery-report.md",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_slot_id_to_suggested_relative_path_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection_metrics -> connection/metrics.json",
            "failover_drill -> failover/drill.json",
            "restore_recovery_recovery -> restore-recovery/recovery.json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_report_id_to_artifact_kind_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "capacity_report -> report_markdown",
            "recovery_report -> report_markdown",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_scenario_family_to_artifact_kind_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection -> metrics_json / capacity_json",
            "failover -> drill_json / recovery_json",
            "restore-recovery -> drill_json / recovery_json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_slot_id_to_artifact_kind_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection_metrics -> metrics_json",
            "failover_drill -> drill_json",
            "restore_recovery_recovery -> recovery_json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_report_id_to_required_sections_mappings_in_public_indexes()
{
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "capacity_report -> input_scale / throughput_summary / tail_latency_summary",
            "recovery_report -> recovery_window / rto_rpo_summary / operator_follow_up",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_slot_id_to_required_contract_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection_metrics -> runId / connectP95Ms",
            "failover_drill -> runId / takeoverDurationMs",
            "capacity_report -> input_scale / throughput_summary / tail_latency_summary",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_scenario_family_to_required_contract_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "connection -> runId / connectP95Ms",
            "failover -> runId / takeoverDurationMs",
            "restore-recovery -> runId / restoreRtoSeconds / previewDiffAccuracy",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_kind_to_suggested_relative_path_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "metrics_json -> connection/metrics.json / message/metrics.json",
            "drill_json -> failover/drill.json / restore-recovery/drill.json",
            "capacity_json -> connection/capacity.json / message/capacity.json",
            "recovery_json -> failover/recovery.json / restore-recovery/recovery.json",
            "report_markdown -> reports/capacity-report.md / reports/recovery-report.md",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_kind_to_slot_id_mappings_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "metrics_json -> connection_metrics / message_metrics",
            "drill_json -> failover_drill / restore_recovery_drill",
            "capacity_json -> connection_capacity / message_capacity",
            "recovery_json -> failover_recovery / restore_recovery_recovery",
            "report_markdown -> capacity_report / recovery_report",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_artifact_kind_to_required_contract_mappings_in_public_indexes()
 {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "metrics_json -> runId / connectionCount / successCount",
            "drill_json -> runId / drainCompletionSeconds / takeoverDurationMs",
            "capacity_json -> runId / peakActiveConnections / messageTps",
            "recovery_json -> runId / restoreRtoSeconds / staleSessionRejectionRate",
            "report_markdown -> input_scale / throughput_summary / operator_follow_up",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_required_family_and_report_arrays_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "requiredScenarioFamilies = connection / message / stream / drain-rebalance / restore-recovery / failover / upgrade-rollback",
            "requiredScenarioFamilies = connection / message / stream / restore-recovery / failover",
            "requiredReports = capacity_report / recovery_report",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_required_outputs_array_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "requiredOutputs",
            "connection -> metrics_json -> runId / connectionCount / successCount",
            "restore-recovery -> recovery_json -> runId / restoreRtoSeconds / dataLossRpoEvents / previewDiffAccuracy",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_operator_and_catalog_path_keys_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "operatorDocPath = docs/部署/性能与灾备演练场景.md",
            "scenarioCatalogPath = tools/perf/step-11-scenario-catalog.json",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_surfaces_profile_key_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        let required_text = "profile = local-default / capacity-dedicated";
        assert!(
            doc.contains(required_text),
            "{doc_label} must contain {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_surfaces_review_backwrite_key_in_public_indexes() {
    let root = workspace_root();
    let repo_readme_path = root.join("README.md");
    let repo_readme = read_utf8_file(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));
    let deployment_index_path = root.join("docs").join("\u{90E8}\u{7F72}").join("README.md");
    let deployment_index = read_utf8_file(&deployment_index_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment index doc: {}",
            deployment_index_path.display()
        )
    });
    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });

    for (doc_label, doc) in [
        ("repository README", repo_readme),
        ("deployment index doc", deployment_index),
        ("step index doc", step_index),
        ("review index doc", review_index),
        ("architecture index doc", architecture_index),
    ] {
        for required_text in [
            "reviewBackwrite",
            "docs/step/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
            "docs/review/continuous-optimization-pre-release-capacity-tier-gates-2026-04-09.md",
            "docs/架构/09AR-pre-release-capacity-tier-gates-implementation-plan-2026-04-09.md",
            "docs/架构/150AR-pre-release-capacity-tier-gates-design-2026-04-09.md",
        ] {
            assert!(
                doc.contains(required_text),
                "{doc_label} must contain {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_failover_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("failover")
        .join("drill.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected failover artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected failover artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "failover");
    assert_eq!(artifact_json["artifactKind"], "drill_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-3-local-failover-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["activeOwnerNodeId"], "node_b");
    assert_eq!(artifact_json["staleDisconnectCode"], "stale_session");
    assert_eq!(artifact_json["staleDisconnectRejected"], true);
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-3-local-drill-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert!(
        (artifact_json["takeoverDurationMs"]
            .as_f64()
            .expect("takeoverDurationMs must be numeric")
            - 0.553)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["ownerSwitchAccuracy"]
            .as_f64()
            .expect("ownerSwitchAccuracy must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["resumeTakeoverSuccessRate"]
            .as_f64()
            .expect("resumeTakeoverSuccessRate must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "failover_drill")
        .expect("Pre-Release Tier must expose failover_drill slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/failover/drill.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "failover/drill.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected failover slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected failover slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("failover/drill.json"),
        "{} must list the collected failover artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected failover checksum",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("failover/drill.json"),
        "{} must list the collected failover artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "failover/drill.json",
        "step11-cp11-3-local-failover-2026-04-08-doc-capture",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs")
            .join("\u{67B6}\u{6784}")
            .join("150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09.md"),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "failover/drill.json",
            "takeoverDurationMs = 0.553",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-failover-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BQ-step11-pre-release-failover-collected-evidence-implementation-plan-2026-04-09",
        "150BQ-step11-pre-release-failover-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_restore_recovery_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("restore-recovery")
        .join("drill.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected restore-recovery artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected restore-recovery artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "restore-recovery");
    assert_eq!(artifact_json["artifactKind"], "drill_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-3-local-restore-recovery-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-3-local-drill-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["expectedRestoredFileCount"], 11);
    assert_eq!(artifact_json["restoredFileCount"], 11);
    assert_eq!(artifact_json["restoreStatus"], "restored");
    assert!(
        (artifact_json["previewDurationMs"]
            .as_f64()
            .expect("previewDurationMs must be numeric")
            - 2.453)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["restoreDurationMs"]
            .as_f64()
            .expect("restoreDurationMs must be numeric")
            - 17.983)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["restoreSuccessRate"]
            .as_f64()
            .expect("restoreSuccessRate must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["restoreRtoSeconds"]
            .as_f64()
            .expect("restoreRtoSeconds must be numeric")
            - 0.017_983)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["previewDiffAccuracy"]
            .as_f64()
            .expect("previewDiffAccuracy must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new restore step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new restore review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BR-step11-pre-release-restore-recovery-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new restore implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BR-step11-pre-release-restore-recovery-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new restore design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "restore_recovery_drill")
        .expect("Pre-Release Tier must expose restore_recovery_drill slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/restore-recovery/drill.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "restore-recovery/drill.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected restore slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected restore slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("restore-recovery/drill.json"),
        "{} must list the collected restore artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected restore checksum",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("restore-recovery/drill.json"),
        "{} must list the collected restore artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "restore-recovery/drill.json",
        "step11-cp11-3-local-restore-recovery-2026-04-08-doc-capture",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BR-step11-pre-release-restore-recovery-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BR-step11-pre-release-restore-recovery-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "restore-recovery/drill.json",
            "restoreDurationMs = 17.983",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 restore collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-restore-recovery-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 restore collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BR-step11-pre-release-restore-recovery-collected-evidence-implementation-plan-2026-04-09",
        "150BR-step11-pre-release-restore-recovery-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_drain_rebalance_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("drain-rebalance")
        .join("drill.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected drain-rebalance artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected drain-rebalance artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "drain-rebalance");
    assert_eq!(artifact_json["artifactKind"], "drill_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-3-local-drain-rebalance-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-3-local-drill-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["expectedRouteCount"], 1);
    assert_eq!(artifact_json["migratedRouteCount"], 1);
    assert_eq!(artifact_json["deliveredEventCount"], 1);
    assert_eq!(artifact_json["deliveryPreserved"], true);
    assert!(
        (artifact_json["drillDurationMs"]
            .as_f64()
            .expect("drillDurationMs must be numeric")
            - 0.983)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["drainCompletionSeconds"]
            .as_f64()
            .expect("drainCompletionSeconds must be numeric")
            - 0.000_983)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["routeMigrationSuccessRate"]
            .as_f64()
            .expect("routeMigrationSuccessRate must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["rebalanceP95Ms"]
            .as_f64()
            .expect("rebalanceP95Ms must be numeric")
            - 0.983)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new drain step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new drain review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BS-step11-pre-release-drain-rebalance-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new drain implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new drain design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "drain_rebalance_drill")
        .expect("Pre-Release Tier must expose drain_rebalance_drill slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/drain-rebalance/drill.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "drain-rebalance/drill.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected drain slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected drain slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("drain-rebalance/drill.json"),
        "{} must list the collected drain artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected drain checksum",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("drain-rebalance/drill.json"),
        "{} must list the collected drain artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "drain-rebalance/drill.json",
        "step11-cp11-3-local-drain-rebalance-2026-04-08-doc-capture",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BS-step11-pre-release-drain-rebalance-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "drain-rebalance/drill.json",
            "drillDurationMs = 0.983",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 drain collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-drain-rebalance-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 drain collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BS-step11-pre-release-drain-rebalance-collected-evidence-implementation-plan-2026-04-09",
        "150BS-step11-pre-release-drain-rebalance-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_upgrade_rollback_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("upgrade-rollback")
        .join("drill.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected upgrade-rollback artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected upgrade-rollback artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "upgrade-rollback");
    assert_eq!(artifact_json["artifactKind"], "drill_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-3-local-upgrade-rollback-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-3-local-drill-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_ha_dr_drill_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["safeClientCount"], 4);
    assert_eq!(artifact_json["compatibleClientCount"], 4);
    assert_eq!(artifact_json["blockedBinding"], "ccp/mqtt/1");
    assert_eq!(artifact_json["disabledCapability"], "payload.cbor");
    assert_eq!(artifact_json["preRollbackRiskyHandshakeAccepted"], true);
    assert!(
        (artifact_json["compatibilityMatrixPassRate"]
            .as_f64()
            .expect("compatibilityMatrixPassRate must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["rollbackActivationMs"]
            .as_f64()
            .expect("rollbackActivationMs must be numeric")
            - 0.007)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["rollbackActivationSeconds"]
            .as_f64()
            .expect("rollbackActivationSeconds must be numeric")
            - 0.000_007)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["killSwitchPropagationSuccessRate"]
            .as_f64()
            .expect("killSwitchPropagationSuccessRate must be numeric")
            - 1.0)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["postRollbackProtocolErrorRate"]
            .as_f64()
            .expect("postRollbackProtocolErrorRate must be numeric")
            - 0.0)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new upgrade step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new upgrade review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new upgrade implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new upgrade design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "upgrade_rollback_drill")
        .expect("Pre-Release Tier must expose upgrade_rollback_drill slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/upgrade-rollback/drill.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "upgrade-rollback/drill.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected upgrade slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected upgrade slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("upgrade-rollback/drill.json"),
        "{} must list the collected upgrade artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected upgrade checksum",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("upgrade-rollback/drill.json"),
        "{} must list the collected upgrade artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "upgrade-rollback/drill.json",
        "step11-cp11-3-local-upgrade-rollback-2026-04-08-doc-capture",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "upgrade-rollback/drill.json",
            "rollbackActivationMs = 0.007",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 upgrade collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-upgrade-rollback-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 upgrade collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BT-step11-pre-release-upgrade-rollback-collected-evidence-implementation-plan-2026-04-09",
        "150BT-step11-pre-release-upgrade-rollback-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_connection_metrics_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("connection")
        .join("metrics.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected connection artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected connection artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "connection");
    assert_eq!(artifact_json["artifactKind"], "metrics_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-2-local-connection-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-2-local-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_quant_baseline_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["connectionCount"], 32);
    assert_eq!(artifact_json["successCount"], 32);
    assert!(
        (artifact_json["totalDurationMs"]
            .as_f64()
            .expect("totalDurationMs must be numeric")
            - 17.754)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["connectP95Ms"]
            .as_f64()
            .expect("connectP95Ms must be numeric")
            - 15.108)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["connectionsPerSecond"]
            .as_f64()
            .expect("connectionsPerSecond must be numeric")
            - 1802.431)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new connection step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new connection review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new connection implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new connection design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "connection_metrics")
        .expect("Pre-Release Tier must expose connection_metrics slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/connection/metrics.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "connection/metrics.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected connection slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected connection slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("connection/metrics.json"),
        "{} must list the collected connection artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected connection checksum",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("connection/metrics.json"),
        "{} must list the collected connection artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "connection/metrics.json",
        "step11-cp11-2-local-connection-2026-04-08-doc-capture",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "connection/metrics.json",
            "connectP95Ms = 15.108",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 connection collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-connection-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 connection collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BU-step11-pre-release-connection-metrics-collected-evidence-implementation-plan-2026-04-09",
        "150BU-step11-pre-release-connection-metrics-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_message_metrics_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("message")
        .join("metrics.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected message artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected message artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "message");
    assert_eq!(artifact_json["artifactKind"], "metrics_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-2-local-message-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-2-local-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_quant_baseline_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["messageCount"], 64);
    assert_eq!(artifact_json["successCount"], 64);
    assert_eq!(
        artifact_json["sourceFieldMapping"]["messageP95Ms"],
        "postP95Ms"
    );
    assert_eq!(
        artifact_json["sourceFieldMapping"]["messagesPerSecond"],
        "messageTps"
    );
    assert!(
        (artifact_json["totalDurationMs"]
            .as_f64()
            .expect("totalDurationMs must be numeric")
            - 8.263)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["messageP95Ms"]
            .as_f64()
            .expect("messageP95Ms must be numeric")
            - 0.152)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["messagesPerSecond"]
            .as_f64()
            .expect("messagesPerSecond must be numeric")
            - 7745.652)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new message step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new message review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BW-step11-pre-release-message-metrics-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new message implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BW-step11-pre-release-message-metrics-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new message design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "message_metrics")
        .expect("Pre-Release Tier must expose message_metrics slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/message/metrics.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "message/metrics.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected message slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected message slot checksum must use sha256 prefix"
    );

    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected message checksum",
        checksum_manifest_path.display()
    );
    assert!(
        !checksum_manifest.contains("# stream/metrics.json"),
        "{} must remove the last pending placeholder after stream collection",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("message/metrics.json"),
        "{} must list the collected message artifact",
        artifact_file_list_path.display()
    );
    assert!(
        artifact_file_list.contains("stream/metrics.json"),
        "{} must list the collected stream artifact",
        artifact_file_list_path.display()
    );
    assert!(
        !artifact_file_list.contains("# stream/metrics.json"),
        "{} must remove the last pending placeholder comment",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "message/metrics.json",
        "step11-cp11-2-local-message-2026-04-08-doc-capture",
        "stream_metrics",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    let step11_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("README.md");
    let step11_root_readme = read_utf8_file(&step11_root_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 tier artifact-root index README: {}",
            step11_root_readme_path.display()
        )
    });
    for required_text in [
        "message/metrics.json",
        "stream/metrics.json",
        "evidence_collected_gate_blocked",
    ] {
        assert!(
            step11_root_readme.contains(required_text),
            "{} must contain {}",
            step11_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BW-step11-pre-release-message-metrics-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BW-step11-pre-release-message-metrics-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_partially_collected",
            "message/metrics.json",
            "messageP95Ms = 0.152",
            "messageP95Ms <- postP95Ms",
            "messagesPerSecond <- messageTps",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 message collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-message-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 message collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BW-step11-pre-release-message-metrics-collected-evidence-implementation-plan-2026-04-09",
        "150BW-step11-pre-release-message-metrics-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_materializes_pre_release_stream_metrics_collected_evidence() {
    let root = workspace_root();
    let artifact_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("stream")
        .join("metrics.json");
    let artifact_raw = read_utf8_file(&artifact_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier collected stream artifact: {}",
            artifact_path.display()
        )
    });
    let artifact_json: Value = serde_json::from_str(&artifact_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier collected stream artifact JSON: {}",
            artifact_path.display()
        )
    });
    let artifact_size = fs::metadata(&artifact_path)
        .unwrap_or_else(|_| panic!("missing metadata for {}", artifact_path.display()))
        .len();

    assert_eq!(artifact_json["step"], "11");
    assert_eq!(artifact_json["tierId"], "pre-release");
    assert_eq!(artifact_json["scenarioFamily"], "stream");
    assert_eq!(artifact_json["artifactKind"], "metrics_json");
    assert_eq!(
        artifact_json["runId"],
        "step11-cp11-2-local-stream-2026-04-08-doc-capture"
    );
    assert_eq!(artifact_json["collectedAt"], "2026-04-08");
    assert_eq!(artifact_json["sourceProfile"], "local-minimal");
    assert_eq!(artifact_json["sourceTier"], "CI Smoke Tier");
    assert_eq!(
        artifact_json["sourceBaselinePath"],
        "tools/perf/step-11-cp11-2-local-baseline.json"
    );
    assert_eq!(
        artifact_json["sourceTestPath"],
        "services/local-minimal-node/tests/performance_quant_baseline_test.rs"
    );
    assert_eq!(
        artifact_json["sourceReviewId"],
        "step-11-performance-ha-dr-2026-04-08"
    );
    assert_eq!(artifact_json["frameCount"], 64);
    assert_eq!(artifact_json["successCount"], 64);
    assert_eq!(
        artifact_json["sourceFieldMapping"]["frameP95Ms"],
        "appendP95Ms"
    );
    assert!(
        (artifact_json["totalDurationMs"]
            .as_f64()
            .expect("totalDurationMs must be numeric")
            - 6.03)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["frameP95Ms"]
            .as_f64()
            .expect("frameP95Ms must be numeric")
            - 0.117)
            .abs()
            < f64::EPSILON
    );
    assert!(
        (artifact_json["framesPerSecond"]
            .as_f64()
            .expect("framesPerSecond must be numeric")
            - 10613.071)
            .abs()
            < f64::EPSILON
    );

    let index_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("pre-release-tier-evidence-index.json");
    let index_raw = read_utf8_file(&index_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier evidence index: {}",
            index_path.display()
        )
    });
    let index_json: Value = serde_json::from_str(&index_raw).unwrap_or_else(|_| {
        panic!(
            "invalid Step 11 Pre-Release Tier evidence index JSON: {}",
            index_path.display()
        )
    });
    assert_current_pre_release_tier_summary(&index_json);
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/step/continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new stream step backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/review/continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new stream review backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new stream implementation-plan backwrite"
    );
    assert!(
        index_json["reviewBackwrite"]
            .as_array()
            .expect("reviewBackwrite must be an array")
            .iter()
            .any(|item| {
                item == "docs/\u{67B6}\u{6784}/150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09.md"
            }),
        "Pre-Release Tier evidence index must reference the new stream design backwrite"
    );

    let evidence_slots = index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots must be an array");
    assert_eq!(evidence_slots.len(), 7);
    let collected_slot = evidence_slots
        .iter()
        .find(|slot| slot["id"] == "stream_metrics")
        .expect("Pre-Release Tier must expose stream_metrics slot");
    assert_eq!(collected_slot["status"], "collected");
    assert_eq!(
        collected_slot["artifactPath"],
        "artifacts/perf/step-11/pre-release/stream/metrics.json"
    );
    assert_eq!(
        collected_slot["suggestedRelativePath"],
        "stream/metrics.json"
    );
    assert_eq!(collected_slot["collectedAt"], "2026-04-08");
    assert_eq!(collected_slot["sizeBytes"].as_u64(), Some(artifact_size));
    let checksum = collected_slot["checksumSha256"]
        .as_str()
        .expect("collected stream slot must expose checksumSha256");
    assert!(
        checksum.starts_with("sha256:"),
        "collected stream slot checksum must use sha256 prefix"
    );
    assert_no_pending_pre_release_slots(evidence_slots);

    let checksum_manifest_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("checksum-manifest.txt");
    let checksum_manifest = read_utf8_file(&checksum_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier checksum manifest: {}",
            checksum_manifest_path.display()
        )
    });
    assert!(
        checksum_manifest.contains("evidence_collected_gate_blocked"),
        "{} must surface the fully-collected gate-blocked state",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains("stream/metrics.json"),
        "{} must list the collected stream artifact",
        checksum_manifest_path.display()
    );
    assert!(
        checksum_manifest.contains(checksum),
        "{} must contain the collected stream checksum",
        checksum_manifest_path.display()
    );
    assert!(
        !checksum_manifest.contains("# stream/metrics.json"),
        "{} must remove the last pending placeholder after stream collection",
        checksum_manifest_path.display()
    );

    let artifact_file_list_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("artifact-file-list.txt");
    let artifact_file_list = read_utf8_file(&artifact_file_list_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 Pre-Release Tier artifact-file-list: {}",
            artifact_file_list_path.display()
        )
    });
    assert!(
        artifact_file_list.contains("stream/metrics.json"),
        "{} must list the collected stream artifact",
        artifact_file_list_path.display()
    );
    assert!(
        !artifact_file_list.contains("# stream/metrics.json"),
        "{} must remove the last pending placeholder comment",
        artifact_file_list_path.display()
    );

    let pre_release_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("pre-release")
        .join("README.md");
    let pre_release_root_readme =
        read_utf8_file(&pre_release_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 Pre-Release Tier artifact-root README: {}",
                pre_release_root_readme_path.display()
            )
        });
    for required_text in [
        "evidence_collected_gate_blocked",
        "stream/metrics.json",
        "step11-cp11-2-local-stream-2026-04-08-doc-capture",
        "not full gate sign-off",
    ] {
        assert!(
            pre_release_root_readme.contains(required_text),
            "{} must contain {}",
            pre_release_root_readme_path.display(),
            required_text
        );
    }

    let step11_root_readme_path = root
        .join("artifacts")
        .join("perf")
        .join("step-11")
        .join("README.md");
    let step11_root_readme = read_utf8_file(&step11_root_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing Step 11 tier artifact-root index README: {}",
            step11_root_readme_path.display()
        )
    });
    for required_text in [
        "stream/metrics.json",
        "evidence_collected_gate_blocked",
        "not full gate sign-off",
    ] {
        assert!(
            step11_root_readme.contains(required_text),
            "{} must contain {}",
            step11_root_readme_path.display(),
            required_text
        );
    }

    for doc_path in [
        root.join("docs").join("review").join(
            "continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("step").join(
            "continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09.md",
        ),
        root.join("docs").join("\u{67B6}\u{6784}").join(
            "150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09.md",
        ),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 backwrite doc: {}", doc_path.display()));
        for required_text in [
            "evidence_collected_gate_blocked",
            "stream/metrics.json",
            "frameP95Ms = 0.117",
            "frameP95Ms <- appendP95Ms",
            "not full gate sign-off",
            "Capacity Tier",
            "template_only_pending_execution",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains(
            "continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 stream collected-evidence doc",
        step_index_path.display()
    );
    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index.contains(
            "continuous-optimization-step11-pre-release-stream-metrics-collected-evidence-2026-04-09"
        ),
        "{} must index the new Step 11 stream collected-evidence review doc",
        review_index_path.display()
    );
    let architecture_index_path = root.join("docs").join("\u{67B6}\u{6784}").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BX-step11-pre-release-stream-metrics-collected-evidence-implementation-plan-2026-04-09",
        "150BX-step11-pre-release-stream-metrics-collected-evidence-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}

#[test]
fn test_continuous_optimization_supersedes_stale_step11_closure_claims_in_historical_docs() {
    let root = workspace_root();

    for doc_path in [
        root.join("docs").join("架构").join("09-实施计划.md"),
        root.join("docs")
            .join("review")
            .join("step-11-执行卡-2026-04-08.md"),
        root.join("docs")
            .join("review")
            .join("step-11-架构回写决议-2026-04-08.md"),
        root.join("docs")
            .join("review")
            .join("step-13-go-no-go清单-2026-04-08.md"),
        root.join("docs")
            .join("review")
            .join("wave-d-93-总验收-2026-04-08.md"),
    ] {
        let doc = read_utf8_file(&doc_path)
            .unwrap_or_else(|_| panic!("missing Step 11 historical doc: {}", doc_path.display()));
        for required_text in [
            "2026-04-09 Correction",
            "Step 11 capability baseline was closed for CI Smoke Tier / local-minimal evidence only.",
            "Pre-Release Tier now moves to evidence_collected_gate_blocked.",
            "Capacity Tier still stays template_only_pending_execution.",
            "message_metrics was collected on 2026-04-09.",
            "stream_metrics was collected on 2026-04-09.",
            "All truthful Pre-Release Tier slots are now materialized.",
            "Pre-Release Tier is still not full gate sign-off because the artifacts are doc-captured from published CI Smoke Tier / local-minimal evidence.",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    for doc_path in [
        root.join("docs")
            .join("review")
            .join("continuous-optimization-step11-closure-claim-supersession-2026-04-09.md"),
        root.join("docs")
            .join("step")
            .join("continuous-optimization-step11-closure-claim-supersession-2026-04-09.md"),
        root.join("docs")
            .join("架构")
            .join("09BV-step11-closure-claim-supersession-implementation-plan-2026-04-09.md"),
        root.join("docs")
            .join("架构")
            .join("150BV-step11-closure-claim-supersession-design-2026-04-09.md"),
    ] {
        let doc = read_utf8_file(&doc_path).unwrap_or_else(|_| {
            panic!(
                "missing Step 11 closure-claim supersession backwrite doc: {}",
                doc_path.display()
            )
        });
        for required_text in [
            "2026-04-09 Correction",
            "Step 11 capability baseline was closed for CI Smoke Tier / local-minimal evidence only.",
            "Pre-Release Tier now moves to evidence_collected_gate_blocked.",
            "Capacity Tier still stays template_only_pending_execution.",
            "All truthful Pre-Release Tier slots are now materialized.",
            "Pre-Release Tier is still not full gate sign-off because the artifacts are doc-captured from published CI Smoke Tier / local-minimal evidence.",
        ] {
            assert!(
                doc.contains(required_text),
                "{} must contain {}",
                doc_path.display(),
                required_text
            );
        }
    }

    let step_index_path = root.join("docs").join("step").join("README.md");
    let step_index = read_utf8_file(&step_index_path)
        .unwrap_or_else(|_| panic!("missing step index doc: {}", step_index_path.display()));
    assert!(
        step_index.contains("continuous-optimization-step11-closure-claim-supersession-2026-04-09"),
        "{} must index the Step 11 closure-claim supersession step doc",
        step_index_path.display()
    );

    let review_index_path = root.join("docs").join("review").join("README.md");
    let review_index = read_utf8_file(&review_index_path)
        .unwrap_or_else(|_| panic!("missing review index doc: {}", review_index_path.display()));
    assert!(
        review_index
            .contains("continuous-optimization-step11-closure-claim-supersession-2026-04-09"),
        "{} must index the Step 11 closure-claim supersession review doc",
        review_index_path.display()
    );

    let architecture_index_path = root.join("docs").join("架构").join("README.md");
    let architecture_index = read_utf8_file(&architecture_index_path).unwrap_or_else(|_| {
        panic!(
            "missing architecture index doc: {}",
            architecture_index_path.display()
        )
    });
    for required_text in [
        "09BV-step11-closure-claim-supersession-implementation-plan-2026-04-09",
        "150BV-step11-closure-claim-supersession-design-2026-04-09",
    ] {
        assert!(
            architecture_index.contains(required_text),
            "{} must contain {}",
            architecture_index_path.display(),
            required_text
        );
    }
}
