use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiError {
    pub code: String,

    pub message: String,
}
