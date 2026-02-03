use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, STANDARD_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Colour {
    id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<u8>,
    _red: u8,
    _green: u8,
    _blue: u8,
}

impl Colour {
    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name).unwrap();
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let colour = Self::read(cursor)?;
        data.order_colours.push(colour.id);
        data.colours.insert(colour.id, colour);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut self._b_name.clone());
        bytes.extend_from_slice(&self._red.to_le_bytes());
        bytes.extend_from_slice(&self._green.to_le_bytes());
        bytes.extend_from_slice(&self._blue.to_le_bytes());

        return bytes;
    }
}
