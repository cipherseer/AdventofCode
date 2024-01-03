use std::collections::HashMap;
use std::time::Instant;

fn count(springs: &str, nums: &[usize], cache: &mut HashMap<(String, Vec<usize>), usize>) -> usize {
    if springs == "" {
        if nums.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(&saved) = cache.get(&(springs.to_string(), nums.to_vec())) {
        return saved;
    }

    if nums.len() == 0 {
        if springs.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    if nums.len() == 1 && nums[0] == springs.len() {
        if springs.contains('.') {
            return 0;
        } else {
            return 1;
        }
    }

    //???.### 1,1,3
    let mut result: usize = 0;

    if springs.chars().nth(0).unwrap() == '.' || springs.chars().nth(0).unwrap() == '?' {
        result += count(&springs[1..], nums, cache);
    }

    if springs.chars().nth(0).unwrap() == '#' || springs.chars().nth(0).unwrap() == '?' {
        if nums[0] < springs.len()
            && !springs[..nums[0]].contains(".")
            && (nums[0] == springs.len() || springs.chars().nth(nums[0]).unwrap() != '#')
        {
            result += count(&springs[(nums[0] + 1)..], &nums[1..], cache);
        }
    }

    cache.insert((springs.to_string(), nums.to_vec()), result);
    result
}
fn main() {
    let start_time = Instant::now();
    let input = include_str!("../input.txt");

    let mut total_count: usize = 0;
    let mut total_count2: usize = 0;
    let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
    for line in input.lines() {
        let values: Vec<&str> = line.split_ascii_whitespace().collect();

        let springs = values[0];
        let duplicated_springs = std::iter::repeat(springs)
            .take(5)
            .collect::<Vec<_>>()
            .join("?");

        let nums: Vec<usize> = values[1]
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        
        let duplicated_nums: Vec<usize> = nums.repeat(5);
        let count1 = count(&springs, &nums, &mut cache);
        let count2 = count(&duplicated_springs, &duplicated_nums, &mut cache);
        total_count += count1;
        total_count2 += count2;

    }
    let end_time = Instant::now();

    println!("Total count: {total_count}");
    println!("Total count- Part 2: {total_count2}");
    println!("Took {} milliseconds", (end_time - start_time).as_millis());
}
