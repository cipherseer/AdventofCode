use core::fmt;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Rock,
    Ball,
}

use Cell::{Ball, Empty, Rock};

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Empty => 0u8.hash(state),
            Rock => 1u8.hash(state),
            Ball => 2u8.hash(state),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Empty => write!(f, "."),
            Rock => write!(f, "#"),
            Ball => write!(f, "O"),
        }
    }
}

fn parse_input(input: &str) -> Array2<Cell> {
    let rows: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    Array2::from_shape_fn((rows[0].len(), rows.len()), |(i, j)| match rows[i][j] {
        '.' => Empty,
        '#' => Rock,
        'O' => Ball,
        _ => unreachable!(),
    })
}

fn part2_iteration(array2: &mut Array2<Cell>, iterations: usize) {
    let sizex: usize = array2.len_of(Axis(1));
    let sizey: usize = array2.len_of(Axis(0));
    for _ in 0..iterations {
        //North iteration
        for i in 0..sizex {
            let mut empty_len: usize = 0;
            for j in 0..sizey {
                match array2[[j, i]] {
                    Empty => empty_len += 1,
                    Rock => empty_len = 0,
                    Ball => {
                        if empty_len > 0 {
                            array2[[j, i]] = Empty;
                            array2[[j - empty_len, i]] = Ball;
                        }
                    }
                }
            }
        }

        //West iteration
        for j in 0..sizey {
            let mut empty_len: usize = 0;
            for i in 0..sizex {
                match array2[[j, i]] {
                    Empty => empty_len += 1,
                    Rock => empty_len = 0,
                    Ball => {
                        if empty_len > 0 {
                            array2[[j, i]] = Empty;
                            array2[[j, i - empty_len]] = Ball;
                        }
                    }
                }
            }
        }

        //South iteration
        for i in 0..sizex {
            let mut empty_len: usize = 0;
            for j in (0..sizey).rev() {
                match array2[[j, i]] {
                    Empty => empty_len += 1,
                    Rock => empty_len = 0,
                    Ball => {
                        if empty_len > 0 {
                            array2[[j, i]] = Empty;
                            array2[[j + empty_len, i]] = Ball;
                        }
                    }
                }
            }
        }

        //East iteration
        for j in 0..sizey {
            let mut empty_len: usize = 0;
            for i in (0..sizex).rev() {
                match array2[[j, i]] {
                    Empty => empty_len += 1,
                    Rock => empty_len = 0,
                    Ball => {
                        if empty_len > 0 {
                            array2[[j, i]] = Empty;
                            array2[[j, i + empty_len]] = Ball;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut array = parse_input(input);
    let mut array2 = parse_input(input);
    let mut load: usize = 0;
    let mut load2: usize = 0;
    let sizey = array.len_of(Axis(0));
    let sizex = array.len_of(Axis(1));
    for i in 0..sizex {
        let mut empty_len: usize = 0;
        for j in 0..sizey {
            match array[[j, i]] {
                Empty => empty_len += 1,
                Rock => empty_len = 0,
                Ball => {
                    if empty_len > 0 {
                        array[[j, i]] = Empty;
                        array[[j - empty_len, i]] = Rock;
                        load += sizey - (j - empty_len);
                    } else {
                        load += sizey - j;
                    }
                }
            }
        }
    }

    println!("Total load: {load}");
    let mut cache: HashMap<Array2<Cell>, usize> = HashMap::new();
    const TOTAL_ITERATIONS: usize = 1000_000_000;
    for counter in 0..TOTAL_ITERATIONS {
        if let Some(prev_iteration) = cache.get(&array2) {
            let cycle_length = counter - *prev_iteration;
            println!("Cycle detected @ {} with length: {}", counter, cycle_length);
            let remaining_iterations = (TOTAL_ITERATIONS - counter) % cycle_length;
            println!("Finishing with {remaining_iterations} more iterations");
            part2_iteration(&mut array2, remaining_iterations);
            break;
        }

        cache.insert(array2.clone(), counter);
        part2_iteration(&mut array2, 1);
    }

    for i in 0..sizex {
        for j in 0..sizey {
            match array2[[j, i]] {
                Ball => load2 += sizey - j,
                _ => {}
            }
        }
    }

    println!("Total load- part 2: {load2}")
}
