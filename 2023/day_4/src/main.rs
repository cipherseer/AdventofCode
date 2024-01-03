use std::collections::HashSet;
use std::collections::VecDeque;
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
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum: u32 = 0;
        let mut list_winners: Vec<(usize, u32)> = Vec::new();

        for (i, line) in lines.enumerate() {
            if let Ok(input) = line {
                let mut elements = input.split_whitespace().skip(2);
                let mut winning_nums: HashSet<&str> = HashSet::new();
                let mut winners: u32 = 0;

                for element in &mut elements {
                    if element == "|" {
                        break;
                    }
                    winning_nums.insert(element);
                }
                for element in elements {
                    if winning_nums.contains(element) {
                        winners += 1;
                    }
                }

                list_winners.push((i, winners));

                if winners != 0 {
                    let value = (2 as u32).pow(winners - 1);
                    sum += value;
                } else {
                }
            }
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut card_count: u32 = 0;

        for winners in &list_winners {
            queue.push_back(winners.0);

            while let Some(card) = queue.pop_front() {
                card_count += 1;
                let bonus_cards = list_winners[card].1;
                if bonus_cards > 0 {
                    let final_card = card + bonus_cards as usize;
                    for i in (card + 1)..=final_card {
                        queue.push_back(list_winners[i].0);
                    }
                }
            }
        }

        println!("total sum: {}", sum);
        println!("total cards: {}", card_count);
    }
}
