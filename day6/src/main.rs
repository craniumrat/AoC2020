use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;
use itertools::Itertools;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut entries : Vec<String> = Vec::new();
    
    let mut entry = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            entries.push(entry.clone());
            entry.clear();
            continue;
        }

        entry = entry + " " + &line;
    }

    if entry.len() != 0 {
        entries.push(entry);
    }

    //Part 1
    let uniques: Vec<i32> = entries.iter().map(|entry| {
        let mut hs = HashSet::new();
        for c in entry.chars() {
            if c != ' ' { hs.insert(c); }
        }
        hs.len() as i32
    }).collect();

    println!("Part 1: {}", uniques.iter().sum::<i32>());

    //Part 2
    //for each entry, split by ' '. 
    //For each substr, add to a new HashSet.
    //intersection all the sets and get length

    let lengths: Vec<i32> = entries.iter().map(|entry| {
        let snippets = entry.split_whitespace();
        let charset: Vec<HashSet<char>> = snippets.map(|snip| {
            let chars : HashSet<char> = snip.chars().collect();
            chars
        }).collect();
        let intersect = charset.into_iter().fold1(|x, y| x.intersection(&y).cloned().collect()).unwrap();
        intersect.len() as i32
    }).collect();

    println!("Part 2: {}", lengths.iter().sum::<i32>());
    Ok(())
}
