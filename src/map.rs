use crate::{Error, Result};
use std::convert::TryFrom;
use std::fmt;

/// The maps of SpyParty.
#[derive(Debug, PartialEq)]
pub enum Map {
    Balcony,
    Ballroom,
    Courtyard,
    Gallery,
    HighRise,
    Library,
    Moderne,
    Pub,
    Terrace,
    Veranda,
    Teien,
    Unknown(u32),
}

impl Default for Map {
    fn default() -> Map {
        Map::Unknown(0)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Map::Balcony => "Balcony",
                Map::Ballroom => "Ballroom",
                Map::Courtyard => "Courtyard",
                Map::Gallery => "Gallery",
                Map::HighRise => "High-Rise",
                Map::Library => "Library",
                Map::Moderne => "Moderne",
                Map::Pub => "Pub",
                Map::Terrace => "Terrace",
                Map::Veranda => "Veranda",
                Map::Teien => "Teien",
                Map::Unknown(_) => "Unknown",
            }
        )
    }
}

impl From<u32> for Map {
    fn from(map: u32) -> Self {
        match map {
            0x1dbd_8e41 => Map::Balcony,
            0x5b12_1925 => Map::Ballroom,
            0x9dc5_bb5e => Map::Courtyard,
            0x7173_b8bf => Map::Gallery,
            0x1a56_c5a1 => Map::HighRise,
            0x168f_4f62 => Map::Library,
            0x2e37_f15b => Map::Moderne,
            0x3b85_fff3 => Map::Pub,
            0x9032_ce22 => Map::Terrace,
            0x6f81_a558 => Map::Veranda,
            0x79df_a0cf => Map::Teien,
            _ => Map::Unknown(map),
        }
    }
}

impl<'a> TryFrom<&'a str> for Map {
    type Error = Error;

    fn try_from(string: &'a str) -> Result<Self> {
        let stripped = string.to_ascii_lowercase().replace(" ", "");

        Ok(match stripped.as_str() {
            "balcony" => Map::Balcony,
            "ballroom" => Map::Ballroom,
            "courtyard" => Map::Courtyard,
            "gallery" => Map::Gallery,
            "highrise" | "high-rise" => Map::HighRise,
            "library" => Map::Library,
            "moderne" => Map::Moderne,
            "pub" => Map::Pub,
            "terrace" => Map::Terrace,
            "veranda" => Map::Veranda,
            "teien" => Map::Teien,
            _ => bail!(Error::UnknownMap(string.to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn known_map() {
        let result: Map = 0x1dbd8e41.into();
        assert_eq!(result, Map::Balcony);
    }

    #[test]
    fn unknown_map() {
        let result: Map = 1u32.into();
        assert_eq!(result, Map::Unknown(1));
    }

    #[test]
    fn string_into_known_map() {
        let result: Map = "pub".try_into().unwrap();
        assert_eq!(result, Map::Pub);
    }

    #[test]
    fn string_into_known_map_ci() {
        let result: Map = "BallRoom".try_into().unwrap();
        assert_eq!(result, Map::Ballroom);
    }

    #[test]
    fn string_into_unknown_map() {
        let result: Result<Map> = "unknown".try_into();

        match result {
            Err(Error::UnknownMap(map)) => assert!(map == "unknown"),
            _ => assert!(false),
        }
    }
}