use serde::{Deserialize, Serialize};

use crate::models::{TimelineViewEntry};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TimelineResponse {
    pub items: Vec<TimelineViewEntry>,
}
