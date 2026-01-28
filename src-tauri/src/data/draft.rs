use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Draft {
    id: i32,
    comp_id: i32,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    gender_name: i8,
    b_is_extinct: u8,
}

impl Draft {
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn is_extinct(&self) -> bool {
        return self.b_is_extinct != 0;
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let draft = Self::read(cursor)?;
        data.drafts.insert(draft.id, draft);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.comp_id.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.b_is_extinct.to_le_bytes());

        return bytes;
    }
}
