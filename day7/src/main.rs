extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "bag.pest"]
pub struct BagParser;

#[derive(Debug, PartialEq, Eq)]
pub struct Bag {
    name: String,
    inner: HashMap<String, i32>
}


fn main() {
    println!("Hello, world!");
}

#[test]
fn parse_one() {
    let input = String::from("plaid coral bags contain 2 pale green bags, 2 faded tomato bags, 2 dark salmon bags, 1 vibrant magenta bag.");

    let outer = String::new();
    let inners: HashMap<String, i32> = HashMap::new();
    let bag  = BagParser::parse(Rule::container, input.as_str()).expect("Unsuccessful parse")
        .next().unwrap();

    for container in bag.into_inner() {
        match container.as_rule() {
            Rule::outer_bag => {
                for types in container.into_inner() {
                    match types.as_rule() {
                        Rule::adjective => outer.add(types.as_str())
                        Rule::color => 
                    }
                } 
            },
            Rule::inner_bag => {

            },
            _ => continue
        }
    } 
}

#[test]
fn test_parse_outer_bag() {
    let input = String::from("plaid coral bags");

    let bag  = BagParser::parse(Rule::outer_bag, input.as_str());
    println!("{:?}", bag);
    assert_eq!(1, 1);
}

#[test]
#[ignore]
fn test_parse_number() {
    let bag  = BagParser::parse(Rule::number, "12");
    println!("{:?}", bag);
    assert_eq!(1, 1);
}

#[test]
#[ignore]
fn test_parse_adjective() {
    let bag  = BagParser::parse(Rule::adjective, "plaid");
    println!("{:?}", bag);
    assert_eq!(1, 1);
}

#[test]
#[ignore]
fn test_parse_color() {
    let bag  = BagParser::parse(Rule::color, "coral");
    println!("{:?}", bag);
    assert_eq!(1, 1);
}
