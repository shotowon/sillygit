use flate2::read::ZlibDecoder;
use std::{
    io::prelude::*,
    fs::File,
    error,
    fmt,
};

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    pub fn new(details: &str) -> Self {
        Error {
            details: details.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for Error {}

pub fn decode_from_file(file: &File) -> Result<String, Box<dyn error::Error>> {
    let mut d = ZlibDecoder::new(file);
    let mut buf = String::new();
    d.read_to_string(&mut buf)?;
    Ok(buf)
}
