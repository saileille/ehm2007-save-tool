// Character conversion.

use std::{collections::HashMap, str::{Utf8Error, from_utf8}};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SI_TO_UTF8: HashMap<u8, Vec<u8>> = HashMap::from([
        (142, Vec::from([197, 189])),  // Ž
        (143, Vec::from([197, 136])),  // ň
        (144, Vec::from([196, 155])),  // ě
        (153, Vec::from([226, 132, 162])),  // ™
        (154, Vec::from([197, 161])),  // š
        (157, Vec::from([197, 165])),  // ť
        (158, Vec::from([197, 190])),  // ž
        (159, Vec::from([197, 175])),  // ů
        (160, Vec::from([196, 141])),  // č
        (167, Vec::from([197, 153])),  // ř
        (179, Vec::from([197, 152])),  // Ř
        (187, Vec::from([196, 143])),  // ď
        (188, Vec::from([196, 140])),  // Č
        (189, Vec::from([197, 160])),  // Š
        (190, Vec::from([196, 142])),  // Ď
        (193, Vec::from([195, 129])),  // Á
        (196, Vec::from([195, 132])),  // Ä
        (197, Vec::from([195, 133])),  // Å
        (201, Vec::from([195, 137])),  // É
        (205, Vec::from([195, 141])),  // Í
        (206, Vec::from([195, 142])),  // Î
        (211, Vec::from([195, 147])),  // Ó
        (214, Vec::from([195, 150])),  // Ö
        (216, Vec::from([195, 152])),  // Ø
        (218, Vec::from([195, 154])),  // Ú
        (220, Vec::from([195, 156])),  // Ü
        (222, Vec::from([195, 158])),  // Þ
        (223, Vec::from([195, 159])),  // ß
        (224, Vec::from([195, 160])),  // à
        (225, Vec::from([195, 161])),  // á
        (226, Vec::from([195, 162])),  // â
        (227, Vec::from([195, 163])),  // ã
        (228, Vec::from([195, 164])),  // ä
        (229, Vec::from([195, 165])),  // å
        (230, Vec::from([195, 166])),  // æ
        (231, Vec::from([195, 167])),  // ç
        (232, Vec::from([195, 168])),  // è
        (233, Vec::from([195, 169])),  // é
        (234, Vec::from([195, 170])),  // ê
        (235, Vec::from([195, 171])),  // ë
        (236, Vec::from([195, 172])),  // ì
        (237, Vec::from([195, 173])),  // í
        (238, Vec::from([195, 174])),  // î
        (239, Vec::from([195, 175])),  // ï
        (240, Vec::from([195, 176])),  // ð
        (241, Vec::from([195, 177])),  // ñ
        (242, Vec::from([195, 178])),  // ò
        (243, Vec::from([195, 179])),  // ó
        (244, Vec::from([195, 180])),  // ô
        (245, Vec::from([195, 181])),  // õ
        (246, Vec::from([195, 182])),  // ö
        (248, Vec::from([195, 184])),  // ø
        (249, Vec::from([195, 185])),  // ù
        (250, Vec::from([195, 186])),  // ú
        (251, Vec::from([195, 187])),  // û
        (252, Vec::from([195, 188])),  // ü
        (253, Vec::from([195, 189])),  // ý
    ]);
}

// Convert a vector of bytes to a string.
pub fn bytes_to_string(bytes: &[u8]) -> Result<String, Utf8Error> {
    let mut converted_bytes = Vec::new();

    for byte in bytes {
        if *byte == 0 {
            break;
        }

        match SI_TO_UTF8.get(byte) {
            Some(v) => converted_bytes.append(&mut v.clone()),
            None => converted_bytes.push(*byte),
        }
    }

    // return chars.into_iter().map(|c| c).collect();
    let string = from_utf8(&converted_bytes)?;
    return Ok(string.to_string());
}

// Get a simple string for debugging purposes.
pub fn bytes_to_string_debug(bytes: &[u8]) -> String {
    let mut chars = Vec::new();
    for byte in bytes {
        if *byte == 0 {
            break;
        }

        chars.push(*byte as char);
    }

    return chars.into_iter().map(|c| c).collect();
}