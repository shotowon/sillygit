use std::fs;
use std::path;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;

use flate2::read::ZlibDecoder;

pub fn cat_file(pretty: bool, object: String) -> Result<(), Error> { 
    match pretty {
        true => {
            let filepath = &format!(
                ".git/objects/{}/{}",
                &object[..2],
                &object[2..]
            );
            let file = path::Path::new(&filepath);
            let file = fs::File::open(&file)?;
            let mut d = ZlibDecoder::new(file);
            let mut buf = String::new();
            d.read_to_string(&mut buf)?;

            let buf: Vec<&str> = buf.split('\0').collect();
            let buf = buf[1];

            println!("{}", buf);
            Ok(())
        },
        _ => Err(Error::new(ErrorKind::InvalidInput, "cat file mod is not specified")),
    }
}

pub fn init() -> Result<(), Error> {
    fs::create_dir(".tinygit")?;
    fs::create_dir(".tinygit/objects")?;
    fs::create_dir(".tinygit/refs")?;
    fs::write(".tinygit/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");
    Ok(())
}
