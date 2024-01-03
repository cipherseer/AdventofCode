use core::fmt;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq)]
struct State {
    position: (usize, usize),
    prev_position: (usize, usize),
    steps: usize,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.steps)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.steps.eq(&other.steps)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;
impl Direction {
    fn get_deltas(&self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
    graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    start: (usize, usize),
    end: (usize, usize),
    rows: usize,
    cols: usize,
}

impl Maze {
    fn new(
        grid: Vec<Vec<char>>,
        graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
        start: (usize, usize),
        end: (usize, usize),
        rows: usize,
        cols: usize,
    ) -> Self {
        Self {
            grid,
            graph,
            start,
            end,
            rows,
            cols,
        }
    }

    fn longest_path_part1(&self) -> usize {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        let mut max_steps: usize = 0;
        let directions = [Up, Down, Left, Right];

        heap.push(State {
            position: self.start,
            prev_position: self.start,
            steps: 0,
        });
        visited.insert(self.start, 0);

        while let Some(State {
            position,
            prev_position,
            steps,
        }) = heap.pop()
        {
            if position == self.end {
                max_steps = max_steps.max(steps);
                continue;
            }

            for direction in directions {
                let (dx, dy) = direction.get_deltas();
                let nx = position.0 as isize + dx;
                let ny = position.1 as isize + dy;

                if prev_position == (nx as usize, ny as usize)
                    || nx < 0
                    || nx >= self.cols as isize
                    || ny < 0
                    || ny >= self.rows as isize
                {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let prev_tile = self.grid[position.1][position.0];
                let tile = self.grid[ny][nx];

                if tile == '#' {
                    continue;
                }
                match (prev_tile, direction) {
                    ('v', Down) | ('^', Up) | ('>', Right) | ('<', Left) | ('.', _) => {}
                    _ => continue,
                }

                if let Some(prev_steps) = visited.get(&(nx, ny)) {
                    if *prev_steps > steps {
                        continue;
                    }
                }

                visited.insert((nx, ny), steps + 1);
                heap.push(State {
                    position: (nx, ny),
                    prev_position: position,
                    steps: steps + 1,
                });
            }
        }

        max_steps
    }

    fn longest_path_part2(&self) -> usize {
        let mut stack: Vec<((usize, usize), usize, HashSet<(usize, usize)>)> =
            vec![(self.start, 0, HashSet::new())];
        let mut max_steps: usize = 0;

        while let Some(((x, y), steps, mut path)) = stack.pop() {
            if (x, y) == self.end {
                max_steps = max_steps.max(steps);
                continue;
            }

            path.insert((x, y));

            if let Some(neighbours) = self.graph.get(&(x, y)) {
                for &neighbour in neighbours.keys() {
                    if !path.contains(&neighbour) {
                        stack.push((neighbour, steps + neighbours[&neighbour], path.clone()));
                    }
                }
            }
        }

        max_steps
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let rows: usize = input.lines().count();
    let cols: usize = input.lines().next().unwrap().len();

    let mut grid: Vec<Vec<char>> = vec![vec!['#'; cols]; rows];
    let start: (usize, usize) = (1, 0);
    let end: (usize, usize) = (cols - 2, rows - 1);

    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            grid[y][x] = ch;
        }
    }

    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    nodes.insert(start);
    nodes.insert(end);
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '#' {
                continue;
            }
            let mut paths: usize = 0;

            for direction in [Up, Down, Left, Right].iter() {
                let (dx, dy) = direction.get_deltas();
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && nx < cols as isize
                    && ny >= 0
                    && ny < rows as isize
                    && grid[ny as usize][nx as usize] != '#'
                {
                    paths += 1;
                }
            }

            if paths >= 3 {
                nodes.insert((x, y));
            }
        }
    }

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = nodes
        .iter()
        .map(|point| (*point, HashMap::new()))
        .collect();

    for (sx, sy) in &nodes {
        let mut stack: Vec<(usize, usize, usize)> = vec![(*sx, *sy, 0)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((*sx, *sy));

        while let Some((x, y, n)) = stack.pop() {
            if n != 0 && nodes.contains(&(x, y)) {
                graph.get_mut(&(*sx, *sy)).unwrap().insert((x, y), n);
                continue;
            }

            for direction in [Up, Down, Left, Right].iter() {
                let (dx, dy) = direction.get_deltas();
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0
                    && nx < cols as isize
                    && ny >= 0
                    && ny < rows as isize
                    && grid[ny as usize][nx as usize] != '#'
                    && !visited.contains(&(nx as usize, ny as usize))
                {
                    stack.push((nx as usize, ny as usize, n + 1));
                    visited.insert((nx as usize, ny as usize));
                }
            }
        }
    }

    let maze: Maze = Maze::new(grid, graph, start, end, rows, cols); 
       
    println!(
        "The longest path in Part 1 is: {}",
        maze.longest_path_part1()
    );

    println!(
        "The longest path in Part 2 is: {}",
        maze.longest_path_part2()
    );
}
