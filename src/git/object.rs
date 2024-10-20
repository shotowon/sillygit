use std::{
    io::prelude::*,
    fs::File,
    path::Path,
    error,
};

use crate::git::consts::REPO_DIRECTORY;
use crate::git::common::{Error, decode_from_file};

pub enum ObjectFile {
    Blob {
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
                        header: header.to_string(), 
                        content: contents[1].to_string(),
                    }
                )
            }
            
            Err(Box::new(Error::new("failed to identify object")))
    }

    pub fn blob_from_file(filepath: &str) -> Result<
        Self,
        Box<dyn error::Error>
            > {
        let filepath = Path::new(filepath);
        let mut file = File::open(filepath)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(
                Box::new(
                    Error::new("not a file")
                    )
                );
        }

        let size = metadata.len();
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(ObjectFile::Blob { header: format!("blob {}", size), content })
    }
}
