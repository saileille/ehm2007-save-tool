use std::{collections::HashMap, io::Cursor};

use binread::{BinRead, Error};
use regex::Regex;
use serde_json::json;
use tauri::webview::cookie::time::util::is_leap_year;

use crate::{
    data::{Data, SIDate, city::City, club::Club, nation::Nation, player::Player},
    research::db,
};

#[derive(BinRead, Clone)]
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
    adaptability: i8,
    _job_for_club: i8,
    ambition: i8,
    determination: i8,
    loyalty: i8,
    pressure: i8,
    professionalism: i8,
    sportsmanship: i8,
    temperament: i8,
    _playing_squad: i8,
    _classification: i8,
    _club_valuation: i8,
    _declared_nation: i8,
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
        bytes.extend_from_slice(&self._declared_nation.to_le_bytes());
        bytes.extend_from_slice(&self._stanley_cups_won.to_le_bytes());
        bytes.extend_from_slice(&self._squad_selected_for.to_le_bytes());
        bytes.extend_from_slice(&self._national_team_job_level.to_le_bytes());
        bytes.extend_from_slice(&self._estimated_wage_weekly.to_le_bytes());

        return bytes;
    }

    pub fn forename(&self, data: &Data) -> String {
        return data.forenames.get(&self.forename_id).unwrap().name();
    }

    pub fn surname(&self, data: &Data) -> String {
        return data.surnames.get(&self.surname_id).unwrap().name();
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

        return format!("{}{}{}", town._name(), state_string, nation_string);
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

    fn _club_contracted(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_contracted_id).cloned();
    }

    pub fn club_contracted_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c.name(),
            None => String::new(),
        };
    }

    pub fn _club_contracted_short_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c._short_name(),
            None => String::new(),
        };
    }

    fn _club_playing(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_playing_id).cloned();
    }

    fn club_playing_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_playing_id) {
            Some(c) => c.name(),
            None => String::new(),
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
        format!("{}, {}", self.surname(data), self.forename(data))
    }

    // Check if the person's name has no special characters.
    pub fn _has_no_special_characters(&self, data: &Data) -> bool {
        let re = Regex::new(r"^[\w ]+$").unwrap();
        re.is_match(format!("{} {}", self.forename(data), self.surname(data)).as_str())
    }

    // Create an array of player data.
    pub fn create_player_view(&self, data: &Data, headers: &[String], counter: usize) -> Option<Vec<serde_json::Value>> {
        let p_option = self.player_data(data);
        if p_option.is_none() {
            return None;
        }

        let p = p_option.unwrap();

        let mut row = Vec::new();

        for header in headers {
            let header = header.as_str();
            row.push(match header {
                "Random" => json!(counter),
                "Name" => json!(self.full_name(data)),
                "Nation" => json!(self.nation_name(data)),
                "Second Nation" => json!(self.second_nation_name(data)),
                "Club Contracted" => json!(self.club_contracted_name(data)),
                "Club Playing" => json!(self.club_playing_name(data)),
                "Birthday" => json!(self.date_of_birth.to_days()),
                "Adaptability" => json!(self.adaptability),
                "Ambition" => json!(self.ambition),
                "Determination" => json!(self.determination),
                "Loyalty" => json!(self.loyalty),
                "Pressure" => json!(self.pressure),
                "Professionalism" => json!(self.professionalism),
                "Sportsmanship" => json!(self.sportsmanship),
                "Temperament" => json!(self.temperament),
                "Current Ability" => json!(p.current_ability),
                "Potential Ability" => json!(p.potential_ability),
                "Acceleration" => json!(p.acceleration),
                "Aggression" => json!(p.aggression),
                "Agility" => json!(p.agility),
                "Bravery" => json!(p.bravery),
                "Consistency" => json!(p.consistency),
                "Dirtiness" => json!(p.dirtiness),
                "Flair" => json!(p.flair),
                "Important Matches" => json!(p.important_matches),
                "Injury Proneness" => json!(p.injury_proneness),
                "Influence" => json!(p.leadership),
                "Natural Fitness" => json!(p.natural_fitness),
                "Speed" => json!(p.pace),
                "Stamina" => json!(p.stamina),
                "Strength" => json!(p.strength),
                "Teamwork" => json!(p.teamwork),
                "Versatility" => json!(p.versatility),
                "Work Rate" => json!(p.work_rate),
                "GK" => json!(p.goaltender),
                "LD" => json!(p.left_defence),
                "RD" => json!(p.right_defence),
                "LW" => json!(p.left_wing),
                "C" => json!(p.center),
                "RW" => json!(p.right_wing),
                "Agitation" => json!(p.agitation),
                "Defensive Role" => json!(p.defensive_role),
                "Offensive Role" => json!(p.offensive_role),
                "Pass Tendency" => json!(p.pass_tendency),

                attribute => json!(p.convert_attribute(attribute)),
            });
        }

        return Some(row);
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
