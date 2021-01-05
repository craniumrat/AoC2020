fn main() {

    //I te0t replaced the 0 in buses with 0 so I dont have to parse it.
    let ready = 1000511;
    let buses: Vec<i64> = vec![29,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,37,0,0,0,0,0,409,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,17,13,19,0,0,0,23,0,0,0,0,0,0,0,353,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,41];
    // let buses: Vec<i64> = vec![29,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,37,0,0,0,0,0,409,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,17,13,19,0,0,0,23,0,0,0,0,0,0,0,353,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,41];

    let pairs: Vec<(_, _)> = buses.iter().enumerate().filter(|(_, &b)| b != 0).collect();

    let buses: Vec<_> = buses.iter().filter(|&b| *b != 0).collect();
    let times: Vec<_> = buses.iter().map(|&b| b - (ready % b)).collect();

    let (inde0, value) = times.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    println!("Part 1: {}", value * buses[inde0]);

    println!("{:?}", pairs);
    let mut pairs: Vec<_> = pairs.iter().map(|(inde0, &value)| ((*inde0 as i64) % value, value)).collect();
    println!("{:?}", pairs);

    pairs.sort_by(|(a, _), (b, _)| a.cmp(b));
    println!("{:?}", pairs);

    //So we need a 0 such that for each (offset, multiplier), 
    // 0 % multiplier = offset
    //Start with the largest to smallest. Multiply the original number
    // until such a time that

    // let mut pairs = vec![(0, 7), (1, 5), (2, 3)];
    // let mut pairs = vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
    // let mut pairs = vec![(0, 17), (2, 13), (3, 19)];
    // let mut pairs = vec![(0, 67), (2, 7), (3, 59), (4, 61)];
    let (_, first) = pairs.remove(0);
    let mut interval: i64 = 1;
    let mut i: i64 = 1;

    for pair in pairs {
        println!("Interval: {}", interval);
        loop {
            i += interval;
            println!("i: {}", i);
            if (first * i + pair.0) % pair.1 == 0 {
                println!("{} * {} + {} % {}", first, i, pair.0, pair.1);
                interval *= pair.1;
                break;
            }
        }
        
        println!("Answer: {}", i * first);
    }
}
