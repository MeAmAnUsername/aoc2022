#![allow(dead_code)]
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part2(INPUT));
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
mod tests_part1 {
    use super::*;

    #[test]
    fn example1() {
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(7, solve_part1(INPUT))
    }

    #[test]
    fn example2() {
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(5, solve_part1(INPUT))
    }

    #[test]
    fn example3() {
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(6, solve_part1(INPUT))
    }

    #[test]
    fn example4() {
        const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(10, solve_part1(INPUT))
    }

    #[test]
    fn example5() {
        const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(11, solve_part1(INPUT))
    }
}

// answer part 2: 3425
pub fn solve_part2(input: &str) -> usize {
    let chars = input.chars().collect_vec();
    for i in 14..chars.len() {
        if chars[i - 14..i].iter().all_unique() {
            return i; // i is outside the window because ranges are exclusive, but that is fine because we need to return 1-indexed
        }
    }
    panic!("Did not find a string of 14 unique characters");
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn example1() {
        const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(19, solve_part2(INPUT))
    }

    #[test]
    fn example2() {
        const INPUT: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(23, solve_part2(INPUT))
    }

    #[test]
    fn example3() {
        const INPUT: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(23, solve_part2(INPUT))
    }

    #[test]
    fn example4() {
        const INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(29, solve_part2(INPUT))
    }

    #[test]
    fn example5() {
        const INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(26, solve_part2(INPUT))
    }
}
