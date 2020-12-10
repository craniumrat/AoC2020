use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    const PREAMBLE_COUNT: usize = 25;
    let mut preamble: HashSet<i64> = HashSet::new();
    let mut input: Vec<i64> = vec![];

    for line in reader.lines() {
        input.push(line.unwrap().parse::<i64>().unwrap());
    }

    for i in 0..PREAMBLE_COUNT {
        preamble.insert(*input.get(i).unwrap());
    }

    let mut part_1 = 0;
    let mut part_1_pos: usize = 0;

    for i in PREAMBLE_COUNT..input.len() {
        let to_check = input.get(i).unwrap();
        let mut found = false;
        for preamble_i in (i - PREAMBLE_COUNT)..i {
            let current_preamble_value =  input.get(preamble_i).unwrap();
            let check_in_preamble = *to_check - current_preamble_value;
            if preamble.contains(&check_in_preamble) && check_in_preamble != *current_preamble_value {
                // println!("{} + {} = {}: {}", check_in_preamble, current_preamble_value, i, to_check);
                found = true;
                break; //Found a pair. On to the next to_check
            }
        }

        if found {
            preamble.insert(*to_check);
            preamble.remove(input.get(i - PREAMBLE_COUNT).unwrap());
        }

        if !found {
            println!("Not found a pair in preamble for: {} at position: {}", to_check, i);
            part_1 = *to_check;
            part_1_pos = i;
            break;
        }
    }

    println!("Part 1: {} at positon: {}", part_1, part_1_pos);

    let mut start_index = 0;
    let mut end_index = 0;

    for start in (0..(part_1_pos)).rev() {
        let mut consecutive_sum = 0;
        let mut found = false;

        println!("Starting at {}", start);
        for i in (0..start).rev() {
            if consecutive_sum <= part_1 {
                end_index = i;
                consecutive_sum += *input.get(end_index).unwrap();
                // println!("Sum: {}", consecutive_sum);
            }

            if consecutive_sum == part_1 {
                println!("Start index: {} End index: {}", start - 1, end_index);
                start_index = start - 1;
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    let mut max = 0;
    let mut min: i64 = i64::MAX;

    for index in end_index..=start_index {
        // println!("Index: {}", index);
        if input[index] < min {
            min = input[index];
            // println!("Min: {}", min);
        }

        if input[index] > max {
            max = input[index];
            // println!("Min: {}", max);
        }
    }

    println!("Part 2: {}", min + max);
}
