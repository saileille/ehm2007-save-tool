// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod chars;
mod globals;
mod commands;
mod data;
mod init;
mod rating;
mod research;
mod to_bytes;
mod views;

use tauri::Manager as _;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Research stuff here...
            // load_databases("D:/Programs/NHL Eastside Hockey Manager 2007/data/database");
            // let data = load_debug_bin("C:/Users/Aleksi/Documents/Sports Interactive/EHM 2007/games/test.sav");

            // let handle = app.handle();
            // let data = load_bin(Path::new("C:/Users/Aleksi/Documents/Sports Interactive/EHM 2007/games/test.sav"));
            // handle.manage(data);

            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_save,
            commands::fetch_players,
            commands::get_ingame_date,
            commands::get_filter_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
