use std::{io::Cursor, str::Utf8Error};

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, STANDARD_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Name {
    #[br(count = STANDARD_TEXT_LENGTH)]
    pub b_name: Vec<u8>,
    id: i32,
    _nation_id: i32,
    _count: i8,
}

impl Name {
    pub fn name(&self) -> Result<String, Utf8Error> {
        return bytes_to_string(&self.b_name);
    }

    pub fn parse_forename(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let name = Self::read(cursor)?;
        data.order_forenames.push(name.id);
        data.forenames.insert(name.id, name);

        return Ok(());
    }

    pub fn parse_surname(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let name = Self::read(cursor)?;
        data.order_surnames.push(name.id);
        data.surnames.insert(name.id, name);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.b_name.clone());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.extend_from_slice(&self._count.to_le_bytes());

        return bytes;
    }
}
