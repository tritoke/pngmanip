use cookie_factory::SerializeFn;
use crc32fast::Hasher;
use log::Level::{Debug, Trace};
use log::{log_enabled, debug, trace};
use nom::IResult;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: String,
    pub chunk_data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::bytes::complete::take;
        use nom::combinator::{map, map_res};
        use nom::number::complete::be_u32;
        use nom::sequence::tuple;

        let (i, length) = be_u32(i)?;
        let (i, (chunk_type, chunk_data, crc)) = tuple((
            map_res(map(take(4_usize), Into::into), String::from_utf8),
            map(take(length), |data: &[u8]| data.to_owned()),
            be_u32,
        ))(i)?;

        if log_enabled!(Trace) {
            trace!("Parsed {} chunk, with length {}, CRC = {:08X}, data = {:?}", chunk_type, length, crc, chunk_data);
        } else if log_enabled!(Debug) {
            debug!("Parsed {} chunk, with length {}, CRC = {:08X}", chunk_type, length, crc);
        }

        Ok((
            i,
            Self {
                length,
                chunk_type,
                chunk_data,
                crc,
            },
        ))
    }

    pub fn serialize<W: Write>(self) -> impl SerializeFn<W> {
        use cookie_factory::{
            bytes::be_u32,
            combinator::{slice, string},
            sequence::tuple,
        };

        tuple((
            be_u32(self.length),
            string(self.chunk_type),
            slice(self.chunk_data),
            be_u32(self.crc),
        ))
    }

    pub fn calc_crc(&self) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(self.chunk_type.as_bytes());
        hasher.update(self.chunk_data.as_slice());
        hasher.finalize()
    }

    pub fn verify(&self) -> bool {
        self.calc_crc() == self.crc
    }

    // fix a chunk's CRC value so that it is valid for the data it contains
    pub fn fix_crc(&mut self) {
        self.crc = self.calc_crc();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cookie_factory::gen;

    #[test]
    fn test_parse_ihdr() {
        let iend_chunk = [
            0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00,
            0x00, 0x20, 0x01, 0x00, 0x00, 0x00, 0x01, 0x2c, 0x06, 0x77, 0xcf,
        ];

        let parsed = Chunk::parse(&iend_chunk).unwrap().1;
        assert_eq!(
            parsed,
            Chunk {
                length: 0xd,
                chunk_type: "IHDR".to_string(),
                chunk_data: vec![
                    0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00, 0x00, 0x01
                ],
                crc: 0x2C0677CF,
            }
        );
        assert!(parsed.verify(), "Chunk failed CRC check.");

        let mut buf: Vec<u8> = Vec::new();
        gen(parsed.serialize(), &mut buf).unwrap();
        assert_eq!(buf, iend_chunk);
    }

    #[test]
    fn test_parse_iend() {
        let iend_chunk = [
            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
        ];
        let parsed = Chunk::parse(&iend_chunk).unwrap().1;
        assert_eq!(
            parsed,
            Chunk {
                length: 0,
                chunk_type: "IEND".to_string(),
                chunk_data: vec![],
                crc: 0xAE426082,
            }
        );
        assert!(parsed.verify(), "Chunk failed CRC check.");

        let mut buf: Vec<u8> = Vec::new();
        gen(parsed.serialize(), &mut buf).unwrap();
        assert_eq!(buf, iend_chunk);
    }
}
