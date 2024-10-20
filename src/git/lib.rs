use std::fs;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::error::Error;

const REPO_DIRECTORY: &'static str = ".git";

use flate2::read::ZlibDecoder;

pub fn cat_file(pretty: bool, object: String) -> Result<(), Box<dyn Error>> { 
    match pretty {
        true => {
            let filepath = &format!(
                "{}/objects/{}/{}",
                REPO_DIRECTORY,
                &object[..2],
                &object[2..]
            );
            let file = Path::new(&filepath);
            let file = fs::File::open(&file)?;
            let mut d = ZlibDecoder::new(file);
            let mut buf = String::new();
            d.read_to_string(&mut buf)?;

            let buf: Vec<&str> = buf.split('\0').collect();
            let buf = buf[1];

            println!("{}", buf);
            Ok(())
        },
        _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "cat file mod is not specified"))),
    }
}

pub fn init() -> Result<(), Box<dyn Error>> {
    fs::create_dir(REPO_DIRECTORY)?;
    fs::create_dir(
            Path::new(&format!("{}/objects", REPO_DIRECTORY))
        )?;
    fs::create_dir(
            Path::new(&format!("{}/refs", REPO_DIRECTORY))
        )?;
    fs::write(
            Path::new(&format!("{}/HEAD", REPO_DIRECTORY)), 
            "ref: refs/heads/main\n"
        )?;
    println!("Initialized git directory");
    Ok(())
}
