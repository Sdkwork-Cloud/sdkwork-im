#[test]
fn test_rtc_signal_window_store_uses_sequence_index() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    assert!(
        !source.contains("signals: Mutex<HashMap<String, Vec<RtcSignalEvent>>>"),
        "rtc signaling runtime must not keep signal events in Vec; reconnect reads need sequence lookup"
    );
    assert!(
        source.contains("signals: Mutex<HashMap<String, BTreeMap<u64, RtcSignalEvent>>>"),
        "rtc signaling runtime should index signal events by signal_seq per rtc session"
    );
    assert!(
        source.contains(".range((Excluded(after_signal_seq), Unbounded))"),
        "rtc signal listing should range-seek from afterSignalSeq"
    );
    assert!(
        source.contains("signal_seq: next_signal_seq"),
        "posted rtc signals should receive a server-assigned signal_seq"
    );
}

#[test]
fn test_rtc_session_runtime_uses_typed_store_with_hot_path_indexes() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let session_store = std::fs::read_to_string(format!(
        "{}/src/session_store.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap_or_default()
    .replace("\r\n", "\n");

    assert!(
        source.contains("mod session_store;"),
        "rtc runtime should isolate session indexing into a dedicated module"
    );
    assert!(
        source.contains("sessions: Mutex<RtcSessionRuntimeStore>"),
        "rtc runtime should not expose raw session HashMap mutations"
    );
    assert!(
        !source.contains("sessions: Mutex<HashMap<String, RtcSession>>"),
        "rtc runtime must not store sessions in a bare HashMap; indexes drift when lifecycle fields change"
    );
    assert!(
        session_store.contains("pub struct RtcSessionRuntimeStore"),
        "rtc session runtime store should be explicit and testable"
    );
    assert!(
        session_store.contains("sessions_by_conversation"),
        "conversation-bound rtc access and future conversation room queries need a tenant+conversation index"
    );
    assert!(
        session_store.contains("sessions_by_state"),
        "rtc lifecycle dashboards and cleanup workers need a state index instead of scanning all sessions"
    );
    assert!(
        source.contains(".insert_session("),
        "session create/restore should go through the typed runtime store so indexes are rebuilt consistently"
    );
    assert!(
        source.contains(".update_session("),
        "session lifecycle mutations should go through the typed runtime store so lifecycle indexes stay consistent"
    );
    assert!(
        source.contains("fn encode_rtc_key_segments"),
        "rtc runtime keys need a single segment-safe encoder for sessions, signals, state, and idempotency"
    );
    assert!(
        source.contains("encode_rtc_key_segments([tenant_id, rtc_session_id])"),
        "rtc scope keys must be length-prefixed by segment instead of delimiter-composed"
    );
    assert!(
        source.contains("encode_rtc_key_segments([\n        auth.tenant_id.as_str(),"),
        "rtc create idempotency keys must use the segment-safe runtime key encoder"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{rtc_session_id}\")"),
        "rtc runtime scope keys must not use delimiter-composed tenant/session ids"
    );
    assert!(
        !source.contains("format!(\"{tenant_id}:{action}:{rtc_session_id}\")"),
        "rtc action idempotency keys must not use delimiter-composed tenant/action/session ids"
    );
    assert!(
        session_store.contains("encode_rtc_key_segments([tenant_id, rtc_session_id])"),
        "rtc session store primary key must use segment-safe runtime key encoding"
    );
    assert!(
        session_store.contains("encode_rtc_key_segments([tenant_id, conversation_id])"),
        "rtc conversation index key must use segment-safe runtime key encoding"
    );
    assert!(
        session_store.contains("encode_rtc_key_segments([tenant_id, state.as_wire_value()])"),
        "rtc state index key must use segment-safe runtime key encoding"
    );
    assert!(
        !session_store.contains("format!(\"{tenant_id}:{rtc_session_id}\")")
            && !session_store.contains("format!(\"{tenant_id}:{conversation_id}\")")
            && !session_store.contains("format!(\"{tenant_id}:{}\", state.as_wire_value())"),
        "rtc session store indexes must not use delimiter-composed keys"
    );
}
