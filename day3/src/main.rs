use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point{
    x: i32,
    y: i32
}

struct Forest {
    map: HashMap<Point, char>,
    x_max: i32,
    y_max: i32
}

impl Forest {
    fn get_char_with_x_y(&self, x: i32, y: i32) -> &char {
        if y > self.y_max {
            panic!("Invalid y. Should be less or equal to {}.", self.y_max);
        }

        let x = x % self.x_max;
        return self.map.get(&Point {x: x, y: y}).expect(format!("Invalid map. Missing entry at ({},{})", x, y).as_str());
    }

    fn get_char_with_pos(&self, point: &Point) -> &char {
        let y = point.y;
        if y > self.y_max {
            panic!("Invalid y. Should be less or equal to {}.", self.y_max);
        }

        let x = point.x % self.x_max;
        return self.map.get(&Point {x: x, y: y} ).expect(format!("Invalid map. Missing entry at ({},{})", x, y).as_str());

    }

    pub fn new(map: HashMap<Point, char>, x_max: i32, y_max: i32) -> Forest {
        Forest { map, x_max, y_max }
    }

    pub fn parse(lines: &Vec<String>) -> Forest {
        let mut x_max = 0;
        let mut y_max = 0;

        let mut map: HashMap<Point, char> = HashMap::new();
    
        for (line_no, line) in lines.iter().enumerate() {
            y_max = line_no;
            for (col_no, c) in line.chars().enumerate() {
                x_max = col_no;
                let p = Point {x: col_no as i32, y: line_no as i32};
                map.insert(p, c);
           }
        }
    
        x_max += 1;
        y_max  += 1;

        Forest {map, x_max: x_max as i32, y_max: y_max as i32}
    }

    fn traverse_forest(&self, delta_x: i32, delta_y: i32) -> String {
        let mut path = String::with_capacity(self.y_max as usize);

        let mut x = 0; 
        let mut y = 0;

        while y < self.y_max {
            path.push(*self.get_char_with_x_y(x, y));
            x += delta_x;
            y += delta_y;
        }

        path
    }
}

fn tree_count(path: String) -> i32 {
    let trees : Vec<char> = path.chars().filter(|c| c == &'#').collect();
    trees.len() as i32
}

fn main() {

    let file = File::open("input.txt").expect("Missing input.txt");
    let reader = io::BufReader::new(file);

    let v : Vec<String> = reader.lines().map(|line|line.unwrap()).collect();
    let forest = Forest::parse(&v);

    let part1 = tree_count(forest.traverse_forest(3, 1));
    println!("Tree count: {}", part1);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let counts: Vec<i32> = slopes.iter().map(|slope| {
        let delta_x = slope.0;
        let delta_y = slope.1;
        let count = tree_count(forest.traverse_forest(delta_x, delta_y));
        count
    }).collect();

    println!("{:?}", counts);

}

#[test]
fn test_forest_max() {
    let lines = vec![String::from("..##......."), String::from("#...#...#..")];
    let forest = Forest::parse(&lines);
    assert_eq!(forest.x_max, 11);
    assert_eq!(forest.y_max, 2);
}

#[test]
fn test_forest_for_chars() {
    let lines = vec![String::from("..##......."), String::from("#...#...#..")];
    let forest = Forest::parse(&lines);
    assert_eq!(forest.map.get(&Point {x: 0, y: 0}).unwrap(), &'.');
    assert_eq!(forest.map.get(&Point {x: 3, y: 0}).unwrap(), &'#');
}

#[test]
fn test_forest_past_right() {
    let lines = vec![String::from("..##......."), String::from("#...#...#..")];
    let forest = Forest::parse(&lines);
    assert_eq!(forest.get_char_with_pos(&Point {x: 14, y: 0}), &'#');
    assert_eq!(forest.get_char_with_x_y(14, 1), &'.');
}

#[test]
fn test_traverse_path() {
    let lines = vec![String::from("..##......."),
        String::from("#...#...#.."),
        String::from(".#....#..#."),
        String::from("..#.#...#.#"),
        String::from(".#...##..#."),
        String::from("..#.##....."),
        String::from(".#.#.#....#"),
        String::from(".#........#"),
        String::from("#.##...#..."),
        String::from("#...##....#"),
        String::from(".#..#...#.#")];

    let forest = Forest::parse(&lines);
    let path = forest.traverse_forest(3, 1);

    assert_eq!(path, String::from("..#.##.####"));
    assert_eq!(tree_count(path), 7);
}