use std::sync::atomic::{AtomicBool, Ordering};

use serde::Deserialize;
use tauri::{
    menu::MenuBuilder,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime, Window, WindowEvent,
};

const MAIN_WINDOW_LABEL: &str = "main";
const TRAY_ICON_ID: &str = "sdkwork_chat_pc_main_tray";
const TRAY_MENU_CHAT_ID: &str = "sdkwork_chat_pc_tray_chat";
const TRAY_MENU_SETTINGS_ID: &str = "sdkwork_chat_pc_tray_settings";
const TRAY_MENU_QUIT_ID: &str = "sdkwork_chat_pc_tray_quit";
const TRAY_EVENT_OPEN_CHAT: &str = "sdkwork-chat-pc://tray/open-chat";
const TRAY_EVENT_OPEN_SETTINGS: &str = "sdkwork-chat-pc://tray/open-settings";

static IS_EXITING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum SdkworkChatPcWindowControlAction {
    Minimize,
    ToggleMaximize,
    CloseToTray,
    Show,
    StartDragging,
}

fn main_window<R: Runtime>(app: &AppHandle<R>) -> Result<tauri::WebviewWindow<R>, String> {
    app.get_webview_window(MAIN_WINDOW_LABEL)
        .ok_or_else(|| "main window is unavailable".to_string())
}

fn show_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let window = main_window(app)?;
    let _ = window.unminimize();
    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())?;
    Ok(())
}

fn hide_main_window_to_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    ensure_tray(app)?;
    main_window(app)?.hide().map_err(|error| error.to_string())
}

fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or_else(|| "default window icon is unavailable".to_string())?;
    let menu = MenuBuilder::new(app)
        .text(TRAY_MENU_CHAT_ID, "聊天")
        .text(TRAY_MENU_SETTINGS_ID, "设置")
        .separator()
        .text(TRAY_MENU_QUIT_ID, "退出")
        .build()
        .map_err(|error| error.to_string())?;

    TrayIconBuilder::with_id(TRAY_ICON_ID)
        .menu(&menu)
        .icon(icon)
        .tooltip("SDKWork Chat PC")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| handle_tray_menu_event(app, event))
        .on_tray_icon_event(|tray, event| handle_tray_icon_event(tray.app_handle(), event))
        .build(app)
        .map_err(|error| error.to_string())?;

    Ok(())
}

fn ensure_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    if app.tray_by_id(TRAY_ICON_ID).is_some() {
        return Ok(());
    }

    create_tray(app)
}

fn handle_tray_icon_event<R: Runtime>(app: &AppHandle<R>, event: TrayIconEvent) {
    if matches!(
        event,
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        }
    ) {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_CHAT, ());
    }
}

fn handle_tray_menu_event<R: Runtime>(app: &AppHandle<R>, event: tauri::menu::MenuEvent) {
    if event.id() == TRAY_MENU_CHAT_ID {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_CHAT, ());
    } else if event.id() == TRAY_MENU_SETTINGS_ID {
        let _ = show_main_window(app);
        let _ = app.emit(TRAY_EVENT_OPEN_SETTINGS, ());
    } else if event.id() == TRAY_MENU_QUIT_ID {
        quit_app(app);
    }
}

fn quit_app<R: Runtime>(app: &AppHandle<R>) {
    IS_EXITING.store(true, Ordering::SeqCst);
    if let Ok(window) = main_window(app) {
        let _ = window.close();
    }
    app.exit(0);
}

fn handle_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    if window.label() != MAIN_WINDOW_LABEL {
        return;
    }

    if let WindowEvent::CloseRequested { api, .. } = event {
        if IS_EXITING.load(Ordering::SeqCst) {
            return;
        }

        api.prevent_close();
        let app = window.app_handle();
        if ensure_tray(&app).is_ok() {
            let _ = window.hide();
        }
    }
}

#[tauri::command]
fn sdkwork_chat_pc_window_control<R: Runtime>(
    app: AppHandle<R>,
    action: SdkworkChatPcWindowControlAction,
) -> Result<(), String> {
    let window = main_window(&app)?;

    match action {
        SdkworkChatPcWindowControlAction::Minimize => {
            window.minimize().map_err(|error| error.to_string())
        }
        SdkworkChatPcWindowControlAction::ToggleMaximize => {
            if window.is_maximized().map_err(|error| error.to_string())? {
                window.unmaximize().map_err(|error| error.to_string())
            } else {
                window.maximize().map_err(|error| error.to_string())
            }
        }
        SdkworkChatPcWindowControlAction::CloseToTray => hide_main_window_to_tray(&app),
        SdkworkChatPcWindowControlAction::Show => show_main_window(&app),
        SdkworkChatPcWindowControlAction::StartDragging => {
            window.start_dragging().map_err(|error| error.to_string())
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            ensure_tray(app.handle()).map_err(Box::<dyn std::error::Error>::from)?;
            Ok(())
        })
        .on_window_event(handle_window_event)
        .invoke_handler(tauri::generate_handler![sdkwork_chat_pc_window_control])
        .run(tauri::generate_context!())
        .expect("failed to run SDKWork Chat PC");
}
