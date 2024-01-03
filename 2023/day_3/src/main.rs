use regex::Regex;
use std::collections::HashMap;
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

fn main() {
    let re = Regex::new("[0-9]+").unwrap();
    if let Ok(lines) = read_lines("input.txt") {
        let mut rows: Vec<String> = Vec::new();
        for line in lines {
            if let Ok(input) = line {
                rows.push(input);
            }
        }

        let mut numbers: Vec<u32> = Vec::new();
        let mut map: HashMap<(usize, usize), u32> = HashMap::new();
        let mut gear_ratios: u32 = 0;

        for i in 0..rows.len() {
            for m in re.find_iter(&rows[i]) {
                let mut valid: bool = false;
                let mut l = m.start();
                let r = m.end();
                let value: u32 = m.as_str().parse().unwrap();

                //check left and right of the number
                if l > 0 {
                    l -= 1;
                    match rows[i].as_bytes()[l] as char {
                        '.' => {}
                        '0'..='9' => {}
                        '*' => {
                            valid = true;
                            let result = map.insert((i, l), value);
                            match result {
                                Some(v) => {
                                    gear_ratios += v * value;
                                }
                                None => {}
                            }
                        }
                        _ => valid = true,
                    }
                }

                if r < rows[i].len() {
                    match rows[i].as_bytes()[r] as char {
                        '.' => {}
                        '0'..='9' => {}
                        '*' => {
                            valid = true;
                            let result = map.insert((i, r), value);

                            match result {
                                Some(v) => {
                                    gear_ratios += v * value;
                                }
                                None => {}
                            }
                        }
                        _ => {
                            valid = true
                        }
                    }
                }

                //check the perimeter of the number
                for j in l..=r {
                    //checking rows directly above and below
                    if j != 0 && j != rows[i].len() {
                        if i != 0 {
                            match rows[i - 1].as_bytes()[j] as char {
                                '.' => {}
                                '0'..='9' => {}
                                '*' => {
                                    valid = true;
                                    let result = map.insert((i - 1, j), value);

                                    match result {
                                        Some(v) => {
                                            gear_ratios += v * value;
                                        }
                                        None => {}
                                    }
                                }
                                _ => valid = true,
                            }
                        }

                        if i != rows.len() - 1 {
                            match rows[i + 1].as_bytes()[j] as char {
                                '.' => {}
                                '0'..='9' => {}
                                '*' => {
                                    valid = true;
                                    let result = map.insert((i + 1, j), value);

                                    match result {
                                        Some(v) => {
                                            gear_ratios += v * value;
                                        }
                                        None => {}
                                    }
                                }
                                _ => valid = true,
                            }
                        }
                    }
                }

                if valid {
                    numbers.push(value);
                }
            }
        }

        println!(
            "The sum of valid numbers is: {}",
            numbers.iter().sum::<u32>()
        );
        println!("The sum of gear ratios is: {}", gear_ratios);
    }
}
