use std::cmp::Ordering;

use crate::model::{Game, HandType};

pub fn solve<F>(games: &mut Vec<Game>, num_from_card: F) -> usize
where
    F: Fn(&char) -> usize,
{
    games.sort_by(|game0, game1| {
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

    let result = games
        .iter()
        .enumerate()
        .map(|(index, game)| (index + 1) * game.bid)
        .sum::<usize>();

    result
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
