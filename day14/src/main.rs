use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use num_traits::pow;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "masks.pest"]
pub struct MaskParser;

struct Emulator {
    mask: Vec<char>,
    mem: HashMap<i64, i64>,
}

impl Emulator {
    fn set_mask(&mut self, mask: &str) {
        self.mask = mask.chars().collect();
    }

    fn get_masked_value(&self, value: i64) -> i64 {

        let mut new_value = 0;

        let mut mask_iter = self.mask.iter();

        for i in (0..36_i64).rev() {
            match mask_iter.next().unwrap() {
                '1' => new_value += pow(2_i64, i as usize),
                '0' => continue,
                'X' => new_value += value & pow(2_i64, i as usize),
                _ => unreachable!(),
            }
        }

        new_value
    }

    fn set_mem(&mut self, address: i64, value: i64) {
        let value = self.get_masked_value(value);
        self.mem.insert(address, value); 
    }

    fn get_masked_addresses(&self, address: i64) -> Vec<i64> {

        let mut mask_iter = self.mask.iter();
        let mut new_addresses: Vec<i64> = Vec::new();

        new_addresses.push(0); //We will end up with at least 1 masked address

        for i in (0..36_i64).rev() {
            // println!("{:?}", new_addresses);

            match mask_iter.next().unwrap() {
                '0' => {
                    // println!("{}: {}", (address & pow(2_i64, i as usize)), pow(2_i64, i as usize));
                    new_addresses = new_addresses.iter()
                        .map(|new_address| *new_address + (address & pow(2_i64, i as usize)))
                        .collect();
                }, 
                '1' => {
                    new_addresses = new_addresses.iter()
                        .map(|new_address| *new_address + pow(2_i64, i as usize)).collect();
                },
                'X' => {
                    let mut add_addresses: Vec<_> = new_addresses.iter()
                        .map(|new_address| *new_address + pow(2_i64, i as usize)).collect();

                    new_addresses.append(&mut add_addresses);

                },
                _ => unreachable!(),
            }
        }

        new_addresses
    }

    fn set_mem_v2(&mut self, address: i64, value: i64) {
        let addresses = self.get_masked_addresses(address);

        // println!("{:?}", addresses);

        for a in addresses {
            self.mem.insert(a, value);
        }
    }
}

fn main() -> Result<(), std::io::Error>{
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut emulator = Emulator { mask: vec!['X'; 36], mem: HashMap::new() };
    let mut emulator_v2 = Emulator { mask: vec!['X'; 36], mem: HashMap::new() };

    for line in reader.lines() {
        let l = line.unwrap();
        let entry = MaskParser::parse(Rule::line, l.as_str()).expect("Unsuccessful parse")
            .next().unwrap();

        let inner = entry.into_inner().next().unwrap(); 

        match inner.as_rule() {
            Rule::mask => {
                let inner_bitmask = inner.into_inner().next().unwrap();
                assert_eq!(inner_bitmask.as_rule(), Rule::bitmask);
                emulator.set_mask(inner_bitmask.as_str());
                emulator_v2.set_mask(inner_bitmask.as_str());
            },
            Rule::mem => {
                let mut inner_mem = inner.into_inner();
                let address = inner_mem.next().unwrap().as_str().parse::<i64>().unwrap();
                let value = inner_mem.next().unwrap().as_str().parse::<i64>().unwrap();

                emulator.set_mem(address, value);
                emulator_v2.set_mem_v2(address, value);

            },
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    for (_, value) in emulator.mem.iter() {
        sum += value;
    }

    println!("Sum (part 1): {}", sum);

    sum = 0;
    for (_, value) in emulator_v2.mem.iter() {
        sum += value;
    }

    println!("Sum (part 2): {}", sum);

    Ok(())
}

#[test]
fn test_mem() {
    let mut emulator = Emulator { mask: vec!['X'; 36], mem: HashMap::new() };
    emulator.set_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

    emulator.set_mem(8, 11);
    assert_eq!(emulator.mem.get(&8).unwrap(), &73);

    emulator.set_mem(7, 101);
    assert_eq!(emulator.mem.get(&7).unwrap(), &101);

    emulator.set_mem(8, 0);
    assert_eq!(emulator.mem.get(&8).unwrap(), &64);
}

#[test]
fn test_mem_v2() {
    let mut emulator = Emulator { mask: vec!['X'; 36], mem: HashMap::new() };
    emulator.set_mask("000000000000000000000000000000X1001X");
    emulator.set_mem_v2(42, 100);

    assert_eq!(emulator.mem.get(&26).unwrap(), &100);
    assert_eq!(emulator.mem.get(&27).unwrap(), &100);
    assert_eq!(emulator.mem.get(&58).unwrap(), &100);
    assert_eq!(emulator.mem.get(&59).unwrap(), &100);
}
