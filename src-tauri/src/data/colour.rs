use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Colour {
    id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    red: u8,
    green: u8,
    blue: u8,
}

impl Colour {
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let colour = Self::read(cursor)?;
        data.colours.insert(colour.id, colour);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.red.to_le_bytes());
        bytes.extend_from_slice(&self.green.to_le_bytes());
        bytes.extend_from_slice(&self.blue.to_le_bytes());

        return bytes;
    }
}
