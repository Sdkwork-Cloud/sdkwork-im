//! Rust RPC service binding scaffold for the `sdkwork-im-rpc-sdk` family.

pub mod config;
pub mod deadline;
pub mod dispatcher;
pub mod error;
pub mod health;
pub mod metadata;
pub mod method_manifest;
pub mod service_binding;
pub mod service_manifest;
pub mod tonic_service_adapters;

pub use config::{ImRpcClientConfig, ImRpcServerConfig};
pub use deadline::RpcDeadline;
pub use dispatcher::{
    ImRpcBoxFuture, ImRpcBoxStream, ImRpcResponseStream, ImRpcRuntimeDispatcher,
    ImRpcStreamRequest, ImRpcStreamResponse, ImRpcUnaryRequest, ImRpcUnaryResponse,
    dispatch_server_stream_rpc, dispatch_unary_rpc,
};
pub use error::{ImRpcError, map_rpc_error_to_status};
pub use health::{ImRpcHealthService, build_im_rpc_health_server};
pub use metadata::{
    METADATA_ACCESS_TOKEN, METADATA_AUTHORIZATION, METADATA_CLIENT_VERSION,
    METADATA_IDEMPOTENCY_KEY, METADATA_REQUEST_HASH, METADATA_REQUEST_ID, METADATA_TRACEPARENT,
    RpcMetadata,
};
pub use method_manifest::RPC_METHOD_BINDINGS;
pub use service_binding::{
    RpcMethodBinding, RpcMethodRuntimeAdapter, RpcRuntimeAdapter, RpcServiceBinding,
    bind_all_rpc_methods, bind_all_rpc_services,
};
pub use service_manifest::{RPC_SDK_FAMILY, RPC_SERVICE_BINDINGS};
pub use tonic_service_adapters::*;
