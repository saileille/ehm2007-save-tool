use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Arena {
    id: i32,
    capacity: i32,
    seating_capacity: i32,
    expansion_capacity: i32,
    city_id: i32,
    nearby_stadium_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    gender_name: i8,
    rink_size: i8,
    ice_condition: i8,
}

impl Arena {
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let arena = Self::read(cursor)?;
        data.arenas.insert(arena.id, arena);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.capacity.to_le_bytes());
        bytes.extend_from_slice(&self.seating_capacity.to_le_bytes());
        bytes.extend_from_slice(&self.expansion_capacity.to_le_bytes());
        bytes.extend_from_slice(&self.city_id.to_le_bytes());
        bytes.extend_from_slice(&self.nearby_stadium_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.rink_size.to_le_bytes());
        bytes.extend_from_slice(&self.ice_condition.to_le_bytes());

        return bytes;
    }
}
