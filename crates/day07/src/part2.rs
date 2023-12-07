use std::collections::HashMap;

use crate::{model::HandType, parser, solver};

pub fn solve(input: &String) {
    let mut game_vec = parser::parse(input, hand_type_from_str);

    let result = solver::solve(&mut game_vec, num_from_card);

    println!("Part 2 solution: {}", result);
}

fn hand_type_from_str(hand: &str) -> HandType {
    let mut map = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let len = map.len();

    if len > 1 {
        if let Some(val) = map.get(&'J') {
            let max_key = map
                .iter()
                .filter(|(key, _)| *key != &'J')
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(key, _)| key)
                .unwrap()
                .clone();

            *map.get_mut(&max_key).unwrap() += val.clone();

            map.remove(&'J');
        }
    }

    let len = map.len();
    let max_len = map.iter().map(|(_, val)| val).max().unwrap();

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
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}
