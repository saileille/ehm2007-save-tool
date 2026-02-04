use std::{collections::HashMap, io::Cursor, str::Utf8Error};

use binread::{BinRead, Error};
use regex::Regex;
use tauri::webview::cookie::time::util::is_leap_year;
use lazy_static::lazy_static;

use crate::{
    chars::_bytes_to_string_debug, data::{Data, SIDate, city::City, club::Club, name::Name, nation::Nation, player::{self, Player}}, rating::stretch, research::db, views
};

lazy_static! {
    pub static ref PERFECT: Staff = Staff {
        adaptability: 20,
        ambition: 20,
        determination: 20,
        loyalty: 20,
        pressure: 20,
        professionalism: 20,
        sportsmanship: 20,
        temperament: 20,

        ..Default::default()
    };

    pub static ref WORST: Staff = Staff {
        adaptability: 1,
        ambition: 1,
        determination: 1,
        loyalty: 1,
        pressure: 1,
        professionalism: 1,
        sportsmanship: 1,
        temperament: 1,

        ..Default::default()
    };
}

#[derive(BinRead, Clone, Default)]
#[br(little)]
pub struct Staff {
    pub id: i32,
    _estimated_wage: i32,
    _estimated_value: i32,
    nation_id: i32,
    second_nation_id: i32,
    player_data_id: i32,
    _preferences_id: i32,
    _non_player_data_id: i32,
    _nation_contracted_id: i32,
    pub club_contracted_id: i32,
    club_playing_id: i32,
    _player_rights_index: i32,
    _birth_town_id: i32,
    forename_id: i32,
    surname_id: i32,
    pub date_of_birth: SIDate,
    _date_joined_nation: SIDate,
    _contract_expires_nation: SIDate,
    _date_joined_club: SIDate,
    _contract_expires_club: SIDate,
    _first_pro_contract: SIDate,
    pub age: i16,
    _international_apps: u8,
    _international_goals: u8,
    _international_assists: u8,
    _job_for_nation: i8,
    pub adaptability: i8,
    _job_for_club: i8,
    pub ambition: i8,
    pub determination: i8,
    pub loyalty: i8,
    pub pressure: i8,
    pub professionalism: i8,
    pub sportsmanship: i8,
    pub temperament: i8,
    _playing_squad: i8,
    _classification: i8,
    _club_valuation: i8,
    declared_nation: i8,
    _stanley_cups_won: i8,
    _squad_selected_for: i8,
    _national_team_job_level: i8,
    _estimated_wage_weekly: i32,
}

impl Staff {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let staff = Self::read(cursor)?;
        data.order_staff.push(staff.id);
        data.staff.insert(staff.id, staff);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._estimated_wage.to_le_bytes());
        bytes.extend_from_slice(&self._estimated_value.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.second_nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.player_data_id.to_le_bytes());
        bytes.extend_from_slice(&self._preferences_id.to_le_bytes());
        bytes.extend_from_slice(&self._non_player_data_id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_contracted_id.to_le_bytes());
        bytes.extend_from_slice(&self.club_contracted_id.to_le_bytes());
        bytes.extend_from_slice(&self.club_playing_id.to_le_bytes());
        bytes.extend_from_slice(&self._player_rights_index.to_le_bytes());
        bytes.extend_from_slice(&self._birth_town_id.to_le_bytes());
        bytes.extend_from_slice(&self.forename_id.to_le_bytes());
        bytes.extend_from_slice(&self.surname_id.to_le_bytes());
        bytes.append(&mut self.date_of_birth._to_bytes());
        bytes.append(&mut self._date_joined_nation._to_bytes());
        bytes.append(&mut self._contract_expires_nation._to_bytes());
        bytes.append(&mut self._date_joined_club._to_bytes());
        bytes.append(&mut self._contract_expires_club._to_bytes());
        bytes.append(&mut self._first_pro_contract._to_bytes());
        bytes.extend_from_slice(&self.age.to_le_bytes());
        bytes.extend_from_slice(&self._international_apps.to_le_bytes());
        bytes.extend_from_slice(&self._international_goals.to_le_bytes());
        bytes.extend_from_slice(&self._international_assists.to_le_bytes());
        bytes.extend_from_slice(&self._job_for_nation.to_le_bytes());
        bytes.extend_from_slice(&self.adaptability.to_le_bytes());
        bytes.extend_from_slice(&self._job_for_club.to_le_bytes());
        bytes.extend_from_slice(&self.ambition.to_le_bytes());
        bytes.extend_from_slice(&self.determination.to_le_bytes());
        bytes.extend_from_slice(&self.loyalty.to_le_bytes());
        bytes.extend_from_slice(&self.pressure.to_le_bytes());
        bytes.extend_from_slice(&self.professionalism.to_le_bytes());
        bytes.extend_from_slice(&self.sportsmanship.to_le_bytes());
        bytes.extend_from_slice(&self.temperament.to_le_bytes());
        bytes.extend_from_slice(&self._playing_squad.to_le_bytes());
        bytes.extend_from_slice(&self._classification.to_le_bytes());
        bytes.extend_from_slice(&self._club_valuation.to_le_bytes());
        bytes.extend_from_slice(&self.declared_nation.to_le_bytes());
        bytes.extend_from_slice(&self._stanley_cups_won.to_le_bytes());
        bytes.extend_from_slice(&self._squad_selected_for.to_le_bytes());
        bytes.extend_from_slice(&self._national_team_job_level.to_le_bytes());
        bytes.extend_from_slice(&self._estimated_wage_weekly.to_le_bytes());

        return bytes;
    }

    pub fn forename(&self, data: &Data) -> String {
        return data.forenames.get(&self.forename_id).unwrap().name().unwrap();
    }

    pub fn surname(&self, data: &Data) -> String {
        return data.surnames.get(&self.surname_id).unwrap().name().unwrap();
    }

    pub fn _forename_object(&self, data: &Data) -> Name {
        return data.forenames.get(&self.forename_id).unwrap().clone();
    }

    pub fn _surname_object(&self, data: &Data) -> Name {
        return data.surnames.get(&self.surname_id).unwrap().clone();
    }

    fn _birth_town(&self, data: &Data) -> Option<City> {
        return data.cities.get(&self._birth_town_id).cloned();
    }

    pub fn _birthplace(&self, data: &Data) -> String {
        let town = self._birth_town(data);
        if town.is_none() {
            return String::new();
        }
        let town = town.unwrap();

        let state_string = match town._state_abbreviation(data) {
            Some(s) => format!(", {s}"),
            None => String::new(),
        };

        let nation_string = match town._nation_three_letter_name(data) {
            Some(s) => format!(", {s}"),
            None => String::new(),
        };

        let town_name = match town._name() {
            Ok(s) => s,
            Err(e) => {
                let s = _bytes_to_string_debug(&town._b_name);
                println!(
                    "{e}\nbytes: {:?}\nstring: {}\nname: {} {}\nbirth year: {}\nnationality: {}\nclub: {}",
                    town._b_name,
                    s,
                    self.forename(data),
                    self.surname(data),
                    self.date_of_birth.year,
                    self.nation_name(data),
                    self.club_contracted_name(data).unwrap(),
                );
                s
            }
        };

        return format!("{}{}{}", town_name, state_string, nation_string);
    }

    fn _nation(&self, data: &Data) -> Nation {
        return data.nations.get(&self.nation_id).cloned().unwrap();
    }

    pub fn nation_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n.name(),
            None => String::new(),
        };
    }

    fn _nation_short_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n._short_name(),
            None => String::new(),
        };
    }

    pub fn _nation_three_letter_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n._three_letter_name(),
            None => String::new(),
        };
    }

    fn _second_nation(&self, data: &Data) -> Option<Nation> {
        return data.nations.get(&self.second_nation_id).cloned();
    }

    pub fn second_nation_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n.name(),
            None => String::new(),
        };
    }

    fn _second_nation_short_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n._short_name(),
            None => String::new(),
        };
    }

    pub fn _second_nation_three_letter_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n._three_letter_name(),
            None => String::new(),
        };
    }

    pub fn _club_contracted(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_contracted_id).cloned();
    }

    pub fn club_contracted_name(&self, data: &Data) -> Result<String, Utf8Error> {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c.name(),
            None => Ok(String::new()),
        };
    }

    pub fn _club_contracted_short_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c._short_name(),
            None => String::new(),
        };
    }

    pub fn club_playing(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_playing_id).cloned();
    }

    pub fn club_playing_name(&self, data: &Data) -> Result<String, Utf8Error> {
        return match data.clubs.get(&self.club_playing_id) {
            Some(c) => c.name(),
            None => Ok(String::new()),
        };
    }

    pub fn _club_playing_short_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_playing_id) {
            Some(c) => c._short_name(),
            None => String::new(),
        };
    }

    pub fn player_data(&self, data: &Data) -> Option<Player> {
        return data.players.get(&self.player_data_id).cloned();
    }

    pub fn full_name(&self, data: &Data) -> String {
        format!("{} {}", self.forename(data), self.surname(data))
    }

    // Get the nation the player has declared for, if one exists.
    fn declared_nation_id(&self) -> Option<i32> {
        if self.declared_nation == 0 {
            return None;
        }

        if self.declared_nation == 1 {
            return Some(self.nation_id);
        }

        return Some(self.second_nation_id);
    }

    pub fn check_player_filters(
            &self,
            data: &Data,
            nation_id: i32,
            national_team_check: bool,
            country_choice_check: bool,
            earliest_birth_year: i16,
            exclude_nhl: bool,
            exclude_na: bool) -> bool {
        return self.has_nationality(nation_id)
        && self.check_national_team(nation_id, national_team_check)
        && self.check_country_choice(nation_id, country_choice_check)
        && self.has_given_age(earliest_birth_year)
        && self.check_na_exclusion(data, exclude_na, exclude_nhl);
    }

    // Check if the player is in the NHL or North America.
    fn check_na_exclusion(&self, data: &Data, exclude_na: bool, exclude_nhl: bool) -> bool {
        if !exclude_nhl && !exclude_na {
            return true;
        }

        let club_playing = self.club_playing(data);
        if club_playing.is_none() {
            return true;
        }

        let club_playing = club_playing.unwrap();

        if exclude_na && data.na_ids.contains(&club_playing.nation_id) {
            return false;
        }

        if exclude_nhl {
            return !data.nhl_ids.contains(&club_playing.division_id);
        }

        return true;
    }

    // Check if the player falls within specified age group.
    fn has_given_age(&self, earliest_birth_year: i16) -> bool {
        return self.date_of_birth.year >= earliest_birth_year;
    }

    // Check if the player can play for the national team of the given nation.
    fn check_national_team(&self, nation_id: i32, national_team_check: bool) -> bool {
        // Cannot check this against non-nations.
        if nation_id < 0 || !national_team_check {
            return true;
        }

        let declared = self.declared_nation_id();
        return declared.is_none() || declared.unwrap() == nation_id;
    }

    // Check if the player can choose to play for this country.
    fn check_country_choice(&self, nation_id: i32, country_choice_check: bool) -> bool {
        // Cannot check this against non-nations.
        if nation_id < 0 || !country_choice_check {
            return true;
        }

        // Cannot choose to play for this country if the player does not have a second country.
        if self.second_nation_id < 0 {
            return false;
        }

        return self.declared_nation_id().is_none();
    }

    // Check if the person has the given nationality.
    fn has_nationality(&self, nation_id: i32) -> bool {
        // -2 is used as any nation.
        if nation_id == -2 {
            return true;
        }
        if nation_id == self.nation_id {
            return true;
        }

        // -1 means no nation, so we do not want to check that against second nation.
        return nation_id != -1 && nation_id == self.second_nation_id;
    }

    // Check if the person's name has no special characters.
    pub fn _has_no_special_characters(&self, data: &Data) -> bool {
        let re = Regex::new(r"^[\w ]+$").unwrap();
        re.is_match(format!("{} {}", self.forename(data), self.surname(data)).as_str())
    }

    // Create an array of player data.
    pub fn create_player_view(&self, p: Player, data: &Data, headers: &[String], counter: usize) -> views::player::Player {
        let mut player = views::player::Player {
            forename: self.forename(data),
            surname: self.surname(data),
            positions: p.position_vec(),

            ..Default::default()
        };

        for header in headers {
            let header = header.as_str();
            player.add_column(data, counter, header, self, &p);
        }

        return player;
    }

    // Get the dates when the person has the current age.
    // Note: Person's age changes only the day AFTER their birthday.
    pub fn dates_with_this_age(&self) -> (SIDate, SIDate) {
        let mut min_day = self.date_of_birth.day;
        let min_year = self.age + self.date_of_birth.year;
        let birth_year_is_leap = is_leap_year(self.date_of_birth.year as i32);
        let min_year_is_leap = is_leap_year(min_year as i32);

        // Align the min date as close to the birthday as possible.
        if birth_year_is_leap && !min_year_is_leap && min_day >= SIDate::LEAP_DAY {
            min_day -= 1;
        }
        else if !birth_year_is_leap && min_year_is_leap && min_day >= SIDate::LEAP_DAY {
            min_day += 1;
        }

        let mut min = SIDate {
            day: min_day,
            year: min_year,
            b_is_leap_year: 0,
        };

        // The min date is one day after the birthday.
        min.add_days(1);

        let mut max = min.clone();

        let min_is_leap = is_leap_year(min.year as i32);
        let max_is_leap = is_leap_year((min.year + 1) as i32);

        if (min_is_leap && min.day <= SIDate::LEAP_DAY)
        || (max_is_leap && min.day >= SIDate::LEAP_DAY) {
            max.add_days(365);
        }
        else {
            max.add_days(364);
        }

        return (min, max);
    }

    // Get the person's 'score' as a goalkeeper.
    fn gk_attribute_score(&self, p: &Player) -> usize {
        let x30 = p.agility as usize + p.bravery as usize + self.determination as usize + p.convert_attribute("Glove") as usize
        + p.convert_attribute("Recovery") as usize + p.convert_attribute("Reflexes") as usize;

        let x20 = p.convert_attribute("Blocker") as usize + p.convert_attribute("One On Ones") as usize
        + p.convert_attribute("Positioning") as usize + p.convert_attribute("Rebound Control") as usize
        + p.stamina as usize;

        let x10 = p.convert_attribute("Balance") as usize + p.convert_attribute("Passing") as usize
        + p.convert_attribute("Pokecheck") as usize + self.pressure as usize + self.professionalism as usize + self.sportsmanship as usize
        + p.convert_attribute("Stickhandling") as usize + self.temperament as usize;

        let x1 = p.acceleration as usize + p.agitation as usize + p.convert_attribute("Anticipation") as usize
        + p.convert_attribute("Decisions") as usize + p.flair as usize + p.leadership as usize + p.natural_fitness as usize + p.pace as usize
        + p.strength as usize + p.teamwork as usize;

        return x30 * 30 + x20 * 20 + x10 * 10 + x1;
    }

    // Get the person's 'score' as a defender.
    fn d_attribute_score(&self, p: &Player) -> usize {
        let x30 = p.acceleration as usize + p.convert_attribute("Anticipation") as usize + p.bravery as usize
        + p.convert_attribute("Checking") as usize + self.determination as usize + p.convert_attribute("Hitting") as usize
        + p.convert_attribute("Passing") as usize + p.convert_attribute("Pokecheck") as usize
        + p.convert_attribute("Positioning") as usize + self.pressure as usize + p.convert_attribute("Slapshot") as usize
        + p.pace as usize + p.teamwork as usize;

        let x20 = p.convert_attribute("Balance") as usize + p.convert_attribute("Decisions") as usize + p.stamina as usize
        + p.strength as usize;

        let x10 = p.agility as usize + p.agitation as usize + p.convert_attribute("Creativity") as usize + p.flair as usize
        + p.convert_attribute("Off The Puck") as usize + self.professionalism as usize + self.sportsmanship as usize
        + p.convert_attribute("Stickhandling") as usize + self.temperament as usize + p.convert_attribute("Wristshot") as usize;

        let x1 = p.convert_attribute("Deflections") as usize + p.convert_attribute("Deking") as usize
        + p.leadership as usize + p.natural_fitness as usize;

        return x30 * 30 + x20 * 20 + x10 * 10 + x1;
    }

    // Get the person's 'score' as a winger.
    fn w_attribute_score(&self, p: &Player) -> usize {
        let x30 = p.acceleration as usize + p.agility as usize + p.convert_attribute("Anticipation") as usize
        + self.determination as usize + p.convert_attribute("Passing") as usize + self.pressure as usize + p.pace as usize
        + p.convert_attribute("Stickhandling") as usize + p.convert_attribute("Wristshot") as usize;

        let x20 = p.convert_attribute("Balance") as usize + p.bravery as usize + p.convert_attribute("Creativity") as usize
        + p.convert_attribute("Decisions") as usize + p.convert_attribute("Deking") as usize + p.flair as usize
        + p.convert_attribute("Off The Puck") as usize + p.convert_attribute("Positioning") as usize + p.stamina as usize
        + p.strength as usize + p.teamwork as usize;

        let x10 = p.agitation as usize + p.convert_attribute("Checking") as usize + p.convert_attribute("Deflections") as usize
        + p.convert_attribute("Hitting") as usize + p.convert_attribute("Pokecheck") as usize + self.professionalism as usize
        + p.convert_attribute("Slapshot") as usize + self.sportsmanship as usize + self.temperament as usize;

        let x1 = p.leadership as usize + p.natural_fitness as usize;

        return x30 * 30 + x20 * 20 + x10 * 10 + x1;
    }

    // Get the person's 'score' as a centre forward.
    fn c_attribute_score(&self, p: &Player) -> usize {
        let x30 = p.acceleration as usize + p.convert_attribute("Anticipation") as usize
        + p.convert_attribute("Creativity") as usize + self.determination as usize + p.convert_attribute("Faceoffs") as usize
        + p.convert_attribute("Passing") as usize + self.pressure as usize + p.pace as usize
        + p.convert_attribute("Stickhandling") as usize + p.strength as usize + p.convert_attribute("Wristshot") as usize;

        let x20 = p.agility as usize + p.convert_attribute("Balance") as usize + p.bravery as usize
        + p.convert_attribute("Checking") as usize + p.convert_attribute("Decisions") as usize
        + p.convert_attribute("Deflections") as usize + p.convert_attribute("Deking") as usize
        + p.flair as usize + p.convert_attribute("Hitting") as usize + p.convert_attribute("Off The Puck") as usize
        + p.convert_attribute("Pokecheck") as usize + p.convert_attribute("Positioning") as usize + p.stamina as usize
        + p.teamwork as usize;

        let x10 = p.agitation as usize + self.professionalism as usize + p.convert_attribute("Slapshot") as usize
        + self.sportsmanship as usize + self.temperament as usize;

        let x1 = p.leadership as usize + p.natural_fitness as usize;

        return x30 * 30 + x20 * 20 + x10 * 10 + x1;
    }

    // Get the person's ability as a goalkeeper.
    pub fn gk_rating(&self, p: &Player) -> f64 {
        let perfect_player = &player::PERFECT;

        let worst_score = WORST.gk_attribute_score(&player::WORST);
        let attribute_score = self.gk_attribute_score(&p) - worst_score;
        let perfect_score = PERFECT.gk_attribute_score(perfect_player) - worst_score;

        let attribute_rating = attribute_score as f64 / perfect_score as f64;
        let ca_rating = (p.current_ability - 1) as f64 / (perfect_player.current_ability - 1) as f64;

        let combined_rating = (attribute_rating + ca_rating) / 2.0;
        return combined_rating * p.consistency_rating();
    }

    // Get the person's ability as a defender.
    fn d_rating(&self, p: &Player) -> f64 {
        let perfect_player = &player::PERFECT;

        let worst_score = WORST.d_attribute_score(&player::WORST);
        let attribute_score = self.d_attribute_score(p) - worst_score;
        let perfect_score = PERFECT.d_attribute_score(perfect_player) - worst_score;

        let attribute_rating = attribute_score as f64 / perfect_score as f64;
        let ca_rating = (p.current_ability - 1) as f64 / (perfect_player.current_ability - 1) as f64;

        let combined_rating = (attribute_rating + ca_rating) / 2.0;
        return combined_rating * p.consistency_rating();
    }

    // Get the person's ability as a winger.
    fn w_rating(&self, p: &Player) -> f64 {
        let perfect_player = &player::PERFECT;

        let worst_score = WORST.w_attribute_score(&player::WORST);
        let attribute_score = self.w_attribute_score(p) - worst_score;
        let perfect_score = PERFECT.w_attribute_score(perfect_player) - worst_score;

        let attribute_rating = attribute_score as f64 / perfect_score as f64;
        let ca_rating = (p.current_ability - 1) as f64 / (perfect_player.current_ability - 1) as f64;

        let combined_rating = (attribute_rating + ca_rating) / 2.0;
        return combined_rating * p.consistency_rating();
    }

    // Get the person's ability as a centre forward.
    pub fn c_rating(&self, p: &Player) -> f64 {
        let perfect_player = &player::PERFECT;

        let worst_score = WORST.c_attribute_score(&player::WORST);
        let attribute_score = self.c_attribute_score(&p) - worst_score;
        let perfect_score = PERFECT.c_attribute_score(perfect_player) - worst_score;

        let attribute_rating = attribute_score as f64 / perfect_score as f64;
        let ca_rating = (p.current_ability - 1) as f64 / (perfect_player.current_ability - 1) as f64;

        let combined_rating = (attribute_rating + ca_rating) / 2.0;
        return combined_rating * p.consistency_rating() * p.position_rating(p.center);
    }

    // Get the person's ability as a left defender.
    pub fn ld_rating(&self, p: &Player) -> f64 {
        return self.d_rating(&p) * p.position_rating(p.left_defence);
    }

    // Get the person's ability as a right defender.
    pub fn rd_rating(&self, p: &Player) -> f64 {
        return self.d_rating(&p) * p.position_rating(p.right_defence);
    }

    // Get the person's ability as a left winger.
    pub fn lw_rating(&self, p: &Player) -> f64 {
        return self.w_rating(&p) * p.position_rating(p.left_wing);
    }

    // Get the person's ability as a right winger.
    pub fn rw_rating(&self, p: &Player) -> f64 {
        return self.w_rating(&p) * p.position_rating(p.right_wing);
    }

    pub fn relative_gk_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if !p.is_goalie() {
            return -1.0;
        }

        let rating = self.gk_rating(&p);
        return stretch(rating, data.worst_gk, data.best_gk);
    }

    pub fn relative_ld_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if p.is_goalie() {
            return -1.0;
        }

        let rating = self.ld_rating(&p);
        return stretch(rating, data.worst_d, data.best_d);
    }

    pub fn relative_rd_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if p.is_goalie() {
            return -1.0;
        }

        let rating = self.rd_rating(&p);
        return stretch(rating, data.worst_d, data.best_d);
    }

    pub fn relative_lw_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if p.is_goalie() {
            return -1.0;
        }

        let rating = self.lw_rating(&p);
        return stretch(rating, data.worst_w, data.best_w);
    }

    pub fn relative_c_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if p.is_goalie() {
            return -1.0;
        }

        let rating = self.c_rating(&p);
        return stretch(rating, data.worst_c, data.best_c);
    }

    pub fn relative_rw_rating(&self, data: &Data) -> f64 {
        let p = self.player_data(data).unwrap();
        if p.is_goalie() {
            return -1.0;
        }

        let rating = self.rw_rating(&p);
        return stretch(rating, data.worst_w, data.best_w);
    }

    pub fn _merge_players(
        &self,
        db: &Data,
        save: &Data,
        key: &[String; 6],
        save_staff: &mut HashMap<[String; 6], Staff>,
    ) -> Option<db::_Player> {
        let db_player = self.player_data(&db).unwrap();

        let save_person = save_staff.remove(key);
        if save_person.is_none() {
            return None;
        }
        let save_person = save_person.unwrap();
        let save_player = save_person.player_data(save).unwrap();

        return Some(db::_Player {
            forename: self.forename(db),
            surname: self.surname(db),
            age: save_person.age,
            birthplace: save_person._birthplace(save),
            nation: save_person._nation_three_letter_name(save),
            second_nation: save_person._second_nation_three_letter_name(save),
            club: save_person._club_playing_short_name(save),
            current_ability: save_player.current_ability,
            anticipation_save: save_player.anticipation_raw,
            anticipation_db: db_player.anticipation_raw,
            balance_save: save_player.balance_raw,
            balance_db: db_player.balance_raw,
            decisions_save: save_player.decisions_raw,
            decisions_db: db_player.decisions_raw,
            movement_save: save_player.movement_raw,
            movement_db: db_player.movement_raw,
            one_on_ones_save: save_player.one_on_ones_raw,
            one_on_ones_db: db_player.one_on_ones_raw,
            passing_save: save_player.passing_raw,
            passing_db: db_player.passing_raw,
            positioning_save: save_player.positioning_raw,
            positioning_db: db_player.positioning_raw,
            reflexes_save: save_player.reflexes_raw,
            reflexes_db: db_player.reflexes_raw,
            vision_save: save_player.vision_raw,
            vision_db: db_player.vision_raw,
            blocker_save: save_player.blocker_raw,
            blocker_db: db_player.blocker_raw,
            checking_save: save_player.checking_raw,
            checking_db: db_player.checking_raw,
            deflections_save: save_player.deflections_raw,
            deflections_db: db_player.deflections_raw,
            deking_save: save_player.deking_raw,
            deking_db: db_player.deking_raw,
            faceoffs_save: save_player.faceoffs_raw,
            faceoffs_db: db_player.faceoffs_raw,
            fighting_save: save_player.fighting_raw,
            fighting_db: db_player.fighting_raw,
            glove_save: save_player.glove_raw,
            glove_db: db_player.glove_raw,
            hitting_save: save_player.hitting_raw,
            hitting_db: db_player.hitting_raw,
            pokecheck_save: save_player.pokecheck_raw,
            pokecheck_db: db_player.pokecheck_raw,
            rebounds_save: save_player.rebounds_raw,
            rebounds_db: db_player.rebounds_raw,
            recovery_save: save_player.recovery_raw,
            recovery_db: db_player.recovery_raw,
            slapshot_save: save_player.slapshot_raw,
            slapshot_db: db_player.slapshot_raw,
            stickhandling_save: save_player.stickhandling_raw,
            stickhandling_db: db_player.stickhandling_raw,
            wristshot_save: save_player.wristshot_raw,
            wristshot_db: db_player.wristshot_raw,
            gk: db_player.goaltender,
            ld: db_player.left_defence,
            rd: db_player.right_defence,
            lw: db_player.left_wing,
            c: db_player.center,
            rw: db_player.right_wing,
        });
    }
}
