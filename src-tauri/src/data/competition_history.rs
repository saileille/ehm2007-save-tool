use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::Data;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct CompetitionHistory {
    id: i32,
    comp_id: i32,
    winner_id: i32,
    runner_up_id: i32,
    third_placed_id: i32,
    host_id: i32,
    regular_season_winner_id: i32,
    year: i16,
}

impl CompetitionHistory {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let history = Self::read(cursor)?;
        data.comp_history.insert(history.id, history);

        return Ok(())
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let history = Self::read(cursor)?;
        data.nat_comp_history.insert(history.id, history);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.comp_id.to_le_bytes());
        bytes.extend_from_slice(&self.winner_id.to_le_bytes());
        bytes.extend_from_slice(&self.runner_up_id.to_le_bytes());
        bytes.extend_from_slice(&self.third_placed_id.to_le_bytes());
        bytes.extend_from_slice(&self.host_id.to_le_bytes());
        bytes.extend_from_slice(&self.regular_season_winner_id.to_le_bytes());
        bytes.extend_from_slice(&self.year.to_le_bytes());

        return bytes;
    }
}
