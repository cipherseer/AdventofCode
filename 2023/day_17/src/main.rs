use core::fmt;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

#[derive(Eq)]
struct State {
    position: (usize, usize),
    cost: u32,
    steps: u32,
    curr_direction: Direction,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}
use Direction::*;
impl Direction {
    fn get_deltas(&self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
            Stop => (0, 0),
        }
    }
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Up => 0u8.hash(state),
            Down => 1u8.hash(state),
            Left => 2u8.hash(state),
            Right => 3u8.hash(state),
            Stop => 4u8.hash(state),
        }
    }
}

fn dijkstra_part1(grid: &Vec<Vec<u32>>) -> u32 {
    let rows: isize = grid.len() as isize;
    let cols: isize = grid[0].len() as isize;

    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = ((rows - 1) as usize, (cols - 1) as usize);

    let mut to_check = BinaryHeap::new();
    let mut costs: HashMap<((usize, usize), Direction, u32), u32> = HashMap::new();

    to_check.push(State {
        position: start,
        cost: 0,
        steps: 0,
        curr_direction: Stop,
    });
    costs.insert(((start.0, start.1), Stop, 1), 0);

    let directions = [Up, Down, Left, Right];

    while let Some(State {
        position,
        cost,
        steps,
        curr_direction,
    }) = to_check.pop()
    {
        if position == goal {
            return cost;
        }

        for direction in &directions {
            match curr_direction {
                Up => {
                    if *direction == Down {
                        continue;
                    }
                }
                Down => {
                    if *direction == Up {
                        continue;
                    }
                }
                Left => {
                    if *direction == Right {
                        continue;
                    }
                }
                Right => {
                    if *direction == Left {
                        continue;
                    }
                }
                Stop => {}
            }
            let (dx, dy) = direction.get_deltas();
            let new_row = position.0 as isize + dx;
            let new_col = position.1 as isize + dy;

            if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                let new_position = (new_row as usize, new_col as usize);
                let new_cost = cost + grid[new_position.0][new_position.1];
                let new_steps = if *direction == curr_direction {
                    steps + 1
                } else {
                    1
                };

                if new_steps <= 3 {
                    let key = ((new_position.0, new_position.1), *direction, new_steps);
                    if !costs.contains_key(&key) || new_cost < costs[&key] {
                        costs.insert(key, new_cost);
                        to_check.push(State {
                            position: new_position,
                            cost: new_cost,
                            steps: new_steps,
                            curr_direction: *direction,
                        });
                    }
                }
            }
        }
    }

    0
}

fn dijkstra_part2(grid: &Vec<Vec<u32>>) -> u32 {
    let rows: isize = grid.len() as isize;
    let cols: isize = grid[0].len() as isize;

    let start: (usize, usize) = (0, 0);
    let goal: (usize, usize) = ((rows - 1) as usize, (cols - 1) as usize);

    let mut to_check = BinaryHeap::new();
    let mut costs: HashMap<((usize, usize), Direction, u32), u32> = HashMap::new();

    to_check.push(State {
        position: start,
        cost: 0,
        steps: 0,
        curr_direction: Stop,
    });
    costs.insert(((start.0, start.1), Stop, 0), 0);

    let directions = [Up, Down, Left, Right];

    while let Some(State {
        position,
        cost,
        steps,
        curr_direction,
    }) = to_check.pop()
    {
        if position == goal && steps >= 4 {
            return cost;
        }

        for direction in &directions {
            if curr_direction != *direction && steps < 4 && curr_direction != Stop {
                continue;
            }

            match curr_direction {
                Up => {
                    if *direction == Down {
                        continue;
                    }
                }
                Down => {
                    if *direction == Up {
                        continue;
                    }
                }
                Left => {
                    if *direction == Right {
                        continue;
                    }
                }
                Right => {
                    if *direction == Left {
                        continue;
                    }
                }
                Stop => {}
            }
            let (dx, dy) = direction.get_deltas();
            let new_row = position.0 as isize + dx;
            let new_col = position.1 as isize + dy;

            if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
                let new_position = (new_row as usize, new_col as usize);
                let new_cost = cost + grid[new_position.0][new_position.1];
                let new_steps = if *direction == curr_direction {
                    steps + 1
                } else {
                    1
                };

                if new_steps <= 10 {
                    let key = ((new_position.0, new_position.1), *direction, new_steps);
                    if !costs.contains_key(&key) || new_cost < costs[&key] {
                        costs.insert(key, new_cost);
                        to_check.push(State {
                            position: new_position,
                            cost: new_cost,
                            steps: new_steps,
                            curr_direction: *direction,
                        });
                    }
                }
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../input.txt");

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();


    let result: u32 = dijkstra_part1(&grid);

    println!("Result: {result}");

    let result2: u32 = dijkstra_part2(&grid);

    println!("Result - Part 2: {result2}");
}
