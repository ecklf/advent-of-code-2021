use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct OctopusMap {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

pub enum Adjacent {
    Up,
    UpLeft,
    UpRight,
    Down,
    DownLeft,
    DownRight,
    Left,
    Right,
}

impl OctopusMap {
    pub fn get_value_at(&self, point: &(usize, usize)) -> u32 {
        self.data[point.1][point.0]
    }

    pub fn get_adjacent_point(&self, point: &(usize, usize), adjacent: Adjacent) -> (usize, usize) {
        match adjacent {
            Adjacent::Up => (point.0, point.1 - 1),

            Adjacent::UpLeft => (point.0 - 1, point.1 - 1),
            Adjacent::UpRight => (point.0 + 1, point.1 - 1),

            Adjacent::Down => (point.0, point.1 + 1),

            Adjacent::DownLeft => (point.0 - 1, point.1 + 1),
            Adjacent::DownRight => (point.0 + 1, point.1 + 1),

            Adjacent::Left => (point.0 - 1, point.1),
            Adjacent::Right => (point.0 + 1, point.1),
        }
    }

    pub fn get_adjacent_points(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        let max_x = self.width - 1;
        let max_y = self.height - 1;

        // Corner points need 2 checks
        if point == &(0, 0) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Down),
                self.get_adjacent_point(point, Adjacent::DownRight),
            ];
        }
        if point == &(0, max_y) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::UpRight),
            ];
        }
        if point == &(max_x, 0) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Down),
                self.get_adjacent_point(point, Adjacent::DownLeft),
            ];
        }
        if point == &(max_x, max_y) {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::UpLeft),
            ];
        }

        // Edge points need 3 checks
        if point.0 == 0 {
            return vec![
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::UpRight),
                self.get_adjacent_point(point, Adjacent::Down),
                self.get_adjacent_point(point, Adjacent::DownRight),
            ];
        }
        if point.0 == max_x {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::UpLeft),
                self.get_adjacent_point(point, Adjacent::Down),
                self.get_adjacent_point(point, Adjacent::DownLeft),
            ];
        }
        if point.1 == 0 {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Down),
                self.get_adjacent_point(point, Adjacent::DownLeft),
                self.get_adjacent_point(point, Adjacent::DownRight),
            ];
        }
        if point.1 == max_y {
            return vec![
                self.get_adjacent_point(point, Adjacent::Left),
                self.get_adjacent_point(point, Adjacent::Right),
                self.get_adjacent_point(point, Adjacent::Up),
                self.get_adjacent_point(point, Adjacent::UpLeft),
                self.get_adjacent_point(point, Adjacent::UpRight),
            ];
        }

        // All other points need 4 checks
        return vec![
            self.get_adjacent_point(point, Adjacent::Left),
            self.get_adjacent_point(point, Adjacent::Right),
            self.get_adjacent_point(point, Adjacent::Up),
            self.get_adjacent_point(point, Adjacent::UpLeft),
            self.get_adjacent_point(point, Adjacent::UpRight),
            self.get_adjacent_point(point, Adjacent::Down),
            self.get_adjacent_point(point, Adjacent::DownLeft),
            self.get_adjacent_point(point, Adjacent::DownRight),
        ];
    }

    pub fn run_lights(&mut self, mut count: usize) -> usize {
        let greater_nine = (0..self.height).into_iter().map(|y| {
            (0..self.width).into_iter().filter(|x| {
                self.get_value_at(&(*x, y)) > 9
            }).count()
        }).sum::<usize>();

        if greater_nine == 0 {
            return count;
        }

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                if self.get_value_at(&(x, y)) > 9 {
                    self.get_adjacent_points(&(x, y)).into_iter().for_each(|ap| {
                        if self.get_value_at(&ap) != 0 {
                            self.data[ap.1][ap.0] += 1;
                        }
                    });
                    count += 1;
                    self.data[y][x] = 0;
                }
            })
        });

        self.run_lights(count)
    }

    pub fn print_map(&self) {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                print!("{}\t", self.data[y][x]);
            });
            println!();
        });
    }

    pub fn run_step(&mut self) -> usize {
        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                self.data[y][x] += 1;
            })
        });

        self.run_lights(0)
    }
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> OctopusMap {
    let data = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = data[0].len();
    let height = data.len();

    OctopusMap {
        data,
        width,
        height,
    }
}

#[aoc(day11, part1)]
pub fn part_one(octopus_map: &OctopusMap) -> usize {
    let mut octopus_map = octopus_map.to_owned();

    (1..=100).fold(0, |mut s, _| {
        s += octopus_map.run_step();
        s
    })
}

#[aoc(day11, part2)]
pub fn part_two(octopus_map: &OctopusMap) -> usize {
    let mut octopus_map = octopus_map.to_owned();
    let mut step_count: usize = 0;

    loop {
        step_count += 1;
        let flash_count = octopus_map.run_step();

        if flash_count == (octopus_map.width * octopus_map.height) {
            break;
        }
    }

    step_count
}