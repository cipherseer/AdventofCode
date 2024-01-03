use core::fmt;
use ndarray::{s, Array3};
use regex::Regex;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct BlockPosition {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    zmin: usize,
    zmax: usize,
}

#[derive(Clone)]
struct Block {
    id: Option<u16>,
}

impl Default for Block {
    fn default() -> Self {
        Block { id: None }
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            Some(id) => write!(f, "{}", id),
            None => write!(f, "."),
        }
    }
}

fn disintegrate_brick(
    s: u16,
    supports: &Vec<HashSet<u16>>,
    supported_by: &Vec<HashSet<u16>>,
) -> usize {
    let mut queue: VecDeque<u16> = VecDeque::new();
    let mut destroyed: HashSet<u16> = HashSet::new();
    let mut count: usize = 0;

    for supported in &supports[s as usize] {
        if supported_by[*supported as usize].len() == 1 {
            queue.push_back(*supported);
            destroyed.insert(*supported);
        }
    }

    while let Some(block) = queue.pop_front() {
        count += 1;
        for supported in &supports[block as usize] {
            if !destroyed.contains(&supported) {
                if supported_by[*supported as usize].is_subset(&destroyed) {
                    queue.push_back(*supported);
                    destroyed.insert(*supported);
                }
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../input.txt");
    let reg = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let mut blocks: Vec<BlockPosition> = Vec::new();

    for c in reg.captures_iter(input) {
        let (xmin, ymin, zmin, xmax, ymax, zmax) = (
            c[1].parse::<usize>().unwrap(),
            c[2].parse::<usize>().unwrap(),
            c[3].parse::<usize>().unwrap() - 1,
            c[4].parse::<usize>().unwrap(),
            c[5].parse::<usize>().unwrap(),
            c[6].parse::<usize>().unwrap() - 1,
        );
        blocks.push(BlockPosition {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        });
    }

    let xmax: usize = blocks
        .iter()
        .max_by(|a, b| a.xmax.partial_cmp(&b.xmax).unwrap())
        .unwrap()
        .xmax
        + 1;

    let ymax: usize = blocks
        .iter()
        .max_by(|a, b| a.ymax.partial_cmp(&b.ymax).unwrap())
        .unwrap()
        .ymax
        + 1;

    let zmax: usize = blocks
        .iter()
        .max_by(|a, b| a.zmax.partial_cmp(&b.zmax).unwrap())
        .unwrap()
        .zmax
        + 1;
    blocks.sort_unstable_by(|a, b| a.zmin.partial_cmp(&b.zmin).unwrap());

    let mut jenga: Array3<Block> = Array3::default((xmax, ymax, zmax));
    let mut collisions: HashSet<u16> = HashSet::new();
    let mut unique_supports: HashSet<u16> = HashSet::new();
    let mut supports: Vec<HashSet<u16>> = vec![HashSet::new(); blocks.len()];
    let mut supported_by: Vec<HashSet<u16>> = vec![HashSet::new(); blocks.len()];

    for (n, b) in blocks.iter().enumerate() {
        let zlen = b.zmax - b.zmin;
        collisions.clear();
        for k in (0..=b.zmin).rev() {
            let mut slice = jenga.slice_mut(s![b.xmin..=b.xmax, b.ymin..=b.ymax, k..=(k + zlen)]);
            for block in slice.iter() {
                if let Some(id) = block.id {
                    collisions.insert(id);
                    supports[id as usize].insert(n as u16);
                    supported_by[n as usize].insert(id);
                }
            }

            if k == 0 && collisions.is_empty() {
                slice.fill(Block { id: Some(n as u16) });
            } else if !collisions.is_empty() {
                jenga
                    .slice_mut(s![
                        b.xmin..=b.xmax,
                        b.ymin..=b.ymax,
                        (k + 1)..=(k + 1 + zlen)
                    ])
                    .fill(Block { id: Some(n as u16) });

                if collisions.len() == 1 {
                    unique_supports.insert(collisions.drain().next().unwrap());
                }
                break;
            }
        }
    }

    println!("Part 1: {}", blocks.len() - unique_supports.len());

    let mut total_count: usize = 0;
    for s in unique_supports {
        total_count += disintegrate_brick(s, &supports, &supported_by);
    }

    println!("Part 2: {total_count}");
}
