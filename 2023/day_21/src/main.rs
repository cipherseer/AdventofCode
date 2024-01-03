use core::fmt;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

#[derive(Clone, PartialEq)]
enum Tile {
    Garden,
    Rock,
}
use Tile::*;

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Garden => write!(f, "."),
            Rock => write!(f, "#"),
        }
    }
}

fn solve(garden: &Vec<Vec<Tile>>, start_x: isize, start_y: isize, total_steps: isize) -> isize {
    let mut final_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize, isize)> = VecDeque::new();
    let sizey: isize = garden.len() as isize;
    let sizex: isize = garden[0].len() as isize;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    visited.insert((start_x, start_y));
    queue.push_back((start_x, start_y, 0));

    while let Some((x, y, step)) = queue.pop_front() {
        //can only return to a tile if steps remaining is even
        if (total_steps - step) % 2 == 0 {
            final_positions.insert((x, y));
        }

        if step == total_steps {
            continue;
        }

        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;
            let nxw = (nx % sizex + sizex) % sizex;
            let nyw = (ny % sizey + sizey) % sizey;
            if garden[nyw as usize][nxw as usize] == Garden && !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                queue.push_back((nx, ny, step + 1));
            }
        }
    }

    final_positions.len() as isize
}

fn solve2(garden: &Vec<Vec<Tile>>, start_x: isize, start_y: isize, total_steps: isize) -> isize {
    let mut final_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut queue: VecDeque<(isize, isize, isize)> = VecDeque::new();
    let sizey: isize = garden.len() as isize;
    let sizex: isize = garden[0].len() as isize;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    visited.insert((start_x, start_y));
    queue.push_back((start_x, start_y, 0));

    while let Some((x, y, step)) = queue.pop_front() {
        //can only return to a tile if steps remaining is even
        if (total_steps - step) % 2 == 0 {
            final_positions.insert((x, y));
        }

        if step == total_steps {
            continue;
        }

        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < sizex
                && ny >= 0
                && ny < sizey
                && garden[ny as usize][nx as usize] == Garden
                && !visited.contains(&(nx, ny))
            {
                visited.insert((nx, ny));
                queue.push_back((nx, ny, step + 1));
            }
        }
    }

    final_positions.len() as isize
}

fn lagrange_interpolation(v: [(isize, isize); 3], x: isize) -> isize {
    let mut result: isize = 0;
    for j in 0..3 {
        let mut acc = v[j].1;
        for m in 0..3 {
            if m != j {
                acc *= (x - v[m].0) / (v[j].0 - v[m].0);
            }
        }
        result += acc;
    }
    result
}

fn main() {
    let input = include_str!("../input.txt");
    let sizey: isize = input.lines().count() as isize;
    let sizex: isize = input.lines().next().unwrap().chars().count() as isize;
    let mut garden: Vec<Vec<Tile>> = vec![vec![Garden; sizex as usize]; sizey as usize];
    let mut start_x: isize = 0;
    let mut start_y: isize = 0;
    for (j, line) in input.lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '#' => garden[j][i] = Rock,
                'S' => {
                    start_x = i as isize;
                    start_y = j as isize;
                }
                _ => {}
            }
        }
    }

    //Part 1

    println!(
        "Total tiles reached Part 1: {:?}",
        solve(&garden, start_x, start_y, 64)
    );

    //Part 2
    let start = Instant::now();
    //input is 131 blocks tall and wide with an open row and column in the middle
    //so if 3 values separated by 131 steps are recorded the quadratic equation can 
    //be inferred with Lagrange interpolation
    let values: [(isize, isize); 3] = [
        (65, solve(&garden, start_x, start_y, 65)),
        (196, solve(&garden, start_x, start_y, 196)),
        (327, solve(&garden, start_x, start_y, 327)),
    ];
    let result = lagrange_interpolation(values, 26501365);
    let finish = start.elapsed();
    println!("Result Part 2: {} in {} ms", result, finish.as_millis());

    let start = Instant::now();
    //comparing to hyperneutrino's method
    let width = 26501365 / sizex - 1;

    let odd_grids = ((width / 2) * 2 + 1).pow(2);
    let even_grids = (((width + 1) / 2) * 2).pow(2);
    let odds = solve2(&garden, start_x, start_y, sizex * 2 + 1);
    let evens = solve2(&garden, start_x, start_y, sizex * 2);

    let t_corner = solve2(&garden, start_x, sizey - 1, sizey - 1);
    let b_corner = solve2(&garden, start_x, 0, sizey - 1);
    let l_corner = solve2(&garden, sizex - 1, start_y, sizex - 1);
    let r_corner = solve2(&garden, 0, start_y, sizex - 1);

    let tr_small = solve2(&garden, 0, sizey - 1, sizey / 2 - 1);
    let br_small = solve2(&garden, 0, 0, sizex / 2 - 1);
    let tl_small = solve2(&garden, sizex - 1, sizey - 1, sizex / 2 - 1);
    let bl_small = solve2(&garden, sizex - 1, 0, sizex / 2 - 1);

    let tr_big = solve2(&garden, 0, sizey - 1, 3 * sizey / 2 - 1);
    let br_big = solve2(&garden, 0, 0, 3 * sizey / 2 - 1);
    let tl_big = solve2(&garden, sizex - 1, sizey - 1, 3 * sizey / 2 - 1);
    let bl_big = solve2(&garden, sizex - 1, 0, 3 * sizey / 2 - 1);

    let result = odd_grids * odds
        + even_grids * evens
        + (t_corner + b_corner + l_corner + r_corner)
        + (width + 1) * (tr_small + br_small + tl_small + bl_small)
        + width * (tr_big + br_big + tl_big + bl_big);
    let finish = start.elapsed();
    println!("Result Part 2: {} in {} ms", result, finish.as_millis());
}
