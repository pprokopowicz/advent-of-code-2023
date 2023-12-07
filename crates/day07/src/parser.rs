use std::collections::HashMap;

use reader::{read_file, File};

use crate::model::{Game, HandType};

pub fn parse() -> Vec<Game> {
    let contents = read_file(File::Day07);

    let result = contents
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand_str = split.next().unwrap();

            let bid = split.next().unwrap().parse::<usize>().unwrap();
            let hand = hand_type_from_str(hand_str);

            Game::new(hand, hand_str.to_string(), bid)
        })
        .collect::<Vec<Game>>();

    result
}

fn hand_type_from_str(hand: &str) -> HandType {
    let map = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let max_len = map.iter().map(|(_, val)| val).max().unwrap();
    let len = map.len();

    match (len, max_len) {
        (1, _) => HandType::FiveOfAKind,
        (2, 4) => HandType::FourOfAKind,
        (2, 3) => HandType::FullHouse,
        (3, 3) => HandType::ThreeOfAKind,
        (3, 2) => HandType::TwoPair,
        (4, 2) => HandType::Pair,
        _ => HandType::HighCard,
    }
}
