use std::{collections::HashMap, io::Cursor};
use binread::{BinRead, Error};


use crate::{data::{Data, convert_attribute}, rating};

#[derive(BinRead, Clone, Default)]
#[br(little)]
pub struct Player {
    pub id: i32,
    pub current_ability: i16,
    pub potential_ability: i16,
    _home_reputation: i16,
    _current_reputation: i16,
    _world_reputation: i16,
    _squad_number: i8,
    _international_squad_number: i8,
    pub acceleration: i8,
    pub aggression: i8,
    pub agility: i8,
    pub anticipation_raw: i8,
    pub balance_raw: i8,
    pub bravery: i8,
    pub consistency: i8,
    pub decisions_raw: i8,
    pub dirtiness: i8,
    pub flair: i8,
    pub important_matches: i8,
    pub injury_proneness: i8,
    pub leadership: i8,
    pub movement_raw: i8,
    pub natural_fitness: i8,
    pub one_on_ones_raw: i8,
    pub pace: i8,
    pub passing_raw: i8,
    pub positioning_raw: i8,
    pub reflexes_raw: i8,
    pub stamina: i8,
    pub strength: i8,
    pub teamwork: i8,
    pub versatility: i8,
    pub vision_raw: i8,
    pub work_rate: i8,
    _handedness: i8,
    _height: u8,
    _weight: u8,
    _favourite_number: u8,
    pub goaltender: i8,
    pub left_defence: i8,
    pub right_defence: i8,
    pub left_wing: i8,
    pub center: i8,
    pub right_wing: i8,
    pub agitation: i8,
    pub blocker_raw: i8,
    pub checking_raw: i8,
    pub defensive_role: i8,
    pub deflections_raw: i8,
    pub deking_raw: i8,
    pub faceoffs_raw: i8,
    pub fighting_raw: i8,
    pub glove_raw: i8,
    pub hitting_raw: i8,
    pub offensive_role: i8,
    pub pass_tendency: i8,
    pub pokecheck_raw: i8,
    pub rebounds_raw: i8,
    pub recovery_raw: i8,
    pub slapshot_raw: i8,
    pub stickhandling_raw: i8,
    pub wristshot_raw: i8,
    _morale: i8,
    _goalie_style: i8,
    _junior_preference: i8,
}

impl Player {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let player = Self::read(cursor)?;
        data.order_players.push(player.id);
        data.players.insert(player.id, player);

        return Ok(());
    }

    // Get the positions of the player as a vector.
    pub fn position_vec(&self) -> Vec<u8> {
        let mut positions = Vec::from([
            (0, self.goaltender),
            (1, self.left_defence),
            (2, self.right_defence),
            (3, self.left_wing),
            (4, self.center),
            (5, self.right_wing),
        ]);

        positions.sort_by(|a, b| b.1.cmp(&a.1));

        return positions.into_iter()
            .filter_map(|(pos, score)| {
                if score < 16 {
                    return None;
                }
                else {
                    return Some(pos);
                }
            })
            .collect();
    }

    pub fn position_string(&self) -> String {
        let positions = HashMap::from([
            (0, "G"),
            (1, "LD"),
            (2, "RD"),
            (3, "LW"),
            (4, "C"),
            (5, "RW"),
        ]);

        let player_positions = self.position_vec();

        let string: Vec<&str> = player_positions.into_iter()
            .map(|pos| *positions.get(&pos).unwrap())
            .collect();

        return string.join("/");
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.current_ability.to_le_bytes());
        bytes.extend_from_slice(&self.potential_ability.to_le_bytes());
        bytes.extend_from_slice(&self._home_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._current_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._world_reputation.to_le_bytes());
        bytes.extend_from_slice(&self._squad_number.to_le_bytes());
        bytes.extend_from_slice(&self._international_squad_number.to_le_bytes());
        bytes.extend_from_slice(&self.acceleration.to_le_bytes());
        bytes.extend_from_slice(&self.aggression.to_le_bytes());
        bytes.extend_from_slice(&self.agility.to_le_bytes());
        bytes.extend_from_slice(&self.anticipation_raw.to_le_bytes());
        bytes.extend_from_slice(&self.balance_raw.to_le_bytes());
        bytes.extend_from_slice(&self.bravery.to_le_bytes());
        bytes.extend_from_slice(&self.consistency.to_le_bytes());
        bytes.extend_from_slice(&self.decisions_raw.to_le_bytes());
        bytes.extend_from_slice(&self.dirtiness.to_le_bytes());
        bytes.extend_from_slice(&self.flair.to_le_bytes());
        bytes.extend_from_slice(&self.important_matches.to_le_bytes());
        bytes.extend_from_slice(&self.injury_proneness.to_le_bytes());
        bytes.extend_from_slice(&self.leadership.to_le_bytes());
        bytes.extend_from_slice(&self.movement_raw.to_le_bytes());
        bytes.extend_from_slice(&self.natural_fitness.to_le_bytes());
        bytes.extend_from_slice(&self.one_on_ones_raw.to_le_bytes());
        bytes.extend_from_slice(&self.pace.to_le_bytes());
        bytes.extend_from_slice(&self.passing_raw.to_le_bytes());
        bytes.extend_from_slice(&self.positioning_raw.to_le_bytes());
        bytes.extend_from_slice(&self.reflexes_raw.to_le_bytes());
        bytes.extend_from_slice(&self.stamina.to_le_bytes());
        bytes.extend_from_slice(&self.strength.to_le_bytes());
        bytes.extend_from_slice(&self.teamwork.to_le_bytes());
        bytes.extend_from_slice(&self.versatility.to_le_bytes());
        bytes.extend_from_slice(&self.vision_raw.to_le_bytes());
        bytes.extend_from_slice(&self.work_rate.to_le_bytes());
        bytes.extend_from_slice(&self._handedness.to_le_bytes());
        bytes.extend_from_slice(&self._height.to_le_bytes());
        bytes.extend_from_slice(&self._weight.to_le_bytes());
        bytes.extend_from_slice(&self._favourite_number.to_le_bytes());
        bytes.extend_from_slice(&self.goaltender.to_le_bytes());
        bytes.extend_from_slice(&self.left_defence.to_le_bytes());
        bytes.extend_from_slice(&self.right_defence.to_le_bytes());
        bytes.extend_from_slice(&self.left_wing.to_le_bytes());
        bytes.extend_from_slice(&self.center.to_le_bytes());
        bytes.extend_from_slice(&self.right_wing.to_le_bytes());
        bytes.extend_from_slice(&self.agitation.to_le_bytes());
        bytes.extend_from_slice(&self.blocker_raw.to_le_bytes());
        bytes.extend_from_slice(&self.checking_raw.to_le_bytes());
        bytes.extend_from_slice(&self.defensive_role.to_le_bytes());
        bytes.extend_from_slice(&self.deflections_raw.to_le_bytes());
        bytes.extend_from_slice(&self.deking_raw.to_le_bytes());
        bytes.extend_from_slice(&self.faceoffs_raw.to_le_bytes());
        bytes.extend_from_slice(&self.fighting_raw.to_le_bytes());
        bytes.extend_from_slice(&self.glove_raw.to_le_bytes());
        bytes.extend_from_slice(&self.hitting_raw.to_le_bytes());
        bytes.extend_from_slice(&self.offensive_role.to_le_bytes());
        bytes.extend_from_slice(&self.pass_tendency.to_le_bytes());
        bytes.extend_from_slice(&self.pokecheck_raw.to_le_bytes());
        bytes.extend_from_slice(&self.rebounds_raw.to_le_bytes());
        bytes.extend_from_slice(&self.recovery_raw.to_le_bytes());
        bytes.extend_from_slice(&self.slapshot_raw.to_le_bytes());
        bytes.extend_from_slice(&self.stickhandling_raw.to_le_bytes());
        bytes.extend_from_slice(&self.wristshot_raw.to_le_bytes());
        bytes.extend_from_slice(&self._morale.to_le_bytes());
        bytes.extend_from_slice(&self._goalie_style.to_le_bytes());
        bytes.extend_from_slice(&self._junior_preference.to_le_bytes());

        return bytes;
    }

    pub fn is_goalie(&self) -> bool {
        return self.goaltender == 20;
    }

    pub fn convert_attribute(&self, attr_name: &str) -> i8 {
        let attribute = match attr_name {
            "Anticipation" => self.anticipation_raw,
            "Balance" => self.balance_raw,
            "Decisions" => self.decisions_raw,
            "Off The Puck" => self.movement_raw,
            "One On Ones" => self.one_on_ones_raw,
            "Passing" => self.passing_raw,
            "Positioning" => self.positioning_raw,
            "Reflexes" => self.reflexes_raw,
            "Creativity" => self.vision_raw,
            "Blocker" => self.blocker_raw,
            "Checking" => self.checking_raw,
            "Deflections" => self.deflections_raw,
            "Deking" => self.deking_raw,
            "Faceoffs" => self.faceoffs_raw,
            "Fighting" => self.fighting_raw,
            "Glove" => self.glove_raw,
            "Hitting" => self.hitting_raw,
            "Pokecheck" => self.pokecheck_raw,
            "Rebound Control" => self.rebounds_raw,
            "Recovery" => self.recovery_raw,
            "Slapshot" => self.slapshot_raw,
            "Stickhandling" => self.stickhandling_raw,
            "Wristshot" => self.wristshot_raw,
            _ => panic!("{attr_name} is not an attribute"),
        };

        return convert_attribute(self.current_ability, attribute);
    }

    // Get the rating of a player.
    pub fn rating(&self, data: &Data, score: usize, low: usize, high: usize) -> f64 {
        let ca = self.current_ability as usize;

        let attribute_rating = rating::stretch(score, low, high);
        let ca_rating = rating::stretch(ca, data.worst_ca as usize, data.best_ca as usize);

        let combined_rating = (attribute_rating + ca_rating) / 2.0;
        return rating::restrict_minimum(combined_rating * self.consistency_rating(), 0.01);
    }

    // Get a 'consistency rating' for the player.
    fn consistency_rating(&self) -> f64 {
        let score = self.consistency + self.important_matches + self.work_rate - 3;
        let perfect_score = (20 + 20 + 20 - 3) as f64;

        return rating::restrict_minimum(score as f64 / perfect_score, 0.5);
    }

    // Get a position rating for the player.
    pub fn position_rating(&self, mut position: i8) -> f64 {
        if position == 20 {
            return 1.0;
        }

        if position == 0 {
            position = 1;
        }

        let score = position + self.versatility - 2;
        let perfect_score = (20 + 20 - 2) as f64;

        return rating::restrict_minimum(score as f64 / perfect_score, 0.5);
    }
}
