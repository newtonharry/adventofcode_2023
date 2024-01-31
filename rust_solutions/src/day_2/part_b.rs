use std::fs;

use crate::{generate_puzzle_input_test, generate_test_input_test};

#[derive(Default)]
struct Game {
    min_red: i32,
    min_green: i32,
    min_blue: i32,
}

impl Game {
    fn power(self) -> i32 {
        self.min_blue * self.min_red * self.min_green
    }
}

pub fn solve(file: &str) -> i32 {
    let data: String = fs::read_to_string(file).expect("Expect puzzle input");
    let lines = data.split('\n');
    let mut sum = 0;
    lines.for_each(|line| {
        let mut game = line.split(':');
        let mut game_obj = Game::default();
        let sets = game.nth(1).unwrap_or("").trim().split(';');
        for set in sets {
            let cubes = set.trim().split(',');
            for cube in cubes {
                let cube_info: Vec<&str> = cube.trim().split(' ').collect();
                let amount = cube_info.first().unwrap().parse().unwrap_or(0);
                let colour = cube_info.last().unwrap().to_owned();
                if colour == "red" && amount > game_obj.min_red {
                    game_obj.min_red = amount;
                } else if colour == "green" && amount > game_obj.min_green {
                    game_obj.min_green = amount;
                } else if colour == "blue" && amount > game_obj.min_blue {
                    game_obj.min_blue = amount;
                }
            }
        }
        sum += game_obj.power();
    });

    sum
}

generate_test_input_test!(2, 2286);
generate_puzzle_input_test!(2, 83105);
