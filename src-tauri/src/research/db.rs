// Database parsing.

use std::{
    fs::File,
    io::{Cursor, Read as _},
    path::Path,
};

use crate::{
    data::Data,
    init::{get_parser_guide, parse_file},
    research::_AttributeData,
};

pub fn _load_files(path_name: &str) -> Data {
    let folder = Path::new(path_name);
    let parser_guide = get_parser_guide();

    let mut db_data = Data::default();
    for (filename, parser) in parser_guide {
        let mut path_buf = folder.to_path_buf();
        path_buf.push(filename.as_str());

        let mut file = match File::open(&path_buf) {
            Ok(f) => f,
            Err(e) => {
                panic!("{} - {}", path_buf.to_str().unwrap(), e);
            }
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let file_size = buffer.len() as u64;
        let mut cursor = Cursor::new(buffer);

        parse_file(
            &mut cursor,
            &parser,
            &mut db_data,
            file_size,
            filename.as_str(),
        );
    }

    return db_data;
}

pub struct _Player {
    pub forename: String,
    pub surname: String,
    pub age: i16,
    pub birthplace: String,
    pub nation: String,
    pub second_nation: String,
    pub club: String,
    pub current_ability: i16,
    pub anticipation_save: i8,
    pub anticipation_db: i8,
    pub balance_save: i8,
    pub balance_db: i8,
    pub decisions_save: i8,
    pub decisions_db: i8,
    pub movement_save: i8,
    pub movement_db: i8,
    pub one_on_ones_save: i8,
    pub one_on_ones_db: i8,
    pub passing_save: i8,
    pub passing_db: i8,
    pub positioning_save: i8,
    pub positioning_db: i8,
    pub reflexes_save: i8,
    pub reflexes_db: i8,
    pub vision_save: i8,
    pub vision_db: i8,
    pub blocker_save: i8,
    pub blocker_db: i8,
    pub checking_save: i8,
    pub checking_db: i8,
    pub deflections_save: i8,
    pub deflections_db: i8,
    pub deking_save: i8,
    pub deking_db: i8,
    pub faceoffs_save: i8,
    pub faceoffs_db: i8,
    pub fighting_save: i8,
    pub fighting_db: i8,
    pub glove_save: i8,
    pub glove_db: i8,
    pub hitting_save: i8,
    pub hitting_db: i8,
    pub pokecheck_save: i8,
    pub pokecheck_db: i8,
    pub rebounds_save: i8,
    pub rebounds_db: i8,
    pub recovery_save: i8,
    pub recovery_db: i8,
    pub slapshot_save: i8,
    pub slapshot_db: i8,
    pub stickhandling_save: i8,
    pub stickhandling_db: i8,
    pub wristshot_save: i8,
    pub wristshot_db: i8,

    pub gk: i8,
    pub ld: i8,
    pub rd: i8,
    pub lw: i8,
    pub c: i8,
    pub rw: i8,
}

impl _Player {
    fn _is_skater(&self) -> bool {
        let skater_positions = [self.ld, self.rd, self.lw, self.c, self.rw];
        return *skater_positions.iter().max().unwrap() >= 20;
    }

    fn _is_goalie(&self) -> bool {
        return self.gk >= 20;
    }

    fn _anticipation(&self) -> String {
        match (1..=20).contains(&self.anticipation_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.anticipation_save, self.anticipation_db),
        }
    }

    fn _balance(&self) -> String {
        match (1..=20).contains(&self.balance_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.balance_save, self.balance_db),
        }
    }

    fn _decisions(&self) -> String {
        match (1..=20).contains(&self.decisions_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.decisions_save, self.decisions_db),
        }
    }

    fn _movement(&self) -> String {
        match (1..=20).contains(&self.movement_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.movement_save, self.movement_db),
        }
    }

    fn _one_on_ones(&self) -> String {
        match (1..=20).contains(&self.one_on_ones_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.one_on_ones_save, self.one_on_ones_db),
        }
    }

    fn _passing(&self) -> String {
        match (1..=20).contains(&self.passing_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.passing_save, self.passing_db),
        }
    }

    fn _positioning(&self) -> String {
        match (1..=20).contains(&self.positioning_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.positioning_save, self.positioning_db),
        }
    }

    fn _reflexes(&self) -> String {
        match (1..=20).contains(&self.reflexes_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.reflexes_save, self.reflexes_db),
        }
    }

    fn _vision(&self) -> String {
        match (1..=20).contains(&self.vision_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.vision_save, self.vision_db),
        }
    }

    fn _blocker(&self) -> String {
        match (1..=20).contains(&self.blocker_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.blocker_save, self.blocker_db),
        }
    }

    fn _checking(&self) -> String {
        match (1..=20).contains(&self.checking_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.checking_save, self.checking_db),
        }
    }

    fn _deflections(&self) -> String {
        match (1..=20).contains(&self.deflections_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.deflections_save, self.deflections_db),
        }
    }

    fn _deking(&self) -> String {
        match (1..=20).contains(&self.deking_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.deking_save, self.deking_db),
        }
    }

    fn _faceoffs(&self) -> String {
        match (1..=20).contains(&self.faceoffs_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.faceoffs_save, self.faceoffs_db),
        }
    }

    fn _fighting(&self) -> String {
        match (1..=20).contains(&self.fighting_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.fighting_save, self.fighting_db),
        }
    }

    fn _glove(&self) -> String {
        match (1..=20).contains(&self.glove_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.glove_save, self.glove_db),
        }
    }

    fn _hitting(&self) -> String {
        match (1..=20).contains(&self.hitting_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.hitting_save, self.hitting_db),
        }
    }

    fn _pokecheck(&self) -> String {
        match (1..=20).contains(&self.pokecheck_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.pokecheck_save, self.pokecheck_db),
        }
    }

    fn _rebounds(&self) -> String {
        match (1..=20).contains(&self.rebounds_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.rebounds_save, self.rebounds_db),
        }
    }

    fn _recovery(&self) -> String {
        match (1..=20).contains(&self.recovery_db) && self._is_goalie() {
            false => ";".to_string(),
            true => format!("{};{}", self.recovery_save, self.recovery_db),
        }
    }

    fn _slapshot(&self) -> String {
        match (1..=20).contains(&self.slapshot_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.slapshot_save, self.slapshot_db),
        }
    }

    fn _stickhandling(&self) -> String {
        match (1..=20).contains(&self.stickhandling_db) {
            false => ";".to_string(),
            true => format!("{};{}", self.stickhandling_save, self.stickhandling_db),
        }
    }

    fn _wristshot(&self) -> String {
        match (1..=20).contains(&self.wristshot_db) && self._is_skater() {
            false => ";".to_string(),
            true => format!("{};{}", self.wristshot_save, self.wristshot_db),
        }
    }

    pub fn _create_csv_row(&self) -> String {
        format!(
            "{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
            self.current_ability,
            self.forename,
            self.surname,
            self.age,
            self.birthplace,
            self.nation,
            self.second_nation,
            self.club,
            self._anticipation(),
            self._balance(),
            self._decisions(),
            self._movement(),
            self._one_on_ones(),
            self._passing(),
            self._positioning(),
            self._reflexes(),
            self._vision(),
            self._blocker(),
            self._checking(),
            self._deflections(),
            self._deking(),
            self._faceoffs(),
            self._fighting(),
            self._glove(),
            self._hitting(),
            self._pokecheck(),
            self._rebounds(),
            self._recovery(),
            self._slapshot(),
            self._stickhandling(),
            self._wristshot(),
        )
    }

    fn _add_attr(
        &self,
        attr_data: &mut _AttributeData,
        db_attr: i8,
        save_attr: i8,
    ) {
        *attr_data
            .entry(self.current_ability)
            .or_default()
            .entry(db_attr)
            .or_default()
            .entry(save_attr)
            .or_insert(0) += 1;
    }

    pub fn _add_to_attr_data(
        &self,
        attr_data: &mut _AttributeData,
    ) {
        if self._anticipation() != ";" {
            self._add_attr(
                attr_data,
                self.anticipation_db,
                self.anticipation_save,
            );
        }

        if self._balance() != ";" {
            self._add_attr(
                attr_data,
                self.balance_db,
                self.balance_save,
            );
        }

        if self._decisions() != ";" {
            self._add_attr(
                attr_data,
                self.decisions_db,
                self.decisions_save,
            );
        }

        if self._movement() != ";" {
            self._add_attr(
                attr_data,
                self.movement_db,
                self.movement_save,
            );
        }

        if self._one_on_ones() != ";" {
            self._add_attr(
                attr_data,
                self.one_on_ones_db,
                self.one_on_ones_save,
            );
        }

        if self._passing() != ";" {
            self._add_attr(
                attr_data,
                self.passing_db,
                self.passing_save,
            );
        }

        if self._positioning() != ";" {
            self._add_attr(
                attr_data,
                self.positioning_db,
                self.positioning_save,
            );
        }

        if self._reflexes() != ";" {
            self._add_attr(
                attr_data,
                self.reflexes_db,
                self.reflexes_save,
            );
        }

        if self._vision() != ";" {
            self._add_attr(
                attr_data,
                self.vision_db,
                self.vision_save,
            );
        }

        if self._blocker() != ";" {
            self._add_attr(
                attr_data,
                self.blocker_db,
                self.blocker_save,
            );
        }

        if self._checking() != ";" {
            self._add_attr(
                attr_data,
                self.checking_db,
                self.checking_save,
            );
        }

        if self._deflections() != ";" {
            self._add_attr(
                attr_data,
                self.deflections_db,
                self.deflections_save,
            );
        }

        if self._deking() != ";" {
            self._add_attr(
                attr_data,
                self.deking_db,
                self.deking_save,
            );
        }

        if self._faceoffs() != ";" {
            self._add_attr(
                attr_data,
                self.faceoffs_db,
                self.faceoffs_save,
            );
        }

        if self._fighting() != ";" {
            self._add_attr(
                attr_data,
                self.fighting_db,
                self.fighting_save,
            );
        }

        if self._glove() != ";" {
            self._add_attr(
                attr_data,
                self.glove_db,
                self.glove_save,
            );
        }

        if self._hitting() != ";" {
            self._add_attr(
                attr_data,
                self.hitting_db,
                self.hitting_save,
            );
        }

        if self._pokecheck() != ";" {
            self._add_attr(
                attr_data,
                self.pokecheck_db,
                self.pokecheck_save,
            );
        }

        if self._rebounds() != ";" {
            self._add_attr(
                attr_data,
                self.rebounds_db,
                self.rebounds_save,
            );
        }

        if self._recovery() != ";" {
            self._add_attr(
                attr_data,
                self.recovery_db,
                self.recovery_save,
            );
        }

        if self._slapshot() != ";" {
            self._add_attr(
                attr_data,
                self.slapshot_db,
                self.slapshot_save,
            );
        }

        if self._stickhandling() != ";" {
            self._add_attr(
                attr_data,
                self.stickhandling_db,
                self.stickhandling_save,
            );
        }

        if self._wristshot() != ";" {
            self._add_attr(
                attr_data,
                self.wristshot_db,
                self.wristshot_save,
            );
        }
    }
}
