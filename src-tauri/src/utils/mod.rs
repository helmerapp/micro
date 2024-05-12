#[cfg(target_os = "linux")]
use std::fs::metadata;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
use std::process::Command;

// courtesy of https://github.com/nwesterhausen/overseers-manual-df/blob/4d404fed7023254d4c495be2de2734b95adbacc2/src-tauri/src/open_explorer.rs
#[tauri::command]
/// Opens the file explorer or finder at the specified path depending on the operating system.
///
/// Arguments:
///
/// * `path`: The `path` parameter is a string that represents the file or folder path that you want to
/// show in the folder.
pub async fn open_file_location(path: String) {
    #[cfg(target_os = "windows")]
    {
        match Command::new("explorer.exe")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
        {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(',') {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        match Command::new("open").args(["-R", &path]).spawn() {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
    }
}