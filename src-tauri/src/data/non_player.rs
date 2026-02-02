use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::Data;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct NonPlayer {
    id: i32,
    _current_ability: i16,
    _potential_ability: i16,
    _home_reputation: i16,
    _current_reputation: i16,
    _world_reputation: i16,
    _attacking: i8,
    _business: i8,
    _coaching_technique: i8,
    _directness: i8,
    _discipline: i8,
    _free_roles: i8,
    _interference: i8,
    _judgement: i8,
    _judging_potential: i8,
    _man_handling: i8,
    _motivating: i8,
    _patience: i8,
    _physiotherapy: i8,
    _resources: i8,
    _tactics: i8,
    _youngsters: i8,
    _tactic_preferred: i8,
    _coaching_forwards: i8,
    _coaching_defensemen: i8,
    _coaching_goaltenders: i8,
    _line_matching: i8,
    _power_play: i8,
    _penalty_kill: i8,
    _physical: i8,
}

impl NonPlayer {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let nonplayer = Self::read(cursor)?;
        data.nonplayers.push((nonplayer.id, nonplayer));

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._current_ability.to_le_bytes());
        bytes.extend_from_slice(&self._potential_ability.to_le_bytes());
        bytes.extend_from_slice(&self._home_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._current_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._world_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._attacking.to_le_bytes());
        bytes.extend_from_slice(&self._business.to_le_bytes());
        bytes.extend_from_slice(&self._coaching_technique.to_le_bytes());
        bytes.extend_from_slice(&self._directness.to_le_bytes());
        bytes.extend_from_slice(&self._discipline.to_le_bytes());
        bytes.extend_from_slice(&self._free_roles.to_le_bytes());
        bytes.extend_from_slice(&self._interference.to_le_bytes());
        bytes.extend_from_slice(&self._judgement.to_le_bytes());
        bytes.extend_from_slice(&self._judging_potential.to_le_bytes());
        bytes.extend_from_slice(&self._man_handling.to_le_bytes());
        bytes.extend_from_slice(&self._motivating.to_le_bytes());
        bytes.extend_from_slice(&self._patience.to_le_bytes());
        bytes.extend_from_slice(&self._physiotherapy.to_le_bytes());
        bytes.extend_from_slice(&self._resources.to_le_bytes());
        bytes.extend_from_slice(&self._tactics.to_le_bytes());
        bytes.extend_from_slice(&self._youngsters.to_le_bytes());
        bytes.extend_from_slice(&self._tactic_preferred.to_le_bytes());
        bytes.extend_from_slice(&self._coaching_forwards.to_le_bytes());
        bytes.extend_from_slice(&self._coaching_defensemen.to_le_bytes());
        bytes.extend_from_slice(&self._coaching_goaltenders.to_le_bytes());
        bytes.extend_from_slice(&self._line_matching.to_le_bytes());
        bytes.extend_from_slice(&self._power_play.to_le_bytes());
        bytes.extend_from_slice(&self._penalty_kill.to_le_bytes());
        bytes.extend_from_slice(&self._physical.to_le_bytes());

        return bytes;
    }
}
