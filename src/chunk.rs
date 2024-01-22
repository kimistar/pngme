use crate::chunk_type::ChunkType;
use core::fmt;
use crc::{Crc, CRC_32_ISO_HDLC};

use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

pub struct Chunk {
    chunk_type: ChunkType,
    crc: u32,
    chunk_data: Vec<u8>,
    length: u32,
}

#[allow(unused)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {
        let crc32 = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc32.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&chunk_data);

        Chunk {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc: digest.finalize(),
        }
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    pub fn data_as_string(&self) -> Result<String> {
        // todo 是否有不用clone更好的方式
        let str = String::from_utf8(self.chunk_data.clone())?;
        Ok(str)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_data: Vec<u8> = self
            .length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect();
        chunk_data
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self> {
        if data.len() < 12 {
            bail!("chunk data too short")
        }

        let length = u32::from_be_bytes(data[..4].try_into()?);
        let chunk_type = ChunkType::try_from([data[4], data[5], data[6], data[7]])?;
        let chunk_data = data[8..8 + length as usize].to_vec();
        let crc = u32::from_be_bytes(data[8 + length as usize..12 + length as usize].try_into()?);

        let chunk = Self::new(chunk_type, chunk_data);
        if chunk.crc() != crc {
            bail!("crc校验失败")
        };

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data_as_string() {
            Ok(v) => write!(f, "{}", v),
            Err(e) => write!(f, "Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
