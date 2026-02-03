use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Nation {
    _iihf_ranking_score: f64,
    _iihf_ranking_score_98: f64,
    _iihf_ranking_score_99: f64,
    _iihf_ranking_score_00: f64,
    _iihf_ranking_score_01: f64,
    _iihf_ranking_score_02: f64,
    _iihf_ranking_score_03: f64,
    _jr_final_position_1: i8,
    _jr_final_position_2: i8,
    _jr_final_position_3: i8,
    _men_final_position_1: i8,
    _men_final_position_2: i8,
    _men_final_position_3: i8,
    pub id: i32,
    _number_staff: i32,
    _continent_id: i32,
    _capital_id: i32,
    _national_stadium_id: i32,
    _foreground_colour1_id: i32,
    _background_colour1_id: i32,
    _trim_colour1_id: i32,
    _foreground_colour2_id: i32,
    _background_colour2_id: i32,
    _trim_colour2_id: i32,
    _foreground_colour3_id: i32,
    _background_colour3_id: i32,
    _trim_colour3_id: i32,
    _rival1_id: i32,
    _rival2_id: i32,
    _rival3_id: i32,
    _b_three_letter_name: [u8; THREE_LETTER_TEXT_LENGTH as usize],
    _number_clubs: i16,
    _reputation: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<u8>,
    _b_short_name: [u8; SHORT_TEXT_LENGTH as usize],
    _b_nationality_name: [u8; SHORT_TEXT_LENGTH as usize],
    _gender_name: i8,
    _gender_name_short: i8,
    _region: i8,
    _actual_region: i8,
    _first_language: i8,
    _second_language: i8,
    _third_language: i8,
    _state_of_development: i8,
    _group_membership: i8,
    _game_importance: i8,
    _league_standard: i8,
    _league_selected: i8,
    _games_played: i8,
    _citizenship_years: i8,
}

impl Nation {
    pub fn _three_letter_name(&self) -> String {
        return bytes_to_string(&self._b_three_letter_name).unwrap();
    }

    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name).unwrap();
    }

    pub fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name).unwrap();
    }

    fn _nationality_name(&self) -> String {
        return bytes_to_string(&self._b_nationality_name).unwrap();
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let nation = Self::read(cursor)?;
        data.order_nations.push(nation.id);
        data.nations.insert(nation.id, nation);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self._iihf_ranking_score.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_98.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_99.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_00.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_01.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_02.to_le_bytes());
        bytes.extend_from_slice(&self._iihf_ranking_score_03.to_le_bytes());
        bytes.extend_from_slice(&self._jr_final_position_1.to_le_bytes());
        bytes.extend_from_slice(&self._jr_final_position_2.to_le_bytes());
        bytes.extend_from_slice(&self._jr_final_position_3.to_le_bytes());
        bytes.extend_from_slice(&self._men_final_position_1.to_le_bytes());
        bytes.extend_from_slice(&self._men_final_position_2.to_le_bytes());
        bytes.extend_from_slice(&self._men_final_position_3.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._number_staff.to_le_bytes());
        bytes.extend_from_slice(&self._continent_id.to_le_bytes());
        bytes.extend_from_slice(&self._capital_id.to_le_bytes());
        bytes.extend_from_slice(&self._national_stadium_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival1_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival2_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival3_id.to_le_bytes());
        bytes.append(&mut self._b_three_letter_name.to_vec());
        bytes.extend_from_slice(&self._number_clubs.to_le_bytes());
        bytes.extend_from_slice(&self._reputation.to_le_bytes());
        bytes.append(&mut self.b_name.clone());
        bytes.append(&mut self._b_short_name.to_vec());
        bytes.append(&mut self._b_nationality_name.to_vec());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self._region.to_le_bytes());
        bytes.extend_from_slice(&self._actual_region.to_le_bytes());
        bytes.extend_from_slice(&self._first_language.to_le_bytes());
        bytes.extend_from_slice(&self._second_language.to_le_bytes());
        bytes.extend_from_slice(&self._third_language.to_le_bytes());
        bytes.extend_from_slice(&self._state_of_development.to_le_bytes());
        bytes.extend_from_slice(&self._group_membership.to_le_bytes());
        bytes.extend_from_slice(&self._game_importance.to_le_bytes());
        bytes.extend_from_slice(&self._league_standard.to_le_bytes());
        bytes.extend_from_slice(&self._league_selected.to_le_bytes());
        bytes.extend_from_slice(&self._games_played.to_le_bytes());
        bytes.extend_from_slice(&self._citizenship_years.to_le_bytes());

        return bytes;
    }
}
