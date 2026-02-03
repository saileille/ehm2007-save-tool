use tauri::{AppHandle, Manager as _};
use tauri_plugin_dialog::DialogExt as _;

use crate::{data::Data, init::load_bin};

#[tauri::command]
// Get the players in the save.
pub fn fetch_players(
        handle: AppHandle,
        headers: Vec<String>,
        nation_id: i32,
        national_team_check: bool,
        country_choice_check: bool,
        earliest_birth_year: i16,
        exclude_nhl: bool,
        exclude_na: bool) -> Vec<Vec<serde_json::Value>> {
    let data = handle.state::<Data>();

    let mut counter = 0;
    let players: Vec<Vec<serde_json::Value>> = data.staff.iter()
        .filter_map(|(_, person)| {
            let player = person.player_data(&data);
            if player.is_none() {
                return None;
            }

            let player = player.unwrap();
            if !person.check_player_filters(&data, nation_id, national_team_check, country_choice_check, earliest_birth_year, exclude_nhl, exclude_na) {
                return None
            }

            let row = person.create_player_view(player, &data, &headers, counter);
            counter += 1;
            return Some(row);
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
// Load a save file. Return false if user cancelled.
pub fn load_save(handle: AppHandle) -> bool {
    let filepath = match handle
        .dialog()
        .file()
        // .set_directory()
        .add_filter("EHM Save Files", &["sav"])
        .blocking_pick_file()
    {
        Some(p) => p,
        None => return false,
    };

    let data = load_bin(filepath.as_path().unwrap());
    let old_data = handle.try_state::<Data>();
    if old_data.is_none() {
        handle.manage(data);
    }
    else {
        // old_data.unwrap().update(data);
    }

    return true;
}

#[tauri::command]
// Get data needed to build the filters.
pub fn get_filter_data(handle: AppHandle) -> Vec<(i32, String)> {
    let data = handle.state::<Data>();

    let mut nations: Vec<(i32, String)> = data.nations.iter().map(|(id, nation)| (*id, nation.name())).collect();
    nations.sort_by(|a, b| a.1.cmp(&b.1));

    let mut all_options = Vec::from([
        (-2, "Any".to_string()),
        (-1, "N/A".to_string()),
    ]);

    all_options.append(&mut nations);
    return all_options;
}