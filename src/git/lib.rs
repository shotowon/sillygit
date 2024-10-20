use std::{
    fs,
    error,
    path::Path,
};

use crate::git::{
    object,
    common::Error,
    consts::REPO_DIRECTORY,
};

pub fn cat_file(pretty: bool, object: String) -> Result<(), Box<dyn error::Error>> { 

    // there will be more flags
    // so I used match instead of if
    // to match tuple
    match pretty {
        true => {
            let object = object::ObjectFile::new(&object)?;
            match object {
                object::ObjectFile::Blob { content, .. } => {
                    print!("{}", content);
                    Ok(())
                }
            }
        },

        _ => Err(
            Box::new(
                Error::new("cat file mod is not specified")
                )
            ),
    }
}

pub fn init() -> Result<(), Box<dyn error::Error>> {
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
