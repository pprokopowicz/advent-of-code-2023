#[derive(Debug)]
pub struct Card {
    pub winning_numbers: Vec<usize>,
    pub numbers: Vec<usize>,
}

impl Card {
    pub fn new(winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Card {
        Card {
            winning_numbers,
            numbers,
        }
    }
}
