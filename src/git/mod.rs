mod object;

use crate::cli_parser::{Commands, Query};
use std::{
    error::Error,
    fs,
    path::Path,
};
use object::Object;

const REPO_DIRECTORY: &'static str = ".sillygit";

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    match query.command {
        Commands::CatFile { sha, .. } => {
            let object = object::Object::from_sha(&sha)?;
            print!("{}", object.content())
        }
        Commands::Init => {
            fs::create_dir(REPO_DIRECTORY)?;
            fs::create_dir(Path::new(&format!("{}/objects", REPO_DIRECTORY)))?;
            fs::create_dir(Path::new(&format!("{}/refs", REPO_DIRECTORY)))?;
            fs::write(
                Path::new(&format!("{}/HEAD", REPO_DIRECTORY)),
                "ref: refs/heads/main\n",
            )?;
            println!("Initialized git directory");
        },
        Commands::HashObject { filepath, .. } => {
            let object = Object::from_file(filepath)?;
            let sha = object.as_hex_hash();
            object.encode_and_write(&sha)?;
            println!("{}", sha);
        },
        _ => {}
    }

    Ok(())
}
