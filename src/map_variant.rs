use crate::{Error, Result};
use std::convert::TryFrom;

/// The variants for the maps of SpyParty.
#[derive(Debug, PartialEq)]
pub enum MapVariant {
    Teien(TeienVariant),
    None,
}

impl Default for MapVariant {
    fn default() -> MapVariant {
        MapVariant::None
    }
}

/// The variants for Teien.
#[derive(Debug, PartialEq)]
pub enum TeienVariant {
    BooksBooksBooks,
    BooksStatuesBooks,
    StatuesBooksBooks,
    StatuesStatuesBooks,
    BooksBooksStatues,
    BooksStatuesStatues,
    StatuesBooksStatues,
    StatuesStatuesStatues,
}

impl TryFrom<u32> for TeienVariant {
    type Error = Error;

    fn try_from(variant: u32) -> Result<Self> {
        match variant {
            0 => Ok(TeienVariant::BooksBooksBooks),
            1 => Ok(TeienVariant::BooksStatuesBooks),
            2 => Ok(TeienVariant::StatuesBooksBooks),
            3 => Ok(TeienVariant::StatuesStatuesBooks),
            4 => Ok(TeienVariant::BooksBooksStatues),
            5 => Ok(TeienVariant::BooksStatuesStatues),
            6 => Ok(TeienVariant::StatuesBooksStatues),
            7 => Ok(TeienVariant::StatuesStatuesStatues),
            _ => Err(Error::UnknownMapVariant(variant)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn known_teien_variant() {
        let result: TeienVariant = 3.try_into().unwrap();
        assert_eq!(result, TeienVariant::StatuesStatuesBooks);
    }

    #[test]
    fn unknown_teien_variant() {
        let result: Result<TeienVariant> = 8.try_into();

        match result {
            Err(Error::UnknownMapVariant(variant)) => assert!(variant == 8),
            _ => assert!(false),
        }
    }
}
