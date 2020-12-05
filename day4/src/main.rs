use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl Passport {
    pub fn blank() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn new(entries: &String) -> Passport {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\S+):(\\S+)\\s*").unwrap();
        }

        let mut passport = Passport::blank();

        for cap in RE.captures_iter(&entries) {
            let key = &cap[1];
            let value = &cap[2];

            // println!("Key: [{}], Value: [{}]", key, value);

            match key {
                "byr" => {
                    passport.byr = Some(String::from(value));
                }
                "iyr" => {
                    passport.iyr = Some(String::from(value));
                },
                "eyr" => {
                    passport.eyr = Some(String::from(value));
                },
                "hgt" => {
                    passport.hgt = Some(String::from(value));
                },
                "hcl" => {
                    passport.hcl = Some(String::from(value));
                },
                "ecl" => {
                    passport.ecl = Some(String::from(value));
                },
                "pid" => {
                    passport.pid = Some(String::from(value));
                },
                "cid" => {
                    passport.cid = Some(String::from(value));
                },
                _ => {}
            }
        }

        passport
    }

    pub fn is_valid_part_1(&self) -> bool {

        //Every entry expect cid must be present
        self.byr != None && self.iyr != None && self.eyr != None 
            && self.hgt != None && self.hcl != None && self.ecl != None
            && self.pid != None
    }

    pub fn is_valid_part_2(&self) -> bool {

        if !self.is_valid_part_1() {
            println!("Failed part 1 here");
            return false;
        }

        println!("Passed part 1");

        //Birth Year
        {
            let byr = match self.byr.as_ref().unwrap().parse::<i32>() {
                Ok(b) => b,
                Err(_) => {
                    return false;
                }
            };

            if byr < 1920 || byr > 2002 {
                return false;
            }
        }

        println!("Passed birth year: {}", self.byr.as_ref().unwrap());

        //Issued Year
        {
            let iyr = match self.iyr.as_ref().unwrap().parse::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    return false;
                }
            };

            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        }

        println!("Passed issued year: {}", self.iyr.as_ref().unwrap());

        //Expiry Year
        {
            let eyr = match self.eyr.as_ref().unwrap().parse::<i32>() {
                Ok(i) => i,
                Err(_) => {
                    return false;
                }
            };

            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        }

        println!("Passed expiry year: {}", self.eyr.as_ref().unwrap());

        //Height
        {
            lazy_static! {
                static ref RE: Regex = Regex::new("^(\\d+)(cm|in)$").unwrap();
            }

            let cap = match RE.captures(&self.hgt.as_ref().unwrap()) {
                Some(cap) => cap,
                None => {
                    return false;
                }
            };

            let hgt = match cap[1].parse::<i32>() {
                Ok(h) => h,
                Err(_) => {
                    return false;
                }
            };

            let meas = &cap[2];

            if (meas == "cm" && (hgt < 150 || hgt > 193)) ||
               (meas == "in" && (hgt < 59 || hgt > 76)) {
                   return false;
            }
        }

        println!("Passed height: {}", self.hgt.as_ref().unwrap());

        //Hair Color
        {
            lazy_static! {
                static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
            }

            if !RE.is_match(&self.hcl.as_ref().unwrap()) {
                return false;
            }
        }

        println!("Passed hair color: {}", self.hcl.as_ref().unwrap());

        //Eye Color
        {
            lazy_static! {
                static ref EYE_COLORS : HashSet<&'static str> = 
                    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
            }

            if !EYE_COLORS.contains(self.ecl.as_ref().unwrap().as_str()) {
                return false;
            }
        }

        println!("Passed eye color: {}", self.ecl.as_ref().unwrap());

        {
            lazy_static! {
                static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
            }

            // println!("{}", self.pid.as_ref().unwrap());

            if !RE.is_match(&self.pid.as_ref().unwrap()) {
                return false;
            }
        }

        println!("Passed PID: {}", self.pid.as_ref().unwrap());

        true
    }
}

fn main() {
    let file = File::open("input.txt").expect("Invalid or missing file");
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

    let passport_valid: Vec<bool> = entries.iter().map(|entry| {
        let passport = Passport::new(entry);
        println!("{}", entry);
        println!("{:?}", passport);
        passport.is_valid_part_2()
    }).filter(|valid| *valid).collect();

    println!("Valid passport count: {}", passport_valid.len());
}

#[test]
fn test_passport_fields() {
    let entry = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm");
    let passport = Passport::new(&entry);

    assert_eq!(passport.ecl, Some(String::from("gry")));
    assert_eq!(passport.pid, Some(String::from("860033327")));
    assert_eq!(passport.eyr, Some(String::from("2020")));
    assert_eq!(passport.hcl, Some(String::from("#fffffd")));
    assert_eq!(passport.byr, Some(String::from("1937")));
    assert_eq!(passport.iyr, Some(String::from("2017")));
    assert_eq!(passport.cid, Some(String::from("147")));
    assert_eq!(passport.hgt, Some(String::from("183cm")));
}

#[test]
fn test_valid_passport() {
    let entries = vec![String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"),
        String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"),
        String::from("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"),
        String::from("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"),
    ];

    let expected = vec![true, false, true, false];

    let output: Vec<bool> = entries.iter().map(|entry| {
        let passport = Passport::new(entry);
        passport.is_valid_part_1()
    }).collect();

    assert_eq!(expected, output);
}

#[test]
fn test_invalid_part_2() {
    let entries = vec![String::from("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
        String::from("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"),
        String::from("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
        String::from("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007")];

        let expected = vec![false, false, false, false];

        let output: Vec<bool> = entries.iter().map(|entry| {
            let passport = Passport::new(entry);
            passport.is_valid_part_2()
        }).collect();
    
        assert_eq!(expected, output);
}

#[test]
fn test_valid_part_2() {
    let entries = vec![String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"),
        String::from("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
        String::from("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"),
        String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719")];

        let expected = vec![true, true, true, true];

        let output: Vec<bool> = entries.iter().map(|entry| {
            let passport = Passport::new(entry);
            passport.is_valid_part_2()
        }).collect();
    
        assert_eq!(expected, output);
}