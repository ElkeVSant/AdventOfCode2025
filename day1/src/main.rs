use std::fs;

fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for rotation in input.lines() {
        rotate(&mut dial, rotation, &mut 0);
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn rotate(dial: &mut i32, rotation: &str, count: &mut i32) {
    let (dir, size) = rotation.split_at(1);
    let size = size.parse::<i32>().unwrap();
    match dir {
        "L" => {
            *dial -= size;
            if *dial % 100 == 0 {
                // && *dial != 0 {
                *count += 1;
            }
            while *dial < 0 {
                if *dial + size != 0 {
                    *count += 1;
                }
                *dial += 100;
            }
        }
        _ => {
            *dial += size;
            while *dial > 99 {
                *dial -= 100;
                if true {
                    *count += 1;
                }
            }
        }
    }
}

fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    for rotation in input.lines() {
        rotate(&mut dial, rotation, &mut count);
    }
    count
}

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("Failed to read input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_left_rotation() {
        let input = "L68";
        let mut dial = 50;
        rotate(&mut dial, &input, &mut 0);
        assert_eq!(dial, 82);
    }

    #[test]
    fn test_right_rotation() {
        let input = "R60";
        let mut dial = 95;
        rotate(&mut dial, &input, &mut 0);
        assert_eq!(dial, 55);
    }

    #[test]
    fn test_r1000_rotation() {
        let input = "R1000";
        assert_eq!(part2(&input), 10);
    }

    #[test]
    fn test_l150r50_rotation() {
        let input = "L150\nR50";
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_l150l50_rotation() {
        let input = "L150\nL50";
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_r150l50_rotation() {
        let input = "R150\nL50";
        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_example() {
        let input: String = fs::read_to_string("example.txt").expect("Failed to read example.txt");
        assert_eq!(part2(&input), 6);
    }
}
