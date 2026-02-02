use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::{Data, SIDate};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Official {
    id: i32,
    _forename_id: i32,
    _surname_id: i32,
    _nation_id: i32,
    _city_id: i32,
    _comp_id: i32,
    _year_of_birth: i16,
    _current_ability: i16,
    _potential_ability: i16,
    _reputation: i16,
    _date_of_birth: SIDate,
    _allowing_roughness: i8,
    _discipline: i8,
    _important_matches: i8,
    _pressure: i8,
    _refereeing: i8,
    _skating_line: i8,
    _allowing_interference: i8,
}

impl Official {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let official = Self::read(cursor)?;
        data.order_officials.push(official.id);
        data.officials.insert(official.id, official);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._forename_id.to_le_bytes());
        bytes.extend_from_slice(&self._surname_id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.extend_from_slice(&self._city_id.to_le_bytes());
        bytes.extend_from_slice(&self._comp_id.to_le_bytes());
        bytes.extend_from_slice(&self._year_of_birth.to_le_bytes());
        bytes.extend_from_slice(&self._current_ability.to_le_bytes());
        bytes.extend_from_slice(&self._potential_ability.to_le_bytes());
        bytes.extend_from_slice(&self._reputation.to_le_bytes());
        bytes.append(&mut self._date_of_birth._to_bytes());
        bytes.extend_from_slice(&self._allowing_roughness.to_le_bytes());
        bytes.extend_from_slice(&self._discipline.to_le_bytes());
        bytes.extend_from_slice(&self._important_matches.to_le_bytes());
        bytes.extend_from_slice(&self._pressure.to_le_bytes());
        bytes.extend_from_slice(&self._refereeing.to_le_bytes());
        bytes.extend_from_slice(&self._skating_line.to_le_bytes());
        bytes.extend_from_slice(&self._allowing_interference.to_le_bytes());

        return bytes;
    }
}
