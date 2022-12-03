#![allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part2(INPUT));
}

fn solve_part1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|snacks| {
            dbg!(snacks)
                .split("\n")
                .map(|snack| {
                    dbg!(snack)
                        .trim()
                        .parse::<u32>()
                        .expect(format!("Could not parse {} to number", snack).as_str())
                })
                .sum()
        })
        .max()
        .expect("Input was empty")
}

fn solve_part2(input: &str) -> u32 {
    use std::mem::swap;

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    input
        .split("\n\n")
        .map(|snacks| {
            dbg!(snacks)
                .split("\n")
                .map(|snack| {
                    dbg!(snack)
                        .trim()
                        .parse::<u32>()
                        .expect(format!("Could not parse {} to number", snack).as_str())
                })
                .sum()
        })
        .for_each(|elf_callories| {
            if elf_callories > third {
                third = elf_callories;
                if third > second {
                    swap(&mut second, &mut third);
                    if second > first {
                        swap(&mut first, &mut second);
                    }
                }
            }
        });
    first + second + third
}

#[test]
fn example_part1() {
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(24000, solve_part1(INPUT))
}

#[test]
fn example_part2() {
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    assert_eq!(45000, solve_part2(INPUT))
}
