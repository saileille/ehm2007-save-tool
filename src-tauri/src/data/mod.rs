// Data types for storing human-readable values.
pub mod arena;
pub mod city;
pub mod club;
pub mod colour;
pub mod competition;
pub mod competition_history;
pub mod continent;
pub mod currency;
pub mod draft;
pub mod injury;
pub mod name;
pub mod nation;
pub mod non_player;
pub mod official;
pub mod player;
pub mod retired_number;
pub mod staff;
pub mod staff_award;
pub mod staff_preferences;
pub mod stage_name;
pub mod state_province;

use std::{collections::HashMap, io::Cursor};

use binread::BinRead;

use crate::{attr_chart::AttributeChart, data::{arena::Arena, city::City, club::Club, colour::Colour, competition::Competition, competition_history::CompetitionHistory, continent::Continent, currency::Currency, draft::Draft, injury::Injury, name::Name, nation::Nation, non_player::NonPlayer, official::Official, player::Player, retired_number::RetiredNumber, staff::Staff, staff_award::StaffAward, staff_preferences::StaffPreferences, stage_name::StageName, state_province::StateProvince}, init::{FileIndex, Header, read_file_indexes}};

static LONG_TEXT_LENGTH: u8 = 101;
static STANDARD_TEXT_LENGTH: u8 = 51;
static SHORT_TEXT_LENGTH: u8 = 26;
static REAL_SHORT_TEXT_LENGTH: u8 = 6;
static THREE_LETTER_TEXT_LENGTH: u8 = 4;
static SIX_LETTER_TEXT_LENGTH: u8 = 7;


// Everything.
#[derive(Default, Clone)]
pub struct Data {
    pub header: Option<Header>,
    pub file_indexes: HashMap<String, FileIndex>,

    continents: HashMap<i32, Continent>,
    officials: HashMap<i32, Official>,
    forenames: HashMap<i32, Name>,
    surnames: HashMap<i32, Name>,
    cities: HashMap<i32, City>,
    clubs: HashMap<i32, Club>,
    nat_clubs: HashMap<i32, Club>,
    staff_awards: HashMap<i32, StaffAward>,
    competitions: HashMap<i32, Competition>,
    nat_competitions: HashMap<i32, Competition>,
    comp_history: HashMap<i32, CompetitionHistory>,
    nat_comp_history: HashMap<i32, CompetitionHistory>,
    colours: HashMap<i32, Colour>,
    nations: HashMap<i32, Nation>,
    arenas: HashMap<i32, Arena>,
    pub staff: HashMap<i32, Staff>,
    nonplayers: HashMap<i32, NonPlayer>,
    pub players: HashMap<i32, Player>,
    staff_preferences: HashMap<i32, StaffPreferences>,
    retired_numbers: HashMap<i32, RetiredNumber>,
    states_provinces: HashMap<i32, StateProvince>,
    injuries: HashMap<i16, Injury>,
    currencies: HashMap<i32, Currency>,
    drafts: HashMap<i32, Draft>,
    stage_names: HashMap<i32, StageName>,

    // Undecoded parts of the save file.
    pub binaries: HashMap<String, Vec<u8>>,
}

impl Data {
    pub fn initialise(cursor: &mut Cursor<Vec<u8>>) -> Self {
        let header = Header::read(cursor).unwrap();
        let file_indexes = read_file_indexes(cursor, &header);
        Self {
            header: Some(header),
            file_indexes,

            ..Default::default()
        }
    }

    // Get a save file of the data.
    fn save_file(&self) {

    }
}

#[derive(BinRead, PartialEq, Clone)]
#[br(little)]
struct SIDate {
    day: i16,
    year: i16,
    b_is_leap_year: u8,
}

impl SIDate {
    fn is_leap_year(&self) -> bool {
        return self.b_is_leap_year != 0;
    }

    fn is_default(&self) -> bool {
        return self.day == 31 && self.year == 1900;
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.day.to_le_bytes());
        bytes.extend_from_slice(&self.year.to_le_bytes());
        bytes.extend_from_slice(&self.b_is_leap_year.to_le_bytes());

        return bytes;
    }
}

// Convert an attribute if possible.
pub fn convert_attribute(chart: &AttributeChart, current_ability: i16, attribute: i8) -> Option<i8> {
    let ca_chart = match chart.get(&current_ability) {
        Some(c) => c,
        None => return None
    };

    for (range, real_attr) in ca_chart {
        if range.contains(&attribute) {
            return Some(*real_attr);
        }
    }

    return None;
}