use super::*;
use serde_json::json;

pub(super) async fn get_home() -> Json<Value> {
    Json(home_snapshot())
}

pub(super) async fn get_access() -> Json<Value> {
    Json(access_snapshot())
}

pub(super) async fn get_workspace(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(workspace_snapshot_for_tenant(auth.tenant_id.as_str())))
}

pub(super) async fn get_dashboard(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(dashboard_snapshot()))
}

pub(super) async fn get_conversations(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(conversations_snapshot()))
}

pub(super) async fn get_realtime(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(realtime_snapshot()))
}

pub(super) async fn get_media(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(media_snapshot()))
}

pub(super) async fn get_automation(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(automation_snapshot()))
}

pub(super) async fn get_governance(
    headers: HeaderMap,
    State(_state): State<AppState>,
) -> Result<Json<Value>, axum::response::Response> {
    let auth = resolve_app_context(&headers).map_err(IntoResponse::into_response)?;
    access::ensure_portal_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(governance_snapshot()))
}

fn home_snapshot() -> Value {
    json!({
        "hero": {
            "eyebrow": "Tenant IM Operations",
            "title": "Craw Chat Tenant Portal",
            "description": "Run a real local tenant console for conversations, realtime posture, media operations, automation, and governance."
        },
        "pillars": [
            { "title": "Conversation oversight", "description": "Track inbox pressure, handoffs, and active customer queues in one place." },
            { "title": "Realtime posture", "description": "Observe subscriptions, reconnect health, and device sync windows from the tenant view." },
            { "title": "Media and governance", "description": "Audit uploads, RTC rooms, provider health, and compliance actions together." }
        ]
    })
}

fn access_snapshot() -> Value {
    json!({
        "eyebrow": "Tenant Access",
        "title": "Nebula Commerce IM access",
        "description": "Read the tenant console entry snapshot. Identity, tenant, organization, and token validation are supplied by the upstream platform context.",
        "details": [
            { "label": "Workspace", "value": "Nebula Commerce IM" },
            { "label": "Role", "value": "Tenant Operations Lead" },
            { "label": "Access", "value": "Validated upstream platform context required" }
        ],
        "primaryActionLabel": "Continue",
        "secondaryActionLabel": "Back to home"
    })
}

pub(super) fn workspace_snapshot_for_tenant(tenant_id: &str) -> Value {
    let (name, slug) = workspace_identity_for_tenant(tenant_id);
    json!({
        "name": name,
        "slug": slug,
        "tier": "Enterprise",
        "region": "CN-East / Multi-AZ",
        "supportPlan": "Platinum",
        "seats": 84,
        "activeBrands": 12,
        "uptime": "99.983%"
    })
}

fn workspace_identity_for_tenant(tenant_id: &str) -> (String, String) {
    if tenant_id.trim().eq_ignore_ascii_case("t_demo") {
        return ("Nebula Commerce IM".into(), "nebula-commerce-im".into());
    }

    let mut segments = tenant_id
        .trim()
        .to_ascii_lowercase()
        .split(|value: char| !value.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();

    if segments.len() > 1
        && matches!(
            segments.first().map(String::as_str),
            Some("tenant") | Some("t")
        )
    {
        segments.remove(0);
    }

    if segments.is_empty() {
        segments.push("tenant".into());
    }

    let slug_stem = segments.join("-");
    let brand = segments
        .iter()
        .map(|segment| title_case_segment(segment))
        .collect::<Vec<_>>()
        .join(" ");

    (
        format!("{brand} Commerce IM"),
        format!("{slug_stem}-commerce-im"),
    )
}

fn title_case_segment(segment: &str) -> String {
    let mut chars = segment.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

fn dashboard_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Tenant operations overview",
            "description": "Keep chat, realtime, media, and governance posture visible from one commercial-grade local console.",
            "kpis": [
                { "label": "Messages today", "value": "284k", "delta": "+12%", "tone": "positive", "caption": "Peak 4.1k/min" },
                { "label": "Pending queues", "value": "34", "delta": "-18%", "tone": "positive", "caption": "VIP 6 / Standard 28" },
                { "label": "Recovery success", "value": "99.42%", "delta": "+0.7 pts", "tone": "positive", "caption": "Last 6h" }
            ]
        },
        "pressure": [
            { "label": "VIP queue", "caption": "High-value customer traffic", "value": "72%", "percent": 72, "tone": "warning" },
            { "label": "Bot assist", "caption": "Needs human takeover", "value": "41%", "percent": 41, "tone": "neutral" },
            { "label": "Night shift readiness", "caption": "Cross-region coverage", "value": "88%", "percent": 88, "tone": "positive" }
        ],
        "posture": [
            { "label": "Realtime ack lag", "value": "0.9s", "status": "Healthy", "tone": "positive", "description": "Ack windows remain within the tenant SLO." },
            { "label": "RTC bridge", "value": "2 rooms waiting", "status": "Watch", "tone": "warning", "description": "Two sessions are waiting for regional callback recovery." },
            { "label": "Automation retries", "value": "7 jobs", "status": "Escalate", "tone": "critical", "description": "Coupon recovery jobs need operator review." }
        ],
        "priorities": [
            { "title": "Stabilize VIP refund queue", "description": "Promote two sessions from bot assist into senior commerce support." },
            { "title": "Validate RTC fallback stock", "description": "Replay Beijing region callback samples before evening peak." },
            { "title": "Close provider binding drift", "description": "Media callback route and current runtime binding are still misaligned." }
        ],
        "timeline": [
            { "title": "09:20 handoff peak", "description": "Campaign blast increased human takeover demand by 17%." },
            { "title": "10:05 presence resync", "description": "Recovery path healed 112 stale devices inside 47 seconds." },
            { "title": "10:40 audit export ready", "description": "Morning governance bundle is available for compliance review." }
        ]
    })
}

fn conversations_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Conversation operations",
            "description": "Monitor inbox flow, handoffs, message edits, and read-state movement without leaving the tenant shell."
        },
        "pipeline": [
            { "label": "New inbox", "value": "34", "percent": 34, "caption": "Awaiting first response", "tone": "warning" },
            { "label": "Bot assist", "value": "18", "percent": 18, "caption": "Needs human takeover", "tone": "neutral" },
            { "label": "VIP escalations", "value": "6", "percent": 6, "caption": "Urgent and high-priority sessions", "tone": "critical" }
        ],
        "handoffs": [
            { "conversation": "Refund / #IM-4821", "owner": "Billing bot", "next": "Commerce support group 3", "wait": "2m", "priority": "Urgent" },
            { "conversation": "Shipping / #IM-4788", "owner": "Delivery bot", "next": "Dispatch desk 1", "wait": "5m", "priority": "High" }
        ],
        "watchlist": [
            { "topic": "VIP refund cluster", "customer": "Starship Retail", "unread": "5", "sentiment": "Fragile", "sla": "04:12" },
            { "topic": "Payment failure loop", "customer": "South City Select", "unread": "3", "sentiment": "Escalating", "sla": "06:48" }
        ],
        "systemChannels": [
            { "title": "Operations broadcast", "description": "Service-wide notices for campaigns and routing changes." },
            { "title": "Risk freeze lane", "description": "Shared system notices for payment and fraud review." },
            { "title": "Store war room", "description": "Peak-event routing and collaboration channel." }
        ]
    })
}

fn realtime_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Realtime posture",
            "description": "Track reconnect health, subscription windows, and device sync readiness from the tenant operator view."
        },
        "posture": [
            { "label": "Session recovery", "value": "99.42%", "status": "Stable", "tone": "positive", "description": "Last 6h recovery volume stayed inside guardrails." },
            { "label": "Heartbeat lag", "value": "1.4s", "status": "Watch", "tone": "warning", "description": "East-region Android fleets show mild jitter." },
            { "label": "Realtime backlog", "value": "182 events", "status": "Healthy", "tone": "positive", "description": "Below the 400-event tenant threshold." }
        ],
        "subscriptions": [
            { "label": "Order alerts", "value": "92%", "percent": 92, "caption": "Primary tenant routing", "tone": "positive" },
            { "label": "Service inbox", "value": "61%", "percent": 61, "caption": "Peak backlog pressure", "tone": "warning" },
            { "label": "Campaign war room", "value": "47%", "percent": 47, "caption": "Post-blast replay coverage", "tone": "neutral" }
        ],
        "devices": [
            { "owner": "Store ops 01", "device": "iPhone 15 Pro", "sync": "7s ago", "lag": "0.8s", "state": "Healthy" },
            { "owner": "Dispatch 12", "device": "Android tablet", "sync": "13s ago", "lag": "1.6s", "state": "Watch" }
        ],
        "events": [
            { "title": "Presence rebuild completed", "description": "112 stale devices were recovered after network jitter." },
            { "title": "Checkpoint window converged", "description": "Campaign room subscribers reset replay cursors cleanly." },
            { "title": "Stale token isolated", "description": "One expired device session was redirected back to sign-in." }
        ]
    })
}

fn media_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Media and RTC",
            "description": "Bring uploads, persisted assets, stream sessions, and RTC rooms into one tenant operating view."
        },
        "assets": [
            { "asset": "refund-proof-240409.zip", "type": "Archive", "state": "Ready", "queue": "0m", "owner": "Commerce desk" },
            { "asset": "campaign-hero.mp4", "type": "Video", "state": "Transcoding", "queue": "3m", "owner": "Growth ops" }
        ],
        "rtcSessions": [
            { "room": "VIP care room 17", "region": "Hangzhou", "participants": "4", "state": "Live", "note": "Recording enabled" },
            { "room": "Peak duty bridge", "region": "Beijing", "participants": "12", "state": "Fallback", "note": "Primary provider degraded" }
        ],
        "providers": [
            { "label": "Media signer", "value": "Healthy", "status": "Healthy", "tone": "positive", "description": "Download signatures remain inside latency budget." },
            { "label": "RTC provider", "value": "Watch", "status": "Fallback", "tone": "warning", "description": "One region is using backup capacity." },
            { "label": "Recording archive", "value": "Drift", "status": "Critical", "tone": "critical", "description": "Archive callback target is misaligned with runtime binding." }
        ],
        "streams": [
            { "title": "Campaign capture", "description": "Active stream still ingesting; 124 frames and 11 checkpoints recorded." },
            { "title": "Merchant onboarding transcript", "description": "Frame sequence is stable and awaiting complete signal." },
            { "title": "Store war room memo", "description": "A stale frame burst triggered an abort threshold review." }
        ]
    })
}

fn automation_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Automation and notifications",
            "description": "Keep workflows, alerts, and operator playbooks visible from one audited execution console."
        },
        "summary": [
            { "label": "Workflow success", "value": "97.8%", "status": "Stable", "tone": "positive", "description": "Last 24h covered 6.2k executions." },
            { "label": "Retry backlog", "value": "7", "status": "Review", "tone": "warning", "description": "Most retries come from coupon and refund reminders." },
            { "label": "Notification timeliness", "value": "99.1%", "status": "Stable", "tone": "positive", "description": "Push and SMS delivery remain within target." }
        ],
        "executions": [
            { "flow": "Refund recovery", "owner": "Commerce ops", "state": "Retrying", "age": "8m", "impact": "92 customers" },
            { "flow": "Night shift warmup", "owner": "Ops console", "state": "Completed", "age": "14m", "impact": "3 squads" }
        ],
        "notifications": [
            { "task": "VIP fallback SMS", "channel": "SMS", "state": "Delivered 98%", "drift": "0.3 pts" },
            { "task": "Merchant reminder", "channel": "Push", "state": "Delivered 95%", "drift": "1.4 pts" }
        ],
        "playbooks": [
            { "title": "Refund rescue", "description": "Escalate to human after the second bot refusal." },
            { "title": "Peak staffing", "description": "Open secondary queue when VIP wait exceeds four minutes." },
            { "title": "RTC fallback drill", "description": "Verify provider failover capability before evening traffic." }
        ]
    })
}

fn governance_snapshot() -> Value {
    json!({
        "hero": {
            "title": "Governance and compliance",
            "description": "Keep audit, provider health, diagnostics, and tenant compliance actions together for continuous operations."
        },
        "auditRecords": [
            { "action": "Preview provider binding", "actor": "Tenant operations lead", "scope": "Media archive", "status": "Reviewed" },
            { "action": "VIP route override", "actor": "Service director", "scope": "Conversation routing", "status": "Applied" }
        ],
        "providerHealth": [
            { "label": "Media signer", "value": "Healthy", "status": "Healthy", "tone": "positive", "description": "95% of signing requests completed in 118ms." },
            { "label": "RTC callback region", "value": "Watch", "status": "Fallback", "tone": "warning", "description": "Callback replay is still in progress." },
            { "label": "User module binding", "value": "Aligned", "status": "Aligned", "tone": "positive", "description": "No runtime drift detected." }
        ],
        "diagnostics": [
            { "title": "Runtime evidence complete", "description": "Morning inspection bundle has been written into the governance ledger." },
            { "title": "Replay posture healthy", "description": "Projection lag remains inside the tenant commitment." },
            { "title": "Archive drift still open", "description": "Callback route differs from the active runtime binding." }
        ],
        "checklist": [
            { "title": "Close archive drift before 18:00", "description": "Required for recorded support-session compliance." },
            { "title": "Export afternoon audit pack", "description": "Prepare the tenant trust review bundle." },
            { "title": "Re-check Beijing RTC fallback", "description": "Protect evening peak stability." }
        ]
    })
}
