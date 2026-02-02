use tauri::{AppHandle, Manager as _};
use tauri_plugin_dialog::DialogExt as _;

use crate::{data::Data, init::load_bin};

#[tauri::command]
// Get the players in the save.
pub fn fetch_players(handle: AppHandle) -> Vec<serde_json::Value> {
    let data = handle.state::<Data>();

    let mut counter = 0;
    let players: Vec<serde_json::Value> = data
        .staff
        .iter()
        .filter_map(|(_, player)| {
            let view = player.create_player_view(&data, counter);
            counter += 1;
            return view;
        })
        .collect();

    return players;
}

#[tauri::command]
// Load a save file.
pub fn load_save(handle: AppHandle) {
    let filepath = match handle
        .dialog()
        .file()
        // .set_directory()
        .add_filter("EHM Save Files", &["sav"])
        .blocking_pick_file()
    {
        Some(p) => p,
        None => return,
    };

    let data = load_bin(filepath.as_path().unwrap());
    handle.manage(data);
}
