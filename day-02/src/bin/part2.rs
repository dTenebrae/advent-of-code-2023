use std::fs;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Cubes {
    fn default() -> Self {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Cubes {
    fn min_valid(&mut self, additive: &Cubes) {
        if additive.red > self.red {
            self.red = additive.red;
        }

        if additive.green > self.green {
            self.green = additive.green;
        }

        if additive.blue > self.blue {
            self.blue = additive.blue;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input1.txt").expect("No file found");

    let result: u32 = contents
        .lines()
        .into_iter()
        .map(|line: &str| game_checker(line))
        .sum();
    println!("{}", result)
}

fn game_checker(line: &str) -> u32 {
    let mut summary_cube = Cubes::default();
    let mut line_iter = line.split(':');
    let _ = line_iter.next().unwrap();

    let data_valid = line_iter
        .next()
        .unwrap()
        .split(';')
        .map(|x| str_to_struct(x));

    for cube in data_valid {
        summary_cube.min_valid(&cube);
    }
    summary_cube.red * summary_cube.green * summary_cube.blue
}

fn str_to_struct(game: &str) -> Cubes {
    let mut current_cubes = Cubes::default();
    let game_iter: Vec<&str> = game.trim().split(',').into_iter().map(str::trim).collect();
    for grab in game_iter {
        let mut grab_iter = grab.split_whitespace();
        let (num, color) = (grab_iter.next().unwrap(), grab_iter.next().unwrap());
        match color {
            "red" => current_cubes.red = num.parse().unwrap(),
            "green" => current_cubes.green = num.parse().unwrap(),
            "blue" => current_cubes.blue = num.parse().unwrap(),
            _ => panic!("Something goes wrong"),
        }
    }
    current_cubes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result: u32 = input
            .lines()
            .into_iter()
            .map(|line: &str| game_checker(line))
            .sum();

        assert_eq!(result, 2286);
    }
}
