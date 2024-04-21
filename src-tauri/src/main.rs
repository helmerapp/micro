// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cropper;
mod editor;
mod recorder;
mod toolbar;
mod tray;

use scap::{capturer::Capturer, frame::Frame};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_global_shortcut;
use tauri_plugin_store::StoreBuilder;
use tokio::sync::Mutex;

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

#[derive(Debug, PartialEq)]
pub enum Status {
    Idle,
    Cropper,
    Recording,
    Editing,
}

pub struct AppState {
    cropped_area: Mutex<Vec<u32>>,
    status: Mutex<Status>,
    frames: Mutex<Vec<Frame>>,
    capturer: Mutex<Option<Capturer>>,
    preview_path: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cropped_area: Mutex::new(Vec::new()),
            status: Mutex::new(Status::Idle),
            frames: Mutex::new(Vec::new()),
            capturer: Mutex::new(None),
            preview_path: Mutex::new(None),
        }
    }
}

const SHORTCUT: &str = "CmdOrCtrl+Shift+2";

fn initialize_micro(app_handle: &AppHandle) {
    // Register global shortcut
    app_handle
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(SHORTCUT)
                .expect("Failed to register global shortcut")
                .with_handler(|app, _| {
                    cropper::toggle_cropper(app);
                })
                .build(),
        )
        .expect("Failed to initialize global shortcut");

    // Build system tray
    tray::build(&app_handle);

    // Initialize cropping window
    cropper::init_cropper(&app_handle);
}

fn main() {
    // Set up Tauri Plugins
    let tp_store = tauri_plugin_store::Builder::default().build();
    let tp_single_instance = tauri_plugin_single_instance::init(|_, _, _| {});
    tauri::Builder::default()
        .plugin(tp_store)
        .plugin(tp_single_instance)
        .plugin(tauri_plugin_store::Builder::default().build())
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

            let recording_permission: bool = scap::has_permission();

            // Check if this is the first run or if the screen recording permission is not set
            if first_run || !recording_permission {
                // Show onboarding screen
                let mut onboarding_win = WebviewWindowBuilder::new(
                    app_handle,
                    "onboarding",
                    WebviewUrl::App("/".into()),
                )
                .accept_first_mouse(true)
                .always_on_top(true)
                .title("Helmer Micro")
                .inner_size(600.0, 600.0)
                .visible(true)
                .focused(true)
                .center();

                #[cfg(target_os = "macos")]
                {
                    onboarding_win = onboarding_win.title_bar_style(tauri::TitleBarStyle::Overlay);
                }

                onboarding_win.build().expect("Failed to open onboarding");

                // Set first run to false
                store.insert("first_run".to_string(), false.into()).unwrap();

                store.save();
            }

            initialize_micro(app_handle);

            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            recorder::start_recording,
            recorder::stop_recording,
            editor::export_handler,
            toolbar::show_toolbar,
            toolbar::hide_toolbar
        ])
        .run(tauri::generate_context!())
        .expect("error while running Helmer Micro");
}
