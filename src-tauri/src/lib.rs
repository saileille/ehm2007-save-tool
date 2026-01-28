// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod attr_chart;
mod commands;
mod data;
mod init;
mod research;
mod to_bytes;

use tauri::Manager as _;

use crate::{init::load_bin, research::load_databases};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Research stuff here...
            // load_databases("D:/Programs/NHL Eastside Hockey Manager 2007/data/database");

            let handle = app.handle();
            let data = load_bin("C:/Users/Aleksi/Documents/Sports Interactive/EHM 2007/games/suomi.sav");
            handle.manage(data);

            #[cfg(debug_assertions)] {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::fetch_players
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
