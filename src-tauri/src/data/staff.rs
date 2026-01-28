use std::{collections::HashMap, io::Cursor};

use binread::{BinRead, Error};
use regex::Regex;
use serde_json::json;

use crate::{data::{Data, SIDate, city::City, club::Club, nation::Nation, player::Player}, research::db};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Staff {
    pub id: i32,
    estimated_wage: i32,
    estimated_value: i32,
    nation_id: i32,
    second_nation_id: i32,
    player_data_id: i32,
    preferences_id: i32,
    non_player_data_id: i32,
    nation_contracted_id: i32,
    club_contracted_id: i32,
    club_playing_id: i32,
    player_rights_index: i32,
    birth_town_id: i32,
    forename_id: i32,
    surname_id: i32,
    date_of_birth: SIDate,
    date_joined_nation: SIDate,
    contract_expires_nation: SIDate,
    date_joined_club: SIDate,
    contract_expires_club: SIDate,
    first_pro_contract: SIDate,
    pub age: i16,
    international_apps: u8,
    international_goals: u8,
    international_assists: u8,
    job_for_nation: i8,
    adaptability: i8,
    job_for_club: i8,
    ambition: i8,
    determination: i8,
    loyalty: i8,
    pressure: i8,
    professionalism: i8,
    sportsmanship: i8,
    temperament: i8,
    playing_squad: i8,
    classification: i8,
    club_valuation: i8,
    declared_nation: i8,
    stanley_cups_won: i8,
    squad_selected_for: i8,
    national_team_job_level: i8,
    estimated_wage_weekly: i32,
}

impl Staff {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let staff = Self::read(cursor)?;
        data.staff.insert(staff.id, staff);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.estimated_wage.to_le_bytes());
        bytes.extend_from_slice(&self.estimated_value.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.second_nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.player_data_id.to_le_bytes());
        bytes.extend_from_slice(&self.preferences_id.to_le_bytes());
        bytes.extend_from_slice(&self.non_player_data_id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_contracted_id.to_le_bytes());
        bytes.extend_from_slice(&self.club_contracted_id.to_le_bytes());
        bytes.extend_from_slice(&self.club_playing_id.to_le_bytes());
        bytes.extend_from_slice(&self.player_rights_index.to_le_bytes());
        bytes.extend_from_slice(&self.birth_town_id.to_le_bytes());
        bytes.extend_from_slice(&self.forename_id.to_le_bytes());
        bytes.extend_from_slice(&self.surname_id.to_le_bytes());
        bytes.append(&mut self.date_of_birth.to_bytes());
        bytes.append(&mut self.date_joined_nation.to_bytes());
        bytes.append(&mut self.contract_expires_nation.to_bytes());
        bytes.append(&mut self.date_joined_club.to_bytes());
        bytes.append(&mut self.contract_expires_club.to_bytes());
        bytes.append(&mut self.first_pro_contract.to_bytes());
        bytes.extend_from_slice(&self.age.to_le_bytes());
        bytes.extend_from_slice(&self.international_apps.to_le_bytes());
        bytes.extend_from_slice(&self.international_goals.to_le_bytes());
        bytes.extend_from_slice(&self.international_assists.to_le_bytes());
        bytes.extend_from_slice(&self.job_for_nation.to_le_bytes());
        bytes.extend_from_slice(&self.adaptability.to_le_bytes());
        bytes.extend_from_slice(&self.job_for_club.to_le_bytes());
        bytes.extend_from_slice(&self.ambition.to_le_bytes());
        bytes.extend_from_slice(&self.determination.to_le_bytes());
        bytes.extend_from_slice(&self.loyalty.to_le_bytes());
        bytes.extend_from_slice(&self.pressure.to_le_bytes());
        bytes.extend_from_slice(&self.professionalism.to_le_bytes());
        bytes.extend_from_slice(&self.sportsmanship.to_le_bytes());
        bytes.extend_from_slice(&self.temperament.to_le_bytes());
        bytes.extend_from_slice(&self.playing_squad.to_le_bytes());
        bytes.extend_from_slice(&self.classification.to_le_bytes());
        bytes.extend_from_slice(&self.club_valuation.to_le_bytes());
        bytes.extend_from_slice(&self.declared_nation.to_le_bytes());
        bytes.extend_from_slice(&self.stanley_cups_won.to_le_bytes());
        bytes.extend_from_slice(&self.squad_selected_for.to_le_bytes());
        bytes.extend_from_slice(&self.national_team_job_level.to_le_bytes());
        bytes.extend_from_slice(&self.estimated_wage_weekly.to_le_bytes());

        return bytes;
    }

    pub fn forename(&self, data: &Data) -> String {
        return data.forenames.get(&self.forename_id).unwrap().name();
    }

    pub fn surname(&self, data: &Data) -> String {
        return data.surnames.get(&self.surname_id).unwrap().name();
    }

    fn birth_town(&self, data: &Data) -> Option<City> {
        return data.cities.get(&self.birth_town_id).cloned();
    }

    pub fn birthplace(&self, data: &Data) -> String {
        let town = self.birth_town(data);
        if town.is_none() { return String::new(); }
        let town = town.unwrap();

        let state_string = match town.state_abbreviation(data) {
            Some(s) => format!(", {s}"),
            None => String::new(),
        };

        let nation_string = match town.nation_three_letter_name(data) {
            Some(s) => format!(", {s}"),
            None => String::new(),
        };

        return format!("{}{}{}", town.name(), state_string, nation_string);
    }

    fn nation(&self, data: &Data) -> Nation {
        return data.nations.get(&self.nation_id).cloned().unwrap();
    }

    pub fn nation_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n.name(),
            None => String::new(),
        };
    }

    fn nation_short_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n.short_name(),
            None => String::new()
        };
    }

    pub fn nation_three_letter_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.nation_id) {
            Some(n) => n.three_letter_name(),
            None => String::new()
        };
    }

    fn second_nation(&self, data: &Data) -> Option<Nation> {
        return data.nations.get(&self.second_nation_id).cloned();
    }

    pub fn second_nation_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n.name(),
            None => String::new(),
        };
    }

    fn second_nation_short_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n.short_name(),
            None => String::new(),
        };
    }

    pub fn second_nation_three_letter_name(&self, data: &Data) -> String {
        return match data.nations.get(&self.second_nation_id) {
            Some(n) => n.three_letter_name(),
            None => String::new(),
        };
    }

    fn club_contracted(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_contracted_id).cloned();
    }

    pub fn club_contracted_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c.name(),
            None => String::new(),
        };
    }

    pub fn club_contracted_short_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_contracted_id) {
            Some(c) => c.short_name(),
            None => String::new(),
        };
    }

    fn club_playing(&self, data: &Data) -> Option<Club> {
        return data.clubs.get(&self.club_playing_id).cloned();
    }

    fn club_playing_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_playing_id) {
            Some(c) => c.name(),
            None => String::new(),
        };
    }

    pub fn club_playing_short_name(&self, data: &Data) -> String {
        return match data.clubs.get(&self.club_playing_id) {
            Some(c) => c.short_name(),
            None => String::new(),
        };
    }

    fn birth_year(&self) -> i16 {
        return self.date_of_birth.year;
    }

    pub fn player_data(&self, data: &Data) -> Option<Player> {
        return data.players.get(&self.player_data_id).cloned();
    }

    // Check if the person's name only has letters in the English alphabet.
    pub fn has_standard_name(&self, data: &Data) -> bool {
        // Only use people whose names have basic characters.
        let re = Regex::new(r"[\w ]+").unwrap();
        return re.is_match(format!("{} {}", self.forename(data), self.surname(data)).as_str());
    }

    // Create an array of player data.
    pub fn create_player_view(&self, data: &Data) -> Option<serde_json::Value> {
        let p_option = self.player_data(data);
        if p_option.is_none() {
            return None;
        }

        let p = p_option.unwrap();
        return Some(json!([
            0.0,    // Used later as random seed.
            self.forename(data),
            self.surname(data),
            self.nation_name(data),
            self.second_nation_name(data),
            self.club_contracted_name(data),
            self.club_playing_name(data),
            self.birth_year(),
            self.adaptability,
            self.ambition,
            self.determination,
            self.loyalty,
            self.pressure,
            self.professionalism,
            self.sportsmanship,
            self.temperament,
            p.current_ability,
            p.potential_ability,
            p.acceleration,
            p.aggression,
            p.agility,
            p.anticipation_raw,
            p.balance_raw,
            p.bravery,
            p.consistency,
            p.decisions_raw,
            p.dirtiness,
            p.flair,
            p.important_matches,
            p.injury_proneness,
            p.leadership,
            p.movement_raw,
            p.natural_fitness,
            p.one_on_ones_raw,
            p.pace,
            p.passing_raw,
            p.positioning_raw,
            p.reflexes_raw,
            p.stamina,
            p.strength,
            p.teamwork,
            p.versatility,
            p.vision_raw,
            p.work_rate,
            p.goaltender,
            p.left_defence,
            p.right_defence,
            p.left_wing,
            p.center,
            p.right_wing,
            p.agitation,
            p.blocker_raw,
            p.checking_raw,
            p.defensive_role,
            p.deflections_raw,
            p.deking_raw,
            p.faceoffs_raw,
            p.fighting_raw,
            p.glove_raw,
            p.hitting_raw,
            p.offensive_role,
            p.pass_tendency,
            p.pokecheck_raw,
            p.rebounds_raw,
            p.recovery_raw,
            p.slapshot_raw,
            p.stickhandling_raw,
            p.wristshot_raw,

        ]));
    }

    pub fn merge_players(&self, db: &Data, save: &Data, key: &[String; 6], save_staff: &mut HashMap<[String; 6], Staff>) -> Option<db::Player> {
        let db_player = self.player_data(&db).unwrap();

        let save_person = save_staff.remove(key);
        if save_person.is_none() { return None; }
        let save_person = save_person.unwrap();
        let save_player = save_person.player_data(save).unwrap();

        return Some(db::Player {
            forename: self.forename(db),
            surname: self.surname(db),
            age: save_person.age,
            birthplace: save_person.birthplace(save),
            nation: save_person.nation_three_letter_name(save),
            second_nation: save_person.second_nation_three_letter_name(save),
            club: save_person.club_playing_short_name(save),
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
            pokecheck_save:save_player.pokecheck_raw,
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
