use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part_one(measurements: &[u32]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day1::part_one;
    const INPUT: &[u32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn sample1() {
       assert_eq!(part_one(INPUT), 7);
    }
}
