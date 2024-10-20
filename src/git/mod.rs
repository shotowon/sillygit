use crate::cli_parser::{Query, Commands};

mod lib;

pub fn run(query: Query) -> Result<(), String> {
    match query.command {
        Commands::CatFile { pretty, object }  => {
            lib::cat_file(pretty, object);
        },
    }

    Ok(())
}
