// Write a Chunk struct with your implementation of PNG chunks.
// Implement TryFrom<&[u8]> for your Chunk.
// Implement Display for your Chunk.

use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc;
use std::fmt::Display;
use std::str;

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let len = data.len() as u32;
        let mut bytes = Vec::<u8>::new();

        // todo: refactor with iterators
        bytes.extend(&chunk_type.bytes());
        bytes.extend(&data);

        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let c = crc.checksum(bytes.as_slice());

        Self {
            length: len,
            chunk_type: chunk_type,
            data: data,
            crc: c, // todo: handle crc
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        str::from_utf8(&self.data)
            .map(String::from)
            .map_err(Into::into)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // todo: refactor with iterators
        let mut bytes = Vec::<u8>::new();

        bytes.extend(&self.length.to_be_bytes());
        bytes.extend(&self.chunk_type.bytes());
        bytes.extend(&self.data);
        bytes.extend(&self.crc.to_be_bytes());

        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        // todo: refactor using iterators

        // extract base informations
        let length = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let chunk_type: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type)?;
        let data: Vec<u8> = Vec::from(&bytes[8..bytes.len() - 4]);
        let crc = u32::from_be_bytes(bytes[bytes.len() - 4..].try_into().unwrap());

        // calculate CRC and see if it matches
        let algorithm = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let all_data: Vec<_> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let computed_crc = algorithm.checksum(&all_data);

        if computed_crc != crc {
            return Err("The computed CRC doesn't match the provided one".into());
        }

        Ok(Self {
            length: length,
            chunk_type: chunk_type,
            data: data,
            crc: crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
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
}
