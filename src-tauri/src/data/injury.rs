use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Injury {
    minimum_days_out: i16,
    extra_days_out: i16,
    id: i16,
    category: i8,
    chance: i8,
    inactive_ratio: i8,
    b_is_recurring: u8,
    severity: i8,
    cause: i8,
    gender: i8,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name_dative: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name_genetive: Vec<char>,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name_special_nominative: Vec<char>,
}

impl Injury {
    fn is_recurring(&self) -> bool {
        return self.b_is_recurring != 0;
    }

    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn name_dative(&self) -> String {
        return bytes_to_string(&self.b_name_dative);
    }

    fn name_genetive(&self) -> String {
        return bytes_to_string(&self.b_name_genetive);
    }

    fn name_special_nominative(&self) -> String {
        return bytes_to_string(&self.b_name_special_nominative);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let injury = Self::read(cursor)?;
        data.order_injuries.push(injury.id);
        data.injuries.insert(injury.id, injury);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.minimum_days_out.to_le_bytes());
        bytes.extend_from_slice(&self.extra_days_out.to_le_bytes());
        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.category.to_le_bytes());
        bytes.extend_from_slice(&self.chance.to_le_bytes());
        bytes.extend_from_slice(&self.inactive_ratio.to_le_bytes());
        bytes.extend_from_slice(&self.b_is_recurring.to_le_bytes());
        bytes.extend_from_slice(&self.severity.to_le_bytes());
        bytes.extend_from_slice(&self.cause.to_le_bytes());
        bytes.extend_from_slice(&self.gender.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_name_dative));
        bytes.append(&mut chars_to_bytes(&self.b_name_genetive));
        bytes.append(&mut chars_to_bytes(&self.b_name_special_nominative));

        return bytes;
    }
}
