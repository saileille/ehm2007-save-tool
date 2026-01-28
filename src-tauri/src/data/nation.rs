use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Nation {
    iihf_ranking_score: f64,
    iihf_ranking_score_98: f64,
    iihf_ranking_score_99: f64,
    iihf_ranking_score_00: f64,
    iihf_ranking_score_01: f64,
    iihf_ranking_score_02: f64,
    iihf_ranking_score_03: f64,
    jr_final_position_1: i8,
    jr_final_position_2: i8,
    jr_final_position_3: i8,
    men_final_position_1: i8,
    men_final_position_2: i8,
    men_final_position_3: i8,
    id: i32,
    number_staff: i32,
    continent_id: i32,
    capital_id: i32,
    national_stadium_id: i32,
    foreground_colour1_id: i32,
    background_colour1_id: i32,
    trim_colour1_id: i32,
    foreground_colour2_id: i32,
    background_colour2_id: i32,
    trim_colour2_id: i32,
    foreground_colour3_id: i32,
    background_colour3_id: i32,
    trim_colour3_id: i32,
    rival1_id: i32,
    rival2_id: i32,
    rival3_id: i32,
    b_three_letter_name: [char; THREE_LETTER_TEXT_LENGTH as usize],
    number_clubs: i16,
    reputation: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    b_nationality_name: [char; SHORT_TEXT_LENGTH as usize],
    gender_name: i8,
    gender_name_short: i8,
    region: i8,
    actual_region: i8,
    first_language: i8,
    second_language: i8,
    third_language: i8,
    state_of_development: i8,
    group_membership: i8,
    game_importance: i8,
    league_standard: i8,
    league_selected: i8,
    games_played: i8,
    citizenship_years: i8,
}

impl Nation {
    pub fn three_letter_name(&self) -> String {
        return bytes_to_string(&self.b_three_letter_name);
    }

    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    pub fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    fn nationality_name(&self) -> String {
        return bytes_to_string(&self.b_nationality_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let nation = Self::read(cursor)?;
        data.nations.insert(nation.id, nation);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.iihf_ranking_score.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_98.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_99.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_00.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_01.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_02.to_le_bytes());
        bytes.extend_from_slice(&self.iihf_ranking_score_03.to_le_bytes());
        bytes.extend_from_slice(&self.jr_final_position_1.to_le_bytes());
        bytes.extend_from_slice(&self.jr_final_position_2.to_le_bytes());
        bytes.extend_from_slice(&self.jr_final_position_3.to_le_bytes());
        bytes.extend_from_slice(&self.men_final_position_1.to_le_bytes());
        bytes.extend_from_slice(&self.men_final_position_2.to_le_bytes());
        bytes.extend_from_slice(&self.men_final_position_3.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.number_staff.to_le_bytes());
        bytes.extend_from_slice(&self.continent_id.to_le_bytes());
        bytes.extend_from_slice(&self.capital_id.to_le_bytes());
        bytes.extend_from_slice(&self.national_stadium_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival1_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival2_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival3_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_three_letter_name));
        bytes.extend_from_slice(&self.number_clubs.to_le_bytes());
        bytes.extend_from_slice(&self.reputation.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_nationality_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self.region.to_le_bytes());
        bytes.extend_from_slice(&self.actual_region.to_le_bytes());
        bytes.extend_from_slice(&self.first_language.to_le_bytes());
        bytes.extend_from_slice(&self.second_language.to_le_bytes());
        bytes.extend_from_slice(&self.third_language.to_le_bytes());
        bytes.extend_from_slice(&self.state_of_development.to_le_bytes());
        bytes.extend_from_slice(&self.group_membership.to_le_bytes());
        bytes.extend_from_slice(&self.game_importance.to_le_bytes());
        bytes.extend_from_slice(&self.league_standard.to_le_bytes());
        bytes.extend_from_slice(&self.league_selected.to_le_bytes());
        bytes.extend_from_slice(&self.games_played.to_le_bytes());
        bytes.extend_from_slice(&self.citizenship_years.to_le_bytes());

        return bytes;
    }
}
