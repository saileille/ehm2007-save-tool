use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StageName {
    id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_long_name: Vec<char>,
    _b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_plural_name: Vec<char>,
    _long_name_gender: i8,
    _short_name_gender: i8,
    _plural_name_gender: i8,
}

impl StageName {
    fn _long_name(&self) -> String {
        return bytes_to_string(&self._b_long_name);
    }

    fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let stage_name = Self::read(cursor)?;
        data.order_stage_names.push(stage_name.id);
        data.stage_names.insert(stage_name.id, stage_name);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_long_name));
        bytes.append(&mut _chars_to_bytes(&self._b_short_name));
        bytes.append(&mut _chars_to_bytes(&self._b_plural_name));
        bytes.extend_from_slice(&self._long_name_gender.to_le_bytes());
        bytes.extend_from_slice(&self._short_name_gender.to_le_bytes());
        bytes.extend_from_slice(&self._plural_name_gender.to_le_bytes());

        return bytes;
    }
}
