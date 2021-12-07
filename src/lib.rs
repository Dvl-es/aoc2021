use std::env;
use std::fs;

pub fn test1(filename: &str) -> i32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut counter = 0;
    let mut prev = 0;
    for line in contents.split('\n') {
        let depth = line.parse::<i32>().unwrap();
        if depth > prev {
            counter += 1;
        }
        prev = depth;
    }
    return counter - 1;
}

pub fn test2(filename: &str) -> i32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut counter = 0;

    let mut index: usize = 0;
    let mut buf: [i32; 3] = [0, 0, 0];
    let mut prev_total = 0;
    let mut total = 0;

    for line in contents.split('\n') {
        let depth = line.parse::<i32>().unwrap();
        if index > 2 {
            total = buf[0] + buf[1] + buf[2];
            if total > prev_total {
                counter += 1;
            }
            prev_total = total;
        }
        buf[index % 3] = depth;
        index += 1;
    }
    return counter;
}

pub fn test3(filename: &str) -> i32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut horizonal = 0;
    let mut vertical = 0;

    for line in contents.split('\n') {
        let pair: Vec<&str> = line.split(' ').collect();
        let command = pair[0];
        let arg = pair[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizonal += arg;
            },
            "up" => {
                vertical -= arg;
            }
            "down" => {
                vertical += arg;
            }
            &_ => {
                panic!("Unknown command: {}", command)
            }
        }
    }
    return horizonal * vertical;
}

pub fn test4(filename: &str) -> i32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut horizonal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for line in contents.split('\n') {
        let pair: Vec<&str> = line.split(' ').collect();
        let command = pair[0];
        let arg = pair[1].parse::<i32>().unwrap();

        match command {
            "forward" => {
                horizonal += arg;
                vertical += aim * arg;
            },
            "up" => {
                aim -= arg;
            }
            "down" => {
                aim += arg;
            }
            &_ => {
                panic!("Unknown command: {}", command)
            }
        }
    }
    return horizonal * vertical;
}

pub fn test5(filename: &str) -> u32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    const TOTAL_LINES: i32 = 1000;
    const BIT_COUNT: usize = 12;

    let mut one_counters: [i32; BIT_COUNT] = [0; BIT_COUNT];

    for line in contents.split('\n') {
        let mut char_idx = 0;
        for x in line.chars() {
            if x == '1' {
                one_counters[char_idx] += 1;
            }
            char_idx += 1;
        }
    }

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for i in 0..BIT_COUNT {
        if one_counters[i] > TOTAL_LINES / 2 {
            gamma_rate += (1 << (BIT_COUNT - i - 1));
        } else {
            epsilon_rate += (1 << (BIT_COUNT - i - 1));
        }
    }
    return gamma_rate * epsilon_rate;
}

const TEST_6_TOTAL_LINES: usize = 1000;
const TEST_6_BIT_COUNT: usize = 12;

fn test6_find_rating(mut candidates: Vec<u32>, is_msb: bool) -> u32 {
    for b in 0..TEST_6_BIT_COUNT {
        let mut mcb = 0;
        let mut one_count = 0;
        let s = TEST_6_BIT_COUNT - b - 1;
        for c in &candidates {
            one_count += (*c >> s) & 1;
        }

        if one_count >= candidates.len() as u32 - one_count  {
            mcb = 1;
        } else {
            mcb = 0;
        }
        if !is_msb {
            mcb = !mcb;
        }

        candidates.retain(|c| {
            let b_bit = (c >> s) & 1;
            println!("c - {:b}, b_bit - {:b}, mcb - {}, total: {}", c, b_bit, mcb, b_bit == mcb);
            b_bit == mcb
        });
        println!("Left: {}", candidates.len());
        if candidates.len() == 1 {
            return candidates[0];
        }
        if candidates.len() == 0 {
            panic!("No candidates!")
        }
    }
    return 0;
}

pub fn test6(filename: &str) -> u32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut o2_candidates: Vec<u32> = vec![0; TEST_6_TOTAL_LINES.try_into().unwrap()];
    let mut co2_candidates: Vec<u32> = vec![0; TEST_6_TOTAL_LINES.try_into().unwrap()];

    let mut i = 0;
    for line in contents.split('\n') {
        let mut char_idx = 0;
        let mut value = 0;
        for x in line.chars() {
            if x == '1' {
                value += (1 << TEST_6_BIT_COUNT - char_idx - 1);
            }
            char_idx += 1;
        }
        o2_candidates[i] = value;
        co2_candidates[i] = value;
        i += 1;
    }

    let o2 = test6_find_rating(o2_candidates, true);
    println!("O2 is {}", o2);
    let co2 = test6_find_rating(co2_candidates, false);
    println!("CO2 is {}", co2);
    return o2 * co2;
}