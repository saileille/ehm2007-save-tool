use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StageName {
    id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_long_name: Vec<char>,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_plural_name: Vec<char>,
    long_name_gender: i8,
    short_name_gender: i8,
    plural_name_gender: i8,
}

impl StageName {
    fn long_name(&self) -> String {
        return bytes_to_string(&self.b_long_name);
    }

    fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let stage_name = Self::read(cursor)?;
        data.order_stage_names.push(stage_name.id);
        data.stage_names.insert(stage_name.id, stage_name);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_long_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_plural_name));
        bytes.extend_from_slice(&self.long_name_gender.to_le_bytes());
        bytes.extend_from_slice(&self.short_name_gender.to_le_bytes());
        bytes.extend_from_slice(&self.plural_name_gender.to_le_bytes());

        return bytes;
    }
}
