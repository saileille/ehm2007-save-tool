use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH, nation::Nation, state_province::StateProvince}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct City {
    latitude: f64,
    longitude: f64,
    id: i32,
    state_id: i32,
    nation_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    gender_name: i8,
    attraction: i8,
}

impl City {
    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn nation(&self, data: &Data) -> Nation {
        return data.nations.get(&self.nation_id).unwrap().clone();
    }

    pub fn nation_three_letter_name(&self, data: &Data) -> Option<String> {
        match data.nations.get(&self.nation_id) {
            Some(n) => Some(n.three_letter_name()),
            None => None,
        }
    }

    fn state(&self, data: &Data) -> StateProvince {
        return data.states_provinces.get(&self.state_id).unwrap().clone();
    }

    pub fn state_abbreviation(&self, data: &Data) -> Option<String> {
        match data.states_provinces.get(&self.state_id) {
            Some(s) => Some(s.abbreviation()),
            None => None
        }
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let city = Self::read(cursor)?;
        data.order_cities.push(city.id);
        data.cities.insert(city.id, city);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.latitude.to_le_bytes());
        bytes.extend_from_slice(&self.longitude.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.state_id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.attraction.to_le_bytes());

        return bytes;
    }
}
