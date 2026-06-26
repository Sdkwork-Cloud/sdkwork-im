pub const PREFIX: &str = "/im/v3/api/spaces";

pub const SPACES: &str = "/im/v3/api/spaces";
pub const SPACE: &str = "/im/v3/api/spaces/{space_id}";
pub const SPACE_MEMBERS: &str = "/im/v3/api/spaces/{space_id}/members";
pub const SPACE_MEMBER: &str = "/im/v3/api/spaces/{space_id}/members/{user_id}";
pub const SPACE_GROUPS: &str = "/im/v3/api/spaces/{space_id}/groups";
pub const SPACE_GROUP: &str = "/im/v3/api/spaces/{space_id}/groups/{group_id}";
pub const SPACE_GROUP_MEMBERS: &str = "/im/v3/api/spaces/{space_id}/groups/{group_id}/members";
pub const SPACE_GROUP_MEMBER: &str =
    "/im/v3/api/spaces/{space_id}/groups/{group_id}/members/{user_id}";
pub const SPACE_CHANNELS: &str = "/im/v3/api/spaces/{space_id}/channels";
pub const SPACE_CHANNEL: &str = "/im/v3/api/spaces/{space_id}/channels/{channel_id}";
pub const SPACE_CHANNEL_ACCESS_RULES: &str =
    "/im/v3/api/spaces/{space_id}/channels/{channel_id}/access_rules";
pub const SPACE_CHANNEL_ACCESS_RULE: &str =
    "/im/v3/api/spaces/{space_id}/channels/{channel_id}/access_rules/{rule_id}";
pub const SPACE_INVITES: &str = "/im/v3/api/spaces/{space_id}/invites";
pub const SPACE_INVITE: &str = "/im/v3/api/spaces/{space_id}/invites/{invite_code}";
pub const SPACE_INVITE_ACCEPT: &str = "/im/v3/api/spaces/{space_id}/invites/{invite_code}/accept";
pub const SPACE_BANS: &str = "/im/v3/api/spaces/{space_id}/bans";
pub const SPACE_BAN: &str = "/im/v3/api/spaces/{space_id}/bans/{user_id}";
