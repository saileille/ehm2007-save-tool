use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, LONG_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct RetiredNumber {
    id: i32,
    _club_id: i32,
    #[br(count = LONG_TEXT_LENGTH)]
    _b_player_name: Vec<char>,
    _number: u8,
}

impl RetiredNumber {
    fn _player_name(&self) -> String {
        return bytes_to_string(&self._b_player_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let number = Self::read(cursor)?;
        data.order_retired_numbers.push(number.id);
        data.retired_numbers.insert(number.id, number);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._club_id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_player_name));
        bytes.extend_from_slice(&self._number.to_le_bytes());

        return bytes;
    }
}
