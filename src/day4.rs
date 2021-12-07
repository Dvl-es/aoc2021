use std::env;
use std::fs;

pub fn test7(filename: &str) -> i32 {
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
