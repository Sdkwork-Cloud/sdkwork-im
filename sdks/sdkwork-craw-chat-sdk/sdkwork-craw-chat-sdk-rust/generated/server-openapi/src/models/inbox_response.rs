use serde::{Deserialize, Serialize};

use crate::models::{ConversationInboxEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InboxResponse {
    pub items: Vec<ConversationInboxEntry>,
}
