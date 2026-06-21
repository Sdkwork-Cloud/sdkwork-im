#![allow(dead_code, unused_imports)]

pub mod block;
pub mod direct_chat;
pub mod external;
pub mod friendship;
pub mod shared_channel;
mod control_routes;
mod http;
mod openapi;
pub mod postgres;
mod runtime;

use serde::{Deserialize, Serialize};

pub use control_routes::{build_control_domain_api_router, build_control_public_router};
pub use http::build_app;
pub use postgres::{
    PostgresAppState, app_state_from_postgres_pool, try_postgres_app_state_from_database_url_env,
};
pub use runtime::SocialRuntime;

pub const SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD: u32 = 3;

pub trait SharedChannelLinkedMemberSyncTrigger: Send + Sync {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelLinkedMemberSyncRequest {
    pub tenant_id: String,
    pub conversation_id: String,
    pub shared_channel_policy_id: String,
    pub external_connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedChannelSyncDeliveryProofStatus {
    TransportAccepted,
    Applied,
    AlreadyLinked,
    Replayed,
    Failed,
}
