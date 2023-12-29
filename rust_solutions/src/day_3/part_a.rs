use std::fs;

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
    let data_chars = data.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let lines = data.split('\n').collect::<Vec<&str>>();

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut has_symbol = false;
    let mut number = String::new();
    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let y: i32 = y as i32;
        for (x, char) in line.chars().enumerate() {
            let x: i32 = x as i32;
            let is_digit = char.is_ascii_digit();
            if is_digit {
                number.push(char);
            } else if !is_digit && !number.is_empty() && has_symbol {
                sum += number.parse::<i32>().unwrap();
                has_symbol = false;
                number.clear();
            } else {
                has_symbol = false;
                number.clear();
            }

            // Check to see if a digit of a number doesn't already border a symbol
            if !has_symbol && is_digit {
                for (rel_x, rel_y) in neighbors {
                    let new_x = x + rel_x;
                    let new_y = y + rel_y;

                    if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
                        let new_index = (new_y * width + new_x) as usize;
                        let adjacent_char: Option<&char> = data_chars.get(new_index);
                        if let Some(c) = adjacent_char {
                            if !c.is_ascii_digit() && *c != '.' {
                                has_symbol = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}
