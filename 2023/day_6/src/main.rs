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

fn concat(vec: &[u64]) -> u64 {
    vec.iter()
        .map(|&val| val.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn main() {
    if let Ok(mut lines) = read_lines("input.txt") {
        let times: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let time2 = concat(&times);

        let distances: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse::<u64>().unwrap())
            .collect();

        let distance2 = concat(&distances);

        let mut result: u64 = 1;
        for i in 0..times.len() {
            let maxf = (times[i] as f64
                + f64::sqrt((times[i] as f64).powf(2.0) - 4.0 * (distances[i] as f64)))
                / 2.0;
            let minf = (times[i] as f64
                - f64::sqrt((times[i] as f64).powf(2.0) - 4.0 * (distances[i] as f64)))
                / 2.0;
            let mut max = f64::floor(maxf) as u64;
            let mut min = f64::ceil(minf) as u64;
            if !((max as f64) < maxf) {
                max -= 1;
            }

            if !((min as f64) > minf) {
                min += 1;
            }
            let value = max - min + 1;
 
            result *= value;
        }

        let mut result2: u64 = 1;
        {
            let maxf = (time2 as f64
                + f64::sqrt((time2 as f64).powf(2.0) - 4.0 * (distance2 as f64)))
                / 2.0;
            let minf = (time2 as f64
                - f64::sqrt((time2 as f64).powf(2.0) - 4.0 * (distance2 as f64)))
                / 2.0;
            let mut max = f64::floor(maxf) as u64;
            let mut min = f64::ceil(minf) as u64;
            if !((max as f64) < maxf) {
                max -= 1;
            }

            if !((min as f64) > minf) {
                min += 1;
            }
            let value = max - min + 1;

            result2 *= value;
        }

        println!("result: {}", result);
        println!("result2: {}", result2);
    }
}
