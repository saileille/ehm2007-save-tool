// Methods that help where to look in order to expand the attribute range.

use std::{fs::File, io::Write};

use crate::data::{self, Data, staff::Staff};

// Make sure players have no unknown values.
pub fn _check_players(save: &mut Data) {
    let mut log = Vec::from([
        "CA;Value;Name;Forename;Surname;Age;Birthplace;Nation;Second Nation;Club Contracted;Club Playing".to_string()
    ]);

    let attributes = Vec::from([
        "Balance",
        "Anticipation",
        "Creativity",
        "Checking",
        "Deflections",
        "Deking",
        "Faceoffs",
        "Hitting",
        "Off The Puck",
        "Passing",
        "Pokecheck",
        "Positioning",
        "Slapshot",
        "Stickhandling",
        "Wristshot",
    ]);

    let mut ca: i16 = 1;
    let mut attribute: i16 = -60;
    for person in save.staff.values() {
        if ca > 200 {
            break;
        }

        if !person._has_no_special_characters(&save) {
            continue;
        }

        let mut player_data = match person.player_data(save) {
            Some(p) => p,
            None => continue,
        };

        // Goalkeepers have less attributes to look for.
        if player_data.is_goalie() {
            continue;
        }

        player_data.current_ability = ca;
        player_data.potential_ability = ca;

        for attr_name in attributes.iter() {
            if attribute > 122 {
                ca += 1;
                attribute = -60;
                break;
            }

            _give_attribute(&mut player_data, &mut attribute, *attr_name);

            let debug_player = _Player::_new(
                person,
                player_data.current_ability,
                (attribute - 1) as i8,
                *attr_name,
                save,
            );
            log.push(debug_player._to_csv_row());
        }

        // Save the player in the database.
        save.players.insert(player_data.id, player_data);
    }

    let mut file =
        File::create("C:/Users/Aleksi/Documents/Sports Interactive/EHM 2007/games/_debug_log2.csv")
            .unwrap();
    file.write_all(log.join("\n").as_bytes()).unwrap();
}

// Assign the attribute to the player.
fn _give_attribute(p: &mut data::player::Player, attribute: &mut i16, attr_name: &str) {
    let attr = match attr_name {
        "Anticipation" => &mut p.anticipation_raw,
        "Balance" => &mut p.balance_raw,
        "Off The Puck" => &mut p.movement_raw,
        "Passing" => &mut p.passing_raw,
        "Positioning" => &mut p.positioning_raw,
        "Creativity" => &mut p.vision_raw,
        "Checking" => &mut p.checking_raw,
        "Deflections" => &mut p.deflections_raw,
        "Deking" => &mut p.deking_raw,
        "Faceoffs" => &mut p.faceoffs_raw,
        "Hitting" => &mut p.hitting_raw,
        "Pokecheck" => &mut p.pokecheck_raw,
        "Slapshot" => &mut p.slapshot_raw,
        "Stickhandling" => &mut p.stickhandling_raw,
        "Wristshot" => &mut p.wristshot_raw,
        _ => panic!("{attr_name} is not an attribute"),
    };

    *attr = *attribute as i8;
    *attribute += 1;
}

// Debug information about the player and one of his attributes.
pub struct _Player {
    forename: String,
    surname: String,
    age: i16,
    birthplace: String,
    nation: String,
    second_nation: String,
    club_contracted: String,
    club_playing: String,
    attribute: i8,
    attribute_name: String,
    current_ability: i16,
}

impl _Player {
    fn _new(
        person: &Staff,
        current_ability: i16,
        attribute: i8,
        attribute_name: &str,
        save: &Data,
    ) -> Self {
        Self {
            forename: person.forename(save),
            surname: person.surname(save),
            age: person.age,
            birthplace: person.birthplace(save),
            nation: person._nation_three_letter_name(save),
            second_nation: person._second_nation_three_letter_name(save),
            club_contracted: person._club_contracted_short_name(save),
            club_playing: person._club_playing_short_name(save),
            attribute,
            attribute_name: attribute_name.to_string(),
            current_ability,
        }
    }

    fn _to_csv_row(&self) -> String {
        format!(
            "{};{};{};{};{};{};{};{};{};{};{}",
            self.current_ability,
            self.attribute,
            self.attribute_name,
            self.forename,
            self.surname,
            self.age,
            self.birthplace,
            self.nation,
            self.second_nation,
            self.club_contracted,
            self.club_playing,
        )
    }
}