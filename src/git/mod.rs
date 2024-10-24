use crate::cli_parser::{Query, Commands};

use std::error::Error;

const REPO_DIRECTORY: &'static str = ".sillygit";
mod lib;
mod common;
mod consts;
mod object;

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    match query.command {
        Commands::CatFile { pretty, object } => lib::cat_file(pretty, object)?,
        Commands::Init => lib::init()?,
        Commands::HashObject { write, filepath } => lib::hash_object(write, filepath)?,
    }

    Ok(())
}
