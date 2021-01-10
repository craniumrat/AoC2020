extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
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
    //Each s in split must be valid for at least one rule
    
    for s in line.split(",") {
        let mut valid = false;
        let value = s.parse::<i32>().unwrap();
        for rule in rules {
            if (value >= rule.range1_start && value <= rule.range1_end)
                || (value >= rule.range2_start && value <= rule.range2_end) {
                valid = true;
            }
        }

        if valid == false {
            return false;
        }
    }

    true
}

fn load_rules() -> HashSet<TicketRule> {
    let rules_file = File::open("rules.txt").expect("Couldnt open rules.txt");
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

        // println!("{:?}", rule);

        rules.insert(rule);
    }

    rules
}

fn main() -> Result<(), std::io::Error> {

    let my_ticket = String::from("139,67,71,59,149,89,101,83,107,103,79,157,151,113,61,109,73,97,137,53");

    let input_file = File::open("input.txt")?;
    let reader = io::BufReader::new(input_file);

    let rules = load_rules();

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


    //For each valid_line, grab the snippets into arrays.
    let mut transpose: Vec<Vec<i32>> = vec![];

    for _ in 0..rules.len() {
        transpose.push(Vec::new());
    }
    
    for line in valid_lines {
        println!("Valid line: {}", line);
        for (i, s) in line.split(",").enumerate() {
            transpose[i].push(s.parse::<i32>().unwrap());
        }
    }

    let mut possible_pos: HashMap<i32, Vec<&String>> = HashMap::new();
    let mut i = 0;
    for _ in 0..rules.len() {
        possible_pos.insert(i, Vec::new());
        i += 1;
    }

    for (column_number, column) in transpose.iter().enumerate() {
        for rule in &rules {
            let mut valid = true;
            for value in column {
                if !((value >= &rule.range1_start && value <= &rule.range1_end) ||
                    (value >= &rule.range2_start && value <= &rule.range2_end)) {
                    println!("Invalid Value: {} for rule: {}", value, rule.name);
                    valid = false;
                    break;
                }
            }

            if valid == true {
                let possibles = possible_pos.get_mut(&(column_number as i32)).unwrap(); 
                possibles.push(&rule.name);

                println!("Adding possible: {} for column: {}", rule.name, column_number);
            }
        }
    }

    println!("{:?}", possible_pos);

    let mut rules_pos: HashMap<i32, &String> = HashMap::new();
    let mut available_names: Vec<&String> = rules.iter().map(|rule| &rule.name).collect();

    while rules_pos.len() != rules.len() {

        for (column, possibles) in &possible_pos {
            if possibles.len() == 1 {
                rules_pos.insert(*column, &possibles[0]);
                available_names = available_names.into_iter().filter(|&name| name != possibles[0]).collect();

                println!("Rules Pos: {:?}", rules_pos);
                println!("Available Names: {:?}", available_names);
            }
        }

        for (_, rule_name) in &rules_pos {
            possible_pos = possible_pos.into_iter().map(|(pos, names)|
            {    
                let names: Vec<&String> = names.into_iter().filter(|n| n != rule_name).collect();
                (pos, names)
            }).collect();
        }

        possible_pos = possible_pos.into_iter().filter(|(_, possibles)| possibles.len() != 0).collect();
        println!("Updated possibles: {:?}", possible_pos);        
    }

    println!("Rules Pos: {:?}", rules_pos);

    rules_pos = rules_pos.into_iter().filter(|(_, name)| name.starts_with("departure")).collect();
    let ticket_pos: Vec<_> = my_ticket.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    let mut answer = 1_i64;
    for (pos, name) in &rules_pos {
        println!("Rule: {}, Value in ticket: {}", name, ticket_pos[*pos as usize]);
        answer *= ticket_pos[*pos as usize] as i64;
    }

    println!("Part 2: {}", answer);
    Ok(())
}