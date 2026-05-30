use std::path::{Path, PathBuf};

use local_minimal_node::{
    CommercialReadinessInputs, CommercialReadinessStatus, CommercialStep11Evidence,
    evaluate_commercial_readiness, evaluate_commercial_readiness_from_workspace,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn blocker_codes(report: &local_minimal_node::CommercialReadinessReport) -> Vec<&str> {
    report
        .blockers
        .iter()
        .map(|blocker| blocker.code.as_str())
        .collect()
}

fn passed_step11_evidence() -> CommercialStep11Evidence {
    CommercialStep11Evidence {
        pre_release_tier_state: Some("gate_passed".into()),
        capacity_tier_state: Some("gate_passed".into()),
        websocket_e2e_state: Some("gate_passed".into()),
    }
}

#[test]
fn test_production_profile_rejects_missing_or_local_storage_provider() {
    let report = evaluate_commercial_readiness(CommercialReadinessInputs {
        runtime_profile: Some("production-postgres".into()),
        storage_provider: Some("local-disk".into()),
        postgres_database_configured: false,
        postgres_runtime_adapter_status: Some("runtime_ready".into()),
        step11_evidence: passed_step11_evidence(),
    });

    assert_eq!(report.status, CommercialReadinessStatus::Blocked);
    assert!(
        blocker_codes(&report).contains(&"production_storage_provider_not_postgresql"),
        "production profile must not silently run on local disk, report: {report:?}"
    );
    assert!(
        blocker_codes(&report).contains(&"postgres_configuration_missing"),
        "production profile must require explicit PostgreSQL connection configuration, report: {report:?}"
    );
}

#[test]
fn test_production_postgres_profile_blocks_while_runtime_adapter_is_contract_only() {
    let report = evaluate_commercial_readiness(CommercialReadinessInputs {
        runtime_profile: Some("production-postgres".into()),
        storage_provider: Some("postgresql".into()),
        postgres_database_configured: true,
        postgres_runtime_adapter_status: Some("contract_only".into()),
        step11_evidence: passed_step11_evidence(),
    });

    assert_eq!(report.status, CommercialReadinessStatus::Blocked);
    assert!(
        blocker_codes(&report).contains(&"postgres_runtime_adapter_contract_only"),
        "PostgreSQL SQL contracts must not be presented as a live runtime adapter, report: {report:?}"
    );
}

#[test]
fn test_current_step11_pending_and_blocked_evidence_blocks_commercial_readiness() {
    let report = evaluate_commercial_readiness_from_workspace(
        repo_root(),
        CommercialReadinessInputs {
            runtime_profile: Some("production-postgres".into()),
            storage_provider: Some("postgresql".into()),
            postgres_database_configured: true,
            postgres_runtime_adapter_status: Some("runtime_ready".into()),
            step11_evidence: CommercialStep11Evidence::default(),
        },
    )
    .expect("commercial readiness gate should load repository Step 11 evidence");

    assert_eq!(report.status, CommercialReadinessStatus::Blocked);
    let codes = blocker_codes(&report);
    assert!(
        codes.contains(&"step11_pre_release_gate_not_passed"),
        "pre-release pending or blocked evidence must block commercial readiness, report: {report:?}"
    );
    assert!(
        codes.contains(&"step11_capacity_gate_not_passed"),
        "capacity pending evidence must block commercial readiness, report: {report:?}"
    );
    assert!(
        codes.contains(&"step11_websocket_e2e_gate_not_passed"),
        "supplemental websocket evidence must not be treated as full pre-release sign-off, report: {report:?}"
    );
}

#[test]
fn test_commercial_readiness_workspace_gate_fails_closed_when_evidence_is_missing() {
    let missing_root = Path::new("target/definitely-missing-commercial-readiness-root");
    let error = evaluate_commercial_readiness_from_workspace(
        missing_root,
        CommercialReadinessInputs {
            runtime_profile: Some("production-postgres".into()),
            storage_provider: Some("postgresql".into()),
            postgres_database_configured: true,
            postgres_runtime_adapter_status: Some("runtime_ready".into()),
            step11_evidence: CommercialStep11Evidence::default(),
        },
    )
    .expect_err("commercial readiness gate should fail closed when Step 11 evidence is missing");

    assert!(
        error.contains("step-11-scenario-catalog.json"),
        "missing Step 11 catalog should be called out explicitly, error: {error}"
    );
}
