use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{
    data::{Data, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH, THREE_LETTER_TEXT_LENGTH},
    init::bytes_to_string,
    to_bytes::_chars_to_bytes,
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Currency {
    id: i32,
    _nation_id: i32,
    _exchange_rate: f64,
    _approx_exchange_rate: f64,
    _very_approx_exchange_rate: f64,
    _name_gender: i8,
    _short_name_gender: i8,
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_name: Vec<char>,
    _b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    _b_three_letter_code: [char; THREE_LETTER_TEXT_LENGTH as usize],
    _b_symbol: [char; THREE_LETTER_TEXT_LENGTH as usize],
}

impl Currency {
    fn _name(&self) -> String {
        return bytes_to_string(&self._b_name);
    }

    fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name);
    }

    fn _three_letter_code(&self) -> String {
        return bytes_to_string(&self._b_three_letter_code);
    }

    fn _symbol(&self) -> String {
        return bytes_to_string(&self._b_symbol);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let currency = Self::read(cursor)?;
        data.order_currencies.push(currency.id);
        data.currencies.insert(currency.id, currency);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._nation_id.to_le_bytes());
        bytes.extend_from_slice(&self._exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self._approx_exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self._very_approx_exchange_rate.to_le_bytes());
        bytes.extend_from_slice(&self._name_gender.to_le_bytes());
        bytes.extend_from_slice(&self._short_name_gender.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self._b_name));
        bytes.append(&mut _chars_to_bytes(&self._b_short_name));
        bytes.append(&mut _chars_to_bytes(&self._b_three_letter_code));
        bytes.append(&mut _chars_to_bytes(&self._b_symbol));

        return bytes;
    }
}
