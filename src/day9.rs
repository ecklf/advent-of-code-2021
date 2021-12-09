use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

pub enum Adjacent {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    pub fn get_value_at(&self, point: &(usize, usize)) -> u32 {
        self.data[point.1][point.0]
    }

    pub fn get_adjacent_point(&self, point: &(usize, usize), adjacent: Adjacent) -> (usize, usize) {
        match adjacent {
            Adjacent::Up => (point.0, point.1 - 1),
            Adjacent::Down => (point.0, point.1 + 1),
            Adjacent::Left => (point.0 - 1, point.1),
            Adjacent::Right => (point.0 + 1, point.1),
        }
    }

    pub fn get_adjacent_points(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        let max_x = self.width - 1 as usize;
        let max_y = self.height - 1 as usize;

        // Corner points need 2 checks
        if point == &(0, 0) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Down),
            ];
        }
        if point == &(0, max_y) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
            ];
        }
        if point == &(max_x, 0) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Down),
            ];
        }
        if point == &(max_x, max_y) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Up),
            ];
        }

        // Edge points need 3 checks
        if point.0 == 0 {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::Down),
            ];
        }
        if point.0 == max_x {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::Down),
            ];
        }
        if point.1 == 0 {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Down),
            ];
        }
        if point.1 == max_y {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
            ];
        }

        // All other points need 4 checks
        return vec![
            self.get_adjacent_point(point, Adjacent::Left),
            self.get_adjacent_point(point, Adjacent::Right),
            self.get_adjacent_point(point, Adjacent::Up),
            self.get_adjacent_point(point, Adjacent::Down),
        ];
    }

    pub fn get_low_points(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .fold(Vec::<(usize, usize)>::new(), |mut s, (y, r)| {
                r.iter().enumerate().for_each(|(x, _c)| {
                    let is_low_point = self
                        .get_adjacent_points(&(x, y))
                        .into_iter()
                        .filter(|ap| self.get_value_at(&(x, y)) >= self.get_value_at(ap))
                        .collect::<Vec<_>>()
                        .len()
                        == 0;

                    if is_low_point {
                        s.push((x, y));
                    }
                });
                s
            })
    }

    pub fn get_basin_size(&self, root: &(usize, usize)) -> usize {
        let rising_from = |p: &(usize, usize)| -> Vec<(usize, usize)> {
            self.get_adjacent_points(p)
                .into_iter()
                .filter(|ap| {
                    self.get_value_at(p) < self.get_value_at(ap) && self.get_value_at(ap) != 9
                })
                .collect::<Vec<_>>()
        };

        let mut visited = HashSet::<(usize, usize)>::new();
        let mut queue = VecDeque::<(usize, usize)>::new();
        queue.push_back(root.clone());

        while queue.len() != 0 {
            let queue_item = queue.pop_front().unwrap();
            visited.insert(queue_item);

            let rising_points = rising_from(&queue_item);
            rising_points.into_iter().for_each(|rp| {
                queue.push_back(rp);
            });
        }

        visited.len()
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> HeightMap {
    let data = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = data[0].len().clone();
    let height = data.len().clone();

    HeightMap {
        data,
        width,
        height,
    }
}

#[aoc(day9, part1)]
pub fn part_one(height_map: &HeightMap) -> u32 {
    let low_points = height_map.get_low_points();
    low_points
        .iter()
        .fold(0, |s, lp| s + (height_map.get_value_at(lp) + 1))
}

#[aoc(day9, part2)]
pub fn part_two(height_map: &HeightMap) -> usize {
    let low_points = height_map.get_low_points();

    let mut basin_sizes = low_points
        .iter()
        .map(|lp| height_map.get_basin_size(lp))
        .collect::<Vec<_>>();

    basin_sizes.sort_by(|l, r| r.cmp(l));
    let largest_three = basin_sizes.into_iter().take(3).collect::<Vec<_>>();

    largest_three.into_iter().reduce(|l, r| l * r).unwrap()
}
