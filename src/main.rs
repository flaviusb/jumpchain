extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "jumpchain.pest"]
struct JumpchainParser;

fn main() {
    println!("Hello, world!");
}
