use std::collections::HashMap;

use regex::Regex;

use crate::{generate_puzzle_input_test, generate_test_input_test, utils};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .fold(1, |lcm_so_far, &num| lcm(lcm_so_far, num))
}

pub fn solve(file: &str) -> u64 {
    let lines = utils::read_file(file);

    // Get left/right instructions first
    let movement_instructions = lines.get(0).unwrap().split("").filter(|c| *c != "");
    let num_of_instructions: usize = movement_instructions.clone().count();
    let mut looping_instructions = movement_instructions.clone().cycle();
    drop(movement_instructions);

    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();
    let line_regex = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();

    // Reassign lines to skip the empty line and start where the mappings start
    lines[1..].into_iter().for_each(|line| {
        line_regex
            .captures_iter(&line)
            .map(|c| c.extract())
            .for_each(|(_, [node, left, right])| {
                mappings.insert(node, (left, right));
            });
    });

    let starting_nodes = mappings.keys().filter(|node| node.ends_with('A'));
    let num_of_starting_nodes = starting_nodes.clone().count();
    let mut counter: u64 = 0;

    let direction = looping_instructions.next().unwrap();
    let mut nodes = starting_nodes
        .map(|starting_node| {
            let (left, right) = mappings.get(starting_node).unwrap();
            if direction == "L" {
                left
            } else {
                right
            }
        })
        .collect::<Vec<&&str>>();
    counter += 1; // We have traversed one node so add one

    let mut discovered_end_nodes: Vec<u64> = vec![];
    while discovered_end_nodes.len() != num_of_starting_nodes {
        let direction = looping_instructions.next().unwrap();
        counter += 1;
        nodes = nodes
            .iter()
            .filter_map(|starting_node| {
                let (left, right) = mappings.get(*starting_node).unwrap();

                if direction == "L" {
                    if left.ends_with('Z') {
                        // Do I neecd to add the distance it takes to get back to the node starting with A?
                        discovered_end_nodes.push(counter);

                        // We are done iterating through this cycle, so don't bother iterating on it next time
                        return None;
                    } else {
                        return Some(left);
                    }
                } else {
                    if right.ends_with('Z') {
                        discovered_end_nodes.push(counter);
                        return None;
                    } else {
                        return Some(right);
                    }
                }
            })
            .collect::<Vec<&&str>>();
    }

    println!("Discovered end nodes: {:?}", discovered_end_nodes);

    // Find the LCM of the discovered_end_nodes
    let lcm = lcm_of_vec(&discovered_end_nodes[..]);
    return lcm;
}

generate_test_input_test!(8, 6);
generate_puzzle_input_test!(8, 15989);
