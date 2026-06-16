//! Space Service - 空间、群组、频道、成员、邀请、封禁

mod ban;
mod bootstrap;
mod channel;
mod channel_access_rule;
mod group;
mod group_member;
mod http;
mod invitation;
mod openapi;
mod service_http;
mod space;
mod space_member;

pub use bootstrap::{
    app_state_from_postgres_pool, try_build_embedded_app_from_database_url_env,
    try_build_public_app_from_database_url_env,
};
pub use http::{AppState, build_app, build_embedded_app, build_public_app};
