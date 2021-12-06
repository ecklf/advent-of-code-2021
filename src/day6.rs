use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<u64> {
    let line = input.lines().take(1).collect::<Vec<_>>();
    line[0].split(",").map(|f| f.parse::<u64>().unwrap()).collect::<Vec<u64>>()
}

#[aoc(day6, part1)]
pub fn part_one(fish_timers: &[u64]) -> usize {
    let mut fish = fish_timers.to_owned();
    let range = 1..=80;

    for x in range {
        let mut new_fish = 0;

        fish.iter_mut().for_each(|f| {
            if *f == 0 {
                *f = 6;
                new_fish += 1;
            } else {
                *f -= 1;
            }
        });

        for _ in 0..new_fish {
            fish.push(8)
        }
    }
    fish.len()
}