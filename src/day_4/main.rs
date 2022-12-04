#![allow(dead_code)]
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part1(INPUT));
}

// answer part 1: 644
fn solve_part1(input: &str) -> usize {
    let regex: Regex = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    input
        .lines()
        .filter(|pair| {
            dbg!(pair);
            let mtch = regex
                .captures(pair)
                .expect(format!("pair does not match regex: {}", pair).as_str());
            let left_start: u32 = mtch.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let left_end: u32 = mtch.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let right_start: u32 = mtch.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let right_end: u32 = mtch.get(4).unwrap().as_str().parse::<u32>().unwrap();
            (left_start <= right_start && left_end >= right_end)
                || (left_start >= right_start && left_end <= right_end)
        })
        .count()
}

#[test]
fn example_part1() {
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(2, solve_part1(INPUT))
}
