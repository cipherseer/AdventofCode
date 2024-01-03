#![feature(linked_list_remove)]
use std::collections::LinkedList;

const NONE_INIT: Option<LinkedList<(&str, u8)>> = None;

fn hash_string(string: &str) -> usize {
    let mut result: usize = 0;
    for ch in string.chars() {
        result = ((result + ch as usize) * 17) % 256;
    }
    result
}

fn modify_boxes<'a>(
    boxes: &mut [Option<LinkedList<(&'a str, u8)>>; 256],
    label: &'a str,
    lens: u8,
    add: bool,
) {
    let index = hash_string(label);
    if add {
        'outer: {
            match &mut boxes[index] {
                Some(lenses) => {
                    for (l, v) in lenses.iter_mut() {
                        if *l == label {
                            *v = lens;
                            break 'outer;
                        }
                    }

                    lenses.push_back((label, lens));
                }
                None => {
                    let mut lenses = LinkedList::new();
                    lenses.push_back((label, lens));
                    boxes[index] = Some(lenses);
                }
            }
        }
    } else {
        if let Some(lenses) = boxes[index].as_mut() {
            if let Some(position) = lenses.iter_mut().position(|&mut (l, _)| l == label) {
                lenses.remove(position);
                if lenses.is_empty() {
                    boxes[index] = None;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let strings: Vec<&str> = input.lines().next().unwrap().split(',').collect();
    //part 1
    let mut total_hash: usize = 0;
    strings
        .iter()
        .for_each(|string| total_hash += hash_string(string));

    println!("Total hash: {total_hash}");

    //part 2
    let mut boxes: [Option<LinkedList<(&str, u8)>>; 256] = [NONE_INIT; 256];
    for string in &strings {
        let operator = string.find(&['=', '-']).unwrap();
        modify_boxes(
            &mut boxes,
            &string[0..operator],
            string[(operator + 1)..].parse::<u8>().unwrap_or_default(),
            string.as_bytes()[operator] == '=' as u8,
        );
    }

    let mut focusing_power: usize = 0;
    for (i, list_option) in boxes.iter().enumerate() {
        println!("{:?}", list_option);
        if let Some(list) = list_option {
            for (j, (_, focal_length)) in list.iter().enumerate() {
                focusing_power += (i + 1) * (j + 1) * (*focal_length as usize);
            }
        }
    }

    println!("Focusing power: {focusing_power}");
}
