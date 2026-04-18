use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConversationActorView {
    pub id: String,

    pub kind: String,
}
