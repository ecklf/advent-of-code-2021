use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Validation {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn validate_line(line: &str) -> Validation {
    let mut s = Vec::new();
    for c in line.chars() {
        match c {
            opening @ ('(' | '[' | '{' | '<') => s.push(opening),
            closing @ (')' | ']' | '}' | '>') => {
                let opening = s.pop().unwrap();
                match (opening, closing) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                    _ => return Validation::Corrupted(closing),
                }
            }
            c => unreachable!("Input should not have this character: {}", c),
        }
    }
    match s.is_empty() {
        true => Validation::Valid,
        false => Validation::Incomplete(s),
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect::<Vec<_>>()
}

#[aoc(day10, part1)]
pub fn part_one(navigation: &Vec<String>) -> u32 {
    navigation
        .to_owned()
        .iter()
        .map(|l| validate_line(l))
        .filter_map(|v| match v {
            Validation::Corrupted(c) => Some(c),
            _ => None,
        })
        .map(|corrupted| match corrupted {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            c => unreachable!("Input should not have this character: {}", c),
        })
        .sum::<u32>()
}

#[aoc(day10, part2)]
pub fn part_two(navigation: &Vec<String>) -> i64 {
    let mut scores = navigation
        .to_owned()
        .iter()
        .map(|l| validate_line(l))
        .filter_map(|v| match v {
            Validation::Incomplete(c) => Some(c),
            _ => None,
        })
        .map(|incomplete| {
            incomplete.iter().rev().fold(0, |s, ic| {
                let score = match ic {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    invalid => unreachable!("Input should not have this character: {}", invalid),
                };
                (s * 5) + score
            })
        })
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::day10::{input_generator, part_one, part_two};

    const INPUT: &'static str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(part_one(&input), 26397);
    }

    #[test]
    fn test_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(part_two(&input), 288957);
    }
}
