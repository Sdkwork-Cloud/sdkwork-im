use sdkwork_web_contract::{HttpMethod, HttpRoute};
use sdkwork_web_core::HttpRouteManifest;

/// API surface: open-api
pub const API_SURFACE: &str = "open-api";

const SOCIAL_OPEN_API_ROUTES: &[HttpRoute] = &[
    HttpRoute::open_api_flexible(
        HttpMethod::Get,
        "/im/v3/api/social/friendships",
        "social",
        "social.friendships.list",
    ),
    HttpRoute::open_api_flexible(
        HttpMethod::Get,
        "/im/v3/api/social/user_blocks",
        "social",
        "social.userBlocks.list",
    ),
    HttpRoute::open_api_flexible(
        HttpMethod::Delete,
        "/im/v3/api/social/user_blocks/{block_id}",
        "social",
        "social.userBlocks.delete",
    ),
    HttpRoute::open_api_flexible(
        HttpMethod::Post,
        "/im/v3/api/social/direct_chats",
        "social",
        "social.directChats.create",
    ),
    HttpRoute::open_api_flexible(
        HttpMethod::Get,
        "/im/v3/api/social/direct_chats",
        "social",
        "social.directChats.list",
    ),
];

pub fn open_routes() -> Vec<HttpRoute> {
    SOCIAL_OPEN_API_ROUTES.to_vec()
}

pub fn open_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(SOCIAL_OPEN_API_ROUTES)
}
