use std::io::Cursor;

use binread::{BinRead, Error};

use crate::data::Data;

#[derive(BinRead, Clone)]
#[br(little)]
pub struct StaffPreferences {
    id: i32,
    favourite_club1_id: i32,
    favourite_club2_id: i32,
    favourite_club3_id: i32,
    disliked_club1_id: i32,
    disliked_club2_id: i32,
    disliked_club3_id: i32,
    favourite_staff1_id: i32,
    favourite_staff2_id: i32,
    favourite_staff3_id: i32,
    disliked_staff1_id: i32,
    disliked_staff2_id: i32,
    disliked_staff3_id: i32,
}

impl StaffPreferences {
    pub fn parse(data: &mut Data, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        let preferences = Self::read(cursor)?;
        data.order_staff_preferences.push(preferences.id);
        data.staff_preferences.insert(preferences.id, preferences);

        return Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_club1_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_club2_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_club3_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_club1_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_club2_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_club3_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self.favourite_staff3_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff1_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff2_id.to_le_bytes());
        bytes.extend_from_slice(&self.disliked_staff3_id.to_le_bytes());

        return bytes;
    }
}
