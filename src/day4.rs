use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> BingoGame {
    let mut lines = input.lines().collect::<Vec<_>>();
    let lucky_numbers = lines
        .remove(0)
        .split(",")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let bingo_rows = lines
        .into_iter()
        .filter(|l| *l != "")
        .map(|l| {
            l.split(" ")
                .filter(|c| *c != "")
                .map(|c| {
                    BingoField {
                        marked: false,
                        value: c.parse::<u32>().unwrap(),
                    }
                })
                .collect::<Vec<BingoField>>()
        })
        .collect::<Vec<_>>();

    let boards = bingo_rows.chunks(5).map(|f| f.to_owned()).collect::<Vec<_>>();

    BingoGame {
        lucky_numbers,
        boards,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BingoField {
    marked: bool,
    value: u32,
}

#[derive(Clone, Debug)]
pub struct BingoGame {
    lucky_numbers: Vec<u32>,
    boards: Vec<Vec<Vec<BingoField>>>,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl BingoGame {
    fn mark_fields(&mut self, lucky_number: u32) {
        self.boards = self.boards.iter().map(|b| b.iter().map(|r| r.iter().map(|f|
            {
                if lucky_number == f.value {
                    return BingoField {
                        value: lucky_number,
                        marked: true,
                    };
                }
                return *f;
            }
        ).collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();
    }

    fn calculate_score(&self, board_index: usize) -> u32 {
        let winner_board = self.boards.get(board_index).unwrap();

        winner_board.iter().fold(0, |sr, r| {
            sr + r.iter().fold(0, |sf, f| {
                if !f.marked {
                    return sf + f.value;
                }
                sf
            })
        })
    }

    fn find_winner(&self) -> Option<usize> {
        let mut winner_index: Option<usize> = None;

        'outer: for board_index in 0..self.boards.len() {
            let board_default = &self.boards[board_index];
            let board_transposed = transpose::<BingoField>(board_default.clone());

            let filter_marked = |f: &&BingoField| f.marked;

            '_inner: for index in 0..5 {
                let marked_rows = board_default[index].iter().filter(filter_marked).collect::<Vec<_>>();

                if marked_rows.len() == 5 {
                    winner_index = Some(board_index);
                    break 'outer;
                }

                let marked_cols = board_transposed[index].iter().filter(filter_marked).collect::<Vec<_>>();

                if marked_cols.len() == 5 {
                    winner_index = Some(board_index);
                    break 'outer;
                }
            }
        }

        winner_index
    }

    fn find_all_winners(&self) -> HashSet<usize> {
        let mut round_winners: HashSet<usize> = HashSet::new();

        for board_index in 0..self.boards.len() {
            let board_default = &self.boards[board_index];
            let board_transposed = transpose::<BingoField>(board_default.clone());

            let filter_marked = |f: &&BingoField| f.marked;

            '_inner: for index in 0..5 {
                let marked_rows = board_default[index].iter().filter(filter_marked).collect::<Vec<_>>();

                if marked_rows.len() == 5 {
                    round_winners.insert(board_index);
                    break '_inner;
                }

                let marked_cols = board_transposed[index].iter().filter(filter_marked).collect::<Vec<_>>();

                if marked_cols.len() == 5 {
                    round_winners.insert(board_index);
                    break '_inner;
                }
            }
        }

        round_winners
    }

    pub fn draw(&mut self, lucky_number: u32) -> Option<usize> {
        self.mark_fields(lucky_number);
        self.find_winner()
    }

    pub fn draw_part_two(&mut self, lucky_number: u32) -> HashSet<usize> {
        self.mark_fields(lucky_number);
        self.find_all_winners()
    }
}

#[aoc(day4, part1)]
pub fn part_one(game: &BingoGame) -> Option<u32> {
    let mut squid_game = game.clone();
    let lucky_numbers = squid_game.lucky_numbers.clone();

    let mut final_score: Option<u32> = None;

    for ln in lucky_numbers {
        let draw_result = squid_game.draw(ln);

        if let Some(bingo_board_index) = draw_result {
            let board_score = squid_game.calculate_score(bingo_board_index);
            final_score = Some(board_score * ln);
            break;
        }
    }

    final_score
}

#[aoc(day4, part2)]
pub fn part_two(game: &BingoGame) -> Option<u32> {
    let mut squid_game = game.clone();
    let lucky_numbers = squid_game.lucky_numbers.clone();

    let mut final_score: Option<u32> = None;
    let mut prev_result: Option<HashSet<usize>> = None;

    for ln in lucky_numbers {
        let draw_result = squid_game.draw_part_two(ln);

        if draw_result.len() == squid_game.boards.len() {
            if let Some(pr) = prev_result {
                let mut result = draw_result.difference(&pr);

                let score = squid_game.calculate_score(result.nth(0).unwrap().clone());
                final_score = Some(score * ln);
            }

            break;
        }

        prev_result = Some(draw_result);
    }

    final_score
}
