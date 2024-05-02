// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cropper;
mod editor;
mod recorder;
mod tray;

use scap::{capturer::Capturer, frame::Frame};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut;
use tauri_plugin_store::StoreBuilder;
use tokio::sync::Mutex;
use tauri_plugin_global_shortcut::ShortcutState;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

pub struct AppState {
    frames: Mutex<Vec<Frame>>,
    recorder: Mutex<Option<Capturer>>,
    cropped_area: Mutex<Vec<u32>>,
    preview_path: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            frames: Mutex::new(Vec::new()),
            recorder: Mutex::new(None),
            cropped_area: Mutex::new(Vec::new()),
            preview_path: Mutex::new(None),
        }
    }
}

const SHORTCUT: &str = "CmdOrCtrl+Shift+2";

fn initialize_micro(app_handle: &AppHandle) {
    // Build system tray
    tray::build(&app_handle);

    // Initialize cropping window
    cropper::init_cropper(&app_handle);

    // Register global shortcut
    app_handle
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
        .expect("Failed to initialize global shortcut");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
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

            // If this is the first run, show onboarding screen
            if first_run || !scap::has_permission() {
                open_onboarding(app_handle);
                store.insert("first_run".to_string(), false.into()).unwrap();
                store.save().expect("Failed to save store")
            }

            initialize_micro(app_handle);

            let _ = tray::check_for_update(app_handle.clone(), true);

            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            tray::is_ok_sharing_usage_data,
            editor::export_gif,
            cropper::update_crop_area,
            recorder::stop_recording,
            recorder::start_recording,
            recorder::request_recording_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Helmer Micro");
}

fn open_onboarding(app_handle: &AppHandle) {
    match app_handle.get_webview_window("onboarding") {
        Some(window) => {
            if window.is_visible().unwrap() {
                window.set_focus().unwrap();
            }
        }
        None => create_onboarding_win(app_handle),
    }
}

fn create_onboarding_win(app_handle: &AppHandle) {
    let mut onboarding_win =
        WebviewWindowBuilder::new(app_handle, "onboarding", WebviewUrl::App("/".into()))
            .accept_first_mouse(true)
            .inner_size(600.0, 580.0)
            .title("Helmer Micro")
            .visible(true)
            .focused(true)
            .center();
    #[cfg(target_os = "macos")]
    {
        onboarding_win = onboarding_win.title_bar_style(tauri::TitleBarStyle::Overlay);
    }
    onboarding_win.build().expect("Failed to open onboarding");
}
