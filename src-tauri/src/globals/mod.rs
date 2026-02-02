pub mod attr_chart;

use std::{collections::HashMap, io::Cursor};
use binread::Error;
use lazy_static::lazy_static;

use crate::data::{Data, arena::Arena, city::City, club::Club, colour::Colour, competition::Competition, competition_history::CompetitionHistory, continent::Continent, currency::Currency, draft::Draft, injury::Injury, name::Name, nation::Nation, non_player::NonPlayer, official::Official, player::Player, retired_number::RetiredNumber, staff::Staff, staff_award::StaffAward, staff_preferences::StaffPreferences, stage_name::StageName, state_province::StateProvince};

pub type ParseFunc = fn(&mut Data, &mut Cursor<Vec<u8>>) -> Result<(), Error>;

lazy_static! {
    pub static ref PARSER_GUIDE: HashMap<String, ParseFunc> = {
        let mut functions: HashMap<String, ParseFunc> = HashMap::new();

        functions.insert("continent.dat".to_string(), Continent::parse);
        functions.insert("officials.dat".to_string(), Official::parse);
        functions.insert("first_names.dat".to_string(), Name::parse_forename);
        functions.insert("second_names.dat".to_string(), Name::parse_surname);
        functions.insert("city.dat".to_string(), City::parse);
        functions.insert("club.dat".to_string(), Club::parse);
        functions.insert("nat_club.dat".to_string(), Club::parse_nat);
        functions.insert("staff_comp.dat".to_string(), StaffAward::parse);
        functions.insert("club_comp.dat".to_string(), Competition::parse);
        functions.insert("nation_comp.dat".to_string(), Competition::parse_nat);
        functions.insert(
            "club_comp_history.dat".to_string(),
            CompetitionHistory::parse,
        );
        functions.insert(
            "nation_comp_history.dat".to_string(),
            CompetitionHistory::parse_nat,
        );
        functions.insert("colour.dat".to_string(), Colour::parse);
        functions.insert("nation.dat".to_string(), Nation::parse);
        functions.insert("stadium.dat".to_string(), Arena::parse);
        functions.insert("staff.dat".to_string(), Staff::parse);
        functions.insert("nonplayer.dat".to_string(), NonPlayer::parse);
        functions.insert("player.dat".to_string(), Player::parse);
        functions.insert("staff_preferences.dat".to_string(), StaffPreferences::parse);
        functions.insert("retired_numbers.dat".to_string(), RetiredNumber::parse);
        functions.insert("states_provinces.dat".to_string(), StateProvince::parse);
        functions.insert("injuries.dat".to_string(), Injury::parse);
        functions.insert("currencies.dat".to_string(), Currency::parse);
        functions.insert("drafts.dat".to_string(), Draft::parse);
        functions.insert("stage_names.dat".to_string(), StageName::parse);

        return functions;
    };
}