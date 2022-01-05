use anyhow::Result;
use cookie_factory::gen;
use pngmanip::*;
use rand::{thread_rng, Fill};
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

    let mut rng = thread_rng();
    let plte_chunk = image.get_first_chunk_mut("PLTE").unwrap();
    //plte_chunk.chunk_data.try_fill(&mut rng)?;
    plte_chunk.fix_crc();

    let out = fs::File::create("out.png")?;
    gen(image.serialize(), out)?;

    Ok(())
}
