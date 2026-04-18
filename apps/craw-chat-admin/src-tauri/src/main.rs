#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Context;
use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    ProductSiteDirs, RouterProductRuntime, RouterProductRuntimeOptions,
};
use tauri::{path::BaseDirectory, AppHandle, Manager};

mod api_key_setup;

#[derive(Clone)]
struct RuntimeState {
    base_url: String,
}

struct RuntimeHandleState {
    _runtime: Mutex<Option<RouterProductRuntime>>,
}

#[tauri::command]
async fn runtime_base_url(state: tauri::State<'_, RuntimeState>) -> Result<String, String> {
    Ok(state.base_url.clone())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let runtime =
                tauri::async_runtime::block_on(start_desktop_runtime(app.handle().clone()))
                    .map_err(box_setup_error)?;
            let base_url = runtime
                .public_base_url()
                .context("desktop runtime did not expose a public base url")
                .map_err(box_setup_error)?
                .to_owned();
            app.manage(RuntimeState { base_url });
            app.manage(RuntimeHandleState {
                _runtime: Mutex::new(Some(runtime)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            runtime_base_url,
            api_key_setup::install_api_router_client_setup,
            api_key_setup::list_api_key_instances
        ])
        .run(tauri::generate_context!())
        .expect("failed to run tauri application");
}

fn box_setup_error(error: anyhow::Error) -> Box<dyn std::error::Error> {
    Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        error.to_string(),
    ))
}

async fn start_desktop_runtime(app: AppHandle) -> anyhow::Result<RouterProductRuntime> {
    let (loader, config) = StandaloneConfigLoader::from_env()?;
    RouterProductRuntime::start(
        loader,
        config,
        RouterProductRuntimeOptions::desktop(resolve_desktop_site_dirs(&app)?),
    )
    .await
}

fn resolve_desktop_site_dirs(app: &AppHandle) -> anyhow::Result<ProductSiteDirs> {
    let workspace_dirs = workspace_site_dirs();
    Ok(ProductSiteDirs::new(
        resolve_resource_or_fallback(app, "embedded-sites/admin", workspace_dirs.admin_site_dir)?,
        resolve_resource_or_fallback(app, "embedded-sites/portal", workspace_dirs.portal_site_dir)?,
    ))
}

fn resolve_resource_or_fallback(
    app: &AppHandle,
    resource_path: &str,
    fallback: PathBuf,
) -> anyhow::Result<PathBuf> {
    let embedded_resource_dir = app
        .path()
        .resolve(resource_path, BaseDirectory::Resource)
        .ok()
        .filter(|resource_dir| resource_dir.is_dir());

    choose_site_dir_for_runtime(
        embedded_resource_dir,
        fallback,
        resource_path,
        cfg!(debug_assertions),
    )
}

fn workspace_site_dirs() -> ProductSiteDirs {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let apps_root = manifest_dir
        .parent()
        .expect("admin src-tauri must live inside the admin app")
        .parent()
        .expect("admin app must live inside the apps directory");
    ProductSiteDirs::new(
        apps_root.join("craw-chat-admin").join("dist"),
        apps_root.join("craw-chat-portal").join("dist"),
    )
}

fn choose_site_dir_for_runtime(
    embedded_resource_dir: Option<PathBuf>,
    fallback: PathBuf,
    resource_path: &str,
    allow_workspace_fallback: bool,
) -> anyhow::Result<PathBuf> {
    if let Some(embedded_resource_dir) = embedded_resource_dir {
        return Ok(embedded_resource_dir);
    }

    if allow_workspace_fallback {
        return Ok(fallback);
    }

    anyhow::bail!(
        "desktop bundled site resource is missing: {resource_path}. Release builds must embed the compiled site in tauri bundle.resources before startup."
    );
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use std::path::PathBuf;

    use super::{box_setup_error, choose_site_dir_for_runtime};

    #[test]
    fn box_setup_error_preserves_context_message() {
        let error = box_setup_error(anyhow!("desktop runtime boot failed"));
        assert_eq!(error.to_string(), "desktop runtime boot failed");
    }

    #[test]
    fn choose_site_dir_for_runtime_prefers_embedded_resource_dir() {
        let embedded = PathBuf::from("embedded-sites/admin");
        let fallback = PathBuf::from("apps/craw-chat-admin/dist");

        let resolved = choose_site_dir_for_runtime(
            Some(embedded.clone()),
            fallback,
            "embedded-sites/admin",
            false,
        )
        .expect("embedded resource dir should be preferred");

        assert_eq!(resolved, embedded);
    }

    #[test]
    fn choose_site_dir_for_runtime_allows_workspace_fallback_in_debug_style_contexts() {
        let fallback = PathBuf::from("apps/craw-chat-admin/dist");

        let resolved =
            choose_site_dir_for_runtime(None, fallback.clone(), "embedded-sites/admin", true)
                .expect("workspace fallback should remain available during local debug runs");

        assert_eq!(resolved, fallback);
    }

    #[test]
    fn choose_site_dir_for_runtime_rejects_release_without_embedded_resources() {
        let error = choose_site_dir_for_runtime(
            None,
            PathBuf::from("apps/craw-chat-admin/dist"),
            "embedded-sites/admin",
            false,
        )
        .expect_err("release runtime should fail fast when embedded site resources are missing");

        assert!(error.to_string().contains("embedded-sites/admin"));
        assert!(error.to_string().contains("bundle.resources"));
    }
}
