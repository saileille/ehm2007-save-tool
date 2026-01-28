use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, SIX_LETTER_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Competition {
    id: i32,
    continent_id: i32,
    nation_id: i32,
    foreground_colour_id: i32,
    background_colour_id: i32,
    trim_colour_id: i32,
    b_six_letter_name: [char; SIX_LETTER_TEXT_LENGTH as usize],
    gender_name_short: i8,
    scope: i8,
    reputation: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_playoff_trophy_name: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_regular_season_trophy_name: Vec<char>,
    playoff_trophy_gender: i8,
    regular_season_trophy_gender: i8,
    selected: i8,
    gender_name: i8,
    upper_age_limit: i8,
}

impl Competition {
    fn six_letter_name(&self) -> String {
        return bytes_to_string(&self.b_six_letter_name);
    }

    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    fn playoff_trophy_name(&self) -> String {
        return bytes_to_string(&self.b_playoff_trophy_name);
    }

    fn regular_season_trophy_name(&self) -> String {
        return bytes_to_string(&self.b_regular_season_trophy_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let comp = Self::read(cursor)?;
        data.competitions.insert(comp.id, comp);

        return Ok(())
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let comp = Self::read(cursor)?;
        data.nat_competitions.insert(comp.id, comp);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.continent_id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_six_letter_name));
        bytes.extend_from_slice(&self.gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self.scope.to_le_bytes());
        bytes.extend_from_slice(&self.reputation.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_playoff_trophy_name));
        bytes.append(&mut chars_to_bytes(&self.b_regular_season_trophy_name));
        bytes.extend_from_slice(&self.playoff_trophy_gender.to_le_bytes());
        bytes.extend_from_slice(&self.regular_season_trophy_gender.to_le_bytes());
        bytes.extend_from_slice(&self.selected.to_le_bytes());
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.upper_age_limit.to_le_bytes());

        return bytes;
    }
}
