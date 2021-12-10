use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<u32>> {
    const RADIX: u32 = 10;
    let to_u32 = |s: &str| {
        s.chars()
            .map(|c| c.to_digit(RADIX).unwrap())
            .collect::<Vec<u32>>()
    };
    input.lines().map(to_u32).collect::<Vec<Vec<u32>>>()
}

pub fn bits_to_dec(bits: &[u32]) -> u32 {
    let binary_str = bits
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("");
    u32::from_str_radix(&binary_str, 2).unwrap()
}

pub fn col_sums(input: &[Vec<u32>]) -> Vec<u32> {
    input
        .to_owned()
        .into_iter()
        .reduce(|prev, curr| prev.iter().enumerate().map(|(i, v)| curr[i] + v).collect())
        .unwrap()
}

#[aoc(day3, part1)]
pub fn part_one(input: &[Vec<u32>]) -> u32 {
    let col_sums = col_sums(input);
    let report_length = input.len() as u32;
    let bits_count = input[0].len() as u32;

    let bits = col_sums
        .into_iter()
        .map(|cs| {
            let one_bits_count = cs;
            let zero_bits_count = report_length - cs;
            match one_bits_count >= zero_bits_count {
                true => 1,
                false => 0,
            }
        })
        .collect::<Vec<u32>>();

    let gamma_rate = bits_to_dec(&bits);

    let mask = (1 << bits_count) - 1;
    // Bitwise NOT on gamma rate, bitwise shift n = bits_count
    let epsilon_rate = !gamma_rate & mask;

    gamma_rate * epsilon_rate
}

pub fn rating(input: Vec<Vec<u32>>, index: usize, inverse: bool) -> Vec<u32> {
    let report_length = input.len() as u32;

    if report_length == 1 {
        return input.get(0).unwrap().to_owned();
    }

    let col_sums = col_sums(&input);
    let one_bits_count = col_sums.get(index).unwrap().to_owned();
    let zero_bits_count = report_length - col_sums.get(index).unwrap();

    let search_bit: u32 = match inverse {
        true => match one_bits_count < zero_bits_count {
            true => 1,
            false => 0,
        },
        false => match one_bits_count >= zero_bits_count {
            true => 1,
            false => 0,
        },
    };

    rating(
        input
            .into_iter()
            .filter(|r| r[index] == search_bit)
            .collect::<Vec<_>>(),
        index + 1,
        inverse,
    )
}

#[aoc(day3, part2)]
pub fn part_two(input: &[Vec<u32>]) -> u32 {
    let o2_rating = bits_to_dec(&rating(input.to_owned(), 0, false));
    let co2_rating = bits_to_dec(&rating(input.to_owned(), 0, true));
    o2_rating * co2_rating
}
