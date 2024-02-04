use serde::{Deserialize, Serialize};
use tauri::{App, AppHandle, Manager, PhysicalSize, Position, Size, WindowBuilder, WindowUrl};

pub fn init_toolbar(app: &AppHandle) {
    println!("toolbar created");
    let mut toolbar_win = WindowBuilder::new(app, "toolbar", WindowUrl::App("/toolbar".into()))
        .always_on_top(true)
        .decorations(false)
        .resizable(false)
        .visible(false)
        .focused(true)
        .center();


    #[cfg(not(target_os = "macos"))]{
        toolbar_win = toolbar_win.transparent(true);
    }
    

    let toolbar_win = toolbar_win.build().expect("Failed to open toolbar");

    let monitor = toolbar_win.primary_monitor().unwrap().unwrap();

    let size = Size::Physical(PhysicalSize {
        width: 150,
        height: 70,
    });

    toolbar_win.set_size(size).unwrap();
}

// create a toggle_toolbar function
pub fn toggle_toolbar(app: &AppHandle) {
    let toolbar_win = app.get_window("toolbar").unwrap();
    if toolbar_win.is_visible().unwrap() {
        toolbar_win.hide().unwrap();
    } else {
        toolbar_win.show().unwrap();
    }
}

#[tauri::command]
pub async fn show_toolbar(button_coords: Vec<u32>, area: Vec<u32>, app: AppHandle) {
    println!("show_toolbar here!");
    if app.get_window("toolbar").is_none() {
        crate::toolbar::init_toolbar(&app);
    }
    println!("Coordinates {:?}", button_coords);
    let toolbar_win = app.get_window("toolbar").unwrap();
    let pos = Position::Physical((button_coords[0], button_coords[1]).into());
    toolbar_win.set_position(pos).unwrap();
    toolbar_win.show().unwrap();
    toolbar_win.set_focus().unwrap();
    toolbar_win.emit("capturing_area", area).unwrap();
}

#[tauri::command]
pub async fn hide_toolbar(app: AppHandle) {
    println!("hide_toolbar here!");
    let toolbar_win = app.get_window("toolbar").unwrap();
    toolbar_win.hide().unwrap();
}