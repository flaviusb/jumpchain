extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::env;
use std::option;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser)]
#[grammar = "jumpchain.pest"]
struct JumpchainParser;

struct Jumpchain<'a> {
  sections: Vec<Section<'a>>,
}

struct Section<'a> {
  name:             &'a str,
  points_increment: i64,
  jump_type:        &'a str,
  jump_type_cost:   i64,
  perk:             Vec<(&'a str, i64)>,
  remainder:        Option<i64>,
  points_spent:     Option<i64>,
  points_remainder: Option<i64>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if(args.len() == 0) {
      println!("Usage: jumpchain [file]")
    } else {
    }
    Ok(())
}
