// The player view.

use serde::Serialize;
use serde_json::json;

use crate::data::{self, Data, staff::Staff};

#[derive(Default, Serialize)]
pub struct Player {
    pub columns: Vec<serde_json::Value>,
    pub forename: String,
    pub surname: String,
    pub positions: Vec<u8>,
}

impl Player {
    pub fn add_column(&mut self, data: &Data, counter: usize, header: &str, s: &Staff, p: &data::player::Player) {
        self.columns.push(match header {
            "Random" => json!(counter),
            "Staff ID" => json!(s.id),
            "Name" => json!(s.full_name(data)),
            "Nation" => json!(s.nation_name(data)),
            "Second Nation" => json!(s.second_nation_name(data)),
            "Club Contracted" => json!(s.club_contracted_name(data).unwrap()),
            "Club Playing" => json!(s.club_playing_name(data).unwrap()),
            "Birthday" => json!(s.date_of_birth.to_days()),
            "Position" => json!(p.position_string()),
            "GK Rating" => json!(s.gk_rating(data)),
            "LD Rating" => json!(s.ld_rating(data)),
            "RD Rating" => json!(s.rd_rating(data)),
            "LW Rating" => json!(s.lw_rating(data)),
            "C Rating" => json!(s.c_rating(data)),
            "RW Rating" => json!(s.rw_rating(data)),
            "Adaptability" => json!(s.adaptability),
            "Ambition" => json!(s.ambition),
            "Determination" => json!(s.determination),
            "Loyalty" => json!(s.loyalty),
            "Pressure" => json!(s.pressure),
            "Professionalism" => json!(s.professionalism),
            "Sportsmanship" => json!(s.sportsmanship),
            "Temperament" => json!(s.temperament),
            "Current Ability" => json!(p.current_ability),
            "Potential Ability" => json!(p.potential_ability),
            "Acceleration" => json!(p.acceleration),
            "Aggression" => json!(p.aggression),
            "Agility" => json!(p.agility),
            "Bravery" => json!(p.bravery),
            "Consistency" => json!(p.consistency),
            "Dirtiness" => json!(p.dirtiness),
            "Flair" => json!(p.flair),
            "Important Matches" => json!(p.important_matches),
            "Injury Proneness" => json!(p.injury_proneness),
            "Influence" => json!(p.leadership),
            "Natural Fitness" => json!(p.natural_fitness),
            "Speed" => json!(p.pace),
            "Stamina" => json!(p.stamina),
            "Strength" => json!(p.strength),
            "Teamwork" => json!(p.teamwork),
            "Versatility" => json!(p.versatility),
            "Work Rate" => json!(p.work_rate),
            "GK" => json!(p.goaltender),
            "LD" => json!(p.left_defence),
            "RD" => json!(p.right_defence),
            "LW" => json!(p.left_wing),
            "C" => json!(p.center),
            "RW" => json!(p.right_wing),
            "Agitation" => json!(p.agitation),
            "Defensive Role" => json!(p.defensive_role),
            "Offensive Role" => json!(p.offensive_role),
            "Pass Tendency" => json!(p.pass_tendency),

            attribute => json!(p.convert_attribute(attribute)),
        });
    }
}