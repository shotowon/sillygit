use crate::cli_parser::{Query, Commands};
use crate::git_lib::cat_file;

pub fn run(query: Query) -> Result<(), String> {
    match query.command {
        Commands::CatFile { pretty, object }  => {
            cat_file(pretty, object);
        },
    }

    Ok(())
}
