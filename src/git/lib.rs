use std::fs;
use std::path::Path;
use std::io;
use std::error::Error;

use crate::git::common;
use crate::git::consts::REPO_DIRECTORY;

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
            let contents = common::decode_from_file(&file)?;
            let contents: Vec<&str> = contents.split('\0').collect();
            let contents = contents[1];

            print!("{}", contents);
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
