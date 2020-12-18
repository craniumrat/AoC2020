use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NEIGHBOURS_3D: Vec<(i64, i64, i64)> = {
        let v = vec![(-1, -1, -1), (-1, -1, 0), (-1, -1, 1), 
            (-1, 0, -1), (-1, 0, 0), (-1, 0, 1), 
            (-1, 1, -1), (-1, 1, 0), (-1, 1, 1),
            (0, -1, -1), (0, -1, 0), (0, -1, 1),
            (0, 0, -1), /*(0, 0, 0),*/ (0, 0, 1), 
            (0, 1, -1), (0, 1, 0), (0, 1, 1), 
            (1, -1, -1), (1, -1, 0), (1, -1, 1),
            (1, 0, -1), (1, 0, 0), (1, 0, 1),
            (1, 1, -1), (1, 1, 0), (1, 1, 1),
            ];
        v
    };

    static ref NEIGHBOURS_4D: Vec<(i64, i64, i64, i64)> = {
        let mut v = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 { continue; }
                        v.push((x, y, z, w));
                    }
                }
            }
        }

        v
    };
}

struct Space {
    points: HashMap<i64, HashMap<i64, HashMap<i64, bool>>>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

#[derive(Debug)]
struct Point3D (i64, i64, i64);

#[derive(Debug)]
struct Point4D (i64, i64, i64, i64);

impl Space {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            min_x: i64::MAX, max_x: i64::MIN,
            min_y: i64::MAX, max_y: i64::MIN,
            min_z: i64::MAX, max_z: i64::MIN,
        }
    }

    fn set_value(&mut self, p: Point3D, value: bool) {
        if !self.points.contains_key(&p.2) {
            self.points.insert(p.2, HashMap::new());
        }

        if p.2 > self.max_z {
            self.max_z = p.2;
        }

        if p.2 < self.min_z {
            self.min_z = p.2;
        }
    
        if p.1 > self.max_y {
            self.max_y = p.1;
        }

        if p.1 < self.min_y {
            self.min_y = p.1;
        }

        if p.0 > self.max_x {
            self.max_x = p.0;
        }

        if p.0 < self.min_x {
            self.min_x = p.0;
        }

        let map = self.points.get_mut(&p.2).unwrap();
        if !map.contains_key(&p.1) {
            map.insert(p.1, HashMap::new());
        }
    
        let line = map.get_mut(&p.1).unwrap();
        if !line.contains_key(&p.0) {
            line.insert(p.0, value);
        }
    }

    fn get_value(&self, p: &Point3D) -> bool {
        if !self.points.contains_key(&p.2) {
            return false;
        }
    
        let map = self.points.get(&p.2).unwrap();
        if !map.contains_key(&p.1) {
            return false;
        }
    
        let line = map.get(&p.1).unwrap();
        if !line.contains_key(&p.0) {
            return false;
        }
    
        *line.get(&p.0).unwrap()
    }

    fn get_occupied_neighbors_count(&self, p: &Point3D) -> u32 {
        let mut occupied = 0;

        for n in NEIGHBOURS_3D.iter() {
            let n = Point3D (p.0 + n.0, p.1 + n.1, p.2 + n.2);
            if self.get_value(&n) == true {
                // println!("p: {:?}, n: {:?}", p, n);
                occupied += 1;
            }
        }

        occupied
    }

    fn update_map(&mut self) {

        let mut space = Space::new();

        for z in self.min_z - 1..=self.max_z + 1 {
            for y in self.min_y - 1..=self.max_y + 1 {
                for x in self.min_x - 1..=self.max_x + 1 {
                    let p = Point3D(x, y, z);
                    let neighbor_occupied_count = self.get_occupied_neighbors_count(&p);
                    let value = self.get_value(&p);

                    // println!("P: {:?}, neighbours: {}", p, neighbor_occupied_count);

                    if neighbor_occupied_count == 3 && value == false {
                        space.set_value(p, true);
                        continue;
                    }

                    if !(neighbor_occupied_count == 2 || neighbor_occupied_count == 3) 
                    && value == true {
                        space.set_value(p, false);
                        continue;
                    }
                    else if value == true {
                        space.set_value(p, true);
                    }
                }
            }
        }
 
        *self = space;
    }

    fn print_map(&self, z: i64) { 
        if !self.points.contains_key(&z) {
            println!();
        }
    
        let map = self.points.get(&z).unwrap();
        // println!("Map: {:?}", map);

        for y in self.min_y..=self.max_y {
            println!();
            if !map.contains_key(&y) {
                println!();
                continue;
            }

            let line = map.get(&y).unwrap();
            // println!("Line: {:?}", line);
            for x in self.min_x..=self.max_x {
                if !line.contains_key(&x) {
                    print!(".");
                    continue;
                }

                if line.get(&x).unwrap() == &true {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
        }
    }

    fn print_space(&self) {
        for z in self.min_z..=self.max_z {
            println!("Z: {}", z);
            self.print_map(z);
            println!();
        }
    }

    fn get_actives(&self) -> u32 {
        let mut actives = 0;

        for z in self.min_z..=self.max_z {
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    if self.get_value(&Point3D(x, y, z)) == true {
                        actives += 1;
                    }
                }
            }
        }

        actives
    }
}


struct Hyperspace {
    points: HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
    min_w: i64,
    max_w: i64,
}

impl Hyperspace {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
            min_x: i64::MAX, max_x: i64::MIN,
            min_y: i64::MAX, max_y: i64::MIN,
            min_z: i64::MAX, max_z: i64::MIN,
            min_w: i64::MAX, max_w: i64::MIN,
        }
    }

    fn set_value(&mut self, p: Point4D, value: bool) {
        if p.3 > self.max_w {
            self.max_w = p.3;
        }

        if p.3 < self.min_w {
            self.min_w = p.3;
        }

        if p.2 > self.max_z {
            self.max_z = p.2;
        }

        if p.2 < self.min_z {
            self.min_z = p.2;
        }
    
        if p.1 > self.max_y {
            self.max_y = p.1;
        }

        if p.1 < self.min_y {
            self.min_y = p.1;
        }

        if p.0 > self.max_x {
            self.max_x = p.0;
        }

        if p.0 < self.min_x {
            self.min_x = p.0;
        }

        if !self.points.contains_key(&p.3) {
            self.points.insert(p.3, HashMap::new());
        }

        let space = self.points.get_mut(&p.3).unwrap();
        if !space.contains_key(&p.2) {
            space.insert(p.2, HashMap::new());
        }

        let map = space.get_mut(&p.2).unwrap();
        if !map.contains_key(&p.1) {
            map.insert(p.1, HashMap::new());
        }
    
        let line = map.get_mut(&p.1).unwrap();
        if !line.contains_key(&p.0) {
            line.insert(p.0, value);
        }
    }

    fn get_value(&self, p: &Point4D) -> bool {
        if !self.points.contains_key(&p.3) {
            return false;
        }

        let space = self.points.get(&p.3).unwrap();
        if !space.contains_key(&p.2) {
            return false;
        }
    
        let map = space.get(&p.2).unwrap();
        if !map.contains_key(&p.1) {
            return false;
        }
    
        let line = map.get(&p.1).unwrap();
        if !line.contains_key(&p.0) {
            return false;
        }
    
        *line.get(&p.0).unwrap()
    }

    fn get_occupied_neighbors_count(&self, p: &Point4D) -> u32 {
        let mut occupied = 0;

        for n in NEIGHBOURS_4D.iter() {
            let n = Point4D (p.0 + n.0, p.1 + n.1, p.2 + n.2, p.3 + n.3);
            if self.get_value(&n) == true {
                // println!("p: {:?}, n: {:?}", p, n);
                occupied += 1;
            }
        }

        occupied
    }

    fn update_map(&mut self) {

        let mut hyperspace = Hyperspace::new();
        
        for w in self.min_w - 1..=self.max_w + 1 {

            for z in self.min_z - 1..=self.max_z + 1 {
                for y in self.min_y - 1..=self.max_y + 1 {
                    for x in self.min_x - 1..=self.max_x + 1 {
                        let p = Point4D(x, y, z, w);
                        let neighbor_occupied_count = self.get_occupied_neighbors_count(&p);
                        let value = self.get_value(&p);

                        // println!("P: {:?}, neighbours: {}", p, neighbor_occupied_count);

                        if neighbor_occupied_count == 3 && value == false {
                            hyperspace.set_value(p, true);
                            continue;
                        }

                        if !(neighbor_occupied_count == 2 || neighbor_occupied_count == 3) 
                        && value == true {
                            hyperspace.set_value(p, false);
                            continue;
                        }
                        else if value == true {
                            hyperspace.set_value(p, true);
                        }
                    }
                }
            }
        }
    
        *self = hyperspace;
    }

    fn print_map(&self, z: i64, w: i64) { 
        if !self.points.contains_key(&w) {
            println!();
            return;
        }

        let space = self.points.get(&w).unwrap();
        if !space.contains_key(&z) {
            println!();
            return;
        }
    
        let map = space.get(&z).unwrap();
        // println!("Map: {:?}", map);

        for y in self.min_y..=self.max_y {
            println!();
            if !map.contains_key(&y) {
                println!();
                continue;
            }

            let line = map.get(&y).unwrap();
            // println!("Line: {:?}", line);
            for x in self.min_x..=self.max_x {
                if !line.contains_key(&x) {
                    print!(".");
                    continue;
                }

                if line.get(&x).unwrap() == &true {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
        }
    }

    fn print_space(&self) {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                println!("Z: {}, W: {}", z, w);
                self.print_map(z, w);
                println!();
            }
        }
    }

    fn get_actives(&self) -> u32 {
        let mut actives = 0;

        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        if self.get_value(&Point4D(x, y, z, w)) == true {
                            actives += 1;
                        }
                    }
                }
            }
        }

        actives
    }
}

fn main() -> Result<(), std::io::Error> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut space = Space::new();
    let mut hyperspace = Hyperspace::new();

    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                // println!("Inserting: x: {}, y:{}, z:{} = {}", x, y, 0, c);
                space.set_value(Point3D(x as i64, y as i64, 0), true);
                hyperspace.set_value(Point4D(x as i64, y as i64, 0, 0), true);
            } 
            else {
                // println!("Inserting: x: {}, y:{}, z:{} = {}", x, y, 0, c);
                space.set_value(Point3D(x as i64, y as i64, 0), false);
                hyperspace.set_value(Point4D(x as i64, y as i64, 0, 0), false);
            }
        }
    }

    for _ in 0..6 {
        space.update_map();
    }
    
    println!("Part 1: {}", space.get_actives());

    for _ in 0..6 {
        hyperspace.update_map();
    }
    
    println!("Part 2: {}", hyperspace.get_actives());


    Ok(())
}
