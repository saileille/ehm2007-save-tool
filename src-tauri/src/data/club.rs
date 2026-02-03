use std::{io::Cursor, str::Utf8Error};

use binread::{BinRead, Error};

use crate::{
    chars::bytes_to_string, data::{Data, REAL_SHORT_TEXT_LENGTH, SHORT_TEXT_LENGTH, STANDARD_TEXT_LENGTH}, to_bytes::_slice_to_bytes
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Club {
    id: i32,
    _cash: i32,
    _attendance: i32,
    _min_attendance: i32,
    _max_attendance: i32,
    _euro_flag: i32,
    _tactic_training: [i32; 4],
    _tactic_selected: i32,
    _fan_support: i32,
    _player_budget: i32,
    pub nation_id: i32,
    pub division_id: i32,
    _last_division_id: i32,
    _reserve_division_id: i32,
    _stadium_id: i32,
    _practice_facilities_id: i32,
    _foreground_colour1_id: i32,
    _background_colour1_id: i32,
    _trim_colour1_id: i32,
    _foreground_colour2_id: i32,
    _background_colour2_id: i32,
    _trim_colour2_id: i32,
    _foreground_colour3_id: i32,
    _background_colour3_id: i32,
    _trim_colour3_id: i32,
    _favourite_staff1_id: i32,
    _favourite_staff2_id: i32,
    _favourite_staff3_id: i32,
    _disliked_staff1_id: i32,
    _disliked_staff2_id: i32,
    _disliked_staff3_id: i32,
    _rival1_id: i32,
    _rival2_id: i32,
    _rival3_id: i32,
    _chairman_id: i32,
    _director_ids: [i32; 3],
    _manager_id: i32,
    _assistant_manager_id: i32,
    _head_coach_id: i32,
    _head_scout_id: i32,
    #[br(count = 75)]
    _squad_ids: Vec<i32>,
    _coach_ids: [i32; 5],
    _scout_ids: [i32; 15],
    _physio_ids: [i32; 3],
    _captain_id: i32,
    _assistant_captain1_id: i32,
    _assistant_captain2_id: i32,
    _reputation: i16,
    _year_founded: i16,
    #[br(count = STANDARD_TEXT_LENGTH)]
    pub b_name: Vec<u8>,
    _b_short_name: [u8; SHORT_TEXT_LENGTH as usize],
    _b_abbreviation: [u8; REAL_SHORT_TEXT_LENGTH as usize],
    #[br(count = STANDARD_TEXT_LENGTH)]
    _b_nickname: Vec<u8>,
    _gender_name_short: i8,
    _last_position: i8,
    _professional_status: i8,
    _own_stadium: i8,
    _home_match_day: i8,
    _training: i8,
    _plc: i8,
    _gender_name: i8,
    _euro_seeding: i8,
    _has_linked_club: i8,
    _market_size: i8,
}

impl Club {
    pub fn name(&self) -> Result<String, Utf8Error> {
        return bytes_to_string(&self.b_name);
    }

    pub fn _short_name(&self) -> String {
        return bytes_to_string(&self._b_short_name).unwrap();
    }

    fn _abbreviation(&self) -> String {
        return bytes_to_string(&self._b_abbreviation).unwrap();
    }

    fn _nickname(&self) -> String {
        return bytes_to_string(&self._b_nickname).unwrap();
    }

    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let club = Self::read(cursor)?;
        data.order_clubs.push(club.id);
        data.clubs.insert(club.id, club);

        return Ok(());
    }

    pub fn parse_nat(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let club = Self::read(cursor)?;
        data.order_nat_clubs.push(club.id);
        data.nat_clubs.insert(club.id, club);

        return Ok(());
    }

    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self._cash.to_le_bytes());
        bytes.extend_from_slice(&self._attendance.to_le_bytes());
        bytes.extend_from_slice(&self._min_attendance.to_le_bytes());
        bytes.extend_from_slice(&self._max_attendance.to_le_bytes());
        bytes.extend_from_slice(&self._euro_flag.to_le_bytes());
        bytes.append(&mut _slice_to_bytes(&self._tactic_training));
        bytes.extend_from_slice(&self._tactic_selected.to_le_bytes());
        bytes.extend_from_slice(&self._fan_support.to_le_bytes());
        bytes.extend_from_slice(&self._player_budget.to_le_bytes());
        bytes.extend_from_slice(&self.nation_id.to_le_bytes());
        bytes.extend_from_slice(&self.division_id.to_le_bytes());
        bytes.extend_from_slice(&self._last_division_id.to_le_bytes());
        bytes.extend_from_slice(&self._reserve_division_id.to_le_bytes());
        bytes.extend_from_slice(&self._stadium_id.to_le_bytes());
        bytes.extend_from_slice(&self._practice_facilities_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour1_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour2_id.to_le_bytes());
        bytes.extend_from_slice(&self._foreground_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._background_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._trim_colour3_id.to_le_bytes());
        bytes.extend_from_slice(&self._favourite_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self._favourite_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self._favourite_staff3_id.to_le_bytes());
        bytes.extend_from_slice(&self._disliked_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self._disliked_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self._disliked_staff3_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival1_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival2_id.to_le_bytes());
        bytes.extend_from_slice(&self._rival3_id.to_le_bytes());
        bytes.extend_from_slice(&self._chairman_id.to_le_bytes());
        bytes.append(&mut _slice_to_bytes(&self._director_ids));
        bytes.extend_from_slice(&self._manager_id.to_le_bytes());
        bytes.extend_from_slice(&self._assistant_manager_id.to_le_bytes());
        bytes.extend_from_slice(&self._head_coach_id.to_le_bytes());
        bytes.extend_from_slice(&self._head_scout_id.to_le_bytes());
        bytes.append(&mut _slice_to_bytes(&self._squad_ids));
        bytes.append(&mut _slice_to_bytes(&self._coach_ids));
        bytes.append(&mut _slice_to_bytes(&self._scout_ids));
        bytes.append(&mut _slice_to_bytes(&self._physio_ids));
        bytes.extend_from_slice(&self._captain_id.to_le_bytes());
        bytes.extend_from_slice(&self._assistant_captain1_id.to_le_bytes());
        bytes.extend_from_slice(&self._assistant_captain2_id.to_le_bytes());
        bytes.extend_from_slice(&self._reputation.to_le_bytes());
        bytes.extend_from_slice(&self._year_founded.to_le_bytes());
        bytes.append(&mut self.b_name.clone());
        bytes.append(&mut self._b_short_name.to_vec());
        bytes.append(&mut self._b_abbreviation.to_vec());
        bytes.append(&mut self._b_nickname.clone());
        bytes.extend_from_slice(&self._gender_name_short.to_le_bytes());
        bytes.extend_from_slice(&self._last_position.to_le_bytes());
        bytes.extend_from_slice(&self._professional_status.to_le_bytes());
        bytes.extend_from_slice(&self._own_stadium.to_le_bytes());
        bytes.extend_from_slice(&self._home_match_day.to_le_bytes());
        bytes.extend_from_slice(&self._training.to_le_bytes());
        bytes.extend_from_slice(&self._plc.to_le_bytes());
        bytes.extend_from_slice(&self._gender_name.to_le_bytes());
        bytes.extend_from_slice(&self._euro_seeding.to_le_bytes());
        bytes.extend_from_slice(&self._has_linked_club.to_le_bytes());
        bytes.extend_from_slice(&self._market_size.to_le_bytes());

        return bytes;
    }
}
