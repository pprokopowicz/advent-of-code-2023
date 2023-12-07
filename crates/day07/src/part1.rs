use std::{cmp::Ordering, collections::HashMap};

use crate::{model::HandType, parser};

pub fn solve(input: &String) {
    let mut game_vec = parser::parse(input, hand_type_from_str);

    game_vec.sort_by(|game0, game1| {
        let order = num_from_hand(&game0.hand_type).cmp(&num_from_hand(&game1.hand_type));

        if order != Ordering::Equal {
            return order;
        }

        let chars0 = game0.hand.chars().collect::<Vec<char>>();
        let chars1 = game1.hand.chars().collect::<Vec<char>>();

        let count = chars0.len();

        for index in 0..count {
            if chars0[index] == chars1[index] {
                continue;
            }

            return num_from_card(&chars0[index]).cmp(&num_from_card(&chars1[index]));
        }

        Ordering::Equal
    });

    let result = game_vec
        .iter()
        .enumerate()
        .map(|(index, game)| (index + 1) * game.bid)
        .sum::<usize>();

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

fn num_from_hand(hand: &HandType) -> usize {
    match hand {
        HandType::FiveOfAKind => 7,
        HandType::FourOfAKind => 6,
        HandType::FullHouse => 5,
        HandType::ThreeOfAKind => 4,
        HandType::TwoPair => 3,
        HandType::Pair => 2,
        HandType::HighCard => 1,
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
