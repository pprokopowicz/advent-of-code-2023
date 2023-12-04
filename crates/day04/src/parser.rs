use reader::{read_file, File};

use crate::model::Card;

pub fn parse() -> Vec<Card> {
    let contents = read_file(File::Day04);

    contents
        .lines()
        .map(|line| {
            let sliced: &str = &line[10..line.len()];
            let split_by_line = sliced.split("|").collect::<Vec<&str>>();

            let winning_numbers = split_by_line[0]
                .split_whitespace()
                .filter_map(|num| {
                    if num.is_empty() {
                        return Option::None;
                    }

                    Option::Some(num.parse::<usize>().unwrap())
                })
                .collect::<Vec<usize>>();

            let numbers = split_by_line[1]
                .split_whitespace()
                .filter_map(|num| {
                    if num.is_empty() {
                        return Option::None;
                    }

                    Option::Some(num.parse::<usize>().unwrap())
                })
                .collect::<Vec<usize>>();

            Card::new(winning_numbers, numbers)
        })
        .collect::<Vec<Card>>()
}
