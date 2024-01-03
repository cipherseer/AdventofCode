use ndarray::{arr1, arr2};
use ndarray_linalg::Solve;
use std::collections::HashMap;

#[derive(Debug)]
struct Trajectory {
    r: Vec<i64>,
    v: Vec<i64>,
}

enum IntersectionError {
    Past(&'static str),
    OutOfBounds((f64, f64)),
}

use IntersectionError::*;

fn intersection(
    a: &Trajectory,
    b: &Trajectory,
    min_bound: f64,
    max_bound: f64,
) -> Result<(f64, f64), IntersectionError> {
    let denominator = a.v[0] * b.v[1] - a.v[1] * b.v[0];

    if denominator == 0 {
        return Err(Past("paths are parallel"));
    }

    let numerator_a = b.v[0] * (a.r[1] - b.r[1]) - b.v[1] * (a.r[0] - b.r[0]);
    let numerator_b = a.v[0] * (a.r[1] - b.r[1]) - a.v[1] * (a.r[0] - b.r[0]);

    match (denominator > 0, numerator_a > 0, numerator_b > 0) {
        (true, false, true) | (false, true, false) => {
            return Err(Past("paths crossed in the past of hailstone A"));
        }
        (true, true, false) | (false, false, true) => {
            return Err(Past("paths crossed in the past of hailstone B"));
        }
        (true, false, false) | (false, true, true) => {
            return Err(Past("paths crossed in the past of both hailstones"));
        }
        (true, true, true) | (false, false, false) => {
            let time_a: f64 = numerator_a as f64 / denominator as f64;
            let x_a: f64 = a.r[0] as f64 + time_a * a.v[0] as f64;
            let y_a: f64 = a.r[1] as f64 + time_a * a.v[1] as f64;
            if x_a >= min_bound && x_a <= max_bound && y_a >= min_bound && y_a <= max_bound {
                return Ok((x_a, y_a));
            } else {
                return Err(OutOfBounds((x_a, y_a)));
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut t: Vec<Trajectory> = Vec::new();
    for line in input.lines() {
        let (position_str, velocity_str) = line.split_once(" @ ").unwrap();

        let position: Vec<_> = position_str
            .split(", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        let velocity: Vec<_> = velocity_str
            .split(", ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        t.push(Trajectory {
            r: position,
            v: velocity,
        });
    }

    let mut count: usize = 0;
    for (i, t1) in t.iter().take(t.len() - 1).enumerate() {
        for t2 in t.iter().skip(i + 1) {
            println!("Hailstone A: {:?}", t1);
            println!("Hailstone B: {:?}", t2);
            match intersection(&t1, &t2, 200000000000000.0, 400000000000000.0) {
                Ok((x, y)) => {
                    println!("Paths will cross inside the test area at (x={x}, y={y})");
                    count += 1;
                }
                Err(OutOfBounds((x, y))) => {
                    println!("Paths will cross outside the test area at (x={x}, y={y})");
                }
                Err(Past(err)) => println!("{}", err),
            }
            println!();
        }
    }

    println!("Total valid intersections: {count}");


    //unfortunately f64 isn't good enough precision to find the correct solution
    //so I used python instead
    let i: usize = 0;
   // for i in 0..t.len() - 2 {
        let x0 = t[i+0].r[0] as f64;
        let x1 = t[i+1].r[0] as f64;
        let x2 = t[i+2].r[0] as f64;
        let y0 = t[i+0].r[1] as f64;
        let y1 = t[i+1].r[1] as f64;
        let y2 = t[i+2].r[1] as f64;
        let z0 = t[i+0].r[2] as f64;
        let z1 = t[i+1].r[2] as f64;
        let z2 = t[i+1].r[2] as f64;
        let vx0 = t[i+0].v[0] as f64;
        let vx1 = t[i+1].v[0] as f64;
        let vx2 = t[i+2].v[0] as f64;
        let vy0 = t[i+0].v[1] as f64;
        let vy1 = t[i+1].v[1] as f64;
        let vy2 = t[i+2].v[1] as f64;
        let vz0 = t[i+0].v[2] as f64;
        let vz1 = t[i+1].v[2] as f64;
        let vz2 = t[i+2].v[2] as f64;
        
        
        let a = arr2(&[
        [vy0-vy1, vx1-vx0,     0.0, y1-y0, x0-x1,   0.0],
        [vy0-vy2, vx2-vx0,     0.0, y2-y0, x0-x2,   0.0],
        [vz0-vz1,     0.0, vx1-vx0, z1-z0,   0.0, x0-x1],
        [vz0-vz2,     0.0, vx2-vx0, z2-z0,   0.0, x0-x2],
        [    0.0, vz0-vz1, vy1-vy0,   0.0, z1-z0, y0-y1],
        [    0.0, vz0-vz2, vy2-vy0,   0.0, z2-z0, y0-y2],
    ]);
        let b = arr1(&[
            x0*vy0-y0*vx0-x1*vy1+y1*vx1,
            x0*vy0-y0*vx0-x2*vy2+y2*vx2,
            x0*vz0-z0*vx0-x1*vz1+z1*vx1,
            x0*vz0-z0*vx0-x2*vz2+z2*vx2,
            y0*vz0-z0*vy0-y1*vz1+z1*vy1,
            y0*vz0-z0*vy0-y2*vz2+z2*vy2,
    ]);
                    

        match a.solve(&b) {
            Ok(solution) => {
                println!("f64 insufficient to find solution - use python script for part 2 Sum: {}", 
                solution[0] + solution[1] + solution[2]);
            }
            Err(err) => {
                println!("Error: {err}");
            }
        }
    
}
