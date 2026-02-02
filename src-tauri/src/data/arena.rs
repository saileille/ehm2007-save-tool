use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Arena {
    id: i32,
    _capacity: i32,
    _seating_capacity: i32,
    _expansion_capacity: i32,
    _city_id: i32,
    _nearby_stadium_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _gender_name: i8,
    _rink_size: i8,
    _ice_condition: i8,
}

impl Arena {
    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let arena = Self::read(cursor)?;
        data.order_arenas.push(arena.id);
        data.arenas.insert(arena.id, arena);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._capacity.to_le_bytes());
        bytes.extend_from_slice(&self._seating_capacity.to_le_bytes());
        bytes.extend_from_slice(&self._expansion_capacity.to_le_bytes());
        bytes.extend_from_slice(&self._city_id.to_le_bytes());
        bytes.extend_from_slice(&self._nearby_stadium_id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._rink_size.to_le_bytes());
        bytes.extend_from_slice(&self._ice_condition.to_le_bytes());

        return bytes;
    }
}
