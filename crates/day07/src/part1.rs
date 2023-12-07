use std::collections::HashMap;

use crate::{model::HandType, parser, solver};

pub fn solve(input: &String) {
    let mut game_vec = parser::parse(input, hand_type_from_str);

    let result = solver::solve(&mut game_vec, num_from_card);

    println!("Part 1 solution: {}", result);
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

fn num_from_card(card: &char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0,
    }
}
