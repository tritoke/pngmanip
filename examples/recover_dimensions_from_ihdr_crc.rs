use anyhow::Result;
use cookie_factory::gen;
use pngmanip::*;
use pngmanip::ihdr_data::*;
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

    let mut image = Image::parse(&im).unwrap().1;

    let ihdr = &mut image.chunks[0];
    let mut ihdr_data = IhdrData::parse(&ihdr.chunk_data).unwrap().1;
    'outer:
    for width in 1..=10_000 {
        for height in 1..=10_000 {
            ihdr_data.width = width;
            ihdr_data.height = height;
            gen(ihdr_data.clone().serialize(), &mut ihdr.chunk_data[..]).unwrap();
            if ihdr.verify() {
                eprintln!("Recovered dimensions: width = {}, height = {}", width, height);
                break 'outer;
            }
        }
    }

    if ihdr.verify() {
        let out = fs::File::create("out.png")?;
        gen(image.serialize(), out)?;
    } else {
        eprintln!("Couldn't recover image.");
    }

    Ok(())
}
