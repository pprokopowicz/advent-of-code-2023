use std::cmp::Ordering;

use crate::model::{Game, HandType};

pub fn solve(input: &Vec<Game>) {
    let mut mut_input = input.to_vec();

    mut_input.sort_by(|game0, game1| {
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

    let result = mut_input
        .iter()
        .enumerate()
        .map(|(index, game)| (index + 1) * game.bid)
        .sum::<usize>();

    println!("Part 1 solution: {}", result);
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
