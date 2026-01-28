use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH}, init::bytes_to_string, to_bytes::chars_to_bytes};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Currency {
    id: i32,
    nation_id: i32,
    exchange_rate: f64,
    approx_exchange_rate: f64,
    very_approx_exchange_rate: f64,
    name_gender: i8,
    short_name_gender: i8,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    b_three_letter_code: [char; THREE_LETTER_TEXT_LENGTH as usize],
    b_symbol: [char; THREE_LETTER_TEXT_LENGTH as usize],
}

impl Currency {
    fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    fn three_letter_code(&self) -> String {
        return bytes_to_string(&self.b_three_letter_code);
    }

    fn symbol(&self) -> String {
        return bytes_to_string(&self.b_symbol);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let currency = Self::read(cursor)?;
        data.currencies.insert(currency.id, currency);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self.approx_exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self.very_approx_exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self.name_gender.to_le_bytes());
        bytes.extend_from_slice(&self.short_name_gender.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_three_letter_code));
        bytes.append(&mut chars_to_bytes(&self.b_symbol));

        return bytes;
    }
}
