//! Contact Service — deprecated compatibility shim.
//!
//! Postgres supplemental handlers now live in `social-service`; route mounting lives in
//! `sdkwork-router-im-social-open-api`.

pub use social_service::{
    PostgresAppState, app_state_from_postgres_pool, try_postgres_app_state_from_database_url_env,
};
pub use sdkwork_router_im_social_open_api::{
    build_supplemental_app, build_supplemental_public_app,
};

pub type AppState = PostgresAppState;
