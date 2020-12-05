use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn pass(pass: &str) -> i32
{
    let mut output = String::new();
    for c in pass.chars() {
        if c == 'F' || c == 'L' {
            output.push('0');
        } else {
            output.push('1');
        }
    }

    i32::from_str_radix(output.as_str(), 2).unwrap()
}

fn main() {
    let file = File::open("input.txt").expect("Invalid or missing input");
    let reader = io::BufReader::new(file);

    let numbers: HashSet<i32> = reader.lines().map(|line| {
        let line = line.unwrap();
        pass(&line)
    }).collect();

    println!("Max value (part 1): {}", numbers.iter().max().unwrap());

    for i in 8..1016 {
        if !numbers.contains(&i) {
            println!("Boarding pass (Part 2): {}", i);
        }
    }
    
}
