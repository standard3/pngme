// See PNG structure : http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

use std::str::FromStr;
use std::{convert::TryFrom, fmt::Display};

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
            && self
                .0
                .iter()
                .all(|&b| b.is_ascii_uppercase() || b.is_ascii_lowercase())
    }

    pub fn is_critical(&self) -> bool {
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary
        self.0[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        // 0 (uppercase) = public, 1 (lowercase) = private
        self.0[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        // Must be 0 (uppercase) in files conforming to this version of PNG
        self.0[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy
        self.0[3].is_ascii_lowercase()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !bytes.iter().all(|b| b.is_ascii_alphabetic()) {
            return Err("Invalid byte: Byte must be in range [65-90] or [97-122]".into());
        }

        Ok(Self(bytes))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("Invalid length: must be 4 wide".into());
        }

        let bytes: [u8; 4] = s.as_bytes().try_into()?;
        if !bytes.iter().all(|b| b.is_ascii_alphabetic()) {
            return Err("Invalid byte: Byte must be in range [65-90] or [97-122]".into());
        }

        Ok(Self(bytes))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        println!("{chunk:?}");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
