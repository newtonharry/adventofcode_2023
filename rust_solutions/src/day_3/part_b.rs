use core::num;
use std::{collections::HashSet, fs};

use crate::{generate_puzzle_input_test, generate_test_input_test};

#[derive(Debug, Eq, PartialEq, Hash)]
struct PartNumber {
    number: i32,
    gear_idx: i32,
}

pub fn solve(file: &str) -> i32 {
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let data = fs::read_to_string(file).expect("File input does not exist");
    let data_chars = data.chars().collect::<Vec<char>>();
    let lines = data.split_inclusive('\n').collect::<Vec<&str>>();

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut gear_idx = -1;
    let mut geared_numbers: Vec<PartNumber> = Vec::new();
    let mut number = String::new();
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let y: i32 = y as i32;
        for (x, char) in line.chars().enumerate() {
            let x: i32 = x as i32;
            let is_digit = char.is_ascii_digit();
            if is_digit {
                number.push(char);
            } else if !is_digit && !number.is_empty() && gear_idx != -1 {
                geared_numbers.push(PartNumber {
                    number: number.parse::<i32>().unwrap(),
                    gear_idx,
                });
                gear_idx = -1;
                number.clear();
            } else {
                gear_idx = -1;
                number.clear();
            }

            // Check to see if a digit doesn't already border a symbol
            if gear_idx == -1 && is_digit {
                for (rel_x, rel_y) in neighbors {
                    let new_x = x + rel_x;
                    let new_y = y + rel_y;

                    if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                        let new_index = (new_y * width + new_x) as usize;
                        let adjacent_char: Option<&char> = data_chars.get(new_index);
                        if let Some(c) = adjacent_char {
                            if *c == '*' {
                                gear_idx = new_index as i32;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut checked_positions: Vec<i32> = vec![];
    let indexes: Vec<i32> = geared_numbers.iter().map(|part| part.gear_idx).collect();
    for index in indexes {
        if !checked_positions.contains(&index) {
            let numbers_with_gears = geared_numbers
                .iter()
                .filter(|part| part.gear_idx == index)
                .map(|part| part.number)
                .collect::<Vec<i32>>();

            if numbers_with_gears.len() > 1 {
                let gear_ratio: i32 = numbers_with_gears.iter().product();
                println!("Gear ratio is {}", gear_ratio);
                sum += gear_ratio;
            }

            checked_positions.push(index);
        }
    }
    sum
}

generate_test_input_test!(3, 467835);
generate_puzzle_input_test!(3, 89471771);
