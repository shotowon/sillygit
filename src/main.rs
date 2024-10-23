#![allow(unused)]
extern crate sillygit;
use clap::Parser;
use sillygit::cli_parser::Query;
use sillygit::git;

fn main() {
    let query = Query::parse();
    git::run(query).unwrap();
}
