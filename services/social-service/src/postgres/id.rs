//! Snowflake entity id generation for Postgres-backed social handlers.

use std::sync::Arc;

use axum::http::StatusCode;
use im_platform_contracts::IdGenerator;
use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;

pub fn build_runtime_id_generator() -> Arc<dyn IdGenerator> {
    match RuntimeSnowflakeIdGenerator::from_env() {
        Ok(generator) => Arc::new(generator),
        Err(error) => {
            tracing::warn!(
                ?error,
                "SDKWORK_IM_ID_NODE_ID missing; using snowflake node 0 for social-service postgres bootstrap"
            );
            Arc::new(
                RuntimeSnowflakeIdGenerator::with_node_id(0)
                    .expect("snowflake node 0 must initialize"),
            )
        }
    }
}

pub fn next_entity_id(id_generator: &Arc<dyn IdGenerator>) -> Result<i64, StatusCode> {
    id_generator.next_id().map_err(|error| {
        tracing::error!(?error, "social-service snowflake id generation failed");
        StatusCode::SERVICE_UNAVAILABLE
    })
}
