use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, REAL_SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StateProvince {
    id: i32,
    nation_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_short_name: Vec<char>,
    gender_name: i8,
    b_abbreviation: [char; REAL_SHORT_TEXT_LENGTH as usize],
}

impl StateProvince {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let province = Self::read(cursor)?;
        data.states_provinces.insert(province.id, province);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_abbreviation));

        return bytes;
    }

    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    pub fn abbreviation(&self) -> String {
        return bytes_to_string(&self.b_abbreviation);
    }
}
