extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

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
  perk:             Vec<(&'a str, i64)>,
  remainder:        Option<i64>,
  points_spent:     Option<i64>,
  points_remainder: Option<i64>,
}

fn main() {
    println!("Hello, world!");
}
