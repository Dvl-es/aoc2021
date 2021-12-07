mod lib;
mod day4;

fn run(f: fn(&str) -> u32, filename: &str, number: u32) {
    let answer = f(filename);
    println!("Test {} Answer is: {}", number, answer)
}

fn main() {
    // run(lib::test1, "input1.txt", 1);
    // run(lib::test2, "input1.txt", 2);
    // run(lib::test3, "input2.txt", 3);
    // run(lib::test4, "input2.txt", 4);
    // run(lib::test5, "input3.txt", 5);
    // run(lib::test6, "input3.txt", 6);
    // run(day4::test7, "input4.txt", 7);
    // run(day4::test8, "input4.txt", 8);
}
