extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs::File;
use std::io::{self, BufRead};
use pest::Parser;

#[derive(Parser)]
#[grammar = "direction.pest"]
pub struct StepParser;

#[derive(Debug)]
struct Step {
    step_type: char,
    step_value: i64, 
}

impl Step {
    fn parse(input: &str) -> Step {
        let step = StepParser::parse(Rule::entry, input).expect("Unsuccessful parse")
            .next().unwrap();

        let mut inner = step.into_inner();
        // println!("Rule: {:?}, Value: {}", inner.as_rule(), inner.as_str());
        let step_type = inner.next().unwrap().as_str().parse::<char>().unwrap();
        let step_value = inner.next().unwrap().as_str().parse::<i64>().unwrap();

        println!("Type: {}, Value: {}", step_type, step_value);

        Step { step_type: step_type, step_value: step_value }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction{
    N,
    E,
    S,
    W,
}

impl Direction {
    fn char_to_dir(dir: &char) -> Direction {
        match dir {
            'N' => Direction::N,
            'E' => Direction::E,
            'S' => Direction::S,
            'W' => Direction::W,
            _ => unreachable!()
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn left(&self) -> Direction {
        match self { 
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }
}

#[derive(Debug)]
struct Ship {
    pos: (i64, i64),
    facing: Direction
}

impl Ship {
    fn step(&mut self, step: &Step) {
        match step.step_type {
            'N' | 'S' | 'E' | 'W' => self.change_pos(&Direction::char_to_dir(&step.step_type), step.step_value),
            'F' => { 
                self.change_pos_facing(step.step_value); 
            },
            'L' | 'R' => self.change_dir(step.step_type, step.step_value),
            _ => unreachable!()
        }
    }

    fn change_pos(&mut self, dir: &Direction, value: i64) {
        match dir {
            Direction::N => self.pos.1 -= value,
            Direction::S => self.pos.1 += value,
            Direction::E => self.pos.0 -= value,
            Direction::W => self.pos.0 += value, 
        }
    }

    fn change_pos_facing(&mut self, value: i64) {
        let facing = self.facing.clone();
        self.change_pos(&facing, value);
    }

    fn change_dir(&mut self, turn: char, mut value: i64) {
        match turn {
            'L' => {
                while value > 0 {
                    self.facing = Direction::left(&self.facing);
                    value -= 90;
                }

            },
            'R' => {
                while value > 0 {
                    self.facing = Direction::right(&self.facing);
                    value -= 90;
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct ShipWaypoint {
    ship: Ship,
    waypoint: (i64, i64), //Relative to the ship
}

impl ShipWaypoint {
    fn step(&mut self, step: &Step) {
        match step.step_type {
            'N' | 'S' | 'E' | 'W' => self.change_pos(&Direction::char_to_dir(&step.step_type), step.step_value),
            'F' => { 
                self.move_ship_and_waypoint(step.step_value); 
            },
            'L' | 'R' => self.change_dir(step.step_type, step.step_value),
            _ => unreachable!()
        }
    }

    fn change_pos(&mut self, dir: &Direction, value: i64) {
        match dir {
            Direction::N => self.waypoint.1 -= value,
            Direction::S => self.waypoint.1 += value,
            Direction::E => self.waypoint.0 += value,
            Direction::W => self.waypoint.0 -= value, 
        }
    }

    fn move_ship_and_waypoint(&mut self, value: i64) {
        let x = self.ship.pos.0;
        let y = self.ship.pos.1;

        let way_x = self.waypoint.0;
        let way_y = self.waypoint.1;

        let rel_x = self.waypoint.0 - self.ship.pos.0;
        let rel_y = self.waypoint.1 - self.ship.pos.1;

        self.ship.pos.0 = x + value * (way_x - x);
        self.ship.pos.1 = y + value * (way_y - y);

        self.waypoint.0 = self.ship.pos.0 + rel_x;
        self.waypoint.1 = self.ship.pos.1 + rel_y;
    }

    fn rotate_waypoint_left(&mut self) {
        //Waypoint is to be rotated left relative to ship.
        let rel_x = self.waypoint.0 - self.ship.pos.0;
        let rel_y = self.waypoint.1 - self.ship.pos.1;

        //Rotate left by 90
        let new_rel_x = rel_y;
        let new_rel_y = -rel_x;

        self.waypoint.0 = self.ship.pos.0 + new_rel_x;
        self.waypoint.1 = self.ship.pos.1 + new_rel_y;
    }

    fn rotate_waypoint_right(&mut self) {
        //Waypoint is to be rotated right relative to ship.
        let rel_x = self.waypoint.0 - self.ship.pos.0;  //10
        let rel_y = self.waypoint.1 - self.ship.pos.1;  //-4

        //Rotate right by 90
        let new_rel_x = -rel_y; //4
        let new_rel_y = rel_x;  //10

        self.waypoint.0 = self.ship.pos.0 + new_rel_x;
        self.waypoint.1 = self.ship.pos.1 + new_rel_y;
    }

    fn change_dir(&mut self, turn: char, mut value: i64) {
        match turn {
            'L' => {
                while value > 0 {
                    self.rotate_waypoint_left();
                    value -= 90;
                }

            },
            'R' => {
                while value > 0 {
                    self.rotate_waypoint_right();
                    value -= 90;
                }
            },
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut ship = Ship { pos: (0, 0), facing: Direction::E };
    let mut ship_waypoint = ShipWaypoint { 
        ship: Ship { pos: (0, 0), facing: Direction::E },
        waypoint: (10, -1),
    };

    for line in reader.lines() {
        let step = Step::parse(&line.unwrap());
        ship.step(&step);
        ship_waypoint.step(&step);

        println!("{:?}", ship_waypoint);
    }

    println!("Part 1: {}", ship.pos.0.abs() + ship.pos.1.abs());
    println!("Part 2: {}", ship_waypoint.ship.pos.0.abs() + ship_waypoint.ship.pos.1.abs());
    Ok(())
}
