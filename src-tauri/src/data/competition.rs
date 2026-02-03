use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, SHORT_TEXT_LENGTH, SIX_LETTER_TEXT_LENGTH, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Competition {
    pub id: i32,
    _continent_id: i32,
    _nation_id: i32,
    _foreground_colour_id: i32,
    _background_colour_id: i32,
    _trim_colour_id: i32,
    _b_six_letter_name: [char; SIX_LETTER_TEXT_LENGTH as usize],
    _gender_name_short: i8,
    _scope: i8,
    _reputation: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_playoff_trophy_name: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_regular_season_trophy_name: Vec<char>,
    _playoff_trophy_gender: i8,
    _regular_season_trophy_gender: i8,
    _selected: i8,
    _gender_name: i8,
    _upper_age_limit: i8,
}

impl Competition {
    fn _six_letter_name(&self) -> String {
        return bytes_to_string(&self._b_six_letter_name);
    }

    pub fn name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name);
    }

    fn _playoff_trophy_name(&self) -> String {
        return bytes_to_string(&self._b_playoff_trophy_name);
    }

    fn _regular_season_trophy_name(&self) -> String {
        return bytes_to_string(&self._b_regular_season_trophy_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let comp = Self::read(cursor)?;
        data.order_competitions.push(comp.id);
        data.competitions.insert(comp.id, comp);

        return Ok(());
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let comp = Self::read(cursor)?;
        data.order_nat_competitions.push(comp.id);
        data.nat_competitions.insert(comp.id, comp);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._continent_id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour_id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_six_letter_name));
        bytes.extend_from_slice(&self._gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self._scope.to_le_bytes());
        bytes.extend_from_slice(&self._reputation.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.append(&mut _chars_to_bytes(&self._b_short_name));
        bytes.append(&mut _chars_to_bytes(&self._b_playoff_trophy_name));
        bytes.append(&mut _chars_to_bytes(&self._b_regular_season_trophy_name));
        bytes.extend_from_slice(&self._playoff_trophy_gender.to_le_bytes());
        bytes.extend_from_slice(&self._regular_season_trophy_gender.to_le_bytes());
        bytes.extend_from_slice(&self._selected.to_le_bytes());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._upper_age_limit.to_le_bytes());

        return bytes;
    }
}
