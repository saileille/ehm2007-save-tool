// Code about attribute research goes here.

pub mod db;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Cursor, Read as _, Write as _},
    path::{Path, PathBuf},
};

use crate::{
    data::{Data, staff::Staff}, globals::PARSER_GUIDE, init::{load_save, parse_file}
};

type _AttributeData = HashMap<i16, HashMap<i8, HashMap<i8, usize>>>;

pub fn _load_databases(folder_name: &str) {
    let mut attr_data = HashMap::new();

    let folder = Path::new(folder_name);

    for db_folder in fs::read_dir(folder).unwrap() {
        let mut path_buf = db_folder.unwrap().path();
        if path_buf.is_file() {
            continue;
        }

        let db_attr = _load_player_data(&mut path_buf);
        _add_attr(&mut attr_data, db_attr);
    }

    let folder = Path::new(
        "D:/Programs/NHL Eastside Hockey Manager 2007/tools/Own Projects/attribute conversion",
    );
    _write_attr_research(folder, "_research", &attr_data);
}

fn _load_player_data(folder: &mut PathBuf) -> _AttributeData {
    folder.push("test.sav");
    let folder_name = folder.to_str().unwrap();
    let file = match File::open(folder_name) {
        Ok(f) => f,
        Err(e) => {
            panic!("{folder_name} - {e}");
        }
    };
    let save = load_save(file);

    // Remove the save file from the buffer.
    folder.pop();

    let db = _load_database(folder);

    let attr_data = _add_to_research(&save, &db);

    _write_attr_research(folder, "_research.txt", &attr_data);

    return attr_data;
}

fn _load_database(folder: &mut PathBuf) -> Data {
    let mut db_data = Data::default();
    for (filename, parser) in PARSER_GUIDE.iter() {
        folder.push(filename.as_str());

        let mut file = match File::open(folder.to_str().unwrap()) {
            Ok(f) => f,
            Err(e) => {
                panic!("{} - {}", folder.to_str().unwrap(), e);
            }
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let file_size = buffer.len() as u64;
        let mut cursor = Cursor::new(buffer);

        parse_file(
            &mut cursor,
            &parser,
            &mut db_data,
            file_size,
            filename.as_str(),
        );

        // Remove the previous filename from the buffer.
        folder.pop();
    }

    return db_data;
}

fn _add_to_research(
    save: &Data,
    db: &Data,
) -> _AttributeData {
    let db_staff = _get_staff_hash(&db);
    let mut save_staff = _get_staff_hash(save);

    let mut attr_data = HashMap::new();

    let players: Vec<db::_Player> = db_staff
        .iter()
        .filter_map(|(id, db_person)| db_person._merge_players(&db, &save, &id, &mut save_staff))
        .collect();

    // let mut path_buf = folder.to_path_buf();
    // path_buf.push("_research.csv");

    // let mut rowlist = vec![String::from(
    //     "Current Ability;Forename;Surname;Age;Birthplace;Nation;Second Nation;Club;Anticipation;In DB;Balance;In DB;Decisions;In DB;Off the Puck;In DB;One-on-Ones;In DB;Passing;In DB;Positioning;In DB;Reflexes;In DB;Creativity;In DB;Blocker;In DB;Checking;In DB;Deflections;In DB;Deking;In DB;Faceoffs;In DB;Fighting;In DB;Glove;In DB;Hitting;In DB;Pokecheck;In DB;Rebound Control;In DB;Recovery;In DB;Slapshot;In DB;Stickhandling;In DB;Wristshot;In DB"
    // )];

    for player in players {
        // rowlist.push(player.create_csv_row());
        player._add_to_attr_data(&mut attr_data);
    }

    // let string = rowlist.join("\n");

    // let mut file = File::create(path_buf).unwrap();
    // file.write_all(string.as_bytes()).unwrap();

    return attr_data;
}

fn _get_staff_hash(data: &Data) -> HashMap<[String; 6], Staff> {
    let mut staff = HashMap::new();

    for (_, person) in data.staff.iter() {
        if person.player_data(data).is_none() {
            continue;
        }

        let key = [
            person.forename(data),
            person.surname(data),
            person.birthplace(data),
            person.nation_name(data),
            person.second_nation_name(data),
            person.club_contracted_name(data).unwrap(),
        ];

        // Remove the entry entirely in case of duplicates.
        let old = staff.insert(key.clone(), person.clone());
        if old.is_some() {
            staff.remove(&key);
        }
    }

    return staff;
}

// Write a text and JSON file about the database findings.
fn _write_attr_research(folder: &Path, filename: &str, attr_data: &_AttributeData) {
    let mut path_buf = folder.to_path_buf();
    path_buf.push(format!("{filename}.json"));
    let mut file = File::create(&path_buf).unwrap();

    let attr_data_json = serde_json::to_string(attr_data).unwrap();
    file.write_all(attr_data_json.as_bytes()).unwrap();

    path_buf.pop();
    path_buf.push(format!("{filename}.txt"));
    file = File::create(&path_buf).unwrap();

    let sorted_attr_data = _sorted_attr_data(attr_data);
    file.write_all(sorted_attr_data.as_bytes()).unwrap();
}

fn _sorted_attr_data(data: &_AttributeData) -> String {
    let mut ca_keys: Vec<i16> = data.keys().map(|k| *k).collect();
    ca_keys.sort();

    let mut sorted_data = Vec::new();

    for ca_key in ca_keys {
        let ca_data = data.get(&ca_key).unwrap();
        let mut save_attr_keys: Vec<i8> = ca_data.keys().map(|k| *k).collect();
        save_attr_keys.sort();

        sorted_data.push(format!("CA: {ca_key}"));

        for save_attr_key in save_attr_keys {
            let attr_data = ca_data.get(&save_attr_key).unwrap();
            let mut data_pairs: Vec<(i8, usize)> = attr_data
                .iter()
                .map(|(db_attr, count)| (*db_attr, *count))
                .collect();
            data_pairs.sort_by(|(_, a), (_, b)| b.cmp(a));

            sorted_data.push(format!("\tDB Attribute: {save_attr_key}"));
            for (db_attr, count) in data_pairs {
                sorted_data.push(format!("\t\tx{count}: {db_attr}"));
            }
        }
    }

    return sorted_data.join("\n");
}

// Add database-specific attribute data to the main attribute data.
fn _add_attr(main: &mut _AttributeData, db: _AttributeData) {
    for (ca, ca_data) in db {
        if !main.contains_key(&ca) {
            main.insert(ca, ca_data);
            continue;
        }

        let main_ca = main.get_mut(&ca).unwrap();
        for (db_attr, db_attr_data) in ca_data {
            if !main_ca.contains_key(&db_attr) {
                main_ca.insert(db_attr, db_attr_data);
                continue;
            }

            let main_db_attr = main_ca.get_mut(&db_attr).unwrap();
            for (save_attr, count) in db_attr_data {
                if !main_db_attr.contains_key(&save_attr) {
                    main_db_attr.insert(save_attr, count);
                } else {
                    *main_db_attr.get_mut(&save_attr).unwrap() += count;
                }
            }
        }
    }
}
