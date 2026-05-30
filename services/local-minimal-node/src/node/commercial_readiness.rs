use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CRAW_CHAT_RUNTIME_PROFILE_ENV: &str = "CRAW_CHAT_RUNTIME_PROFILE";
pub const CRAW_CHAT_STORAGE_PROVIDER_ENV: &str = "CRAW_CHAT_STORAGE_PROVIDER";
pub const CRAW_CHAT_DATABASE_URL_ENV: &str = "CRAW_CHAT_DATABASE_URL";
pub const CRAW_CHAT_POSTGRES_CONFIG_ENV: &str = "CRAW_CHAT_POSTGRES_CONFIG";
pub const CRAW_CHAT_COMMERCIAL_EVIDENCE_ROOT_ENV: &str = "CRAW_CHAT_COMMERCIAL_EVIDENCE_ROOT";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommercialStep11Evidence {
    pub pre_release_tier_state: Option<String>,
    pub capacity_tier_state: Option<String>,
    pub websocket_e2e_state: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommercialReadinessInputs {
    pub runtime_profile: Option<String>,
    pub storage_provider: Option<String>,
    pub postgres_database_configured: bool,
    pub postgres_runtime_adapter_status: Option<String>,
    pub step11_evidence: CommercialStep11Evidence,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommercialReadinessStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommercialReadinessBlocker {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommercialReadinessReport {
    pub status: CommercialReadinessStatus,
    pub blockers: Vec<CommercialReadinessBlocker>,
}

pub fn evaluate_commercial_readiness(
    inputs: CommercialReadinessInputs,
) -> CommercialReadinessReport {
    let mut blockers = Vec::new();
    let runtime_profile = normalize_optional(inputs.runtime_profile.as_deref());
    let storage_provider = normalize_optional(inputs.storage_provider.as_deref());
    let postgres_runtime_adapter_status =
        normalize_optional(inputs.postgres_runtime_adapter_status.as_deref());

    if !is_commercial_runtime_profile(runtime_profile.as_deref()) {
        push_blocker(
            &mut blockers,
            "commercial_runtime_profile_not_selected",
            "commercial readiness requires an explicit production/commercial runtime profile",
        );
    }

    if storage_provider.as_deref() != Some("postgresql") {
        push_blocker(
            &mut blockers,
            "production_storage_provider_not_postgresql",
            "production/commercial profiles must use the PostgreSQL storage provider",
        );
    }

    if !inputs.postgres_database_configured {
        push_blocker(
            &mut blockers,
            "postgres_configuration_missing",
            "production/commercial profiles require explicit PostgreSQL connection configuration",
        );
    }

    match postgres_runtime_adapter_status.as_deref() {
        Some("runtime_ready") => {}
        Some("contract_only") => push_blocker(
            &mut blockers,
            "postgres_runtime_adapter_contract_only",
            "PostgreSQL realtime SQL is contract-only until a live driver-backed adapter is wired",
        ),
        Some(status) => push_blocker(
            &mut blockers,
            "postgres_runtime_adapter_not_ready",
            format!("PostgreSQL realtime runtime adapter is not ready: {status}"),
        ),
        None => push_blocker(
            &mut blockers,
            "postgres_runtime_adapter_status_missing",
            "PostgreSQL realtime runtime adapter status is required for commercial readiness",
        ),
    }

    evaluate_step11_state(
        &mut blockers,
        inputs.step11_evidence.pre_release_tier_state.as_deref(),
        "step11_pre_release_gate_not_passed",
        "Step 11 pre-release tier must have full gate_passed evidence",
    );
    evaluate_step11_state(
        &mut blockers,
        inputs.step11_evidence.capacity_tier_state.as_deref(),
        "step11_capacity_gate_not_passed",
        "Step 11 capacity tier must have full gate_passed evidence",
    );
    evaluate_step11_state(
        &mut blockers,
        inputs.step11_evidence.websocket_e2e_state.as_deref(),
        "step11_websocket_e2e_gate_not_passed",
        "Step 11 WebSocket E2E evidence must be a real pre-release gate_passed run",
    );

    CommercialReadinessReport {
        status: if blockers.is_empty() {
            CommercialReadinessStatus::Ready
        } else {
            CommercialReadinessStatus::Blocked
        },
        blockers,
    }
}

pub fn evaluate_commercial_readiness_from_workspace(
    workspace_root: impl AsRef<Path>,
    mut inputs: CommercialReadinessInputs,
) -> Result<CommercialReadinessReport, String> {
    inputs.step11_evidence = load_commercial_step11_evidence(workspace_root.as_ref())?;
    Ok(evaluate_commercial_readiness(inputs))
}

pub fn evaluate_commercial_readiness_from_env(
    workspace_root: impl AsRef<Path>,
) -> Result<CommercialReadinessReport, String> {
    let postgres_database_configured = has_non_empty_env(CRAW_CHAT_DATABASE_URL_ENV)
        || has_non_empty_env(CRAW_CHAT_POSTGRES_CONFIG_ENV);

    evaluate_commercial_readiness_from_workspace(
        workspace_root,
        CommercialReadinessInputs {
            runtime_profile: std::env::var(CRAW_CHAT_RUNTIME_PROFILE_ENV).ok(),
            storage_provider: std::env::var(CRAW_CHAT_STORAGE_PROVIDER_ENV).ok(),
            postgres_database_configured,
            postgres_runtime_adapter_status: Some("contract_only".into()),
            step11_evidence: CommercialStep11Evidence::default(),
        },
    )
}

pub fn commercial_readiness_required_from_env() -> bool {
    let runtime_profile = std::env::var(CRAW_CHAT_RUNTIME_PROFILE_ENV).ok();
    commercial_readiness_required_for_profile(runtime_profile.as_deref())
}

pub fn commercial_readiness_required_for_profile(runtime_profile: Option<&str>) -> bool {
    is_commercial_runtime_profile(normalize_optional(runtime_profile).as_deref())
}

pub fn format_commercial_readiness_report(report: &CommercialReadinessReport) -> String {
    let mut lines = vec![format!(
        "commercial readiness status: {}",
        commercial_readiness_status_label(&report.status)
    )];
    for blocker in &report.blockers {
        lines.push(format!("- {}: {}", blocker.code, blocker.message));
    }
    lines.join("\n")
}

pub fn format_commercial_readiness_blocked_error(report: &CommercialReadinessReport) -> String {
    let codes = report
        .blockers
        .iter()
        .map(|blocker| blocker.code.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!("commercial readiness blocked: {codes}")
}

pub fn load_commercial_step11_evidence(
    workspace_root: &Path,
) -> Result<CommercialStep11Evidence, String> {
    let catalog_path = workspace_root.join("tools/perf/step-11-scenario-catalog.json");
    let catalog = read_json(&catalog_path, "Step 11 scenario catalog")?;
    let pre_release_tier_state = find_catalog_tier_state(&catalog, "pre-release");
    let capacity_tier_state = find_catalog_tier_state(&catalog, "capacity");

    let websocket_metrics_path =
        workspace_root.join("artifacts/perf/step-11/pre-release/im-websocket-e2e/metrics.json");
    let websocket_metrics = read_json(&websocket_metrics_path, "Step 11 WebSocket E2E metrics")?;
    let websocket_e2e_state = string_field(&websocket_metrics, "state");

    Ok(CommercialStep11Evidence {
        pre_release_tier_state,
        capacity_tier_state,
        websocket_e2e_state,
    })
}

fn evaluate_step11_state(
    blockers: &mut Vec<CommercialReadinessBlocker>,
    state: Option<&str>,
    code: &str,
    message: &str,
) {
    if normalize_optional(state).as_deref() != Some("gate_passed") {
        let detail = state.unwrap_or("missing");
        push_blocker(
            blockers,
            code,
            format!("{message}; current state: {detail}"),
        );
    }
}

fn is_commercial_runtime_profile(profile: Option<&str>) -> bool {
    matches!(
        profile,
        Some("production")
            | Some("prod")
            | Some("commercial")
            | Some("production-postgres")
            | Some("enterprise")
            | Some("pre-release")
            | Some("capacity-dedicated")
    )
}

fn push_blocker(
    blockers: &mut Vec<CommercialReadinessBlocker>,
    code: impl Into<String>,
    message: impl Into<String>,
) {
    blockers.push(CommercialReadinessBlocker {
        code: code.into(),
        message: message.into(),
    });
}

fn read_json(path: &Path, description: &str) -> Result<Value, String> {
    let body = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read {description} at {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&body).map_err(|error| {
        format!(
            "failed to parse {description} at {}: {error}",
            path.display()
        )
    })
}

fn find_catalog_tier_state(catalog: &Value, tier_id: &str) -> Option<String> {
    catalog
        .get("tiers")?
        .as_array()?
        .iter()
        .find(|tier| tier.get("id").and_then(Value::as_str) == Some(tier_id))
        .and_then(|tier| string_field(tier, "state"))
}

fn string_field(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(Value::as_str).map(str::to_owned)
}

fn normalize_optional(value: Option<&str>) -> Option<String> {
    let normalized = value?.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn has_non_empty_env(name: &str) -> bool {
    std::env::var(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn commercial_readiness_status_label(status: &CommercialReadinessStatus) -> &'static str {
    match status {
        CommercialReadinessStatus::Ready => "ready",
        CommercialReadinessStatus::Blocked => "blocked",
    }
}
