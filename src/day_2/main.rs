#![allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part2(INPUT));
}

fn solve_part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|round| {
            let opponent = round.chars().nth(0).unwrap();
            let me = round.chars().nth(2).unwrap();
            match me {
                'X' => match opponent {
                    'A' => 4,
                    'B' => 1,
                    'C' => 7,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                'Y' => match opponent {
                    'A' => 8,
                    'B' => 5,
                    'C' => 2,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                'Z' => match opponent {
                    'A' => 3,
                    'B' => 9,
                    'C' => 6,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                _ => panic!("Unexpexcted shape {} for me", me),
            }
        })
        .sum::<u32>()
}

#[test]
fn example_part1() {
    const INPUT: &str = "A Y
B X
C Z";

    assert_eq!(15, solve_part1(INPUT))
}

fn solve_part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|round| {
            let opponent = round.chars().nth(0).unwrap();
            let me = round.chars().nth(2).unwrap();
            match me {
                'X' => match opponent {
                    'A' => 3,
                    'B' => 1,
                    'C' => 2,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                'Y' => match opponent {
                    'A' => 4,
                    'B' => 5,
                    'C' => 6,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                'Z' => match opponent {
                    'A' => 8,
                    'B' => 9,
                    'C' => 7,
                    _ => panic!("Unexpexcted shape {} for opponent", opponent),
                },
                _ => panic!("Unexpexcted shape {} for me", me),
            }
        })
        .sum::<u32>()
}

#[test]
fn example_part2() {
    const INPUT: &str = "A Y
B X
C Z";

    assert_eq!(12, solve_part2(INPUT))
}
