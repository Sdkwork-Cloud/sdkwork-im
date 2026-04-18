use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PortalUserView {
    pub id: String,

    pub login: String,

    pub name: String,

    pub role: String,

    pub email: String,

    #[serde(rename = "actorKind")]
    pub actor_kind: String,

    #[serde(rename = "clientKind")]
    pub client_kind: String,

    pub permissions: Vec<String>,
}
