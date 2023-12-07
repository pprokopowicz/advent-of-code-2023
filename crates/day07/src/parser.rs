use std::{cmp::Ordering, collections::HashMap};

use reader::{read_file, File};

use crate::model::{Game, HandType};

pub fn parse<F>(contents: &String, mapper: F) -> Vec<Game>
where
    F: Fn(&str) -> HandType,
{
    let result = contents
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand_str = split.next().unwrap();

            let bid = split.next().unwrap().parse::<usize>().unwrap();
            let hand = mapper(hand_str);

            Game::new(hand, hand_str.to_string(), bid)
        })
        .collect::<Vec<Game>>();

    result
}
