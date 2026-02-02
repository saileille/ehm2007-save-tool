use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StaffAward {
    id: i32,
    _continent_id: i32,
    _nation_id: i32,
    _comp_id: i32,
    _foreground_colour_id: i32,
    _background_colour_id: i32,
    _trim_colour_id: i32,
    _reputation: i16,
    _b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _gender_name_short: i8,
    _gender_name: i8,
}

impl StaffAward {
    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let award = Self::read(cursor)?;
        data.order_staff_awards.push(award.id);
        data.staff_awards.insert(award.id, award);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._continent_id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.extend_from_slice(&self._comp_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour_id.to_le_bytes());
        bytes.extend_from_slice(&self._reputation.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_short_name));
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.extend_from_slice(&self._gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());

        return bytes;
    }
}
