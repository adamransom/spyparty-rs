use crate::{Error, Result};
use std::convert::TryFrom;
use std::fmt;

/// The maps of SpyParty.
#[derive(Debug, PartialEq)]
pub enum Map {
    Aquarium,
    Balcony,
    Ballroom,
    Courtyard,
    DoubleModern,
    Gallery,
    HighRise,
    Library,
    Modern,
    Moderne,
    OldBalcony,
    OldBallroom,
    OldCourtyard1,
    OldCourtyard2,
    OldGallery,
    OldVeranda,
    Panopticon,
    Pub,
    Teien,
    Terrace,
    Veranda,
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
                Map::Aquarium => "Aquarium",
                Map::Balcony => "Balcony",
                Map::Ballroom => "Ballroom",
                Map::Courtyard => "Courtyard",
                Map::DoubleModern => "Double Modern",
                Map::Gallery => "Gallery",
                Map::HighRise => "High-Rise",
                Map::Library => "Library",
                Map::Modern => "Modern",
                Map::Moderne => "Moderne",
                Map::OldBalcony => "Old Balcony",
                Map::OldBallroom => "Old Ballroom",
                Map::OldCourtyard1 => "Old Courtyard 1",
                Map::OldCourtyard2 => "Old Courtyard 2",
                Map::OldGallery => "Old Gallery",
                Map::OldVeranda => "Old Veranda",
                Map::Panopticon => "Panopticon",
                Map::Pub => "Pub",
                Map::Teien => "Teien",
                Map::Terrace => "Terrace",
                Map::Veranda => "Veranda",
                Map::Unknown(_) => "Unknown",
            }
        )
    }
}

impl From<u32> for Map {
    fn from(map: u32) -> Self {
        match map {
            0x98e4_5d99 => Map::Aquarium,
            0x1dbd_8e41 => Map::Balcony,
            0x5b12_1925 => Map::Ballroom,
            0x9dc5_bb5e => Map::Courtyard,
            0x7076_e38f => Map::DoubleModern,
            0x7173_b8bf => Map::Gallery,
            0x1a56_c5a1 | 0x3a30_c326 => Map::HighRise,
            0x168f_4f62 => Map::Library,
            0xf3e6_1461 => Map::Modern,
            0x2e37_f15b => Map::Moderne,
            0xb889_1fbc => Map::OldBalcony,
            0x09c2_e7b0 => Map::OldBallroom,
            0xb4cf_686b => Map::OldCourtyard1,
            0x290a_0c75 => Map::OldCourtyard2,
            0x28b3_aa5e => Map::OldGallery,
            0xa8be_a091 => Map::OldVeranda,
            0x3695_f583 => Map::Panopticon,
            0x3b85_fff3 => Map::Pub,
            0x79df_a0cf => Map::Teien,
            0x9032_ce22 => Map::Terrace,
            0x6f81_a558 => Map::Veranda,
            _ => Map::Unknown(map),
        }
    }
}

impl<'a> TryFrom<&'a str> for Map {
    type Error = Error;

    fn try_from(string: &'a str) -> Result<Self> {
        let stripped = string.to_ascii_lowercase().replace(" ", "");

        Ok(match stripped.as_str() {
            "aquarium" => Map::Aquarium,
            "balcony" => Map::Balcony,
            "ballroom" => Map::Ballroom,
            "courtyard" => Map::Courtyard,
            "doublemodern" => Map::DoubleModern,
            "gallery" => Map::Gallery,
            "highrise" | "high-rise" => Map::HighRise,
            "library" => Map::Library,
            "modern" => Map::Modern,
            "moderne" => Map::Moderne,
            "oldbalcony" => Map::OldBalcony,
            "oldballroom" => Map::OldBallroom,
            "oldcourtyard1" | "cy1" => Map::OldCourtyard1,
            "oldcourtyard2" | "cy2" => Map::OldCourtyard2,
            "oldgallery" => Map::OldGallery,
            "oldveranda" => Map::OldVeranda,
            "panopticon" | "panop" => Map::Panopticon,
            "pub" => Map::Pub,
            "teien" => Map::Teien,
            "terrace" => Map::Terrace,
            "veranda" => Map::Veranda,
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
