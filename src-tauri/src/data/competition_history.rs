use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::Data;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct CompetitionHistory {
    id: i32,
    _comp_id: i32,
    _winner_id: i32,
    _runner_up_id: i32,
    _third_placed_id: i32,
    _host_id: i32,
    _regular_season_winner_id: i32,
    _year: i16,
}

impl CompetitionHistory {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let history = Self::read(cursor)?;
        data.order_comp_history.push(history.id);
        data.comp_history.insert(history.id, history);

        return Ok(());
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let history = Self::read(cursor)?;
        data.order_nat_comp_history.push(history.id);
        data.nat_comp_history.insert(history.id, history);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._comp_id.to_le_bytes());
        bytes.extend_from_slice(&self._winner_id.to_le_bytes());
        bytes.extend_from_slice(&self._runner_up_id.to_le_bytes());
        bytes.extend_from_slice(&self._third_placed_id.to_le_bytes());
        bytes.extend_from_slice(&self._host_id.to_le_bytes());
        bytes.extend_from_slice(&self._regular_season_winner_id.to_le_bytes());
        bytes.extend_from_slice(&self._year.to_le_bytes());

        return bytes;
    }
}
