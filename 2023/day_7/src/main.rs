use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum Card {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerHandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct PokerHand {
    hand: [Card; 5],
    hand_type: PokerHandType,
    bid: u32,
}

impl PokerHand {
    fn new(input: &str, bid: u32, q: bool) -> Self {
        let mut frequency_map: HashMap<Card, usize> = HashMap::new();
        let mut hand: [Card; 5] = [Card::Two; 5];
        let mut hand_type: PokerHandType = PokerHandType::HighCard;
        let mut num_jokers: u32 = 0;
        for (i, ch) in input.char_indices() {
            match ch {
                '2' => {
                    hand[i] = Card::Two;
                    *frequency_map.entry(Card::Two).or_insert(0) += 1;
                }
                '3' => {
                    hand[i] = Card::Three;
                    *frequency_map.entry(Card::Three).or_insert(0) += 1;
                }
                '4' => {
                    hand[i] = Card::Four;
                    *frequency_map.entry(Card::Four).or_insert(0) += 1;
                }
                '5' => {
                    hand[i] = Card::Five;
                    *frequency_map.entry(Card::Five).or_insert(0) += 1;
                }
                '6' => {
                    hand[i] = Card::Six;
                    *frequency_map.entry(Card::Six).or_insert(0) += 1;
                }
                '7' => {
                    hand[i] = Card::Seven;
                    *frequency_map.entry(Card::Seven).or_insert(0) += 1;
                }
                '8' => {
                    hand[i] = Card::Eight;
                    *frequency_map.entry(Card::Eight).or_insert(0) += 1;
                }
                '9' => {
                    hand[i] = Card::Nine;
                    *frequency_map.entry(Card::Nine).or_insert(0) += 1;
                }
                'T' => {
                    hand[i] = Card::Ten;
                    *frequency_map.entry(Card::Ten).or_insert(0) += 1;
                }
                'J' => {
                    if q {
                        hand[i] = Card::Joker;
                        num_jokers += 1;
                    } else {
                        hand[i] = Card::Jack;
                        *frequency_map.entry(Card::Jack).or_insert(0) += 1;
                    }
                }
                'Q' => {
                    hand[i] = Card::Queen;
                    *frequency_map.entry(Card::Queen).or_insert(0) += 1;
                }
                'K' => {
                    hand[i] = Card::King;
                    *frequency_map.entry(Card::King).or_insert(0) += 1;
                }
                'A' => {
                    hand[i] = Card::Ace;
                    *frequency_map.entry(Card::Ace).or_insert(0) += 1;
                }
                _ => {}
            }
        }

        if num_jokers != 0 {
            for value in frequency_map.values_mut() {
                *value += num_jokers as usize;
            }
        }

        match frequency_map.values().max() {
            Some(&max_value) => match max_value {
                1 => hand_type = PokerHandType::HighCard,
                2 => {
                    if frequency_map.len() == 4 {
                        hand_type = PokerHandType::OnePair;
                    } else {
                        hand_type = PokerHandType::TwoPair;
                    }
                }
                3 => {
                    if frequency_map.len() == 3 {
                        hand_type = PokerHandType::ThreeOfAKind;
                    } else {
                        hand_type = PokerHandType::FullHouse;
                    }
                }
                4 => hand_type = PokerHandType::FourOfAKind,
                5 => hand_type = PokerHandType::FiveOfAKind,
                _ => {}
            },
            None => hand_type = PokerHandType::FiveOfAKind,
        }

        PokerHand {
            hand,
            hand_type,
            bid,
        }
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        //first check the hand type
        if self.hand_type < other.hand_type {
            return Ordering::Greater;
        } else if self.hand_type > other.hand_type {
            return Ordering::Less;
        }

        //if the hand type is the same check value of individual cards
        for (a, b) in zip(&self.hand, &other.hand) {
            if a < b {
                return Ordering::Greater;
            } else if a > b {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut poker_hands: BinaryHeap<PokerHand> = BinaryHeap::new();
        let mut poker_hands2: BinaryHeap<PokerHand> = BinaryHeap::new();
        for line in lines {
            if let Ok(input) = line {
                let (hand, bid_str) = input.split_at(5);
                let bid: u32 = bid_str.trim().parse::<u32>().unwrap();
                poker_hands.push(PokerHand::new(hand, bid, false));
                poker_hands2.push(PokerHand::new(hand, bid, true));
            }
        }

        let mut counter = 0;
        let mut total_winnings = 0;
        while let Some(hand) = poker_hands.pop() {
            counter += 1;
            total_winnings += counter * hand.bid;
        }

        println!("total winnings: {}", total_winnings);

        counter = 0;
        total_winnings = 0;

        while let Some(hand) = poker_hands2.pop() {
            counter += 1;
            total_winnings += counter * hand.bid;
        }

        println!("total winnings2: {}", total_winnings);
    }
}
