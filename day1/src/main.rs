use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &String) -> u32 {
     input.lines().map(|l| -> u32 {
        let nums: Vec<char> = l.chars().filter(|c| c.is_ascii_digit()).collect();
        nums.first().unwrap().to_digit(10).unwrap() * 10 + nums.last().unwrap().to_digit(10).unwrap()
    }).sum()
}

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn part2(input: &String) -> u32 {
    input.lines().map(|mut l| -> u32 {
        let mut fixed_str = l.replace("one", "o1e");
        fixed_str = fixed_str.replace("two", "t2o");
        fixed_str = fixed_str.replace("three", "t3e");
        fixed_str = fixed_str.replace("four", "f4r");
        fixed_str = fixed_str.replace("five", "f5e");
        fixed_str = fixed_str.replace("six", "s6x");
        fixed_str = fixed_str.replace("seven", "s7n");
        fixed_str = fixed_str.replace("eight", "s8t");
        fixed_str = fixed_str.replace("nine", "n9e");
        let nums: Vec<char> = fixed_str.chars().filter(|c| c.is_ascii_digit()).collect();
        nums.first().unwrap().to_digit(10).unwrap() * 10 + nums.last().unwrap().to_digit(10).unwrap()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("sample-p1.txt").unwrap();
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("sample-p2.txt").unwrap();
        assert_eq!(part2(&input), 281);
    }
}