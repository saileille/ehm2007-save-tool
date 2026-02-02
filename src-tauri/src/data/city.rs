use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{nation::Nation, state_province::StateProvince, Data, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct City {
    _latitude: f64,
    _longitude: f64,
    id: i32,
    _state_id: i32,
    _nation_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _gender_name: i8,
    _attraction: i8,
}

impl City {
    pub fn _name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    fn _nation(&self, data: &Data) -> Nation {
        return data.nations.get(&self._nation_id).unwrap().clone();
    }

    pub fn _nation_three_letter_name(&self, data: &Data) -> Option<String> {
        match data.nations.get(&self._nation_id) {
            Some(n) => Some(n._three_letter_name()),
            None => None,
        }
    }

    fn _state(&self, data: &Data) -> StateProvince {
        return data.states_provinces.get(&self._state_id).unwrap().clone();
    }

    pub fn _state_abbreviation(&self, data: &Data) -> Option<String> {
        match data.states_provinces.get(&self._state_id) {
            Some(s) => Some(s._abbreviation()),
            None => None,
        }
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let city = Self::read(cursor)?;
        data.order_cities.push(city.id);
        data.cities.insert(city.id, city);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self._latitude.to_le_bytes());
        bytes.extend_from_slice(&self._longitude.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._state_id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._attraction.to_le_bytes());

        return bytes;
    }
}
