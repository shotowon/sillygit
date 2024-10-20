#![allow(unused)]
extern crate tinygit;
use clap::Parser;
use tinygit::cli_parser::Query;
use tinygit::git;

fn main() {
    let query = Query::parse();
    git::run(query);
}
