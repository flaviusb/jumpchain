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
  perks:            Vec<(&'a str, i64)>,
  remainder:        Option<i64>,
  points_spent:     Option<i64>,
  points_remainder: Option<i64>,
}

use std::fs;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let help = String::from("--help");
    match &args[..] {
      []            => panic!("Invoked nameless?"),
      [_]           => usage(),
      [_, option] if option == "--help" => usage(),
      [_, filename]  => {
        let unparsed_file = fs::read_to_string(filename.clone()).expect("cannot read file");
        let jumpchain: Jumpchain = parse_jumpchain_file(&unparsed_file).expect("unsuccessful parse");
        println!("{:?}", jumpchain);
      },
      [_, option, filename] if option == "--print-only" => {
        let unparsed_file = fs::read_to_string(filename.clone()).expect("cannot read file");
        let jumpchain: Jumpchain = parse_jumpchain_file(&unparsed_file).expect("unsuccessful parse");
        println!("{:?}", jumpchain);
      },
      [_, option, filename] if option == "--do-calcs-by-section-only" => {
        let unparsed_file = fs::read_to_string(filename.clone()).expect("cannot read file");
        let jumpchain: Jumpchain = parse_jumpchain_file(&unparsed_file).expect("unsuccessful parse");
        let calcs = calc(jumpchain, CalcRule::Individual);
        for section in calcs {
          println!("## {}\nRemainder: {}\nPoints Spent: {}\nAccumulation: {}", section.0, section.1, section.2, section.3);
        }
      },
      _             => usage(),
    }
    Ok(())
}

fn usage() {
  println!("Usage: jumpchain (options?) [file]");
}

#[derive(Debug, PartialEq, Eq)]
enum CalcRule {
  Individual, DoubleOrNothing
}

// Return a vec of (name, points 'leftover', points spent, and accumulation
fn calc<'a>(jumpchain: Jumpchain<'a>, calc_kind: CalcRule) -> Vec<(&'a str, i64, i64, i64)> {
  let mut acc_points: i64 = 0;
  let mut jumps: Vec<(&'a str, i64, i64, i64)> = vec!{};
  for jump in jumpchain.sections.into_iter() {
    let start = (if calc_kind == CalcRule::DoubleOrNothing { acc_points } else { 0 }) + jump.points_increment;
    let mut points_leftover = start;
    let name = jump.name;
    for jump_type in jump.jump_type.into_iter() {
      points_leftover -= jump_type.2;
    }
    for perk in jump.perks.into_iter() {
      points_leftover -= perk.1;
    }
    acc_points = if calc_kind == CalcRule::DoubleOrNothing { points_leftover * 2 } else { 0 };
    jumps.push((name, points_leftover, start - points_leftover, acc_points));
  }
  jumps
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
        [a, b] => {
          let val = b.as_str();
          if val == "Free)" { 0 } else { val.parse::<i64>().unwrap() }
        },
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
      let mut section = Section{name: "", points_increment: 0, jump_type: vec!{}, perks: vec!{}, remainder: None, points_spent: None, points_remainder: None};
      //println!("section_pair.into_inner().next().unwrap().into_inner(): {:?}",section_pair.into_inner().next().unwrap().into_inner());
      let mut section_iterator = section_pair.into_inner();
      section.name = section_iterator.next().unwrap().into_inner().as_str();
      let intermediate = (section_iterator.next().unwrap().into_inner().next().unwrap().as_str());
      section.points_increment = intermediate.parse::<i64>().unwrap();
      // Get all jump type data
      let jump_types_stream = section_iterator.next().unwrap().into_inner();
      let jump_types = jump_types_stream.map(|jump_kv: Pair<Rule>| -> (&str, &str, i64) {
                                                 //println!("jump_kv: {:?}", jump_kv);
                                                 let out: (&str, &str, i64) = match &jump_kv.into_inner().collect::<Vec<Pair<Rule>>>()[..] {
                                                   [k, v]     => (k.as_str(), v.as_str(), 0),
                                                   [k, v, cr] => (k.as_str(), v.as_str(), parse_cost_refund(cr.clone())),
                                                   a          => panic!("Not the right number of jump type parts: {:?}", a),
                                                 };
                                                 out
                                            }).collect::<Vec<(&str, &str, i64)>>();
      section.jump_type = jump_types;
      let perks_stream = section_iterator.next().unwrap().into_inner();
      let perks = perks_stream.map(|perk: Pair<Rule>| -> (&str, i64) {
                                                 //println!("perk: {:?}", perk);
                                                 let out: (&str, i64) = match &perk.into_inner().collect::<Vec<Pair<Rule>>>()[..] {
                                                   [k, cr] => (k.as_str(), parse_cost_refund(cr.clone())),
                                                   a          => panic!("Not the right number of perk parts: {:?}", a),
                                                 };
                                                 out
                                            }).collect::<Vec<(&str, i64)>>();
      section.perks = perks;
      for bits in section_iterator {
        match bits.as_rule() {
          Rule::remainder         => section.remainder        = Some(bits.into_inner().as_str().parse::<i64>().unwrap()),
          Rule::points_spent      => section.points_spent     = Some(bits.into_inner().as_str().parse::<i64>().unwrap()),
          Rule::points_remainder  => section.points_remainder = Some(bits.into_inner().as_str().parse::<i64>().unwrap()),
          Rule::remainder_doubles => section.points_remainder = Some(bits.into_inner().as_str().parse::<i64>().unwrap()),
          _                       => (),
        }
      }

      section
    }
    let sections = jumpchain_stream.filter(|pair| match pair.as_rule() {
      Rule::section => true,
      _             => false,
    }).map(|section| make_section(section)).collect();
    Ok(Jumpchain{sections: sections})
}
