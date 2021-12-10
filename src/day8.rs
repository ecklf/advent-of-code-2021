use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Entry> {
    let lines = input
        .lines()
        .map(|line| {
            let entries = line.split(" | ").collect::<Vec<_>>();
            Entry {
                patterns: entries[0]
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
                output: entries[1]
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();
    lines
}

#[aoc(day8, part1)]
pub fn part_one(entries: &[Entry]) -> i32 {
    let outputs = entries.iter().map(|e| e.output.clone()).collect::<Vec<_>>();

    let mut count = 0;

    outputs.iter().for_each(|e| {
        let unique_digit_count = e.iter().fold(0, |mut s, c| {
            match c.len() {
                // 1
                2 => s += 1,
                // 4
                4 => s += 1,
                // 7
                3 => s += 1,
                // 8
                7 => s += 1,
                _ => {}
            }
            s
        });
        count += unique_digit_count;
    });

    count
}

#[aoc(day8, part2)]
pub fn part_two(entries: &[Entry]) -> i32 {
    let pattern_overlap = |pattern: &String, match_pattern: &String| -> usize {
        pattern
            .chars()
            .filter(|c| match_pattern.contains(&c.to_string()))
            .count()
    };

    let entries = entries.to_owned();

    let results = entries
        .into_iter()
        .map(|c| {
            // Determine patterns for one and four
            let mut patterns = c.patterns;
            patterns.sort_by_key(|l| l.len());
            let one_pattern = &patterns[0];
            let four_pattern = &patterns[2];

            // Sort output
            let output = c.output;

            let response = output.iter().fold(String::new(), |mut s, pattern| {
                let result = match pattern.len() {
                    // 2 Segments => 1
                    2 => 1,
                    // 3 Segemnts => 7
                    3 => 7,
                    // 4 Segments => 4
                    4 => 4,
                    // 7 Segments => 8
                    7 => 8,
                    // 5 Segments => 2 | 3 | 5
                    5 => {
                        // Needs to match one pattern entirely
                        if pattern_overlap(pattern, one_pattern) == 2 {
                            3
                        }
                        // Needs to match four pattern exactly twice
                        else if pattern_overlap(pattern, four_pattern) == 2 {
                            2
                        } else {
                            5
                        }
                    }
                    // 6 Segments => 0 | 6 | 9
                    6 => {
                        // Needs to match one pattern exactly one time
                        if pattern_overlap(pattern, one_pattern) == 1 {
                            6
                        }
                        // Needs to match four pattern entirely
                        else if pattern_overlap(pattern, four_pattern) == 4 {
                            9
                        } else {
                            0
                        }
                    }
                    _ => unreachable!("This pattern length should not exist"),
                };
                s.push_str(&result.to_string());
                s
            });

            response.parse::<i32>().unwrap()
        })
        .collect::<Vec<_>>();

    results.iter().sum::<i32>()
}
