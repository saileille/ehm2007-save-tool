use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StaffAward {
    id: i32,
    continent_id: i32,
    nation_id: i32,
    comp_id: i32,
    foreground_colour_id: i32,
    background_colour_id: i32,
    trim_colour_id: i32,
    reputation: i16,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    gender_name_short: i8,
    gender_name: i8,
}

impl StaffAward {
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let award = Self::read(cursor)?;
        data.order_staff_awards.push(award.id);
        data.staff_awards.insert(award.id, award);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.continent_id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.comp_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self.reputation.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());

        return bytes;
    }
}
