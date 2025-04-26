use std::fs;

use clap::Parser;
use cmd_parser::CmdParser;

mod cmd_parser;
fn main() {
    let args = CmdParser::parse();
    match args.file {
        Some(file) => {
            if !file.exists() {
                panic!("file({:?}) doesn't exsit",file)
            }
            let contents = fs::read_to_string(file).expect("Failed to read file");
            
        },
        None => println!("rep"),
    }
}
