use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, LONG_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct RetiredNumber {
    id: i32,
    _club_id: i32,
    #[br(count = LONG_TEXT_LENGTH)]
    _b_player_name: Vec<u8>,
    _number: u8,
}

impl RetiredNumber {
    fn _player_name(&self) -> String {
        return bytes_to_string(&self._b_player_name).unwrap();
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
        bytes.append(&mut self._b_player_name.clone());
        bytes.extend_from_slice(&self._number.to_le_bytes());

        return bytes;
    }
}
