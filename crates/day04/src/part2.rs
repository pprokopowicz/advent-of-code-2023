use crate::model::Card;

pub fn solve(input: &Vec<Card>) {
    let mut card_amounts: Vec<usize> = vec![1; input.len()];

    input.iter().enumerate().for_each(|(index, card)| {
        let amount_of_winning_nums = card
            .winning_numbers
            .iter()
            .map(|num| if card.numbers.contains(num) { 1 } else { 0 })
            .sum::<usize>();
        let amount_of_cards = card_amounts[index];

        for i in (index + 1)..=(index + amount_of_winning_nums) {
            card_amounts[i] += amount_of_cards;
        }
    });

    let result = card_amounts.iter().sum::<usize>();

    println!("Part 2 solution: {:?}", result);
}
