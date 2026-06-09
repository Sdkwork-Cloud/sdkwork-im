mod qr_code;
mod tray;
mod window_control;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tray::ensure_tray(app.handle()).map_err(Box::<dyn std::error::Error>::from)?;
            Ok(())
        })
        .on_window_event(tray::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            window_control::sdkwork_chat_pc_window_control,
            qr_code::sdkwork_chat_pc_decode_qr_code_image,
            qr_code::sdkwork_chat_pc_decode_qr_code_rgba
        ])
        .run(tauri::generate_context!())
        .expect("failed to run SDKWork Chat PC");
}
