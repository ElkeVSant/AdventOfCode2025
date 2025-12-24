use std::fs;

fn part1(input: &str) -> u64 {
    let id_ranges = input.split(",");
    let mut sum = 0;
    for range in id_ranges {
        let mut ends = range.split("-");
        let from = ends.next().unwrap().parse::<u64>().unwrap();
        let to = ends.last().unwrap().trim().parse::<u64>().unwrap();
        for id in from..to + 1 {
            if !validate_single_repetition(&id.to_string()) {
                sum += id;
            }
        }
    }
    sum
}

fn validate_single_repetition(id: &str) -> bool {
    let (start, end) = id.split_at(id.len() / 2);
    if start == end {
        return false;
    }
    true
}

fn part2(input: &str) -> u64 {
    let id_ranges = input.split(",");
    let mut sum = 0;
    for range in id_ranges {
        let mut ends = range.split("-");
        let from = ends.next().unwrap().parse::<u64>().unwrap();
        let to = ends.last().unwrap().trim().parse::<u64>().unwrap();
        for id in from..to + 1 {
            if !validate_repetitions(&id.to_string()) {
                sum += id;
            }
        }
    }
    sum
}

fn validate_repetitions(id: &str) -> bool {
    for i in 1..id.len() / 2 + 1 {
        let (start, mut part) = id.split_at(i);
        while part.starts_with(start) {
            if part.len() > i {
                let (_, new_part) = part.split_at(i);
                part = new_part;
            } else {
                return false;
            }
        }
    }
    true
}

fn main() {
    let input = fs::read_to_string("day2/input.txt").expect("Failed to read input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_111() {
        assert_eq!(validate_repetitions("111"), false);
    }

    #[test]
    fn test_121212() {
        assert_eq!(validate_repetitions("121212"), false);
    }

    #[test]
    fn test_1188511885() {
        assert_eq!(validate_repetitions("1188511885"), false);
    }

    #[test]
    fn test_part2_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part2(&input), 4174379265);
    }
}
