use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate swf_parser;

use swf_parser::parsers;

use swf_tree as ast;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Missing input path");
    return;
  }

  let file_path = &args[1];
//  println!("Reading file: {}", filename);

  let mut file = File::open(file_path).expect("File not found");
  let mut data: Vec<u8> = Vec::new();
  file.read_to_end(&mut data).expect("Unable to read file");

//  println!("Input:\n{:?}", &data);

  let swf_file_parse_result: nom::IResult<&[u8], ast::Movie> = parsers::movie::parse_movie(&data[..]);

  match swf_file_parse_result {
    Ok((_, parsed)) => {
      println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    }
    Err(error) => {
      println!("Error:\n{:?}", error);
    }
  }
}
