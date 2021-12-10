use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<u64> {
    let line = input.lines().take(1).collect::<Vec<_>>();
    line[0]
        .split(',')
        .map(|f| f.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn simulate_spawns(initial: &[u64], days: usize) -> usize {
    let mut timers: [usize; 9] = [0; 9];
    initial.iter().for_each(|x| timers[*x as usize] += 1);

    (0..days).for_each(|_| {
        let spawn_amount = timers[0];
        (0..8).for_each(|i| timers[i] = timers[i + 1]);
        timers[6] += spawn_amount;
        timers[8] = spawn_amount;
    });
    timers.iter().sum()
}

#[aoc(day6, part1)]
pub fn part_one(fish_timers: &[u64]) -> usize {
    simulate_spawns(fish_timers, 80)
}

#[aoc(day6, part2)]
pub fn part_two(fish_timers: &[u64]) -> usize {
    simulate_spawns(fish_timers, 256)
}

#[cfg(test)]
mod tests {
    use crate::day6::simulate_spawns;

    const INPUT: &[u64] = &[3, 4, 3, 1, 2];

    #[test]
    fn test_part_one() {
        assert_eq!(simulate_spawns(INPUT, 18), 26);
        assert_eq!(simulate_spawns(INPUT, 80), 5934);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(simulate_spawns(INPUT, 256), 26984457539);
    }
}
