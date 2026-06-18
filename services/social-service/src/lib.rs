#![allow(dead_code, unused_imports)]

mod block;
mod direct_chat;
mod external;
mod friendship;
mod http;
mod openapi;
mod postgres;
mod runtime;
mod shared_channel;

use serde::{Deserialize, Serialize};

pub use http::{
    build_app, build_embedded_app, build_public_app, build_public_app_with_contact_extension,
    build_public_app_with_postgres_extension,
};
pub use postgres::{
    PostgresAppState, app_state_from_postgres_pool, build_supplemental_app,
    build_supplemental_public_app, try_postgres_app_state_from_database_url_env,
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
