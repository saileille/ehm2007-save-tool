/* Load stuff. */
pub mod debug;

use std::{
    fs::{File, OpenOptions},
    io::{Cursor, Read as _, Write},
    path::Path,
};

use binread::{BinRead, Error};

use crate::{
    data::
        Data
    , globals::{PARSER_GUIDE, ParseFunc}, init::debug::_check_players, to_bytes::_chars_to_bytes
};

#[derive(BinRead, Clone)]
#[br(little)]
pub struct Header {
    _compressed: i32,
    _header: i32,
    files: i32,
}

impl Header {
    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self._compressed.to_le_bytes());
        bytes.extend_from_slice(&self._header.to_le_bytes());
        bytes.extend_from_slice(&self.files.to_le_bytes());

        return bytes;
    }
}

#[derive(Debug, BinRead, Clone, Default)]
#[br(little)]
pub struct FileIndex {
    pub start_position: u32,
    pub size: u32,

    #[br(count = 260)]
    pub b_name: Vec<char>,
}

impl FileIndex {
    pub fn _to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.start_position.to_le_bytes());
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.append(&mut _chars_to_bytes(&self.b_name));

        return bytes;
    }

    // Get the string of the name.
    pub fn name(&self) -> String {
        return bytes_to_string(&self.b_name);
    }

    // Get the binary and a cursor for it.
    fn bin(&self, global_cursor: &mut Cursor<Vec<u8>>) -> Result<Cursor<Vec<u8>>, Error> {
        // Put the cursor where we need it.
        global_cursor.set_position(self.start_position as u64);

        // Fill the buffer with garbage.
        let mut buffer: Vec<u8> = vec![0; self.size as usize];
        global_cursor.read_exact(&mut buffer)?;

        return Ok(Cursor::new(buffer));
    }

    fn _to_string(&self, index: usize) -> String {
        let name: String = self.b_name.iter().collect();
        format!("{};{};{};{}", index, self.start_position, self.size, name)
    }

    fn _debug_csv(file_indexes: &[Self]) {
        let mut csv = Vec::from(["Index;Start Position;Size;Name".to_string()]);
        for (i, file_index) in file_indexes.iter().enumerate() {
            csv.push(file_index._to_string(i));
        }

        let csv = csv.join("\n");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(r"C:\Users\Aleksi\Documents\Sports Interactive\EHM 2007\games\file_indexes.csv")
            .unwrap();

        writeln!(file, "{csv}").unwrap();
    }
}

// Make chars to a string.
pub fn bytes_to_string(bytes: &[char]) -> String {
    let mut chars = Vec::new();

    for char in bytes {
        if *char == '\0' {
            break;
        }
        chars.push(*char);
    }

    return chars.into_iter().collect();
}

pub fn _load_debug_bin(path: &Path) -> Data {
    let save = load_bin(path);

    let mut debug_save = save.clone();
    _check_players(&mut debug_save);

    let debug_bin = debug_save._save_file();

    let debug_path = Path::new("C:/Users/Aleksi/Documents/Sports Interactive/EHM 2007/games/test_debug.sav");
    let mut file = File::create(debug_path).unwrap();
    file.write_all(&debug_bin).unwrap();

    return load_bin(debug_path);
}

// Load the binary.
pub fn load_bin(path: &Path) -> Data {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("{e} - path: {}", path.to_str().unwrap()),
    };

    return load_save(file);
}

pub fn load_save(mut file: File) -> Data {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let mut cursor = Cursor::new(buffer);

    let mut data = Data::initialise(&mut cursor);

    parse_files(&mut cursor, &mut data);
    data.calculate_ingame_date();
    return data;
}

// Read file indexes.
pub fn read_file_indexes(cursor: &mut Cursor<Vec<u8>>, header: &Header) -> Vec<FileIndex> {
    let mut file_indexes = Vec::new();
    for _ in 0..header.files {
        let index = FileIndex::read(cursor).unwrap();
        file_indexes.push(index);
    }

    return file_indexes;
}

// Parse the files.
pub fn parse_files(global_cursor: &mut Cursor<Vec<u8>>, data: &mut Data) {
    let file_indexes = data.file_indexes.clone();

    // FileIndex::debug_csv(&file_indexes);

    for index in file_indexes {
        let name = index.name();
        let mut cursor = match index.bin(global_cursor) {
            Ok(c) => c,
            Err(e) => panic!("{e} - file name: {name}"),
        };

        match PARSER_GUIDE.get(name.as_str()) {
            Some(parser) => {
                parse_file(&mut cursor, parser, data, index.size as u64, name.as_str());
            }
            None => {
                // Add the save file part into the binaries as-is.
                data.binaries.insert(name.clone(), cursor.into_inner());
            }
        };
    }
}

pub fn parse_file(
    cursor: &mut Cursor<Vec<u8>>,
    parser: &ParseFunc,
    data: &mut Data,
    file_size: u64,
    name: &str,
) {
    loop {
        match parser(data, cursor) {
            Err(_) => break,
            _ => {}
        }
    }

    // Make sure the cursor has reached the end of the file.
    let leftovers = file_size - cursor.position();
    if leftovers != 0 {
        panic!("{name} has {leftovers} leftover bytes");
    }
}
