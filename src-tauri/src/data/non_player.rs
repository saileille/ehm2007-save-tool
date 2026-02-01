use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::Data;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct NonPlayer {
    id: i32,
    current_ability: i16,
    potential_ability: i16,
    home_reputation: i16,
    current_reputation: i16,
    world_reputation: i16,
    attacking: i8,
    business: i8,
    coaching_technique: i8,
    directness: i8,
    discipline: i8,
    free_roles: i8,
    interference: i8,
    judgement: i8,
    judging_potential: i8,
    man_handling: i8,
    motivating: i8,
    patience: i8,
    physiotherapy: i8,
    resources: i8,
    tactics: i8,
    youngsters: i8,
    tactic_preferred: i8,
    coaching_forwards: i8,
    coaching_defensemen: i8,
    coaching_goaltenders: i8,
    line_matching: i8,
    power_play: i8,
    penalty_kill: i8,
    physical: i8,
}

impl NonPlayer {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let nonplayer = Self::read(cursor)?;
        data.nonplayers.push((nonplayer.id, nonplayer));

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.current_ability.to_le_bytes());
        bytes.extend_from_slice(&self.potential_ability.to_le_bytes());
        bytes.extend_from_slice(&self.home_reputation.to_le_bytes());
        bytes.extend_from_slice(&self.current_reputation.to_le_bytes());
        bytes.extend_from_slice(&self.world_reputation.to_le_bytes());
        bytes.extend_from_slice(&self.attacking.to_le_bytes());
        bytes.extend_from_slice(&self.business.to_le_bytes());
        bytes.extend_from_slice(&self.coaching_technique.to_le_bytes());
        bytes.extend_from_slice(&self.directness.to_le_bytes());
        bytes.extend_from_slice(&self.discipline.to_le_bytes());
        bytes.extend_from_slice(&self.free_roles.to_le_bytes());
        bytes.extend_from_slice(&self.interference.to_le_bytes());
        bytes.extend_from_slice(&self.judgement.to_le_bytes());
        bytes.extend_from_slice(&self.judging_potential.to_le_bytes());
        bytes.extend_from_slice(&self.man_handling.to_le_bytes());
        bytes.extend_from_slice(&self.motivating.to_le_bytes());
        bytes.extend_from_slice(&self.patience.to_le_bytes());
        bytes.extend_from_slice(&self.physiotherapy.to_le_bytes());
        bytes.extend_from_slice(&self.resources.to_le_bytes());
        bytes.extend_from_slice(&self.tactics.to_le_bytes());
        bytes.extend_from_slice(&self.youngsters.to_le_bytes());
        bytes.extend_from_slice(&self.tactic_preferred.to_le_bytes());
        bytes.extend_from_slice(&self.coaching_forwards.to_le_bytes());
        bytes.extend_from_slice(&self.coaching_defensemen.to_le_bytes());
        bytes.extend_from_slice(&self.coaching_goaltenders.to_le_bytes());
        bytes.extend_from_slice(&self.line_matching.to_le_bytes());
        bytes.extend_from_slice(&self.power_play.to_le_bytes());
        bytes.extend_from_slice(&self.penalty_kill.to_le_bytes());
        bytes.extend_from_slice(&self.physical.to_le_bytes());

        return bytes;
    }
}
