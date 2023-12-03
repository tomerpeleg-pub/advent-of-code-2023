use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;

fn first_num(line: &String) -> u32 {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }

    0
}

fn last_num(line: &String) -> u32 {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }

    0
}

fn day1_p1() -> u32 {
    let reader = BufReader::new(File::open("src/day1_input.txt").expect("Cannot open file.txt"));

    let mut result = 0;

    for line in reader.lines() {
        let l = &line.unwrap();
        let first = first_num(l);
        let last = last_num(l);
        result += first * 10 + last;
    }

    return result;
}

fn first_num_or_word(line: &String) -> i32 {
    let re = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for (_, [n]) in re.captures_iter(line).map(|c| c.extract()) {
        match n {
            
        }
    }

    return 0
}


fn day1_p2() -> u32 {
    let reader = BufReader::new(File::open("src/day1_example2.txt").expect("Cannot open file.txt"));

    let mut result = 0;

    for line in reader.lines() {
        let l = &line.unwrap();
        let first = first_num(l);
        let last = last_num(l);
        result += first * 10 + last;
    }

    return result;
}

fn main() {
    let result = day1_p1();
    let result2 = day1_p2();

    println!("P1: {}, P2: {}", result, result2)
}
