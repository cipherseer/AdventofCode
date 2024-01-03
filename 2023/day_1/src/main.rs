use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first(s: &str, patterns: &[&str]) -> u32 {
    let mut min_index: usize = s.len();
    let mut min_pattern = "";
    for pattern in patterns.iter() {
        if let Some(index) = s.find(pattern) {

            if min_index > index {
                min_index = index;
                min_pattern = pattern;
            }
        }
    }

    match min_pattern {
        "0" | "zero"     => 0,
        "1" | "one"      => 1,
        "2" | "two"      => 2,
        "3" | "three"    => 3,
        "4" | "four"     => 4,
        "5" | "five"     => 5,
        "6" | "six"      => 6,
        "7" | "seven"    => 7,
        "8" | "eight"    => 8,
        "9" | "nine"     => 9,
        _                => 0
    }
}

fn find_last(s: &str, patterns: &[&str]) -> u32 {
    let mut max_index: usize = 0;
    let mut max_pattern = "";
    for pattern in patterns.iter() {
        if let Some(index) = s.rfind(pattern) {
            if index >= max_index {
                max_index = index;
                max_pattern = pattern;
            }
        }
    }

    match max_pattern {
        "0" | "zero"     => 0,
        "1" | "one"      => 1,
        "2" | "two"      => 2,
        "3" | "three"    => 3,
        "4" | "four"     => 4,
        "5" | "five"     => 5,
        "6" | "six"      => 6,
        "7" | "seven"    => 7,
        "8" | "eight"    => 8,
        "9" | "nine"     => 9,
        _                => 0
    }
}


fn main() {
    let patterns = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                    "zero", "one", "two", "three", "four", "five", "six",
                    "seven", "eight", "nine"];

    if let Ok(lines) = read_lines("input.txt") {
        let mut numbers: Vec<u32> = Vec::new();
        for line in lines {
            if let Ok(input) = line {
                let upper: u32 = find_first(&input, &patterns);
                let lower: u32 = find_last(&input, &patterns);
                let value = 10*upper + lower;
                numbers.push(value);
            }
        }

        println!("Sum is: {}", numbers.iter().sum::<u32>());
    }
}
