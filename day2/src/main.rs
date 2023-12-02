use regex::Regex;
use once_cell::sync::Lazy;
use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&parse_input(&input)));
    println!("Part 2: {}", part2(&parse_input(&input)));
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    pub fn from_str(input: &str) -> Self {
        static MATCH_GAME_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game ([0-9]+):").unwrap());

        let sets: Vec<GameSet> = input.split(';').map(|s| {
            GameSet::from_str(&s.to_string())
        }).collect();

        let id = MATCH_GAME_ID.captures(input).unwrap()[1].parse().unwrap();

        Self {
            id,
            sets,
        }
    }

    pub fn is_possible(&self, total_red: usize, total_green: usize, total_blue: usize) -> bool {
        for s in &self.sets {
            if s.red > total_red || s.green > total_green || s.blue > total_blue {
                return false;
            }
        }
        true
    }

    pub fn get_power(&self) -> usize {
        let smallest_set = self.smallest_set();
        smallest_set.red * smallest_set.green * smallest_set.blue
    }

    fn smallest_set(&self) -> GameSet {
        GameSet {
            red: self.sets.iter().max_by_key(|s| s.red).unwrap().red,
            green: self.sets.iter().max_by_key(|s| s.green).unwrap().green,
            blue: self.sets.iter().max_by_key(|s| s.blue).unwrap().blue,
        }
    }
}

struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    pub fn from_str(set_str: &String) -> Self {
        static MATCH_RED: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]+) red").unwrap());
        static MATCH_GREEN: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]+) green").unwrap());
        static MATCH_BLUE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]+) blue").unwrap());

        let mut set = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        if let Some(caps) = MATCH_RED.captures(set_str) {
            set.red = caps[1].parse().unwrap();
        }
        if let Some(caps) = MATCH_GREEN.captures(set_str) {
            set.green = caps[1].parse().unwrap();
        }
        if let Some(caps) = MATCH_BLUE.captures(set_str) {
            set.blue = caps[1].parse().unwrap();
        }

        set
    }
}

fn parse_input(input: &String) -> Vec<Game> {
    input.lines().map(|l| {
        Game::from_str(l)
    }).collect()
}

fn part1(games: &Vec<Game>) -> usize {
    games.iter().map(|g| {
        if g.is_possible(12, 13, 14) {g.id} else {0}
    }).sum()
}

fn part2(games: &Vec<Game>) -> usize {
    games.iter().map(|g| {
        g.get_power()
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("sample.txt").unwrap();

        assert_eq!(8, part1(&parse_input(&input)));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("sample.txt").unwrap();

        assert_eq!(2286, part2(&parse_input(&input)));
    }
}