use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

use Category::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Greater,
    Lesser,
}

use Operator::*;

#[derive(Debug, Copy, Clone)]
struct Rule {
    cat: Category,
    op: Operator,
    val: u64,
}

#[derive(Debug, Clone)]
struct Sorter {
    rule: Option<Rule>,
    dest: String,
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn sum_parts(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn extract_value(part: &Part, cat: &Category) -> u64 {
    match cat {
        X => part.x,
        M => part.m,
        A => part.a,
        S => part.s,
    }
}

fn follows_rule(part_value: u64, op: &Operator, rule_value: &u64) -> bool {
    match *op {
        Greater => part_value >= *rule_value,
        Lesser => part_value <= *rule_value,
    }
}

fn process<'a>(part: &'a Part, sorters: &'a Vec<Sorter>) -> Option<&'a str> {
    for sorter in sorters {
        match sorter.rule {
            Some(rule) => {
                let value = extract_value(part, &rule.cat);

                if follows_rule(value, &rule.op, &rule.val) {
                    return Some(&sorter.dest);
                }
            }
            None => {
                return Some(&sorter.dest);
            }
        }
    }
    None
}

fn enumerate_possibilities(
    workflows: &HashMap<&str, Vec<Sorter>>,
    mut lpart: Part,
    mut upart: Part,
    dest: &str,
    result: &mut u64,
) {

    if dest == "A" {
        let val = (upart.x - lpart.x + 1)
            * (upart.m - lpart.m + 1)
            * (upart.a - lpart.a + 1)
            * (upart.s - lpart.s + 1);
        *result += val;
        return;
    } else if dest == "R" {
        return;
    }
    let sorters: Vec<Sorter> = (*workflows[dest]).to_vec();

    for sorter in sorters {
        match sorter.rule {
            Some(rule) => match rule.cat {
                X => {
                    if rule.op == Greater {
                        if upart.x > rule.val {
                            let mut new_lpart = lpart.clone();
                            new_lpart.x = rule.val + 1;
                            enumerate_possibilities(
                                workflows,
                                new_lpart,
                                upart.clone(),
                                &sorter.dest,
                                result,
                            );
                        }
                        upart.x = rule.val;
                    } else {
                        if lpart.x < rule.val {
                            let mut new_upart = upart.clone();
                            new_upart.x = rule.val - 1;
                            enumerate_possibilities(
                                workflows,
                                lpart.clone(),
                                new_upart,
                                &sorter.dest,
                                result,
                            );
                        }
                        lpart.x = rule.val;
                    }
                }
                M => {
                    if rule.op == Greater {
                        if upart.m > rule.val {
                            let mut new_lpart = lpart.clone();
                            new_lpart.m = rule.val + 1;
                            enumerate_possibilities(
                                workflows,
                                new_lpart,
                                upart.clone(),
                                &sorter.dest,
                                result,
                            );
                        }
                        upart.m = rule.val;
                    } else {
                        if lpart.m < rule.val {
                            let mut new_upart = upart.clone();
                            new_upart.m = rule.val - 1;
                            enumerate_possibilities(
                                workflows,
                                lpart.clone(),
                                new_upart,
                                &sorter.dest,
                                result,
                            );
                        }
                        lpart.m = rule.val;
                    }
                }
                A => {
                    if rule.op == Greater {
                        if upart.a > rule.val {
                            let mut new_lpart = lpart.clone();
                            new_lpart.a = rule.val + 1;
                            enumerate_possibilities(
                                workflows,
                                new_lpart,
                                upart.clone(),
                                &sorter.dest,
                                result,
                            );
                        }
                        upart.a = rule.val;
                    } else {
                        if lpart.a < rule.val {
                            let mut new_upart = upart.clone();
                            new_upart.a = rule.val - 1;
                            enumerate_possibilities(
                                workflows,
                                lpart.clone(),
                                new_upart,
                                &sorter.dest,
                                result,
                            );
                        }
                        lpart.a = rule.val;
                    }
                }
                S => {
                    if rule.op == Greater {
                        if upart.s > rule.val {
                            let mut new_lpart = lpart.clone();
                            new_lpart.s = rule.val + 1;
                            enumerate_possibilities(
                                workflows,
                                new_lpart,
                                upart.clone(),
                                &sorter.dest,
                                result,
                            );
                        }
                        upart.s = rule.val;
                    } else {
                        if lpart.s < rule.val {
                            let mut new_upart = upart.clone();
                            new_upart.s = rule.val - 1;
                            enumerate_possibilities(
                                workflows,
                                lpart.clone(),
                                new_upart,
                                &sorter.dest,
                                result,
                            );
                        }
                        lpart.s = rule.val;
                    }
                }
            },
            None => {
                enumerate_possibilities(
                    workflows,
                    lpart.clone(),
                    upart.clone(),
                    &sorter.dest,
                    result,
                );
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let sorter_regex = Regex::new(r"(?:([xmas])([<>])([0-9]+):)?([a-zAR]+)").unwrap();
    let parts_regex = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();

    let (sorters, parts) = input.split_once("\n\n").unwrap();


    let mut workflows: HashMap<&str, Vec<Sorter>> = HashMap::new();

    for line in sorters.lines() {
        let mut elements = line.split_terminator(&['{', ',', '}']);
        let name = elements.next().unwrap();
        let mut sorters: Vec<Sorter> = Vec::new();
        for element in elements {
            if let Some(captures) = sorter_regex.captures(element) {
                let dest: String = captures[4].to_string();

                let cat: Option<Category> = captures.get(1).map(|c| match c.as_str() {
                    "x" => X,
                    "m" => M,
                    "a" => A,
                    "s" => S,
                    _ => unreachable!(),
                });

                let op: Option<Operator> = captures.get(2).map(|o| match o.as_str() {
                    ">" => Greater,
                    "<" => Lesser,
                    _ => unreachable!(),
                });

                let val: Option<u64> = captures.get(3).map(|v| v.as_str().parse::<u64>().unwrap());

                let rule_option = match cat {
                    Some(c) => {
                        let rule = Rule {
                            cat: c,
                            op: op.unwrap(),
                            val: val.unwrap(),
                        };
                        Some(rule)
                    }
                    None => None,
                };

                let sorter = Sorter {
                    rule: rule_option,
                    dest,
                };


                sorters.push(sorter);
            }
        }

        workflows.insert(name, sorters);
    }

    let mut parts_list: Vec<Part> = Vec::new();
    for line in parts.lines() {
        if let Some(captures) = parts_regex.captures(line) {
            let x: u64 = captures
                .get(1)
                .map(|v| v.as_str().parse::<u64>().unwrap())
                .unwrap();

            let m: u64 = captures
                .get(2)
                .map(|v| v.as_str().parse::<u64>().unwrap())
                .unwrap();

            let a: u64 = captures
                .get(3)
                .map(|v| v.as_str().parse::<u64>().unwrap())
                .unwrap();

            let s: u64 = captures
                .get(4)
                .map(|v| v.as_str().parse::<u64>().unwrap())
                .unwrap();

            parts_list.push(Part { x, m, a, s });
        }
    }

    let mut total_sum: u64 = 0;

    for part in parts_list {
        let mut sorters = &workflows["in"];

        loop {
            let dest = process(&part, &sorters).unwrap();

            if dest == "A" {
                total_sum += part.sum_parts();
                break;
            } else if dest == "R" {
                break;
            }

            sorters = &workflows[dest];
        }
    }

    println!("Total sum - Part 1: {total_sum}");

    let mut result = 0;
    let lpart = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let upart = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };
    enumerate_possibilities(&workflows, lpart, upart, "in", &mut result);

    println!("Number of possible accepted parts: {result}");
}
