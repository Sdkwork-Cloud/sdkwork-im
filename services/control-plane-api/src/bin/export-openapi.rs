fn main() {
    let document = control_plane_api::export_openapi_document()
        .expect("control-plane openapi document should export");
    println!(
        "{}",
        serde_json::to_string_pretty(&document)
            .expect("control-plane openapi document should serialize"),
    );
}
