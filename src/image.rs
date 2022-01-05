use cookie_factory::SerializeFn;
use nom::IResult;
use std::io::Write;

use crate::Chunk;

const PNG_HEADER: &'static [u8] = b"\x89PNG\r\n\x1a\n";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    pub chunks: Vec<Chunk>,
}

impl Image {
    fn new(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }

    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::{bytes::complete::tag, combinator::map, multi::fold_many1, sequence::preceded};

        map(
            preceded(
                tag(PNG_HEADER),
                fold_many1(Chunk::parse, Vec::new, |mut acc: Vec<_>, item| {
                    acc.push(item);
                    acc
                }),
            ),
            Self::new,
        )(i)
    }

    pub fn serialize<W: Write>(self) -> impl SerializeFn<W> {
        use cookie_factory::{combinator::slice, multi::all, sequence::tuple};

        tuple((
            slice(PNG_HEADER),
            all(self.chunks.into_iter().map(Chunk::serialize)),
        ))
    }

    /// Returns Ok(()) on success
    /// and the index of the first invalid chunk on failure
    pub fn verify(&self) -> Result<(), usize> {
        for (i, chunk) in self.chunks.iter().enumerate() {
            if !chunk.verify() {
                return Err(i);
            }
        }

        Ok(())
    }

    pub fn get_first_chunk(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.chunk_type == chunk_type)
    }

    pub fn get_first_chunk_mut(&mut self, chunk_type: &str) -> Option<&mut Chunk> {
        self.chunks.iter_mut().find(|c| c.chunk_type == chunk_type)
    }
}
