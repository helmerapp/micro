// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capturer;
mod cropper;
mod editor;
mod toolbar;
mod tray;

use scap::{capturer::Capturer, frame::Frame};
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_global_shortcut;
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
    recorder: Mutex<Option<Capturer>>,
    preview_path: Mutex<Option<PathBuf>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cropped_area: Mutex::new(Vec::new()),
            status: Mutex::new(Status::Idle),
            frames: Mutex::new(Vec::new()),
            recorder: Mutex::new(None),
            preview_path: Mutex::new(None),
        }
    }
}

const SHORTCUT: &str = "CmdOrCtrl+Shift+2";

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

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcut(SHORTCUT)?
                    .with_handler(|app, shortcut| {
                        cropper::toggle_cropper(app);
                        println!("Shortcut pressed: {:?}", shortcut);
                    })
                    .build(),
            )?;

            let app_handle = app.app_handle();
            tray::build(&app_handle);
            tray::check_for_updates(&app_handle);
            cropper::init_cropper(&app_handle);

            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            capturer::start_capture,
            capturer::stop_capture,
            editor::export_handler,
            toolbar::show_toolbar,
            toolbar::hide_toolbar
        ])
        .run(tauri::generate_context!())
        .expect("error while running Helmer Micro");
}
