use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Vec<i32> {
    let line = input.lines().take(1).collect::<Vec<_>>();
    line[0]
        .split(',')
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(day7, part1)]
pub fn part_one(positions: &[i32]) -> i32 {
    let mut positions = positions.to_owned();
    positions.sort_unstable();
    let median = positions.get(positions.len() / 2).unwrap();
    positions.iter().fold(0, |mut s, x| {
        s += (x - median).abs();
        s
    })
}

#[aoc(day7, part2)]
pub fn part_two(positions: &[i32]) -> i32 {
    let fuel_cost = |positions: &[i32], target: i32| -> i32 {
        positions
            .iter()
            .map(|&p| {
                let d = (p - target).abs();
                d * (d + 1) / 2
            })
            .sum()
    };

    let positions = positions.to_owned();
    let mean = positions.iter().sum::<i32>() / positions.len() as i32;
    (-1..=1)
        .map(|d| (fuel_cost(&positions, mean + d)))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day7::{part_one, part_two};

    const INPUT: &[i32] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 37);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 168);
    }
}
