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
  jump_type:        Vec<(&'a str, &'a str, i64)>,
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

// Cost is positive, refund is negative, Free is zero
use pest::iterators::Pair;
fn parse_cost_refund(cost_refund: Pair<Rule>) -> i64 {
  match cost_refund.as_rule() {
    Rule::refund => (0 - cost_refund.into_inner().next().unwrap().as_str().parse::<i64>().unwrap()),
    Rule::cost   => {
      match &cost_refund.into_inner().collect::<Vec<Pair<Rule>>>()[..] {
        []     => 0,
        [a]    => a.as_str().parse::<i64>().unwrap(),
        [a, b] => b.as_str().parse::<i64>().unwrap(),
        _      => panic!("Too many things."),
      }
    }
    _            => panic!("parse_cost_refund given {:}", cost_refund),
  }
}

fn parse_jumpchain_file<'a>(file: &'a str) -> Result<Jumpchain<'a>, pest::error::Error<Rule>> {
    let jumpchain_stream = JumpchainParser::parse(Rule::document, file)?;
    println!("Jumpchain stream: {:?}", jumpchain_stream);
    // Make section
    use pest::iterators::Pair;
    fn make_section<'a>(section_pair: Pair<'a, Rule>) -> Section<'a> {
      let mut section = Section{name: "", points_increment: 0, jump_type: vec!{}, jump_type_cost: 0, perk: vec!{}, remainder: None, points_spent: None, points_remainder: None};
      //println!("section_pair.into_inner().next().unwrap().into_inner(): {:?}",section_pair.into_inner().next().unwrap().into_inner());
      let mut section_iterator = section_pair.into_inner();
      section.name = section_iterator.next().unwrap().into_inner().as_str();
      let intermediate = (section_iterator.next().unwrap().into_inner().next().unwrap().as_str());
      section.points_increment = intermediate.parse::<i64>().unwrap();
      // Get all jump type data
      let jump_types_stream = section_iterator.next().unwrap().into_inner();
      section
    }
    let sections = jumpchain_stream.filter(|pair| match pair.as_rule() {
      Rule::section => true,
      _             => false,
    }).map(|section| make_section(section)).collect();
    Ok(Jumpchain{sections: sections})
}
