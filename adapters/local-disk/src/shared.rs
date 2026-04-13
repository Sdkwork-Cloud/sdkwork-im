use std::fs;
use std::path::Path;

use im_platform_contracts::ContractError;
use serde::de::DeserializeOwned;

pub(super) fn read_json_records_or_default<T>(
    file_path: &Path,
    store_name: &str,
) -> Result<T, ContractError>
where
    T: DeserializeOwned + Default,
{
    if !file_path.exists() {
        return Ok(T::default());
    }

    let bytes = fs::read(file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to read {store_name} {}: {error}",
            file_path.display()
        ))
    })?;
    if bytes.is_empty() {
        return Ok(T::default());
    }

    serde_json::from_slice(&bytes).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to parse {store_name} {}: {error}",
            file_path.display()
        ))
    })
}

pub(super) fn write_json_records<T: serde::Serialize + ?Sized>(
    file_path: &Path,
    records: &T,
    store_name: &str,
) -> Result<(), ContractError> {
    let parent = file_path.parent().ok_or_else(|| {
        ContractError::Unavailable(format!(
            "{store_name} path has no parent: {}",
            file_path.display()
        ))
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to create {store_name} dir {}: {error}",
            parent.display()
        ))
    })?;

    let payload = serde_json::to_vec_pretty(records).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to serialize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    let temp_path = file_path.with_extension("json.tmp");
    fs::write(&temp_path, payload).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to write {store_name} temp file {}: {error}",
            temp_path.display()
        ))
    })?;

    if file_path.exists() {
        fs::remove_file(file_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "failed to replace {store_name} {}: {error}",
                file_path.display()
            ))
        })?;
    }

    fs::rename(&temp_path, file_path).map_err(|error| {
        ContractError::Unavailable(format!(
            "failed to finalize {store_name} {}: {error}",
            file_path.display()
        ))
    })?;

    Ok(())
}

pub(super) fn scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

pub(super) fn device_twin_scope_key(tenant_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{device_id}")
}

pub(super) fn stream_scope_key(tenant_id: &str, stream_id: &str) -> String {
    format!("{tenant_id}:{stream_id}")
}

pub(super) fn rtc_scope_key(tenant_id: &str, rtc_session_id: &str) -> String {
    format!("{tenant_id}:{rtc_session_id}")
}

pub(super) fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    format!("{tenant_id}:{notification_id}")
}

pub(super) fn execution_scope_key(
    tenant_id: &str,
    principal_kind: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    format!("{tenant_id}:{principal_kind}:{principal_id}:{execution_id}")
}

pub(super) fn legacy_execution_scope_key(
    tenant_id: &str,
    principal_id: &str,
    execution_id: &str,
) -> String {
    format!("{tenant_id}:{principal_id}:{execution_id}")
}
