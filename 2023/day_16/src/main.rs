use core::fmt;
use std::collections::{HashSet, VecDeque};
use Direction::*;
use TileVariant::*;

#[derive(Clone)]
struct Tile {
    tile: TileVariant,
    energized: bool,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.tile, self.energized) {
            (Empty, false) => write!(f, "."),
            (HSplit, false) => write!(f, "-"),
            (VSplit, false) => write!(f, "|"),
            (LMirror, false) => write!(f, "\\"),
            (RMirror, false) => write!(f, "/"),
            (_, true) => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileVariant {
    Empty,
    HSplit,
    VSplit,
    LMirror,
    RMirror,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Stop,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct LightBeam {
    x: usize,
    y: usize,
    direction: Direction,
}

fn update_direction(direction: Direction, tile: &TileVariant) -> Direction {
    match (direction, *tile) {
        (Up | Down, HSplit) | (Up, RMirror) | (Down, LMirror) => return Right,
        (Up, LMirror) | (Down, RMirror) => return Left,
        (Left | Right, VSplit) | (Left, LMirror) | (Right, RMirror) => return Up,
        (Left, RMirror) | (Right, LMirror) => return Down,
        (Up | Down, Empty | VSplit) | (Left | Right, Empty | HSplit) => return direction,
        (Stop, _) => return Stop,
    }
}

fn lightbeam_step(
    lightbeam: &mut LightBeam,
    tiles: &mut Vec<Vec<Tile>>,
    cache: &mut HashSet<LightBeam>,
    energized_count: &mut usize,
) -> Option<LightBeam> {
    let sizey: usize = tiles.len();
    let sizex: usize = tiles[0].len();

    match lightbeam.direction {
        Up => {
            if lightbeam.y > 0 {
                lightbeam.y -= 1;
                lightbeam.direction =
                    update_direction(lightbeam.direction, &tiles[lightbeam.y][lightbeam.x].tile);

                if cache.contains(lightbeam) {
                    lightbeam.direction = Stop;
                    return None;
                }

                if tiles[lightbeam.y][lightbeam.x].energized != true {
                    tiles[lightbeam.y][lightbeam.x].energized = true;
                    *energized_count += 1;
                }
                cache.insert(*lightbeam);
                if tiles[lightbeam.y][lightbeam.x].tile == HSplit {
                    let new_lightbeam = LightBeam {
                        x: lightbeam.x,
                        y: lightbeam.y,
                        direction: Left,
                    };
                    cache.insert(new_lightbeam);
                    return Some(new_lightbeam);
                }
            } else {
                lightbeam.direction = Stop;
            }
        }
        Down => {
            if lightbeam.y < sizey - 1 {
                lightbeam.y += 1;
                lightbeam.direction =
                    update_direction(lightbeam.direction, &tiles[lightbeam.y][lightbeam.x].tile);

                if cache.contains(lightbeam) {
                    lightbeam.direction = Stop;
                    return None;
                }

                if tiles[lightbeam.y][lightbeam.x].energized != true {
                    tiles[lightbeam.y][lightbeam.x].energized = true;
                    *energized_count += 1;
                }
                cache.insert(*lightbeam);
                if tiles[lightbeam.y][lightbeam.x].tile == HSplit {
                    let new_lightbeam = LightBeam {
                        x: lightbeam.x,
                        y: lightbeam.y,
                        direction: Left,
                    };
                    cache.insert(new_lightbeam);
                    return Some(new_lightbeam);
                }
            } else {
                lightbeam.direction = Stop;
            }
        }
        Left => {
            if lightbeam.x > 0 {
                lightbeam.x -= 1;
                lightbeam.direction =
                    update_direction(lightbeam.direction, &tiles[lightbeam.y][lightbeam.x].tile);

                if cache.contains(lightbeam) {
                    lightbeam.direction = Stop;
                    return None;
                }

                if tiles[lightbeam.y][lightbeam.x].energized != true {
                    tiles[lightbeam.y][lightbeam.x].energized = true;
                    *energized_count += 1;
                }
                cache.insert(*lightbeam);
                if tiles[lightbeam.y][lightbeam.x].tile == VSplit {
                    let new_lightbeam = LightBeam {
                        x: lightbeam.x,
                        y: lightbeam.y,
                        direction: Down,
                    };
                    cache.insert(new_lightbeam);
                    return Some(new_lightbeam);
                }
            } else {
                lightbeam.direction = Stop;
            }
        }
        Right => {
            if lightbeam.x < sizex - 1 {
                lightbeam.x += 1;
                lightbeam.direction =
                    update_direction(lightbeam.direction, &tiles[lightbeam.y][lightbeam.x].tile);
                if cache.contains(lightbeam) {
                    lightbeam.direction = Stop;
                    return None;
                }

                if tiles[lightbeam.y][lightbeam.x].energized != true {
                    tiles[lightbeam.y][lightbeam.x].energized = true;
                    *energized_count += 1;
                }
                cache.insert(*lightbeam);
                if tiles[lightbeam.y][lightbeam.x].tile == VSplit {
                    let new_lightbeam = LightBeam {
                        x: lightbeam.x,
                        y: lightbeam.y,
                        direction: Down,
                    };
                    cache.insert(new_lightbeam);
                    return Some(new_lightbeam);
                }
            } else {
                lightbeam.direction = Stop;
            }
        }
        _ => {}
    }

    None
}

fn lightbeam_propagate(
    lightbeam: &mut LightBeam,
    lightbeams: &mut VecDeque<LightBeam>,
    tiles: &mut Vec<Vec<Tile>>,
    cache: &mut HashSet<LightBeam>,
    energized_count: &mut usize,
) {
    while lightbeam.direction != Stop {
        let lightbeam_option = lightbeam_step(lightbeam, tiles, cache, energized_count);
        match lightbeam_option {
            Some(new_lightbeam) => lightbeams.push_back(new_lightbeam),
            None => {}
        }
    }
}

fn lightbeams_init(
    lightbeams: &mut VecDeque<LightBeam>,
    tile_type: &TileVariant,
    cache: &mut HashSet<LightBeam>,
    x_init: usize,
    y_init: usize,
    direction_init: Direction,
) {
    //light beam enters the grid at 0,0 going Right
    match (tile_type, direction_init) {
        (LMirror, Right) | (RMirror, Left) | (VSplit | Empty, Down) => {
            let lightbeam = LightBeam {
                x: x_init,
                y: y_init,
                direction: Down,
            };
            lightbeams.push_back(lightbeam);
            cache.insert(lightbeam);
        }
        (RMirror, Right) | (LMirror, Left) | (VSplit | Empty, Up) => {
            let lightbeam = LightBeam {
                x: x_init,
                y: y_init,
                direction: Up,
            };
            lightbeams.push_back(lightbeam);
            cache.insert(lightbeam);
        }
        (LMirror, Down) | (RMirror, Up) | (HSplit | Empty, Right) => {
            let lightbeam = LightBeam {
                x: x_init,
                y: y_init,
                direction: Right,
            };
            lightbeams.push_back(lightbeam);
            cache.insert(lightbeam);
        }
        (LMirror, Up) | (RMirror, Down) | (HSplit | Empty, Left) => {
            let lightbeam = LightBeam {
                x: x_init,
                y: y_init,
                direction: Left,
            };
            lightbeams.push_back(lightbeam);
            cache.insert(lightbeam);
        }
        (VSplit, Left | Right) => {
            let lightbeam1 = LightBeam {
                x: x_init,
                y: y_init,
                direction: Up,
            };
            let lightbeam2 = LightBeam {
                x: x_init,
                y: y_init,
                direction: Down,
            };
            lightbeams.push_back(lightbeam1);
            cache.insert(lightbeam1);
            lightbeams.push_back(lightbeam2);
            cache.insert(lightbeam2);
        }
        (HSplit, Up | Down) => {
            let lightbeam1 = LightBeam {
                x: x_init,
                y: y_init,
                direction: Left,
            };
            let lightbeam2 = LightBeam {
                x: x_init,
                y: y_init,
                direction: Right,
            };
            lightbeams.push_back(lightbeam1);
            cache.insert(lightbeam1);
            lightbeams.push_back(lightbeam2);
            cache.insert(lightbeam2);
        }
        (_, Stop) => {}
    }
}

fn tiles_init(tiles: &mut Vec<Vec<Tile>>, input: &str) {
    for (j, line) in input.lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '-' => {
                    tiles[j][i] = Tile {
                        tile: HSplit,
                        energized: false,
                    }
                }
                '|' => {
                    tiles[j][i] = Tile {
                        tile: VSplit,
                        energized: false,
                    }
                }
                '\\' => {
                    tiles[j][i] = Tile {
                        tile: LMirror,
                        energized: false,
                    }
                }
                '/' => {
                    tiles[j][i] = Tile {
                        tile: RMirror,
                        energized: false,
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let sizey: usize = input.lines().count();
    let sizex: usize = input.lines().next().unwrap().len();

    let mut tiles1: Vec<Vec<Tile>> = vec![
        vec![
            Tile {
                tile: Empty,
                energized: false
            };
            sizex
        ];
        sizey
    ];

    tiles_init(&mut tiles1, &input);
    let tiles2 = tiles1.clone();
    {
        let mut lightbeams: VecDeque<LightBeam> = VecDeque::new();
        let mut cache: HashSet<LightBeam> = HashSet::new();

        lightbeams_init(&mut lightbeams, &tiles1[0][0].tile, &mut cache, 0, 0, Right);
        tiles1[0][0].energized = true;
        let mut energized_count: usize = 1;

        while let Some(mut lightbeam) = lightbeams.pop_front() {
            lightbeam_propagate(
                &mut lightbeam,
                &mut lightbeams,
                &mut tiles1,
                &mut cache,
                &mut energized_count,
            );
        }


        println!("Total energized tiles- Part 1: {energized_count}");
    }

    let mut max_count: usize = 0;

    //Check initial conditions going Right
    for y_init in 0..sizey {
        let mut tiles3 = tiles2.clone();
        let mut lightbeams: VecDeque<LightBeam> = VecDeque::new();
        let mut cache: HashSet<LightBeam> = HashSet::new();

        lightbeams_init(
            &mut lightbeams,
            &tiles3[y_init][0].tile,
            &mut cache,
            0,
            y_init,
            Right,
        );
        tiles3[y_init][0].energized = true;
        let mut energized_count: usize = 1;

        while let Some(mut lightbeam) = lightbeams.pop_front() {
            lightbeam_propagate(
                &mut lightbeam,
                &mut lightbeams,
                &mut tiles3,
                &mut cache,
                &mut energized_count,
            );
        }

        if energized_count > max_count {
            max_count = energized_count;
        }
    }
    //Check initial conditions going Down
    for x_init in 0..sizex {
        let mut tiles3 = tiles2.clone();
        let mut lightbeams: VecDeque<LightBeam> = VecDeque::new();
        let mut cache: HashSet<LightBeam> = HashSet::new();

        lightbeams_init(
            &mut lightbeams,
            &tiles3[0][x_init].tile,
            &mut cache,
            x_init,
            0,
            Down,
        );
        tiles3[0][x_init].energized = true;
        let mut energized_count: usize = 1;

        while let Some(mut lightbeam) = lightbeams.pop_front() {
            lightbeam_propagate(
                &mut lightbeam,
                &mut lightbeams,
                &mut tiles3,
                &mut cache,
                &mut energized_count,
            );
        }

        if energized_count > max_count {
            max_count = energized_count;
        }
    }
    //Check initial conditions going Up
    for x_init in 0..sizex {
        let mut tiles3 = tiles2.clone();
        let mut lightbeams: VecDeque<LightBeam> = VecDeque::new();
        let mut cache: HashSet<LightBeam> = HashSet::new();

        lightbeams_init(
            &mut lightbeams,
            &tiles3[sizey - 1][x_init].tile,
            &mut cache,
            x_init,
            sizey - 1,
            Up,
        );
        tiles3[sizey - 1][x_init].energized = true;
        let mut energized_count: usize = 1;

        while let Some(mut lightbeam) = lightbeams.pop_front() {
            lightbeam_propagate(
                &mut lightbeam,
                &mut lightbeams,
                &mut tiles3,
                &mut cache,
                &mut energized_count,
            );
        }

        if energized_count > max_count {
            max_count = energized_count;
        }
    }

    //Check initial conditions going Left
    for y_init in 0..sizey {
        let mut tiles3 = tiles2.clone();
        let mut lightbeams: VecDeque<LightBeam> = VecDeque::new();
        let mut cache: HashSet<LightBeam> = HashSet::new();

        lightbeams_init(
            &mut lightbeams,
            &tiles3[y_init][sizex - 1].tile,
            &mut cache,
            sizex - 1,
            y_init,
            Left,
        );
        tiles3[y_init][sizex - 1].energized = true;
        let mut energized_count: usize = 1;

        while let Some(mut lightbeam) = lightbeams.pop_front() {
            lightbeam_propagate(
                &mut lightbeam,
                &mut lightbeams,
                &mut tiles3,
                &mut cache,
                &mut energized_count,
            );
        }

        if energized_count > max_count {
            max_count = energized_count;
        }
    }

    println!("Maximum energized tiles- Part 2: {max_count}");
}
