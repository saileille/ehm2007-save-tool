use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, SHORT_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Continent {
    _regional_strength: f64,
    id: i32,
    _b_three_letter_name: [u8; THREE_LETTER_TEXT_LENGTH as usize],
    _b_name: [u8; SHORT_TEXT_LENGTH as usize],
    _b_continentality_name: [u8; SHORT_TEXT_LENGTH as usize],
    _gender_name: i8,
}

impl Continent {
    fn _three_letter_name(&self) -> String {
        return bytes_to_string(&self._b_three_letter_name).unwrap();
    }

    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name).unwrap();
    }

    fn _continentality_name(&self) -> String {
        return bytes_to_string(&self._b_continentality_name).unwrap();
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let continent = Self::read(cursor)?;
        data.order_continents.push(continent.id);
        data.continents.insert(continent.id, continent);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self._regional_strength.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut self._b_three_letter_name.to_vec());
        bytes.append(&mut self._b_name.to_vec());
        bytes.append(&mut self._b_continentality_name.to_vec());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());

        return bytes;
    }
}
