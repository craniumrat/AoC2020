extern crate pest;

#[macro_use]
extern crate pest_derive;

use std::fs::File;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::collections::HashMap;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rules.pest"]
pub struct MatchRulesParser;

//For a change, we are manually going to split the 
//input file into 2: rules file and
//the data file to validate the rules on.

#[derive(Debug)]
enum RuleType {
    Prim(char),
    Or{left: Rc<MatchRule>, right: Rc<MatchRule>},
    And(Vec<Rc<MatchRule>>),
}

#[derive(Debug)]
struct MatchRule {
    no: i32,
    rule_type: RuleType, 
}

impl MatchRule {
    fn consume(&self, input: &str, offset: usize) -> (bool, usize) {

        // match self.no {
        //     8|11|42|31 => println!("input: {} of len: {}. Offset: {}. rule: {}", input, input.len(), offset, self.no),
        //     _ => {},
        // };

        match &self.rule_type {
            RuleType::Prim(c) => {
                if input.chars().nth(offset).unwrap() == *c {
                    return (true, offset + 1);
                } else {
                    return (false, offset);
                }
            },
            RuleType::Or{left, right} => {
                let (success, next_offset) = left.consume(input, offset);
                if !success {
                    let (success, next_offset) = right.consume(input, offset);
                    if success {
                        return (success, next_offset);
                    } else {
                        return (success, offset);
                    }
                } else {
                    return (success, next_offset);
                }
            },
            RuleType::And(rules) => {
                if rules.len() == 0 {
                    return (true, offset);
                }

                let (first, rest) = rules.split_first().unwrap();

                let (success, next_offset) = first.consume(input, offset);
                if !success {
                    return (false, offset);
                }

                let subrule = MatchRule{no: self.no, rule_type: RuleType::And(rest.to_vec()) };
                return subrule.consume(input, next_offset);
            },
        }
    }
}

#[derive(Debug)]
enum RuleTypeParsed {
    Prim(char),
    Or{ left: Vec<i32>, right: Vec<i32>},
    And(Vec<i32>),
}

fn parse_rules() -> HashMap<i32, RuleTypeParsed> {
    let file = File::open("rules.txt").expect("Unable to open rules.txt");
    let reader = io::BufReader::new(file);
    let mut rules: HashMap<i32, RuleTypeParsed> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let rule = MatchRulesParser::parse(Rule::rule, line.as_str())
            .expect("Unsuccessful parse").next().unwrap();
        
        let mut inners_iter = rule.into_inner();
        
        let rule_number = inners_iter.next().unwrap().as_str().parse::<i32>().unwrap();
        let rule_type = inners_iter.next().unwrap();
        match rule_type.as_rule() {
            Rule::and => {
                // println!("{}", rule_number);
                let subrules: Vec<i32> = rule_type.as_str().split(" ").map(|s| s.parse::<i32>().unwrap()).collect();
                rules.insert(rule_number, RuleTypeParsed::And(subrules));
            },
            Rule::or => {
                let (left_iter, right_iter) = {
                    let mut subors_iter = rule_type.into_inner();
                    let left = subors_iter.next().unwrap();
                    let right = subors_iter.next().unwrap();
                    (left, right)
                };

                let left: Vec<_> = left_iter.into_inner().map(|s| s.as_str().parse::<i32>().unwrap()).collect();
                let right: Vec<_> = right_iter.into_inner().map(|s| s.as_str().parse::<i32>().unwrap()).collect();
                rules.insert(rule_number, RuleTypeParsed::Or{left, right});
            },
            Rule::prim => {
                let c = rule_type.as_str().chars().nth(1).unwrap();  //Grab the 2nd character. 1st one is quote 
                // println!("rule_type: {}, Rule: {:?}", rule_type.as_str(), rule_type.as_rule());
                rules.insert(rule_number, RuleTypeParsed::Prim(c)); 
            },
            _ => unreachable!(),
        }
    }

    rules
}

fn make_rule_zero(parsed_rules: &HashMap<i32, RuleTypeParsed>) -> MatchRule {
    let zero = make_rule(0, parsed_rules);
    zero
}

fn make_rule(rule_number: i32, parsed_rules: &HashMap<i32, RuleTypeParsed>) -> MatchRule {
    let parsed_rule = parsed_rules.get(&rule_number).unwrap();

    match parsed_rule {
        RuleTypeParsed::Prim(c) => {
            return MatchRule{no: rule_number, rule_type: RuleType::Prim(*c)};
        },
        RuleTypeParsed::And(subrules) => {
            let subrules: Vec<Rc<MatchRule>> = subrules.iter().map(|s| Rc::new(make_rule(*s, parsed_rules))).collect();
            return MatchRule{no: rule_number, rule_type: RuleType::And(subrules)};
        },
        RuleTypeParsed::Or{left, right} => {
            let left: Vec<Rc<MatchRule>> = left.iter().map(|s| Rc::new(make_rule(*s, parsed_rules))).collect();
            let right: Vec<Rc<MatchRule>> = right.iter().map(|s| Rc::new(make_rule(*s, parsed_rules))).collect();
            let left_rule = Rc::new(MatchRule{no: rule_number, rule_type: RuleType::And(left) });
            let right_rule = Rc::new(MatchRule{no: rule_number, rule_type: RuleType::And(right)});
            return MatchRule{ no: rule_number, rule_type: RuleType::Or{left: left_rule, right: right_rule}};
        },
    }
}

fn is_valid_part_2(input: &str, rule_42: &MatchRule, rule_31: &MatchRule) -> bool {
    let mut next_offset = 0;
    let mut count_42 = 0;
    let mut count_31 = 0;

    //For Part 2, thanks to the same input, and some reddit 
    //explanations, the easiest way to solve this is that
    //rule 0: 8 11, and 8: 42 | 42 8 and 11: 42 31 | 42 11 31.
    //So instead of matching against rule 0, we repeatedly 
    //match against rule 42 and then rule 31. If the count of 42 matches
    // >= count of rule 31 matches, then the string is valid.

    loop {
        let (success, offset) = rule_42.consume(input, next_offset);
        next_offset = offset;
        if next_offset >= input.len() {
            return false;
        }

        if !success {
            break;
        } else {
            count_42 += 1;
        }
    }

    if next_offset == input.len() {
        return false;
    }

    loop {
        let (success, offset) = rule_31.consume(input, next_offset);
        if !success {
            return false
        } else {
            next_offset = offset;
            count_31 += 1;
            if offset == input.len() {
                return count_42 > count_31;
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let data_file = File::open("data.txt")?;

    let parsed_rules = parse_rules();
    let zero_rule = make_rule_zero(&parsed_rules);

    // println!("{:?}", zero_rule);

    let rule_42 = make_rule(42, &parsed_rules); 
    let rule_31 = make_rule(31, &parsed_rules);

    let mut valid_count_1 = 0;
    let mut valid_count_2 = 0;

    let data = io::BufReader::new(data_file);
    for line in data.lines() {
        let line = line.unwrap();
        // println!("Line: {}", line);
        let (valid, end_offset) = zero_rule.consume(line.as_str(), 0);
        if valid && end_offset == line.len() {
            // println!("Valid: {}", line);
            valid_count_1 += 1;
        } else {
            // println!("Invalid: {}", line);
        }

        if is_valid_part_2(line.as_str(), &rule_42, &rule_31) {
            println!("Valid: {}", line.as_str());
            valid_count_2 += 1;
        } else {
            println!("Invalid: {}", line.as_str());
        }
    }

    println!("Part1: {}", valid_count_1);
    println!("Part2: {}", valid_count_2);

    Ok(())
}

#[test]
fn test_parser() {
    let rules = parse_rules();
    let prim = rules.get(&5).unwrap();
    match prim {
        RuleTypeParsed::Prim(c) => assert_eq!(c, &'b'),
        _ => panic!("Incorrect parse"),
    };

    let and = rules.get(&0).unwrap();
    match and {
        RuleTypeParsed::And(subrules) => assert_eq!(subrules, &vec![4, 1, 5]),
        _ => panic!("Incorrect parse"),
    }

    let or = rules.get(&1).unwrap();
    match or {
        RuleTypeParsed::Or{left, right} => {
            assert_eq!(left, &vec![2, 3]);
            assert_eq!(right, &vec![3, 2]);
        },
        _ => panic!("Incorrect parse"),
    }
}