use tauri::{AppHandle, Manager as _};
use tauri_plugin_dialog::DialogExt as _;

use crate::{data::Data, init::load_bin};

#[tauri::command]
// Get the players in the save.
pub fn fetch_players(handle: AppHandle, headers: Vec<String>) -> Vec<Vec<serde_json::Value>> {
    let data = handle.state::<Data>();

    let mut counter = 0;
    let players: Vec<Vec<serde_json::Value>> = data
        .staff
        .iter()
        .filter_map(|(_, player)| {
            let row = player.create_player_view(&data, &headers, counter);
            counter += 1;
            return row;
        })
        .collect();

    return players;
}

#[tauri::command]
// Get the possible ingame dates.
pub fn get_ingame_date(handle: AppHandle) -> [usize; 2] {
    let date_range = &handle.state::<Data>().date_range;
    return [
        date_range[0].to_days(),
        date_range[1].to_days(),
    ];
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

#[tauri::command]
// Get data needed to build the filters.
pub fn get_filter_data(handle: AppHandle) -> Vec<(i32, String)> {
    let data = handle.state::<Data>();
    let mut nations: Vec<(i32, String)> = data.nations.iter().map(|(id, nation)| (*id, nation.name())).collect();

    nations.sort_by(|a, b| a.1.cmp(&b.1));
    return nations;
}