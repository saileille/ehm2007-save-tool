use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Name {
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    id: i32,
    nation_id: i32,
    count: i8,
}

impl Name {
    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    pub fn parse_forename(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let name = Self::read(cursor)?;
        data.order_forenames.push(name.id);
        data.forenames.insert(name.id, name);

        return Ok(())
    }

    pub fn parse_surname(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let name = Self::read(cursor)?;
        data.order_surnames.push(name.id);
        data.surnames.insert(name.id, name);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.count.to_le_bytes());

        return bytes;
    }
}
