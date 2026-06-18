//! Contact Service — deprecated compatibility shim.
//!
//! Postgres supplemental handlers now live in `social-service`.

pub use social_service::{
    PostgresAppState, app_state_from_postgres_pool, build_public_app_with_contact_extension,
    build_public_app_with_postgres_extension, build_supplemental_app,
    build_supplemental_public_app, try_postgres_app_state_from_database_url_env,
};

pub type AppState = PostgresAppState;
