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

#[derive(Debug)]
struct Jumpchain<'a> {
  sections: Vec<Section<'a>>,
}

#[derive(Debug)]
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

use std::fs;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if(args.len() == 1) {
      println!("Usage: jumpchain [file]")
    } else {
      let unparsed_file = fs::read_to_string(args[1].clone()).expect("cannot read file");
      let jumpchain: Jumpchain = parse_jumpchain_file(&unparsed_file).expect("unsuccessful parse");
      println!("{:?}", jumpchain);
    }
    Ok(())
}

fn parse_jumpchain_file<'a>(file: &str) -> Result<Jumpchain<'a>, pest::error::Error<Rule>> {
    let jumpchain_stream = JumpchainParser::parse(Rule::document, file)?;
    println!("Jumpchain stream: {:?}", jumpchain_stream);
    // Make section
    use pest::iterators::Pair;
    fn make_section<'a>(section_pair: Pair<Rule>) -> Section<'a> {
      let mut section = Section{name: "", points_increment: 0, jump_type: "", jump_type_cost: 0, perk: vec!{}, remainder: None, points_spent: None, points_remainder: None};
      println!("section_pair.into_inner(): {:?}",section_pair.into_inner());
      section
    }
    let sections = jumpchain_stream.map(|section| make_section(section)).collect();
    Ok(Jumpchain{sections: sections})
}
