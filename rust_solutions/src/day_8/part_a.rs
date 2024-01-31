use std::collections::{HashMap, VecDeque};

use regex::Regex;

use crate::{generate_puzzle_input_test, generate_test_input_test, utils};

pub fn solve(file: &str) -> u32 {
    let mut lines = utils::read_file(file);

    // Get left/right instructions first
    let mut movement_instructions = lines.get(0).unwrap().split("").filter(|c| *c != "").cycle();

    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    let line_regex = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();

    // Reassign lines to skip the empty line and start where the mappings start
    lines[1..].iter().for_each(|line| {
        println!("{}", line);
        line_regex
            .captures_iter(&line)
            .map(|c| c.extract())
            .for_each(|(_, [node, left, right])| {
                println!("Inserting!");
                mappings.insert(node, (left, right));
            });
    });

    let mut counter = 0;
    let mut node = {
        let (left, right) = mappings.get("AAA").unwrap();
        if movement_instructions.next().unwrap() == "L" {
            left
        } else {
            right
        }
    };
    counter += 1;

    while *node != "ZZZ" {
        let (left, right) = mappings.get(node).unwrap();
        if let Some(direction) = movement_instructions.next() {
            counter += 1;
            if direction == "L" {
                println!("Going left");
                node = left;
            } else {
                println!("Going right");
                node = right;
            }
        }
    }

    return counter;
}

//generate_test_input_test!(8, 6);
//generate_puzzle_input_test!(8, 10);

// generate_puzzle_input_test!(8, 248179786);
