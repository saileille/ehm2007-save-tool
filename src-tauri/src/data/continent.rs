use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Continent {
    regional_strength: f64,
    id: i32,
    b_three_letter_name: [char; THREE_LETTER_TEXT_LENGTH as usize],
    b_name: [char; SHORT_TEXT_LENGTH as usize],
    b_continentality_name: [char; SHORT_TEXT_LENGTH as usize],
    gender_name: i8,
}

impl Continent {
    fn three_letter_name(&self) -> String {
        return bytes_to_string(&self.b_three_letter_name);
    }

    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn continentality_name(&self) -> String {
        return bytes_to_string(&self.b_continentality_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let continent = Self::read(cursor)?;
        data.continents.insert(continent.id, continent);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.regional_strength.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_three_letter_name));
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_continentality_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());

        return bytes;
    }
}
