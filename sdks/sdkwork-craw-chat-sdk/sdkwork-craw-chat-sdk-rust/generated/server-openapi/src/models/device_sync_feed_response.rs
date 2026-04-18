use serde::{Deserialize, Serialize};

use crate::models::{DeviceSyncFeedEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeviceSyncFeedResponse {
    pub items: Vec<DeviceSyncFeedEntry>,
}
