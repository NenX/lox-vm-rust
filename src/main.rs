use std::fs;

use clap::Parser;
use cmd_parser::CmdParser;
use lox_vm_rust::interpret;

mod cmd_parser;
fn main() {
    let args = CmdParser::parse();
    match args.file {
        Some(file) => {
            if !file.exists() {
                panic!("file({:?}) doesn't exsit", file)
            }
            let contents = fs::read_to_string(file).expect("read file error");
            interpret(&contents).expect("interpret error");
        }
        None => println!("rep"),
    }
}
