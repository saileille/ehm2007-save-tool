use std::sync::Mutex;

use tauri::{AppHandle, Manager as _};
use tauri_plugin_dialog::DialogExt as _;

use crate::{data::Data, init::load_bin, views};

#[tauri::command]
// Get the players in the save.
pub fn fetch_players(
    handle: AppHandle,
    headers: Vec<String>,
    birth_years: [i16; 2],
    national_team_check: Option<bool>,
    second_nationality_check: Option<bool>,
    declared_check: Option<bool>,
    include_nationalities: Vec<i32>,
    exclude_nationalities: Vec<i32>,
    include_clubs_contracted: Vec<i32>,
    exclude_clubs_contracted: Vec<i32>,
    include_clubs_playing: Vec<i32>,
    exclude_clubs_playing: Vec<i32>,
    include_comps_contracted: Vec<i32>,
    exclude_comps_contracted: Vec<i32>,
    include_comps_playing: Vec<i32>,
    exclude_comps_playing: Vec<i32>,
    include_nations_contracted: Vec<i32>,
    exclude_nations_contracted: Vec<i32>,
    include_nations_playing: Vec<i32>,
    exclude_nations_playing: Vec<i32>,
) -> Vec<views::player::Player> {
    let mutex = handle.state::<Mutex<Data>>();
    let data = mutex.lock().unwrap();

    let mut counter = 0;
    let players: Vec<views::player::Player> = data.staff.iter()
        .filter_map(|(_, person)| {
            let player = person.player_data(&data);
            if player.is_none() {
                return None;
            }

            let player = player.unwrap();
            if !person.check_player_filters(
                &data,
                birth_years,
                national_team_check,
                second_nationality_check,
                declared_check,
                &include_nationalities,
                &exclude_nationalities,
                &include_clubs_contracted,
                &exclude_clubs_contracted,
                &include_clubs_playing,
                &exclude_clubs_playing,
                &include_comps_contracted,
                &exclude_comps_contracted,
                &include_comps_playing,
                &exclude_comps_playing,
                &include_nations_contracted,
                &exclude_nations_contracted,
                &include_nations_playing,
                &exclude_nations_playing,
            ) {
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
    let data = handle.state::<Mutex<Data>>();
    let date_range = &data.lock().unwrap().date_range;
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
    let old_data = handle.try_state::<Mutex<Data>>();
    if old_data.is_none() {
        handle.manage(Mutex::new(data));
    }
    else {
        let old_data = old_data.unwrap();
        *old_data.lock().unwrap() = data;
    }

    return true;
}

#[tauri::command]
// Get all nations and their IDs.
pub fn get_nations(handle: AppHandle) -> Vec<(i32, String)> {
    let data = handle.state::<Mutex<Data>>();

    let mut nations: Vec<(i32, String)> = data.lock().unwrap().nations.iter().map(|(id, nation)| (*id, nation.name())).collect();
    nations.sort_by(|a, b| a.1.cmp(&b.1));

    nations.push((-1, "N/A".to_string()));
    return nations;
}

#[tauri::command]
// Get all clubs and their IDs.
pub fn get_clubs(handle: AppHandle) -> Vec<(i32, String)> {
    let data = handle.state::<Mutex<Data>>();

    let mut clubs: Vec<(i32, String)> = data.lock().unwrap().clubs.iter().map(|(id, club)| (*id, club.name().unwrap())).collect();
    clubs.sort_by(|a, b| a.1.cmp(&b.1));

    clubs.push((-1, "No Club".to_string()));
    return clubs;
}

#[tauri::command]
// Get all competitions and their IDs.
pub fn get_comps(handle: AppHandle) -> Vec<(i32, String)> {
    let data = handle.state::<Mutex<Data>>();

    let mut comps: Vec<(i32, String)> = data.lock().unwrap().competitions.iter().map(|(id, comp)| (*id, comp.name())).collect();
    comps.sort_by(|a, b| a.1.cmp(&b.1));

    comps.push((-1, "No Competition".to_string()));
    return comps;
}