use std::fs::File;
use std::io::{self, BufRead};

fn sum_two_to_2020_and_multiply(expenses: &Vec<i32>) -> i32 {
    for (pos, i) in expenses.iter().enumerate() {
        let sl = &expenses[pos..];
        for j in sl {
            if i + j == 2020 {
                return i * j;
            }
        }  
    }
    0
}

fn sum_three_to_2020_and_multiply(expenses: &Vec<i32>) -> i32 {
    for (pos, i) in expenses.iter().enumerate() {
        let sl = &expenses[pos..];

        for (pos2, j) in sl.iter().enumerate() {
            let sl2 = &expenses[pos2..];
            for k in sl2 {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }

        }
    }
    0
}


fn main() {
    let mut expenses = Vec::new();

    let file = File::open("input.txt").expect("Missing input.txt");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = &line.unwrap();
        let expense = match line.parse::<i32>() { 
            Ok(e) => e,
            Err(_) => continue,
        };

        expenses.push(expense);
    }

    let answer = sum_two_to_2020_and_multiply(&expenses);
    println!("{}", answer);

    let answer = sum_three_to_2020_and_multiply(&expenses);
    println!("{}", answer);
}


#[test]
fn test_sample() {
    let expenses = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(sum_two_to_2020_and_multiply(&expenses), 514579);
}

#[test]
fn test_sample_part2() {
    let expenses = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(sum_three_to_2020_and_multiply(&expenses), 241861950);
}