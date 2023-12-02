use std::io::BufRead;
use std::{fs::File, io::BufReader};

use regex::Regex;

fn main() {
    println!("Part one result is {}", part_one());
    println!("Part two result is {}", part_two());
}

fn part_one() -> u32 {
    let input_file = File::open("./input.txt").expect("File should exist");
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(game_line) = line {
            let game = parse_game(&game_line);
            if game.is_valid(12, 14, 13) {
                sum = sum + Into::<u32>::into(game.id);
            }
        }
    }

    return sum;
}

fn parse_game(row: &str) -> Game {
    let game_id_pattern = Regex::new(r"Game\s(?<gid>\d{1,3})").unwrap();
    let mut game_id: u8 = 0;
    if let Some(caps) = game_id_pattern.captures(&row) {
        game_id = caps
            .name("gid")
            .map(|m| m.as_str().parse::<u8>().unwrap())
            .unwrap();
    };

    let mut sets = vec![];
    for raw_set in row.split(";") {
        sets.push(parse_set(raw_set));
    }
    return Game {
        id: game_id,
        sets: sets,
    };
}

fn parse_set(raw_set: &str) -> GameSet {
    let set_pattern =
        Regex::new(r"(?<count>\d{1,2})\s((?<green>green)|(?<red>red)|(?<blue>blue))").unwrap();

    let mut red: u8 = 0;
    let mut blue: u8 = 0;
    let mut green: u8 = 0;
    for captures in set_pattern.captures_iter(raw_set) {
        let mut count = 0;
        if let Some(c) = captures.name("count") {
            count = c.as_str().parse::<u8>().unwrap();
        }
        if let Some(_) = captures.name("red") {
            red = count;
        }
        if let Some(_) = captures.name("blue") {
            blue = count;
        }
        if let Some(_) = captures.name("green") {
            green = count;
        }
    }
    GameSet { red, blue, green }
}

struct Game {
    id: u8,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_valid(&self, red_count: u8, blue_count: u8, green_count: u8) -> bool {
        for set in self.sets.iter() {
            if set.green > green_count || set.blue > blue_count || set.red > red_count {
                return false;
            }
        }
        return true;
    }
}

struct GameSet {
    red: u8,
    blue: u8,
    green: u8,
}

fn part_two() -> u32 {
    let input_file = File::open("./input.txt").expect("File should exist");
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(game_line) = line {
            let game = parse_game(&game_line);
            let min_red = game.sets.iter().map(|g| g.red).map(Into::<u32>::into).max().unwrap();
            let min_blue = game.sets.iter().map(|g| g.blue).map(Into::<u32>::into).max().unwrap();
            let min_green = game.sets.iter().map(|g| g.green).map(Into::<u32>::into).max().unwrap();

            sum = sum + (min_red * min_green * min_blue);
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::{parse_game, parse_set};

    #[test]
    fn test_parse_set() {
        // Given
        let raw_set = "12 red, 7 blue";

        // When
        let result = parse_set(raw_set);

        // Then
        assert_eq!(result.green, 0);
        assert_eq!(result.blue, 7);
        assert_eq!(result.red, 12);
    }

    #[test]
    fn test_parse_game() {
        // Given
        let raw_game = "Game 39: 10 blue, 1 red, 4 green; 4 green, 4 red, 6 blue; 11 blue";

        // When
        let result = parse_game(raw_game);

        // Then
        assert_eq!(result.id, 39);
        assert_eq!(result.sets[0].red, 1);
        assert_eq!(result.sets[0].blue, 10);
        assert_eq!(result.sets[0].green, 4);
        assert_eq!(result.sets[1].red, 4);
        assert_eq!(result.sets[1].blue, 6);
        assert_eq!(result.sets[1].green, 4);
        assert_eq!(result.sets[2].red, 0);
        assert_eq!(result.sets[2].blue, 11);
        assert_eq!(result.sets[2].green, 0);
    }
}
