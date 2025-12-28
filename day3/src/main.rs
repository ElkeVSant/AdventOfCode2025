use iter_first_max::IterFirstMaxExt as _;
use std::fs;

struct Bank {
    batteries: Vec<u32>,
}

impl Bank {
    fn max_joltage(&self, nb_batteries: usize) -> u64 {
        let (mut idx, first_digit) = self.batteries[..self.batteries.len() - (nb_batteries - 1)]
            .iter()
            .enumerate()
            .first_max_by_key(|(_idx, b)| **b)
            .unwrap();
        let mut result = first_digit.to_string();
        while result.len() < nb_batteries {
            let (add_idx, next_digit) = self.batteries
                [idx + 1..self.batteries.len() - (nb_batteries - result.len() - 1)]
                .iter()
                .enumerate()
                .first_max_by_key(|(_idx, b)| **b)
                .unwrap();
            result += &next_digit.to_string();
            idx += add_idx + 1;
        }
        result.parse::<u64>().unwrap()
    }
}

fn part1(input: &str) -> u64 {
    let mut output = 0;
    for line in input.lines() {
        let bank = Bank {
            batteries: line.chars().flat_map(|b| b.to_digit(10)).collect(),
        };
        output += bank.max_joltage(2);
    }
    output
}

fn part2(input: &str) -> u64 {
    let mut output = 0;
    for line in input.lines() {
        let bank = Bank {
            batteries: line.chars().flat_map(|b| b.to_digit(10)).collect(),
        };
        output += bank.max_joltage(12);
    }
    output
}

fn main() {
    let input = fs::read_to_string("day3/input.txt").expect("Failed to read input.txt");
    println!("Part 1: {}", part1(&input)); // 17127 is too low
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_part2_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part2(&input), 3121910778619);
    }
}
