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

fn mapping(input_vec: &Vec<(u64, u64, u64)>, value: &mut u64) {
    let tmp = *value;
    for map in input_vec {
        //(output_start, source_start, length)
        if map.1 <= tmp && tmp <= map.1 + map.2 {
            *value = map.0 + (tmp - map.1);
        }
    }
}
fn main() {
    if let Ok(mut lines) = read_lines("input.txt") {
        //seeds
        let seeds: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|v| v.parse::<u64>().unwrap())
            .collect();


        let mut seed_to_soil: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(2) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                seed_to_soil.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //soil to fertilizer map
        let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                soil_to_fertilizer.push((entries[0], entries[1], entries[2]));

            } else {
                break;
            }
        }

        //fertilizer to water map
        let mut fertilizer_to_water: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                fertilizer_to_water.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //water to light map
        let mut water_to_light: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                water_to_light.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //light to temperature map
        let mut light_to_temperature: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                light_to_temperature.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //temperature to humidity map
        let mut temperature_to_humidity: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                temperature_to_humidity.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //humidity to location map
        let mut humidity_to_location: Vec<(u64, u64, u64)> = Vec::new();
        for line in lines.by_ref().skip(1) {
            if line.as_ref().is_ok_and(|l| !l.is_empty()) {
                let entries: Vec<u64> = line
                    .unwrap()
                    .split(' ')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect();

                humidity_to_location.push((entries[0], entries[1], entries[2]));
            } else {
                break;
            }
        }

        //Calculate minimum location value
        let mut min_location: u64 = u64::MAX;
        for seed in &seeds {
            let mut value = *seed;

            mapping(&seed_to_soil, &mut value);
            mapping(&soil_to_fertilizer, &mut value);
            mapping(&fertilizer_to_water, &mut value);
            mapping(&water_to_light, &mut value);
            mapping(&light_to_temperature, &mut value);
            mapping(&temperature_to_humidity, &mut value);
            mapping(&humidity_to_location, &mut value);

            if value < min_location {
                min_location = value;
            }
        }

        let mut min_location2: u64 = u64::MAX;
        //Brute force solution... can be improved by using ranges of values instead 
        //of individual values.
        for i in (0..seeds.len()).step_by(2) {
            for seed in seeds[i]..(seeds[i] + seeds[i + 1]) {
                let mut value = seed;
                mapping(&seed_to_soil, &mut value);
                mapping(&soil_to_fertilizer, &mut value);
                mapping(&fertilizer_to_water, &mut value);
                mapping(&water_to_light, &mut value);
                mapping(&light_to_temperature, &mut value);
                mapping(&temperature_to_humidity, &mut value);
                mapping(&humidity_to_location, &mut value);

                min_location2 = min_location2.min(value);
            }
        }

        println!("minimum distance: {}", min_location);
        println!("part 2 minimum distance: {}", min_location2);
    }
}
