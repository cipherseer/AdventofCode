use ndarray::prelude::*;
use std::cmp::min;

fn parse_input(input: &str) -> Array2<char> {
    let rows: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    Array2::from_shape_fn((rows.len(), rows[0].len()), |(i, j)| rows[i][j])
}

fn main() {
    let input = include_str!("../input.txt");

    let mut num_rows1: usize = 0;
    let mut num_cols1: usize = 0;
    let mut num_rows2: usize = 0;
    let mut num_cols2: usize = 0;
    let blocks = input.split("\n\n");
    for block in blocks {
        let array = parse_input(block);
        let sizey = array.nrows();
        let sizex = array.ncols();

        //Check for mirrored rows
        for i in 0..sizey - 1 {
            let minimum = min(i, (sizey - 1) - (i + 1));

            let block1 = array.slice(s![(i - minimum)..=i, 0..sizex]);
            let block2 = array.slice(s![(i+1)..=(i+1+minimum); -1, 0..sizex]);
            let mut diff_count = 0;
            for (a, b) in block1.iter().zip(block2.iter()) {
                if a != b {
                    diff_count += 1;
                    if diff_count > 1 {
                        break;
                    }
                }
            }

            if diff_count == 1 {
                num_rows2 += i + 1;
            } else if diff_count == 0 {
                num_rows1 += i + 1;
            }
        }

        //Check for mirrored columns
        for j in 0..sizex - 1 {
            let minimum = min(j, (sizex - 1) - (j + 1));

            let block1 = array.slice(s![0..sizey, (j - minimum)..=j]);
            let block2 = array.slice(s![0..sizey, (j+1)..=(j+1+minimum); -1]);
            let mut diff_count = 0;
            for (a, b) in block1.iter().zip(block2.iter()) {
                if a != b {
                    diff_count += 1;
                    if diff_count > 1 {
                        break;
                    }
                }
            }

            if diff_count == 1 {
                num_cols2 += j + 1;
            } else if diff_count == 0 {
                num_cols1 += j + 1;
            }
        }
    }
    println!("Solution Part 1: {}", num_cols1 + 100 * num_rows1);
    println!("Solution Part 2: {}", num_cols2 + 100 * num_rows2);
}
