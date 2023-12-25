use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&parse_input(&input)));
    println!("Part 2: {}", part2(&parse_input(&input)));
}

struct Scratchcard {
    winning: Vec<usize>,
    numbers: Vec<usize>,
}

impl From<&str> for Scratchcard {
    fn from(value: &str) -> Self {
        let mut parts = value.split(":").nth(1).unwrap().split("|");
        let winning = Self::number_list_from_str(parts.next().unwrap());
        let numbers = Self::number_list_from_str(parts.next().unwrap());
        Self { winning, numbers }
    }
}

impl Scratchcard {
    fn number_list_from_str(s: &str) -> Vec<usize> {
        s.split(" ")
            .filter(|x| !x.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    }

    fn num_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(*n))
            .count()
    }
}

fn parse_input(input: &String) -> Vec<Scratchcard> {
    input.lines().map(|l| Scratchcard::from(l)).collect()
}

fn part1(cards: &Vec<Scratchcard>) -> usize {
    cards
        .iter()
        .map(|sc| {
            let nm = sc.num_winning();
            if nm > 0 {
                2_usize.pow(nm as u32 - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(cards: &Vec<Scratchcard>) -> usize {
    let mut cn = vec![1_usize; cards.len()];
    for (num, card) in cards.iter().enumerate() {
        let num_card = cn[num];
        let card_wins = card.num_winning();
        for idx in num + 1..num + 1 + card_wins {
            cn[idx] += num_card;
        }
    }
    cn.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("sample.txt").unwrap();

        assert_eq!(13, part1(&parse_input(&input)));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("sample.txt").unwrap();

        assert_eq!(30, part2(&parse_input(&input)));
    }
}
