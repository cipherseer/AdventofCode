use regex::Regex;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
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

#[derive(Debug)]
struct Instruction {
    dir1: Direction,
    dis1: isize,
    dir2: Direction,
    dis2: isize,
}

fn main() {
    let input = include_str!("../input.txt");
    let reg = Regex::new(r"([UDLR]) ([0-9]+) \(#([a-f0-9]{5})([0-3])\)").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();

    for (_, [dir1, dis1, dis2, dir2]) in reg.captures_iter(input).map(|c| c.extract()) {
        let direction1 = match dir1 {
            "U" => Up,
            "D" => Down,
            "R" => Right,
            "L" => Left,
            _ => Stop,
        };

        let distance1 = dis1.parse::<isize>().unwrap();

        let direction2 = match dir2 {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
            _ => Stop,
        };

        let distance2 = isize::from_str_radix(dis2, 16).unwrap();

        instructions.push(Instruction {
            dir1: direction1,
            dis1: distance1,
            dir2: direction2,
            dis2: distance2,
        });
    }

    let mut position1: (isize, isize) = (0, 0);
    let mut vertices1: Vec<(isize, isize)> = Vec::new();
    let mut boundary1: isize = 0;
    let mut position2: (isize, isize) = (0, 0);
    let mut vertices2: Vec<(isize, isize)> = Vec::new();
    let mut boundary2: isize = 0;
    for instruction in instructions {
        let (dx1, dy1) = instruction.dir1.get_deltas();
        let (dx2, dy2) = instruction.dir2.get_deltas();
        vertices1.push(position1);
        vertices2.push(position2);
        let new_position1 = (
            position1.0 + dx1 * instruction.dis1,
            position1.1 + dy1 * instruction.dis1,
        );
        let new_position2 = (
            position2.0 + dx2 * instruction.dis2,
            position2.1 + dy2 * instruction.dis2,
        );
        boundary1 += instruction.dis1;
        position1 = new_position1;
        boundary2 += instruction.dis2;
        position2 = new_position2;
    }

    let total_area1 = (1..vertices1.len())
        .map(|i| vertices1[i - 1].0 * vertices1[i].1 - vertices1[i].0 * vertices1[i - 1].1)
        .sum::<isize>()
        .abs()
        / 2;
    let interior1 = total_area1 - boundary1 / 2 + 1;

    let total_area2 = (1..vertices2.len())
        .map(|i| vertices2[i - 1].0 * vertices2[i].1 - vertices2[i].0 * vertices2[i - 1].1)
        .sum::<isize>()
        .abs()
        / 2;
    let interior2 = total_area2 - boundary2 / 2 + 1;

    println!(
        "Part 1 - total blocks: {} boundary: {boundary1}",
        interior1 + boundary1
    );

    println!(
        "Part 2 - total blocks: {} boundary: {boundary2}",
        interior2 + boundary2
    );
}
