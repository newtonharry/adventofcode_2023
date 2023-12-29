use std::fs;

struct Game {
    game_id: u32,
    impossible: bool,
}

pub fn solve(file: &str) -> u32 {
    let mut games: Vec<Game> = Vec::new();
    let mut impossible = false;
    let data: String = fs::read_to_string(file).expect("Expect puzzle input");
    let lines = data.split('\n');
    for (index, line) in lines.enumerate() {
        impossible = false;
        let mut game = line.split(':');
        let sets = game.nth(1).unwrap_or("").trim().split(';');
        for set in sets {
            let cubes = set.trim().split(',');
            for cube in cubes {
                let cube_info: Vec<&str> = cube.trim().split(' ').collect();
                let amount = cube_info.first().unwrap().parse().unwrap_or(0);
                let colour = cube_info.last().unwrap().to_owned();
                if (colour == "red" && amount > 12)
                    || (colour == "green" && amount > 13)
                    || (colour == "blue" && amount > 14)
                {
                    impossible = true;
                    break;
                }
            }
            if impossible {
                break;
            }
        }
        games.push(Game {
            game_id: (index + 1).try_into().unwrap(),
            impossible,
        });
    }

    games
        .iter()
        .filter(|game| !game.impossible)
        .map(|game| game.game_id)
        .sum()
}
