use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Forward,
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    amount: i32,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let command: Vec<&str> = line.split(" ").collect();
            Command {
                direction: match command[0] {
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => Direction::Forward,
                },
                amount: command[1].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part_one(commands: &[Command]) -> i32 {
    let (horizontal, depth) = commands.iter().fold((0, 0), |s, c| match c.direction {
        Direction::Up => (s.0, s.1 - c.amount),
        Direction::Down => (s.0, s.1 + c.amount),
        Direction::Forward => (s.0 + c.amount, s.1),
    });
    horizontal * depth
}

#[aoc(day2, part2)]
pub fn part_two(commands: &[Command]) -> i32 {
    let (horizontal, depth, aim) = commands.iter().fold((0, 0, 0), |s, c| match c.direction {
        Direction::Up => (s.0, s.1, s.2 - c.amount),
        Direction::Down => (s.0, s.1, s.2 + c.amount),
        Direction::Forward => (s.0 + c.amount, s.1 + (s.2 * c.amount), s.2),
    });
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use crate::day2::{part_one, part_two, Command, Direction};

    const INPUT: &[Command] = &[
        Command {
            direction: Direction::Forward,
            amount: 5,
        },
        Command {
            direction: Direction::Down,
            amount: 5,
        },
        Command {
            direction: Direction::Forward,
            amount: 8,
        },
        Command {
            direction: Direction::Up,
            amount: 3,
        },
        Command {
            direction: Direction::Down,
            amount: 8,
        },
        Command {
            direction: Direction::Forward,
            amount: 2,
        },
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
