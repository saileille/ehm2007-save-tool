use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, STANDARD_TEXT_LENGTH}
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Injury {
    _minimum_days_out: i16,
    _extra_days_out: i16,
    id: i16,
    _category: i8,
    _chance: i8,
    _inactive_ratio: i8,
    _b_is_recurring: u8,
    _severity: i8,
    _cause: i8,
    _gender: i8,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<u8>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name_dative: Vec<u8>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name_genetive: Vec<u8>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name_special_nominative: Vec<u8>,
}

impl Injury {
    fn _is_recurring(&self) -> bool {
        return self._b_is_recurring != 0;
    }

    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name).unwrap();
    }

    fn _name_dative(&self) -> String {
        return bytes_to_string(&self._b_name_dative).unwrap();
    }

    fn _name_genetive(&self) -> String {
        return bytes_to_string(&self._b_name_genetive).unwrap();
    }

    fn _name_special_nominative(&self) -> String {
        return bytes_to_string(&self._b_name_special_nominative).unwrap();
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let injury = Self::read(cursor)?;
        data.order_injuries.push(injury.id);
        data.injuries.insert(injury.id, injury);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self._minimum_days_out.to_le_bytes());
        bytes.extend_from_slice(&self._extra_days_out.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._category.to_le_bytes());
        bytes.extend_from_slice(&self._chance.to_le_bytes());
        bytes.extend_from_slice(&self._inactive_ratio.to_le_bytes());
        bytes.extend_from_slice(&self._b_is_recurring.to_le_bytes());
        bytes.extend_from_slice(&self._severity.to_le_bytes());
        bytes.extend_from_slice(&self._cause.to_le_bytes());
        bytes.extend_from_slice(&self._gender.to_le_bytes());
        bytes.append(&mut self._b_name.clone());
        bytes.append(&mut self._b_name_dative.clone());
        bytes.append(&mut self._b_name_genetive.clone());
        bytes.append(&mut self._b_name_special_nominative.clone());

        return bytes;
    }
}
