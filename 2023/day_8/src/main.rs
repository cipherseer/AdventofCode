use regex::Regex;
use std::collections::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcm_vec(nums: Vec<usize>) -> usize {
    nums.into_iter().fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let input = include_str!("../input.txt");
    let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let instructions = input.lines().next().unwrap();
    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut locations: Vec<&str> = Vec::new();

    for line in input.lines().skip(2) {
        let cap = re.captures(line).unwrap();

        if line.chars().nth(2).unwrap() == 'A' {
            locations.push(cap.get(1).unwrap().as_str());
        }


        mappings.insert(
            cap.get(1).unwrap().as_str(),
            (cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
        );
    }

    let mut value = "AAA";
    let mut steps = 0;
    let mut instruction = 0;
    let total_instructions = instructions.len();
    //Part 1
    while value != "ZZZ" {
        match instructions.as_bytes()[instruction] as char {
            'R' => value = mappings.get(value).unwrap().1,
            'L' => value = mappings.get(value).unwrap().0,
            _ => {}
        }

        steps += 1;
        instruction = steps % total_instructions;
    }

    println!("Part 1- steps: {}", steps);

    //Part 2
    instruction = 0;
    let mut steps_sol: Vec<usize> = vec![0; locations.len()];

    for (i, location) in locations.iter_mut().enumerate() {
        while location.as_bytes()[2] as char != 'Z' {
            match instructions.as_bytes()[instruction] as char {
                'L' => *location = mappings.get(location).unwrap().0,
                'R' => *location = mappings.get(location).unwrap().1,
                _ => {}
            }

            steps_sol[i] += 1;
            instruction = steps_sol[i] % total_instructions;

            if location.as_bytes()[2] as char == 'Z' {
                break;
            }
        }
    }

    println!("Part 2- steps: {}", lcm_vec(steps_sol));
}
