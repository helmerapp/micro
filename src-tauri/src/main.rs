// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cropper;
mod editor;
mod recorder;
mod tray;

use scap::{capturer::Capturer, frame::Frame, Target};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut::ShortcutState;
use tauri_plugin_store::StoreBuilder;
use tokio::sync::Mutex;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

use tauri_plugin_decorum::WebviewWindowExt;

pub struct AppState {
    frames: Mutex<Vec<Frame>>,
    recorder: Mutex<Option<Capturer>>,
    cropped_area: Mutex<Vec<u32>>,
    preview_path: Mutex<Option<PathBuf>>,
    targets: Vec<Target>,
    current_target: Mutex<Option<scap::Target>>,

    #[cfg(target_os = "macos")]
    shown_permission_prompt: Mutex<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            frames: Mutex::new(Vec::new()),
            recorder: Mutex::new(None),
            cropped_area: Mutex::new(Vec::new()),
            preview_path: Mutex::new(None),
            targets: scap::get_all_targets(),
            // Negative value means signifies no current target.
            current_target: Mutex::new(Option::None),

            #[cfg(target_os = "macos")]
            shown_permission_prompt: Mutex::new(false),
        }
    }
}

const SHORTCUT: &str = "CmdOrCtrl+Shift+2";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(SHORTCUT)
                .expect("Failed to register global shortcut")
                .with_handler(|app, _, event| {
                    if event.state == ShortcutState::Pressed {
                        cropper::toggle_cropper(app);
                    }
                })
                .build(),
        )
        .setup(|app| {
            // Set activation policy to Accessory on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            let mut store = StoreBuilder::new("app_data.bin").build(app.handle().clone());
            store.load().unwrap_or_default();

            // let first_run = true;
            let first_run = store
                .get("first_run".to_string())
                .unwrap_or(&serde_json::Value::Bool(true))
                .as_bool()
                .unwrap();

            // If this is the first run, show welcome screen
            if first_run || !scap::has_permission() {
                open_welcome_window(app_handle);
                store.insert("first_run".to_string(), false.into()).unwrap();
                store.save().expect("Failed to save store")
            }

            tray::build(&app_handle);
            cropper::init_cropper(&app_handle);

            tray::check_for_update(app_handle.clone(), true).expect("Failed to check for update");

            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            tray::is_ok_sharing_usage_data,
            editor::export_gif,
            cropper::hide_cropper,
            cropper::update_crop_area,
            recorder::stop_recording,
            recorder::start_recording,
            recorder::request_recording_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Helmer Micro");
}

fn open_welcome_window(app_handle: &AppHandle) {
    match app_handle.get_webview_window("welcome") {
        Some(window) => {
            if window.is_visible().unwrap() {
                window.set_focus().unwrap();
            }
        }
        None => create_welcome_win(app_handle),
    }
}

fn create_welcome_win(app_handle: &AppHandle) {
    let mut welcome_win =
        WebviewWindowBuilder::new(app_handle, "welcome", WebviewUrl::App("/".into()))
            .accept_first_mouse(true)
            .inner_size(600.0, 580.0)
            .title("Helmer Micro")
            .visible(false)
            .focused(true)
            .center();
    #[cfg(target_os = "macos")]
    {
        welcome_win = welcome_win
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .hidden_title(true);
    }
    let welcome_win = welcome_win.build().expect("Failed to build welcome window");

    welcome_win.create_overlay_titlebar().unwrap();
    welcome_win.show().expect("Failed to show welcome window");
}
