use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum ModuleVariant {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<&'static str, bool>),
}

use ModuleVariant::*;

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleVariant,
    targets: Vec<&'static str>,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let input = include_str!("../input.txt");
    let mut modules: HashMap<&str, Module> = HashMap::new();
    for line in input.lines() {
        let (mut name, list) = line.split_once(" -> ").unwrap();
        let destinations = list.split(", ");

        match name.chars().nth(0).unwrap() {
            'b' => modules.insert(
                name,
                Module {
                    module_type: Broadcaster,
                    targets: Vec::new(),
                },
            ),
            '%' => {
                name = &name[1..];
                modules.insert(
                    name,
                    Module {
                        module_type: FlipFlop(false),
                        targets: Vec::new(),
                    },
                )
            }
            '&' => {
                name = &name[1..];
                modules.insert(
                    name,
                    Module {
                        module_type: Conjunction(HashMap::new()),
                        targets: Vec::new(),
                    },
                )
            }
            _ => unreachable!(),
        };
        for destination in destinations {
            modules.get_mut(name).unwrap().targets.push(destination);
        }
    }
    //Initialize Conjunction modules (need to know if there are 1 or 2 inputs)
    let mut conjunction_init: Vec<(&str, &str)> = Vec::new();
    let mut rx_parent: &str = "";
    for (name, module) in &modules {
        for target in &module.targets {
            if *target == "rx" {
                rx_parent = name;
            }
            if let Some(m) = modules.get(target) {
                match m.module_type {
                    Conjunction(_) => {
                        conjunction_init.push((target, name));
                    }
                    _ => {}
                }
            }
        }
    }

    for conjunction in conjunction_init {
        match &mut modules.get_mut(conjunction.0).unwrap().module_type {
            Conjunction(memory) => {
                memory.insert(conjunction.1, false);
            }
            _ => {}
        }
    }

    let mut pulse_queue: VecDeque<(&str, bool, &str)> = VecDeque::new();
    let mut low_pulses: usize = 0;
    let mut high_pulses: usize = 0;

    //track all modules that send signals to rx
    let mut rx_parent_parent: HashMap<&str, (usize, usize, usize)> = HashMap::new();
    for (name, module) in &modules {
        for target in &module.targets {
            if *target == rx_parent {
                rx_parent_parent.insert(name, (0, 0, 0));
            }
        }
    }

    let mut button_presses: usize = 0;
    let mut min_rx_pressed: usize = 0;

    'outer: loop {
        button_presses += 1;
        pulse_queue.push_back(("button", false, "broadcaster"));
        while let Some((sender, pulse, destination)) = pulse_queue.pop_front() {
            //Logic for part 1
            if pulse && button_presses <= 1000 {
                high_pulses += 1;
            } else if !pulse && button_presses <= 1000 {
                low_pulses += 1;
            }

            //Logic for part 2
            if destination == rx_parent && pulse {
                rx_parent_parent
                    .entry(sender)
                    .and_modify(|(count, first_high, cycle_length)| {
                        *count += 1;

                        if *count == 1 {
                            *first_high = button_presses;
                        } else if *count == 2 {
                            *cycle_length = button_presses - *first_high;
                            println!("detected a cycle of {} for {sender}", *cycle_length);
                        }
                    });

                if rx_parent_parent.values().all(|(count, _, _)| *count >= 2) {
                    min_rx_pressed = rx_parent_parent
                        .values()
                        .fold(1, |acc, (_, _, x)| lcm(acc, *x));
                    break 'outer;
                }
            }
            if let Some(module) = modules.get_mut(destination) {
                match &mut module.module_type {
                    FlipFlop(switch) => {
                        if !pulse {
                            *switch = !*switch;
                            for target in &module.targets {
                                pulse_queue.push_back((destination, *switch, target));
                            }
                        }
                    }
                    Conjunction(memory) => {
                        memory.entry(sender).and_modify(|b| *b = pulse);

                        if memory.values().all(|&b| b) {
                            for target in &module.targets {
                                pulse_queue.push_back((destination, false, target));
                            }
                        } else {
                            for target in &module.targets {
                                pulse_queue.push_back((destination, true, target));
                            }
                        }
                    }
                    Broadcaster => {
                        for target in &module.targets {
                            pulse_queue.push_back((destination, false, target));
                        }
                    }
                }
            }
        }
    }

    println!(
        "Answer to Part 1: low pulses: {low_pulses} high pulses: {high_pulses} result: {}",
        low_pulses * high_pulses
    );
    println!("Answer to Part 2: {}", min_rx_pressed);
}
