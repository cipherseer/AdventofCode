use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let size = input.lines().count();

    let mut universe: Vec<Vec<bool>> = vec![vec![false; size]; size];
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                universe[i][j] = true;
            }
        }
    }

    let rows_to_insert: Vec<i64> = universe
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|&e| e == false) {
                Some(i as i64)
            } else {
                None
            }
        })
        .collect();


    let cols_to_insert: Vec<i64> = (0..universe[0].len())
        .filter_map(|j| {
            if universe.iter().all(|row| row[j] == false) {
                Some(j as i64)
            } else {
                None
            }
        })
        .collect();


    let mut chart: HashMap<usize, (i64, i64)> = HashMap::new();
    let mut galaxy_number: usize = 1;
    for (i, row) in universe.iter().enumerate() {
        for (j, &element) in row.iter().enumerate() {
            if element {
                chart.insert(galaxy_number, (i as i64, j as i64));
                galaxy_number += 1;
            }
        }
    }

    let mut total_distance: i64 = 0;
    let mut total_distance2: i64 = 0;
    for i in 1..galaxy_number - 1 {
        for j in (i + 1)..galaxy_number {
            let (x1, y1) = chart[&i];
            let (x2, y2) = chart[&j];
            let mut countx: i64 = 0;
            let mut county: i64 = 0;
            for row in rows_to_insert.iter() {
                if (*row > x1 && *row < x2) || (*row < x1 && *row > x2) {
                    countx += 1;
                }
            }

            for col in cols_to_insert.iter() {
                if (*col > y1 && *col < y2) || (*col < y1 && *col > y2) {
                    county += 1;
                }
            }

            let distance = (x2 - x1).abs() + (y2 - y1).abs() + 1 * countx + 1 * county;
            let distance2 =
                (x2 - x1).abs() + (y2 - y1).abs() + (1000000 - 1) * countx + (1000000 - 1) * county;
            total_distance += distance;
            total_distance2 += distance2;
        }
    }

    println!("Summed distance between galaxies: {total_distance}");
    println!("Summed distance between galaxies part 2: {total_distance2}");
}
