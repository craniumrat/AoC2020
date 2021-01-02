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

    println!("One jump: {}, three jump: {}. Part 1: {}", one_jump, three_jump, one_jump * three_jump);

    //Got the solution from tsoding on YouTube. Trying to implement this
    //without plagarizing.
    let mut dp : Vec<i64> = vec![];
    dp.push(1);


    //For each element, count the ways to get to the element from the prev. elements
    //conditional so that each prev. element should be <= 3 less than current element.
    for i in 1..input.len() {
        dp.push(0);
        for j in (0..i).rev() { //For each element (i-1), (i - 2).... 
            //Since this is sorted, input[j] is  guaranteed to be < input[i]
            if input[i] - input[j] > 3 {
                break;
            }

            dp[i] += dp[j];
        }
    }

    println!("Part 2: {}", dp.last().unwrap());



    Ok(())
}
