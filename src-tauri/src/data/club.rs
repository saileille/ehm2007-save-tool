use std::io::Cursor;

use binread::{BinRead, Error};

use crate::{data::{Data, REAL_SHORT_TEXT_LENGTH, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, init::bytes_to_string, to_bytes::{chars_to_bytes, slice_to_bytes}};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Club {
    id: i32,
    cash: i32,
    attendance: i32,
    min_attendance: i32,
    max_attendance: i32,
    euro_flag: i32,
    tactic_training: [i32; 4],
    tactic_selected: i32,
    fan_support: i32,
    player_budget: i32,
    nation_id: i32,
    division_id: i32,
    last_division_id: i32,
    reserve_division_id: i32,
    stadium_id: i32,
    practice_facilities_id: i32,
    foreground_colour1_id: i32,
    background_colour1_id: i32,
    trim_colour1_id: i32,
    foreground_colour2_id: i32,
    background_colour2_id: i32,
    trim_colour2_id: i32,
    foreground_colour3_id: i32,
    background_colour3_id: i32,
    trim_colour3_id: i32,
    favourite_staff1_id: i32,
    favourite_staff2_id: i32,
    favourite_staff3_id: i32,
    disliked_staff1_id: i32,
    disliked_staff2_id: i32,
    disliked_staff3_id: i32,
    rival1_id: i32,
    rival2_id: i32,
    rival3_id: i32,
    chairman_id: i32,
    director_ids: [i32; 3],
    manager_id: i32,
    assistant_manager_id: i32,
    head_coach_id: i32,
    head_scout_id: i32,
    #[br(count = 75)]
    squad_ids: Vec<i32>,
    coach_ids: [i32; 5],
    scout_ids: [i32; 15],
    physio_ids: [i32; 3],
    captain_id: i32,
    assistant_captain1_id: i32,
    assistant_captain2_id: i32,
    reputation: i16,
    year_founded: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_name: Vec<char>,
    b_short_name: [char; SHORT_TEXT_LENGTH as usize],
    b_abbreviation: [char; REAL_SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    b_nickname: Vec<char>,
    gender_name_short: i8,
    last_position: i8,
    professional_status: i8,
    own_stadium: i8,
    home_match_day: i8,
    training: i8,
    plc: i8,
    gender_name: i8,
    euro_seeding: i8,
    has_linked_club: i8,
    market_size: i8,
}

impl Club {
    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    pub fn short_name(&self) -> String {
        return bytes_to_string(&self.b_short_name);
    }

    fn abbreviation(&self) -> String {
        return bytes_to_string(&self.b_abbreviation);
    }

    fn nickname(&self) -> String {
        return bytes_to_string(&self.b_nickname);
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let club = Self::read(cursor)?;
        data.clubs.insert(club.id, club);

        return Ok(())
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let club = Self::read(cursor)?;
        data.nat_clubs.insert(club.id, club);

        return Ok(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.cash.to_le_bytes());
        bytes.extend_from_slice(&self.attendance.to_le_bytes());
        bytes.extend_from_slice(&self.min_attendance.to_le_bytes());
        bytes.extend_from_slice(&self.max_attendance.to_le_bytes());
        bytes.extend_from_slice(&self.euro_flag.to_le_bytes());
        bytes.append(&mut slice_to_bytes(&self.tactic_training));
        bytes.extend_from_slice(&self.tactic_selected.to_le_bytes());
        bytes.extend_from_slice(&self.fan_support.to_le_bytes());
        bytes.extend_from_slice(&self.player_budget.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.division_id.to_le_bytes());
        bytes.extend_from_slice(&self.last_division_id.to_le_bytes());
        bytes.extend_from_slice(&self.reserve_division_id.to_le_bytes());
        bytes.extend_from_slice(&self.stadium_id.to_le_bytes());
        bytes.extend_from_slice(&self.practice_facilities_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self.foreground_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.background_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.trim_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff3_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff3_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival1_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival2_id.to_le_bytes());
        bytes.extend_from_slice(&self.rival3_id.to_le_bytes());
        bytes.extend_from_slice(&self.chairman_id.to_le_bytes());
        bytes.append(&mut slice_to_bytes(&self.director_ids));
        bytes.extend_from_slice(&self.manager_id.to_le_bytes());
        bytes.extend_from_slice(&self.assistant_manager_id.to_le_bytes());
        bytes.extend_from_slice(&self.head_coach_id.to_le_bytes());
        bytes.extend_from_slice(&self.head_scout_id.to_le_bytes());
        bytes.append(&mut slice_to_bytes(&self.squad_ids));
        bytes.append(&mut slice_to_bytes(&self.coach_ids));
        bytes.append(&mut slice_to_bytes(&self.scout_ids));
        bytes.append(&mut slice_to_bytes(&self.physio_ids));
        bytes.extend_from_slice(&self.captain_id.to_le_bytes());
        bytes.extend_from_slice(&self.assistant_captain1_id.to_le_bytes());
        bytes.extend_from_slice(&self.assistant_captain2_id.to_le_bytes());
        bytes.extend_from_slice(&self.reputation.to_le_bytes());
        bytes.extend_from_slice(&self.year_founded.to_le_bytes());
        bytes.append(&mut chars_to_bytes(&self.b_name));
        bytes.append(&mut chars_to_bytes(&self.b_short_name));
        bytes.append(&mut chars_to_bytes(&self.b_abbreviation));
        bytes.append(&mut chars_to_bytes(&self.b_nickname));
        bytes.extend_from_slice(&self.gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self.last_position.to_le_bytes());
        bytes.extend_from_slice(&self.professional_status.to_le_bytes());
        bytes.extend_from_slice(&self.own_stadium.to_le_bytes());
        bytes.extend_from_slice(&self.home_match_day.to_le_bytes());
        bytes.extend_from_slice(&self.training.to_le_bytes());
        bytes.extend_from_slice(&self.plc.to_le_bytes());
        bytes.extend_from_slice(&self.gender_name.to_le_bytes());
        bytes.extend_from_slice(&self.euro_seeding.to_le_bytes());
        bytes.extend_from_slice(&self.has_linked_club.to_le_bytes());
        bytes.extend_from_slice(&self.market_size.to_le_bytes());

        return bytes;
    }
}
