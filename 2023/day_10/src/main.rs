use ndarray::Array2;

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Stop,
}

#[derive(Clone)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
    pipe: u8,
}

fn hero_init(hero: &mut State, terrain: &Array2<u8>) {
    //find starting direction
    //always want a position and direction
    let sizex = terrain.len_of(ndarray::Axis(0));
    let sizey = terrain.len_of(ndarray::Axis(1));

    //Check North
    if hero.y > 0 {
        let t = terrain[[hero.x, hero.y - 1]];
        match t as char {
            '|' => {
                hero.direction = Direction::North;
                hero.y -= 1;
                hero.pipe = t;
                return;
            }
            '7' => {
                hero.direction = Direction::West;
                hero.y -= 1;
                hero.pipe = t;
                return;
            }
            'F' => {
                hero.direction = Direction::East;
                hero.y -= 1;
                hero.pipe = t;
                return;
            }
            _ => {}
        }
    }
    //Check South
    if hero.y < sizey - 1 {
        let t = terrain[[hero.x, hero.y + 1]];
        match t as char {
            '|' => {
                hero.direction = Direction::South;
                hero.y += 1;
                hero.pipe = t;
                return;
            }
            'L' => {
                hero.direction = Direction::East;
                hero.y += 1;
                hero.pipe = t;
                return;
            }
            'J' => {
                hero.direction = Direction::West;
                hero.y += 1;
                hero.pipe = t;
                return;
            }
            _ => {}
        }
    }

    //Check West
    if hero.x > 0 {
        let t = terrain[[hero.x - 1, hero.y]];
        match t as char {
            '-' => {
                hero.direction = Direction::West;
                hero.x -= 1;
                hero.pipe = t;
                return;
            }
            'L' => {
                hero.direction = Direction::North;
                hero.x -= 1;
                hero.pipe = t;
                return;
            }
            'F' => {
                hero.direction = Direction::South;
                hero.x -= 1;
                hero.pipe = t;
                return;
            }
            _ => {}
        }
    }

    //Check East
    if hero.x < sizex - 1 {
        let t = terrain[[hero.x + 1, hero.y]];
        match t as char {
            '-' => {
                hero.direction = Direction::East;
                hero.x += 1;
                hero.pipe = t;
                return;
            }
            'J' => {
                hero.direction = Direction::North;
                hero.x += 1;
                hero.pipe = t;
                return;
            }
            '7' => {
                hero.direction = Direction::South;
                hero.x += 1;
                hero.pipe = t;
                return;
            }
            _ => {}
        }
    }
}

fn calculate_direction(prev_direction: &Direction, direction: u8) -> Direction {
    match prev_direction {
        Direction::North => match direction as char {
            '|' => return Direction::North,
            '7' => return Direction::West,
            'F' => return Direction::East,
            _ => {}
        },
        Direction::South => match direction as char {
            '|' => return Direction::South,
            'L' => return Direction::East,
            'J' => return Direction::West,
            _ => {}
        },
        Direction::West => match direction as char {
            '-' => return Direction::West,
            'L' => return Direction::North,
            'F' => return Direction::South,
            _ => {}
        },
        Direction::East => match direction as char {
            '-' => return Direction::East,
            'J' => return Direction::North,
            '7' => return Direction::South,
            _ => {}
        },
        Direction::Stop => {}
    }

    return Direction::Stop;
}

fn adjust_positions(direction: &Direction, pipe: &u8, x: &mut f64, y: &mut f64) {
    match *direction {
        Direction::North => match *pipe as char {
            '|' => {
                *x -= 0.5;
            }
            'L' => {
                *x -= 0.5;
                *y += 0.5;
            }
            'J' => {
                *x -= 0.5;
                *y -= 0.5;
            }
            _ => {}
        },
        Direction::South => match *pipe as char {
            '|' => {
                *x += 0.5;
            }
            '7' => {
                *x += 0.5;
                *y -= 0.5;
            }
            'F' => {
                *x += 0.5;
                *y += 0.5;
            }
            _ => {}
        },
        Direction::West => match *pipe as char {
            '-' => {
                *y += 0.5;
            }
            'J' => {
                *x += 0.5;
                *y += 0.5;
            }
            '7' => {
                *x -= 0.5;
                *y += 0.5;
            }
            _ => {}
        },
        Direction::East => match *pipe as char {
            '-' => {
                *y -= 0.5;
            }
            'L' => {
                *x += 0.5;
                *y -= 0.5;
            }
            'F' => {
                *x -= 0.5;
                *y -= 0.5;
            }
            _ => {}
        },
        Direction::Stop => {}
    }
}

fn calculate_area(hero: &State, herop: &State) -> f64 {
    let mut x = hero.x as f64;
    let mut xp = herop.x as f64;
    let mut y = hero.y as f64;
    let mut yp = herop.y as f64;

    adjust_positions(&herop.direction, &herop.pipe, &mut xp, &mut yp);
    adjust_positions(&hero.direction, &hero.pipe, &mut x, &mut y);

    xp * y - x * yp
}

fn take_step(hero: &mut State, terrain: &Array2<u8>) {
    match hero.direction {
        Direction::North => {
            hero.y -= 1;
            hero.direction = calculate_direction(&hero.direction, terrain[[hero.x, hero.y]]);
            hero.pipe = terrain[[hero.x, hero.y]];
        }
        Direction::South => {
            hero.y += 1;
            hero.direction = calculate_direction(&hero.direction, terrain[[hero.x, hero.y]]);
            hero.pipe = terrain[[hero.x, hero.y]];
        }
        Direction::West => {
            hero.x -= 1;
            hero.direction = calculate_direction(&hero.direction, terrain[[hero.x, hero.y]]);
            hero.pipe = terrain[[hero.x, hero.y]];
        }
        Direction::East => {
            hero.x += 1;
            hero.direction = calculate_direction(&hero.direction, terrain[[hero.x, hero.y]]);
            hero.pipe = terrain[[hero.x, hero.y]];
        }
        Direction::Stop => {}
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let sizey = input.lines().count();
    let sizex = input.lines().next().unwrap().len();
    let mut terrain: Array2<u8> = Array2::<u8>::zeros((sizex, sizey));

    let mut hero: State = State {
        x: 0,
        y: 0,
        direction: Direction::North,
        pipe: 'S' as u8,
    };
    for (j, line) in input.lines().enumerate() {
        for (i, ch) in line.bytes().enumerate() {
            terrain[[i, j]] = ch;
            if ch as char == 'S' {
                hero.x = i;
                hero.y = j;
            }
        }
    }

    hero_init(&mut hero, &terrain);
    let mut steps: usize = 1;
    let mut total_area: f64 = 0.0;
    let mut hero_prev = hero.clone();
    let hero_start = hero.clone();

    //traverse the pipe
    loop {
        take_step(&mut hero, &terrain);
        steps += 1;
        if hero.direction == Direction::Stop {
            break;
        }
        total_area += calculate_area(&hero, &hero_prev);

        hero_prev = hero.clone();
    }

    hero.direction = hero_start.direction.clone();

    match hero_prev.direction {
        Direction::North => match hero.direction {
            Direction::North => hero.pipe = '|' as u8,
            Direction::East => hero.pipe = 'F' as u8,
            Direction::West => hero.pipe = '7' as u8,
            _ => {}
        },
        Direction::South => match hero.direction {
            Direction::South => hero.pipe = '|' as u8,
            Direction::East => hero.pipe = 'L' as u8,
            Direction::West => hero.pipe = 'J' as u8,
            _ => {}
        },
        Direction::West => match hero.direction {
            Direction::West => hero.pipe = '-' as u8,
            Direction::North => hero.pipe = 'L' as u8,
            Direction::South => hero.pipe = 'F' as u8,
            _ => {}
        },
        Direction::East => match hero.direction {
            Direction::East => hero.pipe = '-' as u8,
            Direction::North => hero.pipe = 'J' as u8,
            Direction::South => hero.pipe = '7' as u8,
            _ => {}
        },
        Direction::Stop => {}
    }

    total_area += calculate_area(&hero, &hero_prev);
    total_area += calculate_area(&hero_start, &hero);

    println!("Steps: {}  Area: {}", steps / 2, total_area.abs() / 2.0);
}
