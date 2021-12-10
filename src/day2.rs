use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Debug, Error)]
pub enum ParseCommandError {
    #[error("Delimiter ' ' not found")]
    MissingDelimiter,
    #[error("Unknown instruction {0}")]
    UnknownCommand(String),
    #[error("Failed to parse distance: {0}")]
    MalformedDistance(#[from] ParseIntError),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, amount) = s
            .split_once(' ')
            .ok_or(ParseCommandError::MissingDelimiter)?;
        let amount = amount.parse::<i32>()?;
        match command {
            "up" => Ok(Command::Up(amount)),
            "down" => Ok(Command::Down(amount)),
            "forward" => Ok(Command::Forward(amount)),
            _ => Err(ParseCommandError::UnknownCommand(command.to_owned())),
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Result<Vec<Command>, ParseCommandError> {
    input.lines().map(Command::from_str).collect()
}

#[aoc(day2, part1)]
pub fn part_one(commands: &[Command]) -> i32 {
    let (horizontal, depth) = commands.iter().fold((0, 0), |s, c| match c {
        Command::Up(x) => (s.0, s.1 - x),
        Command::Down(x) => (s.0, s.1 + x),
        Command::Forward(x) => (s.0 + x, s.1),
    });
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn part_two(commands: &[Command]) -> i32 {
    let (horizontal, depth, _aim) = commands.iter().fold((0, 0, 0), |s, c| match c {
        Command::Up(x) => (s.0, s.1, s.2 - x),
        Command::Down(x) => (s.0, s.1, s.2 + x),
        Command::Forward(x) => (s.0 + x, s.1 + (s.2 * x), s.2),
    });
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{part_one, part_two, Command};

    const INPUT: &[Command] = &[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 150);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 900);
    }
}
