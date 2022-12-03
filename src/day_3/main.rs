#![allow(dead_code)]
use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part2(INPUT));
}

// answer part 1: 7766
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let len: usize = rucksack.chars().count() / 2;
            let first_half: HashSet<char> = rucksack[..len].chars().collect();
            let last_half: HashSet<char> = rucksack[len..].chars().collect();
            let mixed: Vec<&char> = first_half.intersection(&last_half).collect();
            assert!(
                mixed.len() == 1,
                "More than one mixed item type: {:?}",
                mixed
            );
            let mixed_type = mixed.first().unwrap();
            match mixed_type {
                'a'..='z' => **mixed_type as u32 - 'a' as u32 + 1,
                'A'..='Z' => **mixed_type as u32 - 'A' as u32 + 27,
                _ => panic!("Unexpected type {}", mixed_type),
            }
        })
        .sum::<u32>()
}

#[test]
fn example_part1() {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(157, solve_part1(INPUT))
}

// answer part 2: 2415
fn solve_part2(input: &str) -> u32 {
    use itertools::Itertools;

    input
        .split('\n')
        .chunks(3)
        .into_iter()
        .map(|group| {
            let (elf1, elf2, elf3) = group
                .map(|rucksack| rucksack.chars())
                .collect_tuple()
                .unwrap();

            let elf1: HashSet<char> = elf1.collect();
            let intersection: HashSet<char> = elf2.filter(|x| elf1.contains(x)).collect();
            let intersection = elf3
                .filter(|x| intersection.contains(x))
                .dedup()
                .collect_vec();
            assert!(
                intersection.len() == 1,
                "More than one common item type: {:?}",
                intersection
            );

            let batch = intersection[0];
            match batch {
                'a'..='z' => batch as u32 - 'a' as u32 + 1,
                'A'..='Z' => batch as u32 - 'A' as u32 + 27,
                _ => panic!("Unexpected type {}", batch),
            }
        })
        .sum::<u32>()
}

#[test]
fn example_part2() {
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(70, solve_part2(INPUT))
}
