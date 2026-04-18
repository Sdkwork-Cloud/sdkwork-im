use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SummarySenderView {
    pub id: String,

    pub kind: String,
}
