use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::{Data, SIDate};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Official {
    id: i32,
    forename_id: i32,
    surname_id: i32,
    nation_id: i32,
    city_id: i32,
    comp_id: i32,
    year_of_birth: i16,
    current_ability: i16,
    potential_ability: i16,
    reputation: i16,
    date_of_birth: SIDate,
    allowing_roughness: i8,
    discipline: i8,
    important_matches: i8,
    pressure: i8,
    refereeing: i8,
    skating_line: i8,
    allowing_interference: i8,
}

impl Official {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let official = Self::read(cursor)?;
        data.order_officials.push(official.id);
        data.officials.insert(official.id, official);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.forename_id.to_le_bytes());
        bytes.extend_from_slice(&self.surname_id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.city_id.to_le_bytes());
        bytes.extend_from_slice(&self.comp_id.to_le_bytes());
        bytes.extend_from_slice(&self.year_of_birth.to_le_bytes());
        bytes.extend_from_slice(&self.current_ability.to_le_bytes());
        bytes.extend_from_slice(&self.potential_ability.to_le_bytes());
        bytes.extend_from_slice(&self.reputation.to_le_bytes());
        bytes.append(&mut self.date_of_birth.to_bytes());
        bytes.extend_from_slice(&self.allowing_roughness.to_le_bytes());
        bytes.extend_from_slice(&self.discipline.to_le_bytes());
        bytes.extend_from_slice(&self.important_matches.to_le_bytes());
        bytes.extend_from_slice(&self.pressure.to_le_bytes());
        bytes.extend_from_slice(&self.refereeing.to_le_bytes());
        bytes.extend_from_slice(&self.skating_line.to_le_bytes());
        bytes.extend_from_slice(&self.allowing_interference.to_le_bytes());

        return bytes;
    }
}
