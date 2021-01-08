extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Parser)]
#[grammar = "rules.pest"]
pub struct RulesParser;

#[derive(Debug, Eq)]
struct TicketRule {
    name: String,
    range1_start: i32,
    range1_end: i32,
    range2_start: i32,
    range2_end: i32,
}

impl Hash for TicketRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for TicketRule {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

fn is_valid_line(rules: &HashSet<TicketRule>, line: &str) -> bool {
    let mut valid = false;

    for s in line.split(",") {
        let value = s.parse::<i32>().unwrap();
        for rule in rules {
            if (value >= rule.range1_start && value <= rule.range1_end)
                || (value >= rule.range2_start && value <= rule.range2_end) {
                valid = true;
                break;
            }
        }
    }

    valid
}

fn main() -> Result<(), std::io::Error> {

    let rules_file = File::open("rules.txt")?;
    let reader = io::BufReader::new(rules_file);

    let mut rules: HashSet<TicketRule> = HashSet::new();

    for line in reader.lines() {
        let l = line.unwrap(); 
        let rule = RulesParser::parse(Rule::line, l.as_str()).expect("Unsuccessful parse")
            .next().unwrap();

        let mut inner = rule.into_inner();
        let name = String::from(inner.next().unwrap().as_str());
        let r1_start = inner.next().unwrap().as_str().parse::<i32>().unwrap();
        let r1_end = inner.next().unwrap().as_str().parse::<i32>().unwrap();
        let r2_start = inner.next().unwrap().as_str().parse::<i32>().unwrap();
        let r2_end = inner.next().unwrap().as_str().parse::<i32>().unwrap();

        let rule = TicketRule{ name: name, range1_start: r1_start, range1_end: r1_end,
            range2_start: r2_start, range2_end: r2_end};

        println!("{:?}", rule);

        rules.insert(rule);
    }

    let input_file = File::open("input.txt")?;
    let reader = io::BufReader::new(input_file);

    let mut invalids = 0;

    let mut valid_lines: Vec<String> = vec![];

    for line in reader.lines() {
        let l = line.unwrap();

        if is_valid_line(&rules, l.as_str()) {
            valid_lines.push(String::from(l.as_str()));
        }


        for s in l.split(",") {
            let value = s.parse::<i32>().unwrap();
            let mut valid = false;
            for rule in &rules {
                if (value >= rule.range1_start && value <= rule.range1_end)
                    || (value >= rule.range2_start && value <= rule.range2_end) {
                    valid = true;
                    break;
                }
            }

            if !valid {
                invalids += value;
            }
        }
    }

    

    println!("Part 1: {}", invalids);

    Ok(())
}
