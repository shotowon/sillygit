use std::{
    fs::File,
    path::Path,
    error,
};

use crate::git::consts::REPO_DIRECTORY;
use crate::git::common::{Error, decode_from_file};

pub enum ObjectFile {
    Blob {
        name: String,
        header: String,
        content: String,
    },
}

impl ObjectFile {
    pub fn read(object: &str) -> Result<Self, Box<dyn error::Error>> {
            let filepath = &format!(
                "{}/objects/{}/{}",
                REPO_DIRECTORY,
                &object[..2],
                &object[2..]
            );

            let file = Path::new(&filepath);
            let file = File::open(&file)?;
            let contents = decode_from_file(&file)?;

            if contents.starts_with("blob") {
                let contents: Vec<&str> = contents.split('\0').collect();
                let header = contents[0];
                return Ok(
                    ObjectFile::Blob {
                        name: object.to_string(),
                        header: header.to_string(), 
                        content: contents[1].to_string(),
                    }
                )
            }
            
            Err(Box::new(Error::new("failed to identify object")))
    }
}
