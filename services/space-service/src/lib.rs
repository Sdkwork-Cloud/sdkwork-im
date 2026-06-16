//! Space Service - 空间、群组、频道、成员、邀请、封禁

mod space;
mod space_member;
mod group;
mod group_member;
mod channel;
mod channel_access_rule;
mod invitation;
mod ban;
mod http;
mod openapi;

pub use http::{AppState, build_app, build_public_app};
