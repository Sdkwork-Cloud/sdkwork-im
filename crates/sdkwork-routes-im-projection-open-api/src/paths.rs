pub const PREFIX: &str = "/im/v3/api/chat";

pub const CONTACTS: &str = "/im/v3/api/chat/contacts";
pub const INBOX: &str = "/im/v3/api/chat/inbox";
pub const CONVERSATION: &str = "/im/v3/api/chat/conversations/{conversation_id}";
pub const CONVERSATION_READ_CURSOR: &str =
    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor";
pub const CONVERSATION_MEMBER_DIRECTORY: &str =
    "/im/v3/api/chat/conversations/{conversation_id}/member_directory";
pub const CONVERSATION_PINS: &str = "/im/v3/api/chat/conversations/{conversation_id}/pins";
pub const MESSAGE_INTERACTION_SUMMARY: &str =
    "/im/v3/api/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary";
pub const CONVERSATION_MESSAGES: &str =
    "/im/v3/api/chat/conversations/{conversation_id}/messages";
