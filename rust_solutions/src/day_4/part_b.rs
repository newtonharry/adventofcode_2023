use std::{collections::HashMap, fs};

use crate::{generate_puzzle_input_test, generate_test_input_test, utils::read_file};

fn calculate_matching_numbers(winning_numbers: &Vec<i32>, numbers: &str) -> usize {
    let numbers = numbers
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap());

    numbers.map(|x| winning_numbers.contains(&x) as usize).sum()
}

fn update_total_cards_and_queue(
    total_cards: &mut usize,
    queue: &mut Vec<usize>,
    card_num: usize,
    num_of_cards: usize,
    score: usize,
) {
    *total_cards += score;
    if ((card_num + 1) + score) > num_of_cards + 1 {
        queue.extend((card_num + 1)..(num_of_cards + 1));
    } else {
        queue.extend((card_num + 1)..((card_num + 1) + score));
    }
}

pub fn solve(file: &str) -> usize {
    let lines = read_file(file);
    let mut total_cards = 0;
    let mut queue: Vec<usize> = Vec::new();
    let cards = lines
        .iter()
        .map(|line: &String| -> (Vec<i32>, &str) {
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

    let mut scores = HashMap::new();
    let num_of_cards = cards.len();
    total_cards += num_of_cards; // Add the inital number of cards

    queue.extend(1..(num_of_cards + 1)); // Initalize the queue with the starting number of cards

    // It doesn't matter which end you process the queue, as long as each element is processed properly
    while let Some(card_num) = queue.pop() {
        if let Some(score) = scores.get(&card_num) {
            if *score == 0 || (card_num + 1) > num_of_cards {
                continue;
            } else {
                update_total_cards_and_queue(
                    &mut total_cards,
                    &mut queue,
                    card_num,
                    num_of_cards,
                    *score,
                );
            }
        } else {
            if let Some(card) = cards.get(card_num - 1) {
                let (wnums, nums) = card;
                let temp_score = calculate_matching_numbers(wnums, nums);
                scores.insert(card_num, temp_score);
                if temp_score == 0 || (card_num + 1) > num_of_cards {
                    continue;
                } else {
                    update_total_cards_and_queue(
                        &mut total_cards,
                        &mut queue,
                        card_num,
                        num_of_cards,
                        temp_score,
                    );
                }
            }
        }
    }
    total_cards
}

generate_test_input_test!(4, 30);
generate_puzzle_input_test!(4, 8570000);
