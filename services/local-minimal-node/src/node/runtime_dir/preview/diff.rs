use super::*;

pub(super) fn stable_runtime_dir_restore_preview_fingerprint(
    material: &RuntimeDirRestorePreviewFingerprintMaterial<'_>,
) -> String {
    let payload = serde_json::to_vec(material)
        .expect("runtime-dir restore preview fingerprint material should serialize");
    let mut hash = 14695981039346656037u64;
    for byte in payload {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1099511628211u64);
    }
    format!("rvp1-{hash:016x}")
}

pub(super) fn summarize_runtime_restore_preview_change(
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewChangeSummaryView> {
    let source_value = serde_json::from_slice::<serde_json::Value>(source_payload).ok()?;
    let target_value = serde_json::from_slice::<serde_json::Value>(target_payload).ok()?;

    match (source_value, target_value) {
        (serde_json::Value::Object(source_map), serde_json::Value::Object(target_map)) => {
            let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
            let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
            let added_keys = source_keys
                .difference(&target_keys)
                .cloned()
                .collect::<Vec<_>>();
            let removed_keys = target_keys
                .difference(&source_keys)
                .cloned()
                .collect::<Vec<_>>();

            let mut modified_keys = Vec::new();
            let mut unchanged_key_count = 0usize;
            for key in source_keys.intersection(&target_keys) {
                if source_map.get(key) == target_map.get(key) {
                    unchanged_key_count += 1;
                } else {
                    modified_keys.push(key.clone());
                }
            }

            Some(RuntimeDirRestorePreviewChangeSummaryView {
                summary_kind: "json_object_keys".into(),
                source_key_count: source_map.len(),
                target_key_count: target_map.len(),
                added_keys,
                removed_keys,
                modified_keys,
                unchanged_key_count,
            })
        }
        _ => None,
    }
}

pub(super) fn summarize_disconnect_fence_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-disconnect-fences.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeDisconnectFenceRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeDisconnectFenceRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut owner_node_changed_keys = Vec::new();
    let mut session_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source disconnect fence entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target disconnect fence entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let owner_changed = source_entry.owner_node_id != target_entry.owner_node_id;
        let session_changed = source_entry.session_id != target_entry.session_id;
        if owner_changed {
            owner_node_changed_keys.push(key.clone());
        }
        if session_changed {
            session_changed_keys.push(key.clone());
        }
        if !owner_changed && !session_changed {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "disconnect_fences".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys,
        session_changed_keys,
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: None,
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

pub(super) fn summarize_realtime_checkpoint_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-checkpoints.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeCheckpointRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeCheckpointRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut latest_advanced_keys = Vec::new();
    let mut latest_rewound_keys = Vec::new();
    let mut acked_advanced_keys = Vec::new();
    let mut acked_rewound_keys = Vec::new();
    let mut trimmed_advanced_keys = Vec::new();
    let mut trimmed_rewound_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source realtime checkpoint entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target realtime checkpoint entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let latest_advanced = source_entry.latest_realtime_seq > target_entry.latest_realtime_seq;
        let latest_rewound = source_entry.latest_realtime_seq < target_entry.latest_realtime_seq;
        let acked_advanced = source_entry.acked_through_seq > target_entry.acked_through_seq;
        let acked_rewound = source_entry.acked_through_seq < target_entry.acked_through_seq;
        let trimmed_advanced = source_entry.trimmed_through_seq > target_entry.trimmed_through_seq;
        let trimmed_rewound = source_entry.trimmed_through_seq < target_entry.trimmed_through_seq;

        if latest_advanced {
            latest_advanced_keys.push(key.clone());
        }
        if latest_rewound {
            latest_rewound_keys.push(key.clone());
        }
        if acked_advanced {
            acked_advanced_keys.push(key.clone());
        }
        if acked_rewound {
            acked_rewound_keys.push(key.clone());
        }
        if trimmed_advanced {
            trimmed_advanced_keys.push(key.clone());
        }
        if trimmed_rewound {
            trimmed_rewound_keys.push(key.clone());
        }

        let sequence_changed = latest_advanced
            || latest_rewound
            || acked_advanced
            || acked_rewound
            || trimmed_advanced
            || trimmed_rewound;
        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.principal_id != target_entry.principal_id
            || source_entry.device_id != target_entry.device_id;
        let timestamp_changed = source_entry.updated_at != target_entry.updated_at;

        if !sequence_changed && !identity_changed && timestamp_changed {
            timestamp_only_changed_keys.push(key.clone());
        }
        if identity_changed {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "realtime_checkpoints".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: Some(latest_advanced_keys),
        latest_rewound_keys: Some(latest_rewound_keys),
        acked_advanced_keys: Some(acked_advanced_keys),
        acked_rewound_keys: Some(acked_rewound_keys),
        trimmed_advanced_keys: Some(trimmed_advanced_keys),
        trimmed_rewound_keys: Some(trimmed_rewound_keys),
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

#[derive(Clone)]
struct RealtimeSubscriptionScopeSummary {
    exact_item: RealtimeSubscription,
    event_types: BTreeSet<String>,
}

fn realtime_subscription_scope_key(scope_type: &str, scope_id: &str) -> String {
    format!("{scope_type}:{scope_id}")
}

fn qualified_realtime_subscription_scope_key(record_key: &str, scope_key: &str) -> String {
    format!("{record_key}#{scope_key}")
}

fn summarize_realtime_subscription_items(
    items: &[RealtimeSubscription],
) -> Option<BTreeMap<String, RealtimeSubscriptionScopeSummary>> {
    let mut summary = BTreeMap::new();
    for item in items {
        let scope_key =
            realtime_subscription_scope_key(item.scope_type.as_str(), item.scope_id.as_str());
        let scope_summary = RealtimeSubscriptionScopeSummary {
            exact_item: item.clone(),
            event_types: item.event_types.iter().cloned().collect(),
        };
        if summary.insert(scope_key, scope_summary).is_some() {
            return None;
        }
    }
    Some(summary)
}

pub(super) fn summarize_realtime_subscription_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-subscriptions.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeSubscriptionRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeSubscriptionRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut added_scope_keys = Vec::new();
    let mut removed_scope_keys = Vec::new();
    let mut event_types_added_scope_keys = Vec::new();
    let mut event_types_removed_scope_keys = Vec::new();
    let mut subscribed_at_only_changed_scope_keys = Vec::new();
    let mut synced_timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_scope_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source realtime subscription entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target realtime subscription entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.principal_id != target_entry.principal_id
            || source_entry.device_id != target_entry.device_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let Some(source_scope_map) =
            summarize_realtime_subscription_items(source_entry.items.as_slice())
        else {
            other_modified_keys.push(key.clone());
            continue;
        };
        let Some(target_scope_map) =
            summarize_realtime_subscription_items(target_entry.items.as_slice())
        else {
            other_modified_keys.push(key.clone());
            continue;
        };

        let source_scope_keys: BTreeSet<String> = source_scope_map.keys().cloned().collect();
        let target_scope_keys: BTreeSet<String> = target_scope_map.keys().cloned().collect();
        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        for scope_key in source_scope_keys.difference(&target_scope_keys) {
            added_scope_keys.push(qualified_realtime_subscription_scope_key(
                key.as_str(),
                scope_key.as_str(),
            ));
            record_has_semantic_change = true;
        }
        for scope_key in target_scope_keys.difference(&source_scope_keys) {
            removed_scope_keys.push(qualified_realtime_subscription_scope_key(
                key.as_str(),
                scope_key.as_str(),
            ));
            record_has_semantic_change = true;
        }

        for scope_key in source_scope_keys.intersection(&target_scope_keys) {
            let source_scope = source_scope_map
                .get(scope_key)
                .expect("source realtime subscription scope should exist");
            let target_scope = target_scope_map
                .get(scope_key)
                .expect("target realtime subscription scope should exist");
            if source_scope.exact_item == target_scope.exact_item {
                unchanged_scope_count += 1;
                continue;
            }

            let qualified_scope_key =
                qualified_realtime_subscription_scope_key(key.as_str(), scope_key.as_str());
            let source_has_added_event_types = source_scope
                .event_types
                .difference(&target_scope.event_types)
                .next()
                .is_some();
            let source_has_removed_event_types = target_scope
                .event_types
                .difference(&source_scope.event_types)
                .next()
                .is_some();
            let subscribed_at_changed =
                source_scope.exact_item.subscribed_at != target_scope.exact_item.subscribed_at;

            if source_has_added_event_types {
                event_types_added_scope_keys.push(qualified_scope_key.clone());
                record_has_semantic_change = true;
            }
            if source_has_removed_event_types {
                event_types_removed_scope_keys.push(qualified_scope_key.clone());
                record_has_semantic_change = true;
            }
            if !source_has_added_event_types
                && !source_has_removed_event_types
                && subscribed_at_changed
            {
                subscribed_at_only_changed_scope_keys.push(qualified_scope_key);
                record_has_semantic_change = true;
            } else if !source_has_added_event_types
                && !source_has_removed_event_types
                && !subscribed_at_changed
            {
                record_has_other_change = true;
            }
        }

        let synced_at_changed = source_entry.synced_at != target_entry.synced_at;
        if !record_has_semantic_change && !record_has_other_change && synced_at_changed {
            synced_timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "realtime_subscriptions".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(synced_timestamp_only_changed_keys),
        added_scope_keys: Some(added_scope_keys),
        removed_scope_keys: Some(removed_scope_keys),
        event_types_added_scope_keys: Some(event_types_added_scope_keys),
        event_types_removed_scope_keys: Some(event_types_removed_scope_keys),
        subscribed_at_only_changed_scope_keys: Some(subscribed_at_only_changed_scope_keys),
        unchanged_scope_count: Some(unchanged_scope_count),
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

fn compare_optional_u64(source: Option<u64>, target: Option<u64>) -> std::cmp::Ordering {
    match (source, target) {
        (Some(source), Some(target)) => source.cmp(&target),
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn qualified_stream_frame_key(record_key: &str, frame_seq: u64) -> String {
    format!("{record_key}#frame:{frame_seq}")
}

fn summarize_stream_frames(
    frames: &[im_domain_core::stream::StreamFrame],
) -> Option<BTreeMap<u64, im_domain_core::stream::StreamFrame>> {
    let mut summary = BTreeMap::new();
    for frame in frames {
        if summary.insert(frame.frame_seq, frame.clone()).is_some() {
            return None;
        }
    }
    Some(summary)
}

pub(super) fn summarize_stream_state_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "stream-state.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, StreamStateRecord>>(source_payload).ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, StreamStateRecord>>(target_payload).ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut stream_state_changed_keys = Vec::new();
    let mut stream_last_frame_advanced_keys = Vec::new();
    let mut stream_last_frame_rewound_keys = Vec::new();
    let mut stream_checkpoint_advanced_keys = Vec::new();
    let mut stream_checkpoint_rewound_keys = Vec::new();
    let mut stream_result_message_changed_keys = Vec::new();
    let mut added_frame_keys = Vec::new();
    let mut removed_frame_keys = Vec::new();
    let mut modified_frame_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_frame_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source stream state entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target stream state entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.stream_id != target_entry.stream_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        let state_changed = source_entry.session.state != target_entry.session.state;
        let last_frame_cmp = source_entry
            .session
            .last_frame_seq
            .cmp(&target_entry.session.last_frame_seq);
        let checkpoint_cmp = compare_optional_u64(
            source_entry.session.last_checkpoint_seq,
            target_entry.session.last_checkpoint_seq,
        );
        let result_message_changed =
            source_entry.session.result_message_id != target_entry.session.result_message_id;
        let session_contract_changed = source_entry.session.owner_principal_id
            != target_entry.session.owner_principal_id
            || source_entry.session.owner_principal_kind
                != target_entry.session.owner_principal_kind
            || source_entry.session.stream_type != target_entry.session.stream_type
            || source_entry.session.scope_kind != target_entry.session.scope_kind
            || source_entry.session.scope_id != target_entry.session.scope_id
            || source_entry.session.durability_class != target_entry.session.durability_class
            || source_entry.session.ordering_scope != target_entry.session.ordering_scope
            || source_entry.session.schema_ref != target_entry.session.schema_ref;

        if state_changed {
            stream_state_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if last_frame_cmp == std::cmp::Ordering::Greater {
            stream_last_frame_advanced_keys.push(key.clone());
            record_has_semantic_change = true;
        } else if last_frame_cmp == std::cmp::Ordering::Less {
            stream_last_frame_rewound_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if checkpoint_cmp == std::cmp::Ordering::Greater {
            stream_checkpoint_advanced_keys.push(key.clone());
            record_has_semantic_change = true;
        } else if checkpoint_cmp == std::cmp::Ordering::Less {
            stream_checkpoint_rewound_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if result_message_changed {
            stream_result_message_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if session_contract_changed {
            record_has_other_change = true;
        }

        let Some(source_frames) = summarize_stream_frames(source_entry.frames.as_slice()) else {
            other_modified_keys.push(key.clone());
            continue;
        };
        let Some(target_frames) = summarize_stream_frames(target_entry.frames.as_slice()) else {
            other_modified_keys.push(key.clone());
            continue;
        };

        let source_frame_keys: BTreeSet<u64> = source_frames.keys().copied().collect();
        let target_frame_keys: BTreeSet<u64> = target_frames.keys().copied().collect();
        for frame_seq in source_frame_keys.difference(&target_frame_keys) {
            added_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
            record_has_semantic_change = true;
        }
        for frame_seq in target_frame_keys.difference(&source_frame_keys) {
            removed_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
            record_has_semantic_change = true;
        }
        for frame_seq in source_frame_keys.intersection(&target_frame_keys) {
            let source_frame = source_frames
                .get(frame_seq)
                .expect("source stream frame should exist");
            let target_frame = target_frames
                .get(frame_seq)
                .expect("target stream frame should exist");
            if source_frame == target_frame {
                unchanged_frame_count += 1;
            } else {
                modified_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
                record_has_semantic_change = true;
            }
        }

        let updated_at_changed = source_entry.updated_at != target_entry.updated_at;
        if !record_has_semantic_change && !record_has_other_change && updated_at_changed {
            timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "stream_state".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: Some(stream_state_changed_keys),
        stream_last_frame_advanced_keys: Some(stream_last_frame_advanced_keys),
        stream_last_frame_rewound_keys: Some(stream_last_frame_rewound_keys),
        stream_checkpoint_advanced_keys: Some(stream_checkpoint_advanced_keys),
        stream_checkpoint_rewound_keys: Some(stream_checkpoint_rewound_keys),
        stream_result_message_changed_keys: Some(stream_result_message_changed_keys),
        added_frame_keys: Some(added_frame_keys),
        removed_frame_keys: Some(removed_frame_keys),
        modified_frame_keys: Some(modified_frame_keys),
        unchanged_frame_count: Some(unchanged_frame_count),
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

fn qualified_rtc_signal_key(record_key: &str, signal_index: usize) -> String {
    format!("{record_key}#signal:{signal_index}")
}

fn summarize_rtc_signals(
    signals: &[im_domain_core::rtc::RtcSignalEvent],
) -> BTreeMap<usize, im_domain_core::rtc::RtcSignalEvent> {
    signals
        .iter()
        .enumerate()
        .map(|(index, signal)| (index, signal.clone()))
        .collect()
}

pub(super) fn summarize_rtc_state_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "rtc-state.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RtcStateRecord>>(source_payload).ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RtcStateRecord>>(target_payload).ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut rtc_state_changed_keys = Vec::new();
    let mut rtc_signaling_stream_changed_keys = Vec::new();
    let mut rtc_artifact_message_changed_keys = Vec::new();
    let mut added_signal_keys = Vec::new();
    let mut removed_signal_keys = Vec::new();
    let mut modified_signal_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_signal_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source rtc state entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target rtc state entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.rtc_session_id != target_entry.rtc_session_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        let state_changed = source_entry.session.state != target_entry.session.state;
        let signaling_stream_changed =
            source_entry.session.signaling_stream_id != target_entry.session.signaling_stream_id;
        let artifact_message_changed =
            source_entry.session.artifact_message_id != target_entry.session.artifact_message_id;
        let session_contract_changed = source_entry.session.conversation_id
            != target_entry.session.conversation_id
            || source_entry.session.rtc_mode != target_entry.session.rtc_mode
            || source_entry.session.initiator_id != target_entry.session.initiator_id
            || source_entry.session.initiator_kind != target_entry.session.initiator_kind;

        if state_changed {
            rtc_state_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if signaling_stream_changed {
            rtc_signaling_stream_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if artifact_message_changed {
            rtc_artifact_message_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if session_contract_changed {
            record_has_other_change = true;
        }

        let source_signals = summarize_rtc_signals(source_entry.signals.as_slice());
        let target_signals = summarize_rtc_signals(target_entry.signals.as_slice());
        let source_signal_keys: BTreeSet<usize> = source_signals.keys().copied().collect();
        let target_signal_keys: BTreeSet<usize> = target_signals.keys().copied().collect();

        for signal_index in source_signal_keys.difference(&target_signal_keys) {
            added_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
            record_has_semantic_change = true;
        }
        for signal_index in target_signal_keys.difference(&source_signal_keys) {
            removed_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
            record_has_semantic_change = true;
        }
        for signal_index in source_signal_keys.intersection(&target_signal_keys) {
            let source_signal = source_signals
                .get(signal_index)
                .expect("source rtc signal should exist");
            let target_signal = target_signals
                .get(signal_index)
                .expect("target rtc signal should exist");
            if source_signal == target_signal {
                unchanged_signal_count += 1;
            } else {
                modified_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
                record_has_semantic_change = true;
            }
        }

        let updated_at_changed = source_entry.updated_at != target_entry.updated_at;
        if !record_has_semantic_change && !record_has_other_change && updated_at_changed {
            timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "rtc_state".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: Some(rtc_state_changed_keys),
        rtc_signaling_stream_changed_keys: Some(rtc_signaling_stream_changed_keys),
        rtc_artifact_message_changed_keys: Some(rtc_artifact_message_changed_keys),
        added_signal_keys: Some(added_signal_keys),
        removed_signal_keys: Some(removed_signal_keys),
        modified_signal_keys: Some(modified_signal_keys),
        unchanged_signal_count: Some(unchanged_signal_count),
    })
}
