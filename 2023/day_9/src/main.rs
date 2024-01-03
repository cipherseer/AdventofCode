fn main() {
    let input = include_str!("../input.txt");

    let mut total_sum: i64 = 0;
    let mut total_previous_sum: i64 = 0;

    for line in input.lines() {
        let mut numbers: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        let mut not_complete: bool = true;
        let mut i: usize = 0;
        let mut length: usize = numbers.len();
        while not_complete {
            not_complete = false;
            for j in i..(i + length - 1) {
                let difference = numbers[j + 1] - numbers[j];
                numbers.push(difference);
                if difference != 0 {
                    not_complete = true;
                }
            }
            i += length;
            length -= 1;
        }

        let mut sum: i64 = 0;
        let mut previous_sum: i64 = 0;
        while i != 0 {
            length += 1;
            sum += numbers[i - 1];
            i -= length;
            previous_sum = numbers[i] - previous_sum;
        }

        total_sum += sum;
        total_previous_sum += previous_sum;
    }

    println!("Total sum: {total_sum}");
    println!("Previous total sum: {total_previous_sum}");
}
