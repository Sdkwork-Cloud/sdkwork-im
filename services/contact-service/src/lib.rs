//! Contact Service - 好友请求、好友关系、用户屏蔽、单聊、用户资料

mod friend_request;
mod friendship;
mod block;
mod direct_chat;
mod user_profile;
mod user_settings;
mod http;
mod openapi;

pub use http::{AppState, build_app, build_public_app};
