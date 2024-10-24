use std::{
    error::Error,
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
};

use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

use crate::git::REPO_DIRECTORY;

pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl ObjectKind {
    fn from_str(object_type: &str) -> Result<Self, Box<dyn Error>> {
        match object_type {
            "blob" => Ok(ObjectKind::Blob),
            "tree" => Ok(ObjectKind::Tree),
            "commit" => Ok(ObjectKind::Blob),
            "tag" => Ok(ObjectKind::Tree),
            _ => Err(Box::from(format!(
                "cannot match object type: got {}",
                object_type
            ))),
        }
    }

    fn to_str<'a>(&'a self) -> &'a str {
        match self {
            ObjectKind::Blob => "blob",
            ObjectKind::Tree => "tree",
            ObjectKind::Commit => "commit",
            ObjectKind::Tag => "tag",
        }
    }
}

pub struct Object {
    pub kind: ObjectKind,
    size: u64,
    content: String,
}

impl Object {
    pub fn from_file(filepath: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut file = fs::File::open(filepath)?;
        let metadata = file.metadata()?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;
        let size = metadata.len();

        Ok(Object {
            kind: ObjectKind::Blob,
            size,
            content,
        })
    }

    pub fn from_sha(sha: &str) -> Result<Self, Box<dyn Error>> {
        if sha.len() != 40 {
            return Err(Box::from("error: invalid sha"));
        }

        let object_content = Self::decode_and_read(sha)?;
        Self::parse_object(&object_content)
    }

    fn parse_object(object_content: &str) -> Result<Self, Box<dyn Error>> {
        let null_pos = object_content
            .find('\0')
            .ok_or("invalid object file content")?;
        let (header, content) = object_content.split_at(null_pos);
        let content = &content[1..];

        let space_pos = header.find(' ').ok_or("invalid object file content")?;
        let (object_type, size) = header.split_at(space_pos);
        let size = &size[1..];

        let size: u64 = size.parse()?;
        let kind = ObjectKind::from_str(object_type)?;
        let content = content.to_string();

        Ok(Object {
            kind,
            size,
            content,
        })
    }

    fn path_from_sha(sha: &str) -> Result<PathBuf, Box<dyn Error>> {
        if sha.len() != 40 {
            return Err(Box::from("error: invalid sha"));
        }

        let (folder, file) = sha.split_at(2);
        let mut path = PathBuf::new();

        path.push(&format!("{}/objects/{}/{}", REPO_DIRECTORY, folder, file));
        Ok(path)
    }

    fn decode_and_read(sha: &str) -> Result<String, Box<dyn Error>> {
        let path = Self::path_from_sha(sha)?;
        let file = fs::File::open(path.as_path())?;
        let mut d = ZlibDecoder::new(file);

        let mut buf = String::new();
        d.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn encode_and_write(&self, sha: &str) -> Result<(), Box<dyn Error>> {
        let path = Self::path_from_sha(sha)?;
        let content = self.to_object_content();

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(content.as_bytes())?;
        let compressed = e.finish()?;

        let dir = path.parent().ok_or("invalid path")?;
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
        
        let mut file = fs::File::create(path)?;
        file.write_all(&compressed)?;
        
        Ok(()) 
    }

    pub fn content<'a>(&'a self) -> &'a str {
        &self.content
    }
    
    fn to_object_content(&self) -> String {
        format!("{} {}\0{}", self.kind.to_str(), self.size, self.content)
    }

    pub fn as_hex_hash(&self) -> String {
        let content_to_hash = self.to_object_content();
        let mut hasher = Sha1::new();
        hasher.update(content_to_hash.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
