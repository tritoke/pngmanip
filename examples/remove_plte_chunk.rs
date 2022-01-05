use anyhow::Result;
use cookie_factory::gen;
use pngmanip::ihdr_data::*;
use pngmanip::*;
use std::env;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let im = if let Some(filename) = env::args().nth(1) {
        fs::read(filename)?
    } else {
        eprintln!("Usage {} <filename>", env::args().next().unwrap());
        std::process::exit(-1);
    };

    let image = Image::parse(&im).unwrap().1;

    let new_ihdr_data = {
        let ihdr = image
            .chunks
            .iter()
            .find(|c| c.chunk_type == "IHDR")
            .expect("Image contained no IHDR chunk");

        let mut ihdr_data = IhdrData::parse(&ihdr.chunk_data).unwrap().1.clone();

        ihdr_data.color_type = ColorType::GreyScale;
        ihdr_data.bit_depth = 8;

        let mut buf = Vec::new();
        gen(ihdr_data.serialize(), &mut buf).unwrap();
        buf
    };

    let mut chunks_without_palette: Vec<Chunk> = image
        .chunks
        .iter()
        .filter(|c| c.chunk_type != "PLTE")
        .cloned()
        .collect();

    chunks_without_palette[0].chunk_data = new_ihdr_data;
    chunks_without_palette[0].fix_crc();

    let new_image = Image {
        chunks: chunks_without_palette,
    };

    let out = fs::File::create("out.png")?;
    gen(new_image.serialize(), out)?;

    Ok(())
}
