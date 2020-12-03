use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn is_password_valid(input: &str) -> bool {
    //1-3 a: abcde
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(.*)").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    println!("[{}]-[{}] [{}]: [{}]", &cap[1], &cap[2], &cap[3], &cap[4]);

    if !RE.is_match(input) {
        return false
    }

    let min = &cap[1];
    let mut min = min.parse::<i32>().expect("invalid min");
    let max = &cap[2];
    let mut max = max.parse::<i32>().expect("invalid max");
    let character = &cap[3];
    let character = character.parse::<char>().expect("invalid character");
    let password = &cap[4];

    let mut characters: HashMap<char, i32> = HashMap::new();
    for c in password.chars() {
        characters.entry(c)
            .and_modify(|count| *count += 1).or_insert(1);
    }

    let check_count = characters.entry(character).or_default();

    if check_count >= &mut min && check_count <= &mut max {
        return true;
    }

    false
}

fn is_password_valid_2(input: &str) -> bool {
    //1-3 a: abcde
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(.*)").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    println!("[{}]-[{}] [{}]: [{}]", &cap[1], &cap[2], &cap[3], &cap[4]);

    if !RE.is_match(input) {
        return false
    }

    let min = &cap[1];
    let min = min.parse::<usize>().expect("invalid min");
    let max = &cap[2];
    let max = max.parse::<usize>().expect("invalid max");
    let character = &cap[3];
    let character = character.parse::<char>().expect("invalid character");
    let password = &cap[4];

    let pos_1 = password.chars().nth(min - 1) == Some(character);
    let pos_2 = password.chars().nth(max - 1) == Some(character);

    println!("Pos 1: {}, Pos 2: {}", pos_1, pos_2);

    pos_1 ^ pos_2
}

fn main() {
    let file = File::open("input.txt").expect("Invalid input file");
    let reader = io::BufReader::new(file);

    // let output: Vec<bool> = reader.lines().map(|line| is_password_valid(line.unwrap().as_str()))
    //     .filter(|valid| *valid).collect();

    // println!("Part1: {}", output.len());

    let output: Vec<bool> = reader.lines().map(|line| is_password_valid_2(line.unwrap().as_str()))
        .filter(|valid| *valid).collect();

    println!("Part1: {}", output.len());
}

#[test]
fn test_valid_1() {
    let inputs = vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc");
    let outputs: Vec<bool> = inputs.iter().map(|&input| is_password_valid(input)).collect();
    let expected = vec!(true, false, true);

    assert_eq!(outputs, expected);
}

#[test]
fn test_valid_2() {
    let inputs = vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc");
    let outputs: Vec<bool> = inputs.iter().map(|&input| is_password_valid_2(input)).collect();
    let expected = vec!(true, false, false);

    assert_eq!(outputs, expected);
}
