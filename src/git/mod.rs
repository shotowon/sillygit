use crate::cli_parser::{Query, Commands};
use crate::git_lib::cat_file;

mod lib;

pub fn run(query: Query) -> Result<(), String> {
    match query.command {
        Commands::CatFile { pretty, object }  => {
            cat_file(pretty, object);
        },
    }

    Ok(())
}
