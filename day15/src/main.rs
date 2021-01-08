use std::collections::HashMap;

fn get_next_unoptimized(ages: &mut HashMap<u64, (Option<u64>, Option<u64>)>, top: u64) -> u64 {

    //I am upating the hashmap for every turn. It's probably
    //not a good idea to update them all for every input.
    
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
    let next: u64;

    let value = ages.get(&top).unwrap();
    let (prev, prev_prev) = *value;
    match (prev, prev_prev) {
        (Some(p), None) => next = p, 
        (Some(p), Some(pp)) => {
            next = pp - p;
            // println!("K:{}, P: {}, PP: {}", top, p, pp);
        },
        _ => unreachable!()
    }

    next
}

fn get_next(ages: &mut HashMap<u64, u64>, tick: u64, top: u64) -> u64 {

    let next;
    if !ages.contains_key(&top) {
        ages.insert(top, tick);
        next = 0;
    } else {
        next = tick - *ages.get(&top).unwrap();
        ages.insert(top, tick);
    }

    next
}

fn main() -> Result<(), std::io::Error> {

    //--- START -- For unoptimized
    //let mut ages: HashMap<u64, (Option<u64>, Option<u64>)> = HashMap::new();

    // ages.insert(8, (Some(0), None));
    // ages.insert(0, (Some(1), None));
    // ages.insert(1, (Some(2), None));
    // ages.insert(3, (Some(3), None));
    // ages.insert(9, (Some(4), None));

    // let mut counter = 6 ;
    // let mut top: u64 = 4;
    //---- END

    let mut ages: HashMap<u64, u64> = HashMap::new();

    ages.insert(8, 4);
    ages.insert(0, 3);
    ages.insert(1, 2);
    ages.insert(3, 1);
    ages.insert(9, 0);

    let mut counter = 5;
    let mut top: u64 = 4;

    // loop {
    //     top = get_next(&mut ages, counter, top);
    //     counter += 1;
    //     // println!("Counter: {}. Top: {}", counter + 1, top);

    //     if counter == 2019 {
    //         break;
    //     }
    // }

    // println!("Part 1: {}", top);

    loop {
        top = get_next(&mut ages, counter, top);
        counter += 1;

        if counter % 100000 == 0 {
            println!("Counter: {}. Top: {}", counter, top);
        }

        if counter == ( 30000000 - 1) {
            break;
        }
    }

    println!("Part 2: {}", top);

    Ok(())
}
