use std::fmt;
use std::str;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    codes: [u8; 4],
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match str::from_utf8(&self.codes[..]) {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "Error: {}", e),
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        let chunk_type = ChunkType { codes: bytes };
        if !chunk_type.is_valid() {
            bail!("类型不符合规范")
        }
        Ok(chunk_type)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let codes: [u8; 4] = s.as_bytes().try_into()?;
        let chunk_type = Self::try_from(codes)?;
        Ok(chunk_type)
    }
}

#[allow(unused)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.codes
    }

    pub fn is_valid(&self) -> bool {
        for byte in self.codes {
            if !Self::is_valid_byte(byte) {
                return false;
            }
        }
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.codes[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.codes[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.codes[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.codes[3].is_ascii_lowercase()
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(!chunk.is_ok());
    }

    #[test]
    fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust");
        assert!(!chunk.is_ok());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
