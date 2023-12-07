#[derive(Clone, Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub hand_type: HandType,
    pub hand: String,
    pub bid: usize,
}

impl Game {
    pub fn new(hand_type: HandType, hand: String, bid: usize) -> Game {
        Game {
            hand_type,
            hand,
            bid,
        }
    }
}
