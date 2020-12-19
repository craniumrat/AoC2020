extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "expression.pest"]
struct ExpressionParser;

use pest::prec_climber::{Assoc, PrecClimber, Operator};

lazy_static! {
    static ref PREC_CLIMBER_PART1: PrecClimber<Rule> = {

        //add and multiply have the same precedence order, so '|' them
        PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::multiply, Assoc::Left)
        ])
    };

    static ref PREC_CLIMBER_PART2: PrecClimber<Rule> = {

        //add has a higher precedence order so it has to be *after* multiply
        PrecClimber::new(vec![
            Operator::new(Rule::multiply, Assoc::Left), 
            Operator::new(Rule::add, Assoc::Left),
        ])
    };
}

//Add and multiply have the same order precedence levels
fn eval_part1(expression: Pairs<Rule>) -> i64 {
        PREC_CLIMBER_PART1.climb(expression,
            |pair: Pair<Rule> | match pair.as_rule() {
                Rule::num => pair.as_str().parse::<i64>().unwrap(),
                Rule::expr => eval_part1(pair.into_inner()),
                _ => unreachable!(),
            },
        |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::multiply => lhs * rhs,
            _ => unreachable!(),
        },
    )
}

fn eval_part2(expression: Pairs<Rule>) -> i64 {
    PREC_CLIMBER_PART2.climb(expression,
        |pair: Pair<Rule> | match pair.as_rule() {
            Rule::num => pair.as_str().parse::<i64>().unwrap(),
            Rule::expr => eval_part2(pair.into_inner()),
            _ => unreachable!(),
        },
    |lhs: i64, op: Pair<Rule>, rhs: i64| match op.as_rule() {
        Rule::add => lhs + rhs,
        Rule::multiply => lhs * rhs,
        _ => unreachable!(),
    },
)
}


fn main() -> Result<(), std::io::Error> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut sum_part1: i64 = 0;
    let mut sum_part2: i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let calculate = line.as_str();

        let calculation = ExpressionParser::parse(Rule::calculation, calculate)
        .expect("Invalid string");

        let e = eval_part1(calculation);
        println!("1: {}", e);
        sum_part1 += e;

        let calculation = ExpressionParser::parse(Rule::calculation, calculate)
        .expect("Invalid string");
        let e = eval_part2(calculation);
        println!("2: {}", e);
        sum_part2 += e;
    }

    println!("Part1: {}", sum_part1);
    println!("Part2: {}", sum_part2);

    Ok(())
}