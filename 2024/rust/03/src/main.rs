use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

pub fn process_part1(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("Nom parser failed {}", e))?;
    let result: u32 = instructions
        .iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum();
    Ok(result.to_string())
}

pub fn process_part2(input: &str) -> miette::Result<String> {
    let (_input, instructions) = parse(input).map_err(|e| miette!("Nom parser failed {}", e))?;
    let (_, result) = instructions.iter().fold(
        (ShouldProcess::Do, 0),
        |(process, accumulator), ins| match ins {
            Instruction::Mul(a, b) => {
                if process == ShouldProcess::Do {
                    (process, accumulator + a * b)
                } else {
                    (process, accumulator)
                }
            }
            Instruction::Do => (ShouldProcess::Do, accumulator),
            Instruction::DoNot => (ShouldProcess::DoNot, accumulator),
        },
    );
    Ok(result.to_string())
}

#[derive(PartialEq, Eq)]
enum ShouldProcess {
    Do,
    DoNot,
}

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    DoNot,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::DoNot, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

fn part1(input: &str) -> String {
    process_part1(input).unwrap()
}

fn part2(input: &str) -> String {
    process_part2(input).unwrap()
}
