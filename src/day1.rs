use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn increasing_window_sum_count(measurements: &[u32], size: usize) -> usize {
    measurements
        .windows(size + 1)
        .filter(|blob| blob[0] < blob[size])
        .count()
}

#[aoc(day1, part1)]
pub fn part_one(measurements: &[u32]) -> usize {
    increasing_window_sum_count(&measurements, 1)
}

#[aoc(day1, part2)]
pub fn part_two(measurements: &[u32]) -> usize {
    // A + B + C < B + C + D => A < D
    increasing_window_sum_count(&measurements, 3)
}

#[cfg(test)]
mod tests {
    use crate::day1::{part_one, part_two};

    const INPUT: &[u32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 7);
    }

    const INPUT2: &[u32] = &[607, 618, 618, 617, 647, 716, 769, 792];

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT2), 5);
    }
}
