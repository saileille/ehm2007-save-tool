/* Load stuff. */
pub mod debug;

use std::{collections::HashMap, fs::File, io::{Cursor, Read as _}, path::Path};

use binread::{BinRead, Error};

use crate::{data::{Data, arena::Arena, city::City, club::Club, colour::Colour, competition::Competition, competition_history::CompetitionHistory, continent::Continent, currency::Currency, draft::Draft, injury::Injury, name::Name, nation::Nation, non_player::NonPlayer, official::Official, player::Player, retired_number::RetiredNumber, staff::Staff, staff_award::StaffAward, staff_preferences::StaffPreferences, stage_name::StageName, state_province::StateProvince}, init::debug::check_players};

type ParseFunc = fn (&mut Data, &mut Cursor<Vec<u8>>) -> Result<(), Error>;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Header {
    compressed: i32,
    header: i32,
    files: i32,
}

#[derive(BinRead, Clone)]
#[br(little)]
pub struct FileIndex {
    start_position: u32,
    size: u32,

    #[br(count = 260)]
    b_name: Vec<char>,
}

impl FileIndex {
    // Get the string of the name.
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    // Get the binary and a cursor for it.
    fn bin(&self, global_cursor: &mut Cursor<Vec<u8>>) -> Cursor<Vec<u8>> {
        // Put the cursor where we need it.
        global_cursor.set_position(self.start_position as u64);

        // Fill the buffer with garbage.
        let mut buffer: Vec<u8> = vec![0; self.size as usize];
        global_cursor.read_exact(&mut buffer).unwrap();

        return Cursor::new(buffer);
    }
}

// Make chars to a string.
pub fn bytes_to_string(bytes: &[char]) -> String {
    let mut chars = Vec::new();

    for char in bytes {
        if *char == '\0' {
            break;
        }
        chars.push(*char);
    }

    return chars.into_iter().collect();
}

// Load the binary.
pub fn load_bin(path_name: &str) -> Data {
    let path = Path::new(path_name);

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("{path_name}");
            panic!("{e}");
        }
    };

    let save = load_save(file);
    let mut debug_save = save.clone();
    // check_players(&mut debug_save);


    return save;
}

pub fn load_save(mut file: File) -> Data {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut cursor = Cursor::new(buffer);

    let mut data = Data::initialise(&mut cursor);

    parse_files(&mut cursor, &mut data);
    return data;
}

// Read file indexes.
pub fn read_file_indexes(cursor: &mut Cursor<Vec<u8>>, header: &Header) -> HashMap<String, FileIndex> {
    let mut file_indexes = HashMap::new();
    for _ in 0..header.files {
        let index = FileIndex::read(cursor).unwrap();
        file_indexes.insert(index.name(), index);
    }

    return file_indexes;
}

pub fn get_parser_guide() -> HashMap<String, ParseFunc> {
    let mut functions: HashMap<String, ParseFunc> = HashMap::new();

    functions.insert("continent.dat".to_string(), Continent::parse);
    functions.insert("officials.dat".to_string(), Official::parse);
    functions.insert("first_names.dat".to_string(), Name::parse_forename);
    functions.insert("second_names.dat".to_string(), Name::parse_surname);
    functions.insert("city.dat".to_string(), City::parse);
    functions.insert("club.dat".to_string(), Club::parse);
    functions.insert("nat_club.dat".to_string(), Club::parse_nat);
    functions.insert("staff_comp.dat".to_string(), StaffAward::parse);
    functions.insert("club_comp.dat".to_string(), Competition::parse);
    functions.insert("nation_comp.dat".to_string(), Competition::parse_nat);
    functions.insert("club_comp_history.dat".to_string(), CompetitionHistory::parse);
    functions.insert("nation_comp_history.dat".to_string(), CompetitionHistory::parse_nat);
    functions.insert("colour.dat".to_string(), Colour::parse);
    functions.insert("nation.dat".to_string(), Nation::parse);
    functions.insert("stadium.dat".to_string(), Arena::parse);
    functions.insert("staff.dat".to_string(), Staff::parse);
    functions.insert("nonplayer.dat".to_string(), NonPlayer::parse);
    functions.insert("player.dat".to_string(), Player::parse);
    functions.insert("staff_preferences.dat".to_string(), StaffPreferences::parse);
    functions.insert("retired_numbers.dat".to_string(), RetiredNumber::parse);
    functions.insert("states_provinces.dat".to_string(), StateProvince::parse);
    functions.insert("injuries.dat".to_string(), Injury::parse);
    functions.insert("currencies.dat".to_string(), Currency::parse);
    functions.insert("drafts.dat".to_string(), Draft::parse);
    functions.insert("stage_names.dat".to_string(), StageName::parse);

    return functions;
}

// Parse the files.
pub fn parse_files(global_cursor: &mut Cursor<Vec<u8>>, data: &mut Data) {
    let parser_guide = get_parser_guide();
    let file_indexes = data.file_indexes.clone();

    for (name, file) in file_indexes {
        let mut cursor = file.bin(global_cursor);
        match parser_guide.get(name.as_str()) {
            Some(parser) => {
                parse_file(&mut cursor, parser, data, file.size as u64, name.as_str());
                println!("[x] {name}");
            },
            None => {
                // Add the save file part into the binaries as-is.
                data.binaries.insert(name.clone(), cursor.into_inner());
                println!("{name}");
            }
        };
    }
}

pub fn parse_file(cursor: &mut Cursor<Vec<u8>>, parser: &ParseFunc, data: &mut Data, file_size: u64, name: &str) {
    loop {
        match parser(data, cursor) {
            Err(_) => break,
            _ => {},
        }
    }

    // Make sure the cursor has reached the end of the file.
    let leftovers = file_size - cursor.position();
    if leftovers != 0 {
        panic!("{name} has {leftovers} leftover bytes");
    }
}