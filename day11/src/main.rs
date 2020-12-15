use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DELTAS: Vec<(isize, isize)> = {
        let v = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        v
    };
}

fn get_occupied_neighbor_count (seats: &HashMap<(isize, isize), char>, x: isize, y: isize, max_x: isize, max_y: isize) -> i32 {

    //All neighbours are: 
    //(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), 
    // (x - 1, y), (x + 1, y), 
    // (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
    let mut occupied = 0;

    for delta in DELTAS.iter() {
        if x == 0 && delta.0 == -1 {
            continue;
        }

        if y == 0 && delta.1 == -1 {
            continue;
        }

        if x == max_x - 1 && delta.0 == 1 {
            continue;
        }

        if y == max_y - 1 && delta.1 == 1 {
            continue;
        }

        let x = x + delta.0;
        let y = y + delta.1;

        if seats.get(&(x, y)).unwrap() == &'#' {
            occupied += 1;
        }
    }

    occupied
}

fn get_occupied_line_of_sight_count(seats: &HashMap<(isize, isize), char>, x: isize, y: isize, max_x: isize, max_y: isize) -> u32 {
    //From x, y, go in all 8 directions until we hit an edge or a seat (L, or #).
    //If the seat is occupied, add one.
    //If the seat is empty, continue
    let mut occupied = 0;

    for delta in DELTAS.iter() {
        for step in 1.. {
            let new_x = x + step * delta.0;
            let new_y = y + step * delta.1;

            if new_x < 0 || new_x == max_x || new_y < 0 || new_y == max_y {
                break;
            }

            let value = seats.get(&(new_x, new_y)).unwrap();
            if *value == '#' {
                occupied += 1;
                break;
            } else if *value == 'L' {
                break;
            }
        }
    }

    occupied
}

fn update_seats(seats: &mut HashMap<(isize, isize), char>, max_x: isize, max_y: isize) -> u32 {
    //return false if any changes are made
    let mut updated_count: u32 = 0;

    let mut updated_seats: HashMap<(isize, isize), char> = HashMap::new();

    for x in 0..max_x {
        for y in 0..max_y {
            // println!("Update: {}: {}", x, y);

            let value = seats.get(&(x, y)).unwrap();
            if let Some(position) = seats.get(&(x, y)) {
                if position == &'.' {
                    updated_seats.insert((x, y), '.');
                    continue;
                }
                else if get_occupied_neighbor_count(seats, x, y, max_x, max_y) >= 4 && *value == '#' {
                        updated_seats.insert((x, y), 'L');
                        updated_count += 1;
                }
                else if get_occupied_neighbor_count(seats, x, y, max_x, max_y) == 0 && *value == 'L' {
                        updated_seats.insert((x, y), '#');
                        updated_count += 1;
                }
                else {
                    updated_seats.insert((x, y), *value);
                }
            } 

            // println!("Updated: {}: {} - {}", x, y, updated_seats.get(&(x, y)).unwrap());
        }
    }

    // println!("> Updated Seats");
    print_seats(&updated_seats, max_x, max_y);
    // println!("< Updated Seats");

    seats.clear();
    for (k, v) in updated_seats.into_iter() {
        seats.insert(k, v);
    }

    updated_count
}

fn update_seats_part_2(seats: &mut HashMap<(isize, isize), char>, max_x: isize, max_y: isize) -> u32 {
    //return false if any changes are made
    let mut updated_count: u32 = 0;

    let mut updated_seats: HashMap<(isize, isize), char> = HashMap::new();

    for x in 0..max_x {
        for y in 0..max_y {
            // println!("Update: {}: {}", x, y);

            let value = seats.get(&(x, y)).unwrap();
            if let Some(position) = seats.get(&(x, y)) {
                if position == &'.' {
                    updated_seats.insert((x, y), '.');
                    continue;
                }
                else if get_occupied_line_of_sight_count(seats, x, y, max_x, max_y) >= 5 && *value == '#' {
                        updated_seats.insert((x, y), 'L');
                        updated_count += 1;
                }
                else if get_occupied_line_of_sight_count(seats, x, y, max_x, max_y) == 0 && *value == 'L' {
                        updated_seats.insert((x, y), '#');
                        updated_count += 1;
                }
                else {
                    updated_seats.insert((x, y), *value);
                }
            } 

            // println!("Updated: {}: {} - {}", x, y, updated_seats.get(&(x, y)).unwrap());
        }
    }

    // println!("> Updated Seats");
    print_seats(&updated_seats, max_x, max_y);
    // println!("< Updated Seats");

    seats.clear();
    for (k, v) in updated_seats.into_iter() {
        seats.insert(k, v);
    }

    updated_count
}

fn print_seats(seats: &HashMap<(isize, isize), char>, max_x: isize, max_y: isize) {
    for y in 0..max_y {
        for x in 0..max_x {
            // println!("{}:{}", x, y);
            print!("{}", seats.get(&(x, y)).unwrap());
        }
        println!();
    }
}

fn get_occupied_count(seats: &HashMap<(isize, isize), char>) -> i32 {
    let mut occupied_count = 0;
    for (_, val) in seats.iter() {
        if *val == '#' {
            occupied_count += 1;

        }
    }

    occupied_count
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut seats: HashMap<(isize, isize), char> = HashMap::new();
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            seats.insert((x as isize, y as isize), c);
            max_x = x as isize;
        }
        max_y = y as isize;
    }

    max_x += 1;
    max_y += 1;

    print_seats(&seats, max_x, max_y);

    // loop {
    //     if update_seats(&mut seats, max_x, max_y) == 0 {
    //         break;
    //     }
    // }

    let occupied = get_occupied_count(&seats);
    println!("Part 1: {}", occupied);

    loop {
        if update_seats_part_2(&mut seats, max_x, max_y) == 0 {
            break;
        }
    }

    let occupied = get_occupied_count(&seats);
    println!("Part 2: {}", occupied);

    Ok(())
}
