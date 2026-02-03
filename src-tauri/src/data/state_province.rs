use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, REAL_SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StateProvince {
    id: i32,
    _nation_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<u8>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_short_name: Vec<u8>,
    _gender_name: i8,
    _b_abbreviation: [u8; REAL_SHORT_TEXT_LENGTH as usize],
}

impl StateProvince {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let province = Self::read(cursor)?;
        data.order_states_provinces.push(province.id);
        data.states_provinces.insert(province.id, province);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.append(&mut self._b_name.clone());
        bytes.append(&mut self._b_short_name.clone());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.append(&mut self._b_abbreviation.to_vec());

        return bytes;
    }

    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name).unwrap();
    }

    fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name).unwrap();
    }

    pub fn _abbreviation(&self) -> String {
        return bytes_to_string(&self._b_abbreviation).unwrap();
    }
}
