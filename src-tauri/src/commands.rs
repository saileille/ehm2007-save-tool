use tauri::Manager as _;

use crate::data::Data;

#[tauri::command]
// Get the players in the save.
pub fn fetch_players(handle: tauri::AppHandle) -> Vec<serde_json::Value> {
    let data = handle.state::<Data>();

    let players: Vec<serde_json::Value> = data.staff.iter().filter_map(
        |(_, player)|
        player.create_player_view(&data)
    ).collect();

    return players;
}