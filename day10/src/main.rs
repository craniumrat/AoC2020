use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut input: Vec<i32> = vec![];
    for line in reader.lines() {
        input.push(line.unwrap().parse::<i32>().unwrap());
    }

    input.push(0);
    input.sort();

    let mut one_jump = 0;
    let mut three_jump = 0;
    for i in 0..(input.len() - 1) {
        match input[i + 1] - input[i] {
            1 => {
                one_jump += 1;
                // println!("From {} to {}: One jumps: {}", input[i], input[i + 1], one_jump);
            },
            3 => {
                three_jump += 1;
                // println!("From {} to {}: Three jumps: {}", input[i], input[i + 1], three_jump);
            },
            _ => continue
        }
    }

    three_jump += 1;

    //Add one 
    println!("One jump: {}, three jump: {}. Part 1: {}", one_jump, three_jump, one_jump * three_jump);
    Ok(())
}
