use std::{
    error::Error,
    fs,
    io::prelude::*,
    path::{Path, PathBuf},
};

use flate2::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};

use crate::git::consts::REPO_DIRECTORY;

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
            fs::create_dir_all(dir);
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

    fn as_hex_hash(&self) -> String {
        let content_to_hash = self.to_object_content();
        let mut hasher = Sha1::new();
        hasher.update(content_to_hash.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

pub enum ObjectFile {
    Blob { header: String, content: String },
}
/*
impl ObjectFile {
    pub fn read(object: &str) -> Result<Self, Box<dyn Error>> {
        let filepath = &format!(
            "{}/objects/{}/{}",
            REPO_DIRECTORY,
            &object[..2],
            &object[2..]
        );

        let file = Path::new(&filepath);
        let file = fs::File::open(&file)?;
        let contents = decode_from_file(&file)?;

        if contents.starts_with("blob") {
            let contents: Vec<&str> = contents.split('\0').collect();
            let header = contents[0];
            return Ok(ObjectFile::Blob {
                header: header.to_string(),
                content: contents[1].to_string(),
            });
        }

        Err(Box::from("failed to identify object"))
    }

    pub fn blob_from_file(filepath: &str) -> Result<Self, Box<dyn Error>> {
        let filepath = Path::new(filepath);
        let mut file = fs::File::open(filepath)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(Box::from("not a file"));
        }

        let size = metadata.len();
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(ObjectFile::Blob {
            header: format!("blob {}", size),
            content,
        })
    }

    pub fn as_compressed_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        if let ObjectFile::Blob { header, content } = self {
            let content_to_compress = format!("{}\0{}", header, content);
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

            e.write_all(content_to_compress.as_bytes())?;
            let compressed = e.finish()?.to_vec();
            return Ok(compressed);
        }

        Err(Box::from("Object is not a blob"))
    }

    pub fn blob_as_hex_hash(&self) -> Result<String, Box<dyn Error>> {
        if let ObjectFile::Blob { header, content } = self {
            let content_to_hash = format!("{}\0{}", header, content);
            let mut hasher = Sha1::new();
            hasher.update(content_to_hash.as_bytes());
            let result = hasher.finalize();
            return Ok(format!("{:x}", result));
        }

        Err(Box::from("Object is not a blob"))
    }

    pub fn hash_write(&self) -> Result<(), Box<dyn Error>> {
        let hash = self.blob_as_hex_hash()?;
        let content = self.as_compressed_bytes()?;

        let folder_path = format!("./{}/objects/{}", REPO_DIRECTORY, &hash[..2]);

        if !Path::new(&folder_path).exists() {
            fs::create_dir_all(&folder_path)?;
        }

        let object_file_path = Path::new(&folder_path).join(&hash[2..]);
        let mut file = fs::File::create(object_file_path)?;
        file.write_all(&content[..])?;
        println!("{}", hash);

        Ok(())
    }
}

*/
