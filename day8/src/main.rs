use pest::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{ HashSet, HashMap };

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "computer.pest"]
pub struct ComputerParser;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum OpCode {
    NoOp { value: i32},
    Jump { offset: i32},
    Acc { value: i32}
}

impl OpCode {
    fn parse(input: &str) -> OpCode {
        let mut instruction = "";
        let mut number = 0;
    
        let opcode = ComputerParser::parse(Rule::expr, input).expect("Unable to parse").next().unwrap();
    
        for inner in opcode.into_inner() {
            match inner.as_rule() {
                Rule::instruction => {
                    instruction = inner.as_str();
                },
                Rule::number => {
                    number = inner.as_str().parse::<i32>().expect("Unable to parse");
                },
                _ => unreachable!(),
    
            }
        }
    
        let output = match instruction {
            "nop" => OpCode::NoOp { value: number },
            "acc" => OpCode::Acc { value: number},
            "jmp" => OpCode::Jump { offset: number},
            _ => unreachable!(),
        };

        output
    }

    fn execute(&self, execution_state: &mut ExecutionState) {
        match self {
            OpCode::NoOp { value } => {
                execution_state.instruction_pointer += 1;
            },
            OpCode::Acc { value } => {
                execution_state.instruction_pointer += 1;
                execution_state.accumulator += value;
            },
            OpCode::Jump { offset } => {
                execution_state.instruction_pointer += offset;
            },
        }
    }
}

struct ExecutionState {
    accumulator: i32,
    instruction_pointer: i32,
}

fn execute_instructions_part_1(instructions: &HashMap<i32, OpCode>) -> i32 {

    let mut visited: HashSet<i32> = HashSet::new();
    let mut execution_state = ExecutionState { accumulator: 0, instruction_pointer: 0 };

    while !visited.contains(&execution_state.instruction_pointer) {
        visited.insert(execution_state.instruction_pointer);
        let opcode = instructions.get(&execution_state.instruction_pointer).unwrap();
        opcode.execute(&mut execution_state);
    }

    execution_state.accumulator
}

fn try_execute_instructions_till_end(instructions: &HashMap<i32, OpCode>) -> Option<i32> {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut execution_state = ExecutionState { accumulator: 0, instruction_pointer: 0 };

    while execution_state.instruction_pointer != (instructions.len() as i32) {
        if visited.contains(&execution_state.instruction_pointer) {
            return None;
        }
        visited.insert(execution_state.instruction_pointer);
        let opcode = instructions.get(&execution_state.instruction_pointer).unwrap();
        opcode.execute(&mut execution_state);
    }

    Some(execution_state.accumulator)
}

fn change_instruction_and_try_execute(instructions: &HashMap<i32, OpCode>) -> Option<i32> {
    //For i = 0 len(hashmap)
    //  if instruction == jmp, change to noop, else if instruction == nop, change to jmp, and execute it
    //if return is None, next
    //if Some, then done.

    let mut changed_instructions;
    
    for i in 0..(instructions.len() as i32) {
        match instructions.get(&i).unwrap() {
            OpCode::Jump { offset} => {
                changed_instructions = instructions.clone();
                changed_instructions.insert(i, OpCode::NoOp { value: *offset});
            },
            OpCode::NoOp { value} => {
                changed_instructions = instructions.clone();
                changed_instructions.insert(i, OpCode::Jump { offset: *value});
            }
            ,
            _ => continue
        }

        match try_execute_instructions_till_end(&changed_instructions) {
            None => continue,
            Some(value) => return Some(value)
        }
    }

    None
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut instructions: HashMap<i32, OpCode> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        instructions.insert(index as i32, OpCode::parse(line.unwrap().as_str()));
    }

    let acc = execute_instructions_part_1(&instructions);

    println!("Part 1: {}", acc);

    let acc = change_instruction_and_try_execute(&instructions);
    println!("Part 2: {:?}", acc);

    Ok(())
}

#[test]
fn test_parse_noop() {
    let input = "nop +0";
    let expected = OpCode::NoOp;

    let output = OpCode::parse(input);
    assert_eq!(output, expected);
}

#[test]
fn test_parse_acc() {
    let input = "acc -12";
    let expected = OpCode::Acc {value: -12 };

    let output = OpCode::parse(input);
    assert_eq!(output, expected);
}

#[test]
fn test_parse_jmp() {
    let input = "jmp +99";
    let expected = OpCode::Jump {offset: 99 };

    let output = OpCode::parse(input);
    assert_eq!(output, expected);
}