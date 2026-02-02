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

use std::{cmp::Ordering, collections::HashMap, io::Cursor, mem};

use binread::BinRead;
use tauri::webview::cookie::time::util::is_leap_year;

use crate::{
    data::{
        arena::Arena, city::City, club::Club, colour::Colour, competition::Competition,
        competition_history::CompetitionHistory, continent::Continent, currency::Currency,
        draft::Draft, injury::Injury, name::Name, nation::Nation, non_player::NonPlayer,
        official::Official, player::Player, retired_number::RetiredNumber, staff::Staff,
        staff_award::StaffAward, staff_preferences::StaffPreferences, stage_name::StageName,
        state_province::StateProvince,
    }, globals::attr_chart::ATTRIBUTE_CHART, init::{FileIndex, Header, read_file_indexes}
};

static LONG_TEXT_LENGTH: u8 = 101;
static STANDARD_TEXT_LENGTH: u8 = 51;
static SHORT_TEXT_LENGTH: u8 = 26;
static REAL_SHORT_TEXT_LENGTH: u8 = 6;
static THREE_LETTER_TEXT_LENGTH: u8 = 4;
static SIX_LETTER_TEXT_LENGTH: u8 = 7;

// Everything.
#[derive(Default, Clone)]
pub struct Data {
    pub _header: Option<Header>,
    pub file_indexes: Vec<FileIndex>,

    pub date_range: [SIDate; 2],

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
    nonplayers: Vec<(i32, NonPlayer)>, // Multiple IDs can exist?
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

    // The order in which the entries should be saved.
    order_continents: Vec<i32>,
    order_officials: Vec<i32>,
    order_forenames: Vec<i32>,
    order_surnames: Vec<i32>,
    order_cities: Vec<i32>,
    order_clubs: Vec<i32>,
    order_nat_clubs: Vec<i32>,
    order_staff_awards: Vec<i32>,
    order_competitions: Vec<i32>,
    order_nat_competitions: Vec<i32>,
    order_comp_history: Vec<i32>,
    order_nat_comp_history: Vec<i32>,
    order_colours: Vec<i32>,
    order_nations: Vec<i32>,
    order_arenas: Vec<i32>,
    order_staff: Vec<i32>,
    order_players: Vec<i32>,
    order_staff_preferences: Vec<i32>,
    order_retired_numbers: Vec<i32>,
    order_states_provinces: Vec<i32>,
    order_injuries: Vec<i16>,
    order_currencies: Vec<i32>,
    order_drafts: Vec<i32>,
    order_stage_names: Vec<i32>,
}

impl Data {
    pub fn initialise(cursor: &mut Cursor<Vec<u8>>) -> Self {
        let header = Header::read(cursor).unwrap();
        let file_indexes = read_file_indexes(cursor, &header);
        Self {
            _header: Some(header),
            file_indexes,

            ..Default::default()
        }
    }

    // Get a save file of the data.
    pub fn _save_file(&mut self) -> Vec<u8> {
        // Encode all save data.
        let mut encoded = self.binaries.clone();

        encoded.insert(
            "continent.dat".to_string(),
            self.order_continents
                .iter()
                .flat_map(|id| self.continents.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "officials.dat".to_string(),
            self.order_officials
                .iter()
                .flat_map(|id| self.officials.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "first_names.dat".to_string(),
            self.order_forenames
                .iter()
                .flat_map(|id| self.forenames.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "second_names.dat".to_string(),
            self.order_surnames
                .iter()
                .flat_map(|id| self.surnames.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "city.dat".to_string(),
            self.order_cities
                .iter()
                .flat_map(|id| self.cities.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "club.dat".to_string(),
            self.order_clubs
                .iter()
                .flat_map(|id| self.clubs.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "nat_club.dat".to_string(),
            self.order_nat_clubs
                .iter()
                .flat_map(|id| self.nat_clubs.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "staff_comp.dat".to_string(),
            self.order_staff_awards
                .iter()
                .flat_map(|id| self.staff_awards.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "club_comp.dat".to_string(),
            self.order_competitions
                .iter()
                .flat_map(|id| self.competitions.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "nation_comp.dat".to_string(),
            self.order_nat_competitions
                .iter()
                .flat_map(|id| self.nat_competitions.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "club_comp_history.dat".to_string(),
            self.order_comp_history
                .iter()
                .flat_map(|id| self.comp_history.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "nation_comp_history.dat".to_string(),
            self.order_nat_comp_history
                .iter()
                .flat_map(|id| self.nat_comp_history.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "colour.dat".to_string(),
            self.order_colours
                .iter()
                .flat_map(|id| self.colours.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "nation.dat".to_string(),
            self.order_nations
                .iter()
                .flat_map(|id| self.nations.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "stadium.dat".to_string(),
            self.order_arenas
                .iter()
                .flat_map(|id| self.arenas.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "staff.dat".to_string(),
            self.order_staff
                .iter()
                .flat_map(|id| self.staff.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "nonplayer.dat".to_string(),
            self.nonplayers
                .iter()
                .flat_map(|(_, a)| a._to_bytes())
                .collect(),
        );
        encoded.insert(
            "player.dat".to_string(),
            self.order_players
                .iter()
                .flat_map(|id| self.players.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "staff_preferences.dat".to_string(),
            self.order_staff_preferences
                .iter()
                .flat_map(|id| self.staff_preferences.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "retired_numbers.dat".to_string(),
            self.order_retired_numbers
                .iter()
                .flat_map(|id| self.retired_numbers.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "states_provinces.dat".to_string(),
            self.order_states_provinces
                .iter()
                .flat_map(|id| self.states_provinces.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "injuries.dat".to_string(),
            self.order_injuries
                .iter()
                .flat_map(|id| self.injuries.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "currencies.dat".to_string(),
            self.order_currencies
                .iter()
                .flat_map(|id| self.currencies.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "drafts.dat".to_string(),
            self.order_drafts
                .iter()
                .flat_map(|id| self.drafts.get(id).unwrap()._to_bytes())
                .collect(),
        );
        encoded.insert(
            "stage_names.dat".to_string(),
            self.order_stage_names
                .iter()
                .flat_map(|id| self.stage_names.get(id).unwrap()._to_bytes())
                .collect(),
        );

        // Update the sizes of the save file parts and order them according to the file indexes.
        let mut order = Vec::new();
        for index in self.file_indexes.iter() {
            let name = index.name();

            // 'Steals' the contents of the vector.
            let bin = match encoded.get_mut(&name) {
                Some(b) => b,
                None => panic!("'{name}' is not a filename."),
            };

            let bin = mem::take(bin);

            let size = bin.len() as u32;
            order.push((size, bin));
        }

        // Get the start position of the first file.
        let start_position = self.file_indexes.get(0).unwrap().start_position;
        let mut total_size = start_position;

        // Bundle the content into one byte array.
        let mut content_bin = Vec::new();
        for (i, (size, mut bin)) in order.into_iter().enumerate() {
            let file_index = self.file_indexes.get_mut(i).unwrap();
            content_bin.append(&mut bin);

            file_index.start_position = total_size;
            file_index.size = size;
            total_size += size;
        }

        // Put the save file together.
        let mut bin = self._header.as_ref().unwrap()._to_bytes();
        bin.append(
            &mut self
                .file_indexes
                .iter()
                .flat_map(|a| a._to_bytes())
                .collect(),
        );

        // Pad the end of indexes and the start of content with NUL.
        bin.resize(start_position as usize, 0);

        bin.append(&mut content_bin);
        return bin;
    }

    // Determine what the in-game date could be.
    pub fn calculate_ingame_date(&mut self) {
        self.date_range[0] = SIDate { day: 0, year: i16::MIN, b_is_leap_year: 0 };
        self.date_range[1] = SIDate { day: 366, year: i16::MAX, b_is_leap_year: 0 };

        for staff in self.staff.values() {
            let (min_date, max_date) = staff.dates_with_this_age();
            if min_date > self.date_range[0] {
                self.date_range[0] = min_date;
            }
            if max_date < self.date_range[1] {
                self.date_range[1] = max_date;
            }

            // The date has been determined when the dates are equal.
            if self.date_range[0] == self.date_range[1] {
                break;
            }
        }
    }
}

#[derive(BinRead, PartialEq, Clone)]
#[br(little)]
pub struct SIDate {
    day: i16,
    year: i16,
    b_is_leap_year: u8,
}

impl Default for SIDate {
    fn default() -> Self {
        Self {
            day: 31,
            year: 1900,
            b_is_leap_year: 0,
        }
    }
}

impl PartialOrd for SIDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.year < other.year {
            return Some(Ordering::Less);
        }
        if self.year > other.year {
            return Some(Ordering::Greater);
        }
        if self.day < other.day {
            return Some(Ordering::Less);
        }
        if self.day > other.day {
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Equal);
    }

    fn lt(&self, other: &Self) -> bool {
        return self.partial_cmp(other).unwrap() == Ordering::Less;
    }

    fn le(&self, other: &Self) -> bool {
        let result = self.partial_cmp(other).unwrap();
        return result == Ordering::Less || result == Ordering::Equal;
    }

    fn gt(&self, other: &Self) -> bool {
        return self.partial_cmp(other).unwrap() == Ordering::Greater;
    }

    fn ge(&self, other: &Self) -> bool {
        let result = self.partial_cmp(other).unwrap();
        return result == Ordering::Greater || result == Ordering::Equal;
    }
}

impl SIDate {
    // The day of Feb 29.
    const LEAP_DAY: i16 = 59;

    fn _is_leap_year(&self) -> bool {
        return self.b_is_leap_year != 0;
    }

    fn _is_default(&self) -> bool {
        return self.day == 31 && self.year == 1900;
    }

    fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.day.to_le_bytes());
        bytes.extend_from_slice(&self.year.to_le_bytes());
        bytes.extend_from_slice(&self.b_is_leap_year.to_le_bytes());

        return bytes;
    }

    // Get days from the default.
    pub fn to_days(&self) -> usize {
        return self.days_between(Self::default());
    }

    // Get days between this date and another. This date must be more recent.
    fn days_between(&self, other: Self) -> usize {
        // Add days from the earlier date's year.
        let mut days = match is_leap_year(other.year as i32) {
            true => 366 - other.day,
            false => 365 - other.day
        } as usize;

        // Add days from full years between the two dates.
        for complete_year in other.year + 1 .. self.year {
            days += match is_leap_year(complete_year as i32) {
                true => 366,
                false => 365
            };
        }

        // Add days from this date's year.
        days += self.day as usize;
        return days;
    }

    // Add this many days to the date. Negative values subtract.
    fn add_days(&mut self, mut days: isize) {
        // Starting from the 1st of January because it is easier.
        days += self.day as isize;
        let add = days >= 0;

        loop {
            // Determine the year we are looking at - future or past.
            let year = if add { self.year } else { self.year - 1 };
            let year_days = if is_leap_year(year as i32) { 366 } else { 365 };

            if days >= year_days {
                self.year += 1;
                days -= year_days;
            }
            else if days <= year_days * -1 {
                self.year -= 1;
                days += year_days;
            }
            else {
                break;
            }
        }

        if days < 0 {
            self.year -= 1;
            let year_days = if is_leap_year(self.year as i32) { 366 } else { 365 };
            self.day = (year_days + days) as i16;
        }
        else {
            self.day = days as i16;
        }
    }
}

// Convert an attribute from save file to in-game.
pub fn convert_attribute(current_ability: i16, attribute: i8) -> i8 {
    let ca_chart = ATTRIBUTE_CHART.get(&current_ability).unwrap();

    for (real_attr, range) in ca_chart {
        if range.contains(&attribute) {
            return *real_attr;
        }
    }

    panic!("current ability '{current_ability}' does not have save-file attribute '{attribute}'");
}
