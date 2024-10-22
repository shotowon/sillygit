use std::{
    io::prelude::*,
    fs::File,
    path::Path,
    error::Error,
};

use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::git::consts::REPO_DIRECTORY;
use crate::git::common::decode_from_file;

pub enum ObjectFile {
    Blob {
        header: String,
        content: String,
    },
}

impl ObjectFile {
    pub fn read(object: &str) -> Result<Self, Box<dyn Error>> {
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
            
            Err(Box::from("failed to identify object"))
    }

    pub fn blob_from_file(filepath: &str) -> Result<
        Self,
        Box<dyn Error>
            > {
        let filepath = Path::new(filepath);
        let mut file = File::open(filepath)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(Box::from("not a file"));
        }

        let size = metadata.len();
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(ObjectFile::Blob { header: format!("blob {}", size), content })
    }

    pub fn as_compressed_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        if let ObjectFile::Blob { header, content } = self {
                let content_to_compress = format!("{}\0{}", header, content);
                let mut e = ZlibEncoder::new(
                    Vec::new(),
                    Compression::default()
                    );

                e.write_all(content_to_compress.as_bytes())?;
                let compressed = e.finish()?;
                return Ok(compressed);
        }

        Err(Box::from("Object is not a blob"))
    }
}
