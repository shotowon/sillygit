use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

pub fn decode_from_file(file: &File) -> Result<String, Box<dyn Error>> {
    let mut d = ZlibDecoder::new(file);
    let mut buf = String::new();
    d.read_to_string(&mut buf)?;
    Ok(buf)
}
