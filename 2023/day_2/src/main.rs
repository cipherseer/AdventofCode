use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    let regex = Regex::new(": |, |; | ").unwrap();
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: u32 = 0;
        let mut powers: u32 = 0;
        for (n, line) in lines.enumerate() {
            let mut valid = true;
            if let Ok(input) = line {
                let entries: Vec<&str> = regex.split(&input).collect(); 
                
                let mut blue_max: u32 = 0;
                let mut red_max: u32 = 0;
                let mut green_max: u32 = 0;

                for i in (2..entries.len()).step_by(2) {
                    let value: u32 = entries[i].parse().unwrap();
                    match entries[i+1] {
                        "blue" => {
                            if value > BLUE {
                                valid = false;
                            }

                            if value > blue_max {
                                blue_max = value;
                            }
                        },
                        "red" => {
                            if value > RED {
                                valid = false;
                            }

                            if value > red_max {
                                red_max = value;
                            }
                        },
                        "green" => {
                            if value > GREEN {
                                valid = false;
                            }

                            if value > green_max {
                                green_max = value;
                            }
                        },
                        _ => {},
                    };

                }
            
                if valid {
                    sum += 1 + n as u32;
                }

                powers += blue_max*red_max*green_max;
            }
        }

        println!("The sum of valid games is: {}", sum);
        println!("The sum of powers is: {}", powers);



    }
}
