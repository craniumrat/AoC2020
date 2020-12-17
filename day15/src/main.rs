use std::collections::HashMap;

fn get_next(ages: &mut HashMap<u32, (Option<u32>, Option<u32>)>, top: u32) -> u32 {

    for (_, value) in ages.iter_mut() {

        let (prev, prev_prev) = *value;
        match (prev, prev_prev) {
            (Some(p), None) => *value = (Some(p + 1), None),
            (Some(p), Some(pp)) => *value = (Some(p + 1), Some(pp + 1)),
            _ => unreachable!()
        }
    }
    
    if !ages.contains_key(&top) {
        ages.insert(top, (Some(0), None));
    }
    else {
        let value = ages.get_mut(&top).unwrap();
        let (prev, _) = *value;
        match prev {
            Some(p) => *value = (Some(0), Some(p)),
            _ => unreachable!()
        }
    }

    // println!("{:?}", ages);

    //Calculate the next value to return
    let next: u32;

    let value = ages.get(&top).unwrap();
    let (prev, prev_prev) = *value;
    match (prev, prev_prev) {
        (Some(p), None) => next = p, 
        (Some(p), Some(pp)) => {
            next = pp - p;
            println!("K:{}, P: {}, PP: {}", top, p, pp);
        },
        _ => unreachable!()
    }

    next
}

fn main() -> Result<(), std::io::Error> {
    let mut ages: HashMap<u32, (Option<u32>, Option<u32>)> = HashMap::new();

    ages.insert(8, (Some(0), None));
    ages.insert(0, (Some(1), None));
    ages.insert(1, (Some(2), None));
    ages.insert(3, (Some(3), None));
    ages.insert(9, (Some(4), None));

    let mut counter = 6 ;
    let mut top: u32 = 4;

    // ages.insert(3, (Some(0), None));
    // ages.insert(0, (Some(1), None));

    // let mut counter = 3 ;
    // let mut top: u32 = 6;

    loop {
        top = get_next(&mut ages, top);
        counter += 1;
        println!("Counter: {}. Top: {}", counter, top);

        if counter == 2021 {
            break;
        }
    }

    println!("Part 1: {}", top);

    Ok(())
}
