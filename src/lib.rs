#![feature(arbitrary_enum_discriminant)]

mod image;
pub use image::Image;

mod chunk;
pub use chunk::Chunk;

pub mod ihdr_data;
