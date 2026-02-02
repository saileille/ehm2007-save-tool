use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Draft {
    id: i32,
    _comp_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _gender_name: i8,
    _b_is_extinct: u8,
}

impl Draft {
    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    fn _is_extinct(&self) -> bool {
        return self._b_is_extinct != 0;
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let draft = Self::read(cursor)?;
        data.order_drafts.push(draft.id);
        data.drafts.insert(draft.id, draft);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._comp_id.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._b_is_extinct.to_le_bytes());

        return bytes;
    }
}
