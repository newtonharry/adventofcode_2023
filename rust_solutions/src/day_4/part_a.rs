use std::fs;

use crate::{generate_puzzle_input_test, generate_test_input_test, utils::read_file};

fn calculate_score_on_scratchcard(
    winning_numbers: Vec<i32>,
    numbers: impl Iterator<Item = i32>,
) -> i32 {
    let mut sum_per_card = 0;
    for num in numbers {
        if winning_numbers.contains(&num) {
            if sum_per_card == 0 {
                sum_per_card = 1;
            } else {
                sum_per_card *= 2;
            }
        }
    }
    sum_per_card
}

pub fn solve(file: &str) -> i32 {
    let mut sum = 0;
    let lines = read_file(file);
    let cards = lines.iter().map(|line: &String| -> (Vec<i32>, &str) {
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
    });

    // Loop through the cards and build up a queue of cards to calculate

    for (winning_numbers, numbers) in cards {
        let numbers = numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());

        let score = calculate_score_on_scratchcard(winning_numbers, numbers);
        sum += score;
        println!("Sum is = {}", sum);
    }
    sum
}

generate_test_input_test!(4, 13);
generate_puzzle_input_test!(4, 23847);
