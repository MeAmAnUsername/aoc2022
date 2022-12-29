#![allow(dead_code)]
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part1(INPUT));
}

// answer part 1: 1833
pub fn solve_part1(input: &str) -> usize {
    input
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .filter(|(_, (a, b, c, d))| [a, b, c, d].iter().all_unique())
        .nth(0)
        .expect("No window with only unique elements found")
        .0
        + 3 // first 3 characters do not form a complete window yet
        + 1 // 1 based counting instead of 0 based indexing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(7, solve_part1(INPUT))
    }

    #[test]
    fn example2_part1() {
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(5, solve_part1(INPUT))
    }

    #[test]
    fn example3_part1() {
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(6, solve_part1(INPUT))
    }

    #[test]
    fn example4_part1() {
        const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(10, solve_part1(INPUT))
    }

    #[test]
    fn example5_part1() {
        const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(11, solve_part1(INPUT))
    }
}
