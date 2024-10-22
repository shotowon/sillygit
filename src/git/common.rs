use flate2::read::ZlibDecoder;
use std::{
    io::prelude::*,
    fs::File,
    error,
    fmt,
};

pub fn decode_from_file(file: &File) -> Result<String, Box<dyn error::Error>> {
    let mut d = ZlibDecoder::new(file);
    let mut buf = String::new();
    d.read_to_string(&mut buf)?;
    Ok(buf)
}
