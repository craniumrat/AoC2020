extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Parser)]
#[grammar = "bag.pest"]
pub struct BagParser;

#[derive(Debug, PartialEq, Eq)]
pub struct Bag {
    name: String,
    inner: HashMap<String, i32>
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Bag {
    fn parse(input: &str) -> Bag {
        let mut outer = String::new();
        let mut inners: HashMap<String, i32> = HashMap::new();
        let bag  = BagParser::parse(Rule::container, input).expect("Unsuccessful parse")
            .next().unwrap();
    
        for container in bag.into_inner() {
            match container.as_rule() {
                Rule::outer_bag => {
                    for types in container.into_inner() {
                        match types.as_rule() {
                            Rule::desc => outer.push_str(types.as_str()),
                            _ => unreachable!(),
                        }
                    }    
                },
                Rule::inner_bags => {
                    for inner_bags in container.into_inner() {
                        let mut count = 0;
                        for inner_bag in inner_bags.into_inner() {
                            let mut inner = String::new();
                            match inner_bag.as_rule() {
                                Rule::number => count = inner_bag.as_str().parse::<i32>().unwrap(),
                                Rule::desc => { 
                                    inner.push_str(inner_bag.as_str()); 
                                    inners.insert(inner, count);
                                },
                                _ => println!("{:?} | {:?}", inner_bag.as_rule(), inner_bag.as_str()),
                            }
                        }
                    }
                },
                _ => continue
            }
        }
        
        Bag {
            name: outer,
            inner: inners,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut bags: HashSet<Bag> = HashSet::new();

    for line in reader.lines() {
        bags.insert(Bag::parse(&line.unwrap()));
    }

    // println!("{}", bags.len());

    //We start with a hashmap of <name of bag, visited?>.
    //For each entry, if our bag is an "inner" of another bag,
    //
    //loop till hashmap is all visited = true.

    let mut next = vec!["shiny gold"];
    let mut visited: HashSet<&str> = HashSet::new();

    while next.len() != 0 {
        let to_test = next.pop().unwrap();

        println!("Testing: {}", to_test);

        visited.insert(to_test);

        for bag in bags.iter() {
            if bag.inner.contains_key(to_test) && !visited.contains(bag.name.as_str()) {
                println!("Adding to next: {}", bag.name);
                next.push(&bag.name);
            }
        }
    }

    println!("Part 1: {}", visited.len() - 1); //We have to subtract 1 for "shiny gold" itself

    

    Ok(())
}

#[test]
fn parse_one() {
    let input = String::from("plaid coral bags contain 1 pale green bags, 2 faded tomato bags, 3 dark salmon bags, 4 vibrant magenta bag.");

    let bag = Bag::parse(&input);
    assert_eq!(bag.name, "plaid coral");
    assert_eq!(bag.inner.len(), 4);
}

#[test]
fn parse_no_other_bags() {
    let input = String::from("faded beige bags contain no other bags.");

    let bag = Bag::parse(&input);
    assert_eq!(bag.name, "faded beige");
    assert_eq!(bag.inner.len(), 0);
}


#[test]
#[ignore]
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
    let bag  = BagParser::parse(Rule::desc, "plaid coral");
    println!("{:?}", bag);
    assert_eq!(1, 1);
}
