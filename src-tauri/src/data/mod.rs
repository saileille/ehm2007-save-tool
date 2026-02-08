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

use std::{cmp::Ordering, collections::HashMap, i16, io::Cursor, mem};

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
    pub clubs: HashMap<i32, Club>,
    nat_clubs: HashMap<i32, Club>,
    staff_awards: HashMap<i32, StaffAward>,
    pub competitions: HashMap<i32, Competition>,
    nat_competitions: HashMap<i32, Competition>,
    comp_history: HashMap<i32, CompetitionHistory>,
    nat_comp_history: HashMap<i32, CompetitionHistory>,
    colours: HashMap<i32, Colour>,
    pub nations: HashMap<i32, Nation>,
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

    // The best attribute scores in the database.
    pub best_gk: usize,
    pub best_d: usize,
    pub best_w: usize,
    pub best_c: usize,
    pub best_ca: i16,

    // The worst attribute in the database.
    pub worst_gk: usize,
    pub worst_d: usize,
    pub worst_w: usize,
    pub worst_c: usize,
    pub worst_ca: i16,
}

impl Data {
    pub fn initialise(cursor: &mut Cursor<Vec<u8>>) -> Self {
        let header = Header::read(cursor).unwrap();

        let file_indexes = read_file_indexes(cursor, &header);
        let data = Self {
            _header: Some(header),
            file_indexes,

            best_gk: usize::MIN,
            best_d: usize::MIN,
            best_w: usize::MIN,
            best_c: usize::MIN,
            best_ca: i16::MIN,

            worst_gk: usize::MAX,
            worst_d: usize::MAX,
            worst_w: usize::MAX,
            worst_c: usize::MAX,
            worst_ca: i16::MAX,

            ..Default::default()
        };

        return data;
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
        self.date_range[0] = SIDate { day: i16::MIN, year: i16::MIN, b_is_leap_year: 0 };
        self.date_range[1] = SIDate { day: i16::MAX, year: i16::MAX, b_is_leap_year: 0 };

        for staff in self.staff.values() {
            let (min_date, max_date) = staff.dates_with_this_age();
            if min_date > self.date_range[0] {
                self.date_range[0] = min_date;

                println!(
                    "{} - {}\n{}, {}, {}\n",
                    self.date_range[0].to_string(),
                    self.date_range[1].to_string(),
                    staff.full_name(self),
                    staff.age,
                    staff.date_of_birth.to_string(),
                );
            }
            if max_date < self.date_range[1] {
                self.date_range[1] = max_date;

                println!(
                    "{} - {}\n{}, {}, {}\n",
                    self.date_range[0].to_string(),
                    self.date_range[1].to_string(),
                    staff.full_name(self),
                    staff.age,
                    staff.date_of_birth.to_string(),
                );
            }

            // The date has been determined when the dates are equal.
            // if self.date_range[0] == self.date_range[1] {
            //     break;
            // }
        }

        println!("\n");
    }

    // Calculate the best and the worst player ratings the save file has.
    pub fn calculate_rating_boundaries(&mut self) {
        for person in self.staff.values() {
            let p = person.player_data(&self);
            if p.is_none() { continue; }
            let p = p.unwrap();

            if p.current_ability > self.best_ca { self.best_ca = p.current_ability }
            if p.current_ability < self.worst_ca { self.worst_ca = p.current_ability }

            if p.is_goalie() {
                let gk = person.gk_attribute_score(&p);
                if gk > self.best_gk { self.best_gk = gk; }
                if gk < self.worst_gk { self.worst_gk = gk; }
            }
            else {
                let d = person.d_attribute_score(&p);
                let w = person.w_attribute_score(&p);
                let c = person.c_attribute_score(&p);

                if d > self.best_d { self.best_d = d; }
                if d < self.worst_d { self.worst_d = d; }

                if w > self.best_w { self.best_w = w; }
                if w < self.worst_w { self.worst_w = w; }

                if c > self.best_c { self.best_c = c; }
                if c < self.worst_c { self.worst_c = c; }
            }


        }
    }

}

#[derive(BinRead, PartialEq, Clone)]
#[br(little)]
pub struct SIDate {
    day: i16,
    pub year: i16,
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
    const MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    fn new(year: i16, day: i16) -> Self {
        let mut date = SIDate { day: 0, year, b_is_leap_year: 0 };
        date.add_days(day as isize);
        return date;
    }

    // Get an instance from year, month and day.
    fn _new_from_date(year: i16, month: u8, month_day: u8) -> Self {
        let days_in_february = if is_leap_year(year as i32) { 29 } else { 28 };
        let mut day = 0;
        for i in 0..month as usize - 1 {
            let month_days = match i == 1 {
                true => days_in_february,
                false => Self::MONTH_DAYS[i]
            };

            day += month_days as i16;
        }

        day += month_day as i16 - 1;
        return Self { year, day, b_is_leap_year: 0 };
    }

    // SI bullshit.
    fn _is_leap_year_si(&self) -> bool {
        return self.b_is_leap_year != 0;
    }

    // Actual leap year.
    fn is_leap_year(&self) -> bool {
        return is_leap_year(self.year as i32);
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

    fn to_string(&self) -> String {
        return format!(
            "({}, {}, {}, {})",
            self.year,
            self.day,
            is_leap_year(self.year as i32),
            self.b_is_leap_year,
        );
    }

    // Get days since the default (1.2.1900).
    fn _to_days(&self) -> usize {
        return self._days_between(Self::default());
    }

    // Get the year, month and day of the date.
    pub fn to_year_month_day(&self) -> (i16, u8, u8) {
        let february_days = if self.is_leap_year() { 29 } else { 28 };
        let mut days = self.day;
        let mut month = 13;

        for (i, d) in Self::MONTH_DAYS.iter().enumerate() {
            let month_days = match i == 1 {
                true => february_days,
                false => *d as i16
            };

            if days >= month_days {
                days -= month_days;
            }
            else {
                month = i as u8 + 1;
                break;
            }
        }

        return (self.year, month, days as u8 + 1);
    }

    // Get days between this date and another. This date must be more recent.
    fn _days_between(&self, other: Self) -> usize {
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

    // Add this many years to the date. Negative years subtract.
    fn _add_years(&mut self, years: i16) {
        self.year += years as i16;

        if self.day == 365 && !self.is_leap_year() {
            self.day -= 1;
        }
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
