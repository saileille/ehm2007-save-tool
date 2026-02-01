use core::num;
use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, LONG_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct RetiredNumber {
    id: i32,
    club_id: i32,
    #[br(count = LONG_TEXT_LENGTH)]
    b_player_name: Vec<char>,
    number: u8,
}

impl RetiredNumber {
    fn player_name(&self) -> String {
        return bytes_to_string(&self.b_player_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let number = Self::read(cursor)?;
        data.order_retired_numbers.push(number.id);
        data.retired_numbers.insert(number.id, number);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.club_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_player_name));
        bytes.extend_from_slice(&self.number.to_le_bytes());

        return bytes;
    }
}
