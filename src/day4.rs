use std::env;
use std::fs;

#[derive(Debug)]
struct Card {
    numbers: [[u32; 5]; 5],
    filled: [[bool; 5]; 5],
    won: bool,
}

impl Card {
    fn mark(&mut self, number: u32) -> bool {
        for i in 0..5 {
            for k in 0..5 {
                if self.numbers[i][k] == number {
                    self.filled[i][k] = true;
                    return true
                }
            }
        }
        return false;
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for i in 0..5 {
            for k in 0..5 {
                if !self.filled[i][k] {
                    sum += self.numbers[i][k];
                }
            }
        }
        return sum;
    }

    fn is_win(&self) -> Result<u32, ()> {
        for i in 0..5 {
            if self.filled[i][0] && self.filled[i][1] && self.filled[i][2] && self.filled[i][3]
                && self.filled[i][4] {
                return Ok(self.unmarked_sum());
            }
            if self.filled[0][i] && self.filled[1][i] && self.filled[2][i] && self.filled[3][i]
                && self.filled[4][i] {
                return Ok(self.unmarked_sum());
            }
        }
        return Err(());
    }
}

pub fn test7(filename: &str) -> u32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // PARSING
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut numbers: Vec<u32> = Vec::new();
    for number in lines[0].split(',') {
        numbers.push(number.parse::<u32>().unwrap());
    }

    let mut i = 0;
    let mut cards: Vec<Card> = Vec::new();
    let mut card: Card = Card { numbers: [[0; 5]; 5], filled: [[false; 5]; 5] , won: false};;
    for l in lines[2..].into_iter() {
        let k = i % 6;
        if k != 5 {
            let mut column = 0;
            for r in l.split(' ') {
                println!("{}", r);
                let parse_result = r.parse::<u32>();
                match parse_result {
                    Ok(number) => {
                        card.numbers[k][column] = number;
                        column += 1;
                    }
                    Err(e) => continue
                }
            }
            println!("---")
        }

        if k == 5 {
            println!("{:?}", card);
            cards.push(card);
            card = Card { numbers: [[0; 5]; 5], filled: [[false; 5]; 5] , won: false};
        }

        i += 1;
    }
    cards.push(card);
    // PARSING ENDS

    println!("{:?}", cards);

    for number in numbers {
        println!("Number is: {}", number);
        for card in cards.iter_mut() {
            let r = card.mark(number);
            if r {
                match card.is_win() {
                    Ok(sum) => {
                        println!("Sum: {}", sum);
                        return sum * number
                    },
                    _ => {}
                }
            }
        }
    }
    panic!("No winner :(");
}

pub fn test8(filename: &str) -> u32 {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // PARSING
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut numbers: Vec<u32> = Vec::new();
    for number in lines[0].split(',') {
        numbers.push(number.parse::<u32>().unwrap());
    }

    let mut i = 0;
    let mut cards: Vec<Card> = Vec::new();
    let mut card: Card = Card { numbers: [[0; 5]; 5], filled: [[false; 5]; 5], won: false };;
    for l in lines[2..].into_iter() {
        let k = i % 6;
        if k != 5 {
            let mut column = 0;
            for r in l.split(' ') {
                println!("{}", r);
                let parse_result = r.parse::<u32>();
                match parse_result {
                    Ok(number) => {
                        card.numbers[k][column] = number;
                        column += 1;
                    }
                    Err(e) => continue
                }
            }
            println!("---")
        }

        if k == 5 {
            println!("{:?}", card);
            cards.push(card);
            card = Card { numbers: [[0; 5]; 5], filled: [[false; 5]; 5], won: false };
        }

        i += 1;
    }
    cards.push(card);
    // PARSING ENDS

    println!("{:?}", cards);

    let mut last_win = 0;

    for number in numbers {
        println!("Number is: {}", number);
        for card in cards.iter_mut() {
            if card.won {
                continue;
            }
            let r = card.mark(number);
            if r {
                match card.is_win() {
                    Ok(sum) => {
                        println!("Sum: {}", sum);
                        // return sum * number
                        last_win = sum * number;
                        card.won = true;
                    },
                    _ => {}
                }
            }
        }
    }
    return last_win;
}