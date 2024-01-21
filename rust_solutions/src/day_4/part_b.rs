use core::num;
use std::{collections::HashMap, fs};

fn calculate_matching_numbers(winning_numbers: &Vec<i32>, numbers: &str) -> usize {
    let numbers = numbers
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap());

    numbers.map(|x| winning_numbers.contains(&x) as usize).sum()
}

pub fn solve(file: &str) -> usize {
    let data = fs::read_to_string(file).expect("Input needs to exist");
    let lines = data.split('\n').collect::<Vec<&str>>();
    let mut total_cards = 0;
    let mut queue: Vec<usize> = Vec::new();
    let cards = lines
        .iter()
        .map(|line: &&str| -> (Vec<i32>, &str) {
            let parts = line
                .split(':')
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap()
                .split('|')
                .collect::<Vec<&str>>();

            // Need to collect the iterator as we need to be able to know if if numbers are "contained" within it
            // Iterators don't offer this
            let left = parts
                .first()
                .unwrap()
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            // Can't return an iterator from a closure so will have to just return as a &str
            let right = parts.last().unwrap().trim();
            (left, right)
        })
        .collect::<Vec<_>>();

    // Calculate the matching numbers and store in a hashmap to reduce unnecessary computations
    let scores = cards
        .iter()
        .enumerate()
        .map(|(card_num, (wnums, nums))| (card_num + 1, calculate_matching_numbers(wnums, nums)))
        .collect::<HashMap<usize, usize>>();

    let num_of_cards = cards.len();
    total_cards += num_of_cards; // Add the inital number of cards

    queue.extend(1..(num_of_cards + 1)); // Initalize the queue with the starting number of cards

    // It doesn't matter which end you process the queue, as long as each element is processed properly
    while let Some(card_num) = queue.pop() {
        if let Some(score) = scores.get(&card_num) {
            if *score == 0 || (card_num + 1) > num_of_cards {
                continue;
            } else {
                total_cards += score;
                if ((card_num + 1) + score) > num_of_cards + 1 {
                    queue.extend((card_num + 1)..(num_of_cards + 1));
                } else {
                    queue.extend((card_num + 1)..((card_num + 1) + score));
                }
            }
        }
    }
    total_cards
}
