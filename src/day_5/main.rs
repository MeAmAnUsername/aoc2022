#![allow(dead_code)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_till;
use nom::character::complete::one_of;
use nom::combinator::all_consuming;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Solution: {}", solve_part1(INPUT));
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn from_vertical_vecs(vertical_stacks: Vec<Vec<Option<char>>>) -> Stacks {
        let mut horizontal_stacks: Vec<Vec<char>> = Vec::new();
        for _ in 0..vertical_stacks[0].len() {
            horizontal_stacks.push(Vec::new());
        }

        for line in vertical_stacks.iter().rev() {
            for (i, crate_name) in line.iter().enumerate() {
                if let Some(crate_name) = crate_name {
                    horizontal_stacks[i].push(crate_name.clone());
                }
            }
        }

        Stacks {
            stacks: dbg!(horizontal_stacks),
        }
    }

    fn apply_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.apply_command(command);
        }
    }

    fn apply_command(&mut self, command: Command) {
        for _ in 0..command.count {
            let from = &mut self.stacks[command.from - 1];
            let crate_name = from.pop().expect(
                format!(
                    "Could not take a crate from stack {}, it is empty",
                    command.from
                )
                .as_str(),
            );
            let to = &mut self.stacks[command.to - 1];
            to.push(crate_name);
        }
        // println!(
        //     "Move {} from {} to {}: {:#?}",
        //     command.count, command.from, command.to, &self.stacks,
        // )
    }

    fn get_top_crates(&self) -> Vec<Option<&char>> {
        self.stacks.iter().map(|stack| stack.last()).collect()
    }
}

struct Command {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_empty(input: &str) -> IResult<&str, ()> {
    tag("   ")(input).map(|(rest, _)| (rest, ()))
}

fn parse_item(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("[")(input)?;
    let (input, crate_str) = take(1usize)(input)?;
    let (input, _) = tag("]")(input)?;

    let res: char = crate_str.chars().nth(0).expect("crate_str was empty");
    IResult::Ok((input, res))
}

fn parse_item_or_empty(input: &str) -> IResult<&str, Option<char>> {
    alt((
        parse_empty.map(|_| Option::None),
        parse_item.map(|crate_name| Option::Some(crate_name)),
    ))(input)
}

fn parse_stacks_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(tag(" "), parse_item_or_empty)(input)
}

fn parse_stacks(input: &str) -> IResult<&str, Stacks> {
    let (input, stacks) = separated_list1(tag("\n"), parse_stacks_line)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = take_till(|c| c == '\n')(input)?;

    IResult::Ok((input, Stacks::from_vertical_vecs(stacks)))
}

fn decimal(input: &str) -> IResult<&str, usize> {
    map_res(many1(one_of("0123456789")), |str| {
        str.into_iter().collect::<String>().parse()
    })(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        decimal,
        tag(" from "),
        decimal,
        tag(" to "),
        decimal,
    ))(input)?;

    IResult::Ok((input, Command { count, from, to }))
}

fn parse(input: &str) -> Result<(Stacks, Vec<Command>), nom::Err<nom::error::Error<&str>>> {
    let (input, stacks) = parse_stacks(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, commands) = all_consuming(separated_list1(tag("\n"), parse_command))(input)?;
    assert_eq!(input, "");

    Ok((stacks, commands))
}

// answer part 1: DHBJQJCCW
fn solve_part1(input: &str) -> String {
    let (mut stacks, commands) = parse(input).expect("Parsing failed");
    stacks.apply_commands(commands);
    stacks
        .get_top_crates()
        .iter()
        .map(|crate_maybe| crate_maybe.expect("Stack was empty"))
        .collect()
}

#[test]
fn example_part1() {
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("CMZ", solve_part1(INPUT))
}
