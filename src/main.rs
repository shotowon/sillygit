#![allow(unused)]
extern crate tinygit;
use clap::Parser;
use tinygit::cli_parser::Query;


fn main() {
    let query = Query::parse();
}
