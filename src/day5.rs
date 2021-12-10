use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::RangeInclusive;

const OVERLAP: u32 = 2;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn get_x_range(&self, reverse: bool) -> RangeInclusive<i32> {
        match reverse {
            false => self.start.x..=self.end.x,
            true => self.end.x..=self.start.x,
        }
    }

    pub fn get_y_range(&self, reverse: bool) -> RangeInclusive<i32> {
        match reverse {
            false => self.start.y..=self.end.y,
            true => self.end.y..=self.start.y,
        }
    }

    pub fn pitch(&self) -> Option<i32> {
        if self.end.x == self.start.x {
            return None;
        }

        let dy = self.end.y as i32 - self.start.y as i32;
        let dx = self.end.x as i32 - self.start.x as i32;

        Some(dy / dx)
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .into_iter()
        .map(|l| {
            let points = l
                .split(" -> ")
                .collect::<Vec<_>>()
                .into_iter()
                .map(|p| {
                    let p = p
                        .split(',')
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|p| p.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    Point { x: p[0], y: p[1] }
                })
                .collect::<Vec<_>>();

            Line {
                start: points[0],
                end: points[1],
            }
        })
        .collect::<Vec<Line>>()
}

pub fn filter_overlaps(lines: &[Line], overlap_count: u32, with_diagonals: bool) -> usize {
    let mut vents_map = HashMap::<(i32, i32), u32>::new();

    let filtered_lines = lines.iter().filter(|l| match l.pitch() {
        None => true,
        Some(v) => match v {
            0 => true,
            1 | -1 => with_diagonals,
            _ => false,
        },
    });

    let mut upsert_vents_map = |x: i32, y: i32| match vents_map.get_mut(&(x, y)) {
        Some(m) => *m += 1,
        None => {
            vents_map.insert((x, y), 1);
        }
    };

    filtered_lines.into_iter().for_each(|l| match l.pitch() {
        None => {
            let x = l.start.x;
            let reverse = l.start.y > l.end.y;
            let y_range = l.get_y_range(reverse);

            y_range.for_each(|y| {
                upsert_vents_map(x, y);
            });
        }
        Some(0) => {
            let y = l.start.y;
            let reverse = l.start.x > l.end.x;
            let x_range = l.get_x_range(reverse);

            x_range.for_each(|x| {
                upsert_vents_map(x, y);
            });
        }
        Some(p) => {
            let reverse = l.start.x > l.end.x;
            let mut y = match reverse {
                false => l.start.y,
                true => l.end.y,
            };

            let x_range = match reverse {
                false => l.start.x..=l.end.x,
                true => l.end.x..=l.start.x,
            };

            x_range.for_each(|x| {
                upsert_vents_map(x, y);
                y += p;
            });
        }
    });

    vents_map
        .into_iter()
        .filter(|(_, val)| val >= &overlap_count)
        .count()
}

#[aoc(day5, part1)]
pub fn part_one(input: &[Line]) -> usize {
    filter_overlaps(input, OVERLAP, false)
}

#[aoc(day5, part2)]
pub fn part_two(input: &[Line]) -> usize {
    filter_overlaps(input, OVERLAP, true)
}
